//! Adapted from Pyxel 3.0.0 pyxel-binding/src/cube/node.rs (MIT; see vendor/pyxel-cube-core/LICENSE).
//! Node-local playback following the official binding's frame-based phases.
use super::{key, subtree, NodeScene};
use crate::cube::{Mat4, Node, RcMesh, RcMotion, RcNode};
use std::rc::Rc;

pub(super) struct MeshInstance {
    source: RcMesh,
    rest: Vec<Option<Mat4>>,
}
impl MeshInstance {
    pub(super) fn new(source: &RcMesh) -> Result<Rc<Self>, String> {
        let mesh = source.borrow();
        mesh.validate()?;
        let mut rest = vec![None; mesh.primitives.len()];
        for motion in &mesh.motions {
            for (part, pose) in motion.borrow().animated_rest_pose() {
                if let Some(target) = rest.get_mut(part) {
                    *target = Some(pose);
                }
            }
        }
        Ok(Rc::new(Self {
            source: source.clone(),
            rest,
        }))
    }
}
#[derive(Clone)]
pub(super) struct MotionPlayer {
    motion: RcMotion,
    frame: f32,
    speed: f32,
    looping: bool,
}
impl MotionPlayer {
    fn finished(&self) -> bool {
        let length = self.motion.borrow().length;
        !self.looping
            && (length <= 0.
                || (self.speed >= 0. && self.frame >= length)
                || (self.speed < 0. && self.frame <= 0.))
    }
}
/// Snapshot of an active player. A stopped/finished player returns None;
/// stopping retains the last sampled node transforms.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeMotionState {
    pub name: String,
    pub frame: f32,
    pub speed: f32,
    pub looping: bool,
}

impl NodeScene {
    fn validate_motion(
        &self,
        node: &RcNode,
        motion: &RcMotion,
    ) -> Result<Rc<MeshInstance>, String> {
        let instance = self
            .records
            .get(&key(node))
            .and_then(|r| r.instance.clone())
            .ok_or("Node motion requires a tree created by from_mesh/attach_mesh")?;
        let mesh = instance.source.borrow();
        if !mesh.motions.iter().any(|m| Rc::ptr_eq(m, motion)) {
            return Err("Motion must belong to the same source Mesh as the node instance".into());
        }
        let clip = motion.borrow();
        if !clip.length.is_finite()
            || clip.length < 0.
            || clip
                .channels
                .iter()
                .any(|c| c.part_index >= instance.rest.len())
        {
            return Err("Invalid motion duration or target part".into());
        }
        drop(clip);
        drop(mesh);
        Ok(instance)
    }
    fn instance_nodes(&self, root: &RcNode, instance: &Rc<MeshInstance>) -> Vec<(RcNode, usize)> {
        subtree(root)
            .into_iter()
            .filter_map(|node| {
                let r = self.records.get(&key(&node))?;
                if !r.instance.as_ref().is_some_and(|i| Rc::ptr_eq(i, instance)) {
                    return None;
                }
                Some((node, r.part?))
            })
            .collect()
    }
    fn reset_motion_pose(&self, node: &RcNode, instance: &Rc<MeshInstance>) {
        for (node, part) in self.instance_nodes(node, instance) {
            if let Some(Some(pose)) = instance.rest.get(part) {
                node.borrow_mut().transform = Mat4::from_rows(pose.data);
            }
        }
    }
    fn sample_motion(
        &self,
        node: &RcNode,
        instance: &Rc<MeshInstance>,
        motion: &RcMotion,
        frame: f32,
        looping: bool,
    ) {
        let sampled: std::collections::HashMap<_, _> =
            motion.borrow().sample(frame, looping).into_iter().collect();
        for (node, part) in self.instance_nodes(node, instance) {
            if let Some(pose) = sampled.get(&part) {
                node.borrow_mut().transform = Mat4::from_rows(pose.data);
            }
        }
    }
    /// Sample only the motion's channels in this instance's requested subtree.
    /// Does not start/stop a player or reset other channels.
    pub fn apply_motion(
        &mut self,
        node: &RcNode,
        motion: &RcMotion,
        frame: f32,
        looping: bool,
    ) -> Result<(), String> {
        if !frame.is_finite() {
            return Err("Motion frame must be finite".into());
        }
        let instance = self.validate_motion(node, motion)?;
        self.sample_motion(node, &instance, motion, frame, looping);
        Ok(())
    }
    /// Start node-local playback in imported Pyxel frames. Negative/zero speeds
    /// are supported. All animated parts reset to their authored rest pose first.
    /// Each NodeScene::update advances by speed once, independent of physics dt.
    pub fn play_motion(
        &mut self,
        node: &RcNode,
        motion: &RcMotion,
        looping: bool,
        speed: f32,
        start_frame: f32,
    ) -> Result<(), String> {
        if !speed.is_finite() || !start_frame.is_finite() {
            return Err("Motion speed and start frame must be finite".into());
        }
        let instance = self.validate_motion(node, motion)?;
        let frame = if looping {
            motion.borrow().resolve_frame(start_frame, true)
        } else {
            start_frame
        };
        let player = MotionPlayer {
            motion: motion.clone(),
            frame,
            speed,
            looping,
        };
        self.reset_motion_pose(node, &instance);
        self.sample_motion(node, &instance, motion, frame, looping);
        self.record(node).player = (!player.finished()).then_some(player);
        Ok(())
    }
    pub fn stop_motion(&mut self, node: &RcNode) {
        if let Some(record) = self.records.get_mut(&key(node)) {
            record.player = None;
        }
    }
    pub fn motion_state(&self, node: &RcNode) -> Option<NodeMotionState> {
        let p = self.records.get(&key(node))?.player.as_ref()?;
        Some(NodeMotionState {
            name: p.motion.borrow().name.clone(),
            frame: p.frame,
            speed: p.speed,
            looping: p.looping,
        })
    }
    pub(super) fn advance_motions(&mut self) -> Result<(), String> {
        if Node::parent(&self.root).is_some_and(|parent| !Node::effective_active(&parent)) {
            return Ok(());
        }
        let mut stack = vec![self.root.clone()];
        while let Some(node) = stack.pop() {
            if !node.borrow().active {
                continue;
            }
            let playback = self
                .records
                .get(&key(&node))
                .and_then(|r| Some((r.player.clone()?, r.instance.clone()?)));
            if let Some((mut player, instance)) = playback {
                let clip = player.motion.borrow();
                if !clip.length.is_finite() || clip.length < 0. {
                    return Err("Invalid active motion duration".into());
                }
                let next = player.frame + player.speed;
                let next = if player.looping && !next.is_finite() && clip.length > 0. {
                    (f64::from(player.frame) + f64::from(player.speed))
                        .rem_euclid(f64::from(clip.length)) as f32
                } else {
                    next
                };
                player.frame = clip.resolve_frame(next, player.looping);
                drop(clip);
                self.sample_motion(
                    &node,
                    &instance,
                    &player.motion,
                    player.frame,
                    player.looping,
                );
                self.record(&node).player = (!player.finished()).then_some(player);
            }
            stack.extend(Node::children(&node).into_iter().rev());
        }
        Ok(())
    }
}
