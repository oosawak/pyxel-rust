//! Rust callback layer for the official Node scene graph and lifecycle kernels.
//! Each NodeScene owns callback registrations; RcNode remains the official data type.
use crate::cube::{
    draw::{self, DrawState},
    mesh::Material,
    scene::{DrawContext, Scene},
    Mat4, Node, Primitive, RcContact, RcMesh, RcNode, RcPrimitive,
};
use crate::RcImage;
use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};
type UpdateHook = Rc<RefCell<dyn FnMut(&mut NodeScene, &RcNode) -> Result<(), String>>>;
type DrawHook =
    Rc<RefCell<dyn FnMut(&mut NodeScene, &RcNode, &mut DrawContext) -> Result<(), String>>>;
type CollideHook =
    Rc<RefCell<dyn FnMut(&mut NodeScene, &RcNode, &RcNode, &RcContact) -> Result<(), String>>>;
#[derive(Default)]
struct Record {
    node: Weak<RefCell<Node>>,
    update: Option<UpdateHook>,
    draw: Option<DrawHook>,
    collide: Option<CollideHook>,
    destroy: Option<UpdateHook>,
    drawable: Option<(RcPrimitive, Material)>,
}
struct Busy(Rc<Cell<bool>>);
impl Busy {
    fn enter(flag: &Rc<Cell<bool>>) -> Result<Self, String> {
        if flag.replace(true) {
            return Err("Recursive NodeScene update/draw is not supported".into());
        }
        Ok(Self(flag.clone()))
    }
}
impl Drop for Busy {
    fn drop(&mut self) {
        self.0.set(false);
    }
}
/// The graph is public so existing Node hierarchy, collider and spatial-query APIs apply.
/// Register Rust closures instead of subclassing Python Node. A callback can replace
/// its own registration; dispatch never holds a borrow of the registration map.
pub struct NodeScene {
    pub root: RcNode,
    records: HashMap<usize, Record>,
    busy: Rc<Cell<bool>>,
}
fn key(node: &RcNode) -> usize {
    Rc::as_ptr(node) as usize
}
fn subtree(root: &RcNode) -> Vec<RcNode> {
    let mut out = Vec::new();
    let mut stack = vec![root.clone()];
    let mut seen = HashSet::new();
    while let Some(node) = stack.pop() {
        if !seen.insert(key(&node)) {
            continue;
        }
        stack.extend(Node::children(&node).into_iter().rev());
        out.push(node);
    }
    out
}
fn contains(root: &RcNode, node: &RcNode) -> bool {
    let mut current = Some(node.clone());
    let mut seen = HashSet::new();
    while let Some(node) = current {
        if Rc::ptr_eq(&node, root) {
            return true;
        }
        if !seen.insert(key(&node)) {
            break;
        }
        current = Node::parent(&node);
    }
    false
}
impl NodeScene {
    pub fn new(root: RcNode) -> Self {
        Self {
            root,
            records: HashMap::new(),
            busy: Rc::new(Cell::new(false)),
        }
    }
    fn record(&mut self, node: &RcNode) -> &mut Record {
        let record = self.records.entry(key(node)).or_default();
        if record.node.upgrade().is_none() {
            *record = Record {
                node: Rc::downgrade(node),
                ..Record::default()
            };
        }
        record
    }
    /// Build the official placement root, named part nodes and parent hierarchy.
    /// Primitive/material handles and authored transforms follow official sharing rules.
    pub fn from_mesh(mesh: &RcMesh) -> Result<Self, String> {
        let mut scene = Self::new(Node::new());
        let root = scene.root.clone();
        scene.populate_mesh(&root, mesh)?;
        Ok(scene)
    }
    pub fn attach_mesh(&mut self, parent: &RcNode, mesh: &RcMesh) -> Result<RcNode, String> {
        mesh.borrow().validate()?;
        if parent.borrow().destroyed {
            return Err("Cannot attach to a destroyed node".into());
        }
        let root = Node::new();
        self.populate_mesh(&root, mesh)?;
        if !Node::add_child(parent, &root) {
            return Err("Cannot attach mesh root".into());
        }
        Ok(root)
    }
    fn populate_mesh(&mut self, root: &RcNode, mesh: &RcMesh) -> Result<(), String> {
        let mesh = mesh.borrow();
        mesh.validate()?;
        let nodes: Vec<_> = (0..mesh.primitives.len())
            .map(|i| {
                let node = Node::new();
                node.borrow_mut().name = mesh.names.get(i).cloned().unwrap_or_default();
                node.borrow_mut().transform = mesh.transforms[i].clone();
                if let Some(p) = &mesh.primitives[i] {
                    self.record(&node).drawable = Some((p.clone(), mesh.material_for_part(i)));
                }
                node
            })
            .collect();
        for (i, &parent) in mesh.parents.iter().enumerate() {
            Node::add_child(
                if parent < 0 {
                    root
                } else {
                    &nodes[parent as usize]
                },
                &nodes[i],
            );
        }
        Ok(())
    }
    pub fn on_update(
        &mut self,
        node: &RcNode,
        hook: impl FnMut(&mut Self, &RcNode) -> Result<(), String> + 'static,
    ) {
        self.record(node).update = Some(Rc::new(RefCell::new(hook)));
    }
    pub fn on_draw(
        &mut self,
        node: &RcNode,
        hook: impl FnMut(&mut Self, &RcNode, &mut DrawContext) -> Result<(), String> + 'static,
    ) {
        self.record(node).draw = Some(Rc::new(RefCell::new(hook)));
    }
    pub fn on_collide(
        &mut self,
        node: &RcNode,
        hook: impl FnMut(&mut Self, &RcNode, &RcNode, &RcContact) -> Result<(), String> + 'static,
    ) {
        self.record(node).collide = Some(Rc::new(RefCell::new(hook)));
    }
    pub fn on_destroy(
        &mut self,
        node: &RcNode,
        hook: impl FnMut(&mut Self, &RcNode) -> Result<(), String> + 'static,
    ) {
        self.record(node).destroy = Some(Rc::new(RefCell::new(hook)));
    }
    pub fn remove_callbacks(&mut self, node: &RcNode) {
        let r = self.record(node);
        r.update = None;
        r.draw = None;
        r.collide = None;
        r.destroy = None;
    }
    /// Whole-subtree phases: callbacks → core motion → contacts → destruction.
    /// Animation playback remains explicit through MeshAnimationPlayer.
    pub fn update(&mut self, frame_seconds: f32) -> Result<(), String> {
        if !frame_seconds.is_finite() || frame_seconds < 0. {
            return Err("Invalid frame time".into());
        }
        let _busy = Busy::enter(&self.busy)?;
        for node in subtree(&self.root) {
            if contains(&self.root, &node) && Node::effective_active(&node) {
                let hook = self.records.get(&key(&node)).and_then(|r| r.update.clone());
                if let Some(hook) = hook {
                    (hook.borrow_mut())(self, &node)?;
                }
            }
        }
        Scene::integrate_motion(&self.root, frame_seconds);
        for pair in Scene::detect_contacts(&self.root) {
            let a = self
                .records
                .get(&key(&pair.node_a))
                .and_then(|r| r.collide.clone());
            let b = self
                .records
                .get(&key(&pair.node_b))
                .and_then(|r| r.collide.clone());
            if let Some(hook) = a {
                (hook.borrow_mut())(self, &pair.node_a, &pair.node_b, &pair.contact_a)?;
            }
            if let Some(hook) = b {
                (hook.borrow_mut())(self, &pair.node_b, &pair.node_a, &pair.contact_b)?;
            }
        }
        let destroyed = Scene::collect_destroyed_post_order(&self.root);
        for node in &destroyed {
            if Scene::take_destroy_notification(node) {
                let hook = self.records.get(&key(node)).and_then(|r| r.destroy.clone());
                if let Some(hook) = hook {
                    (hook.borrow_mut())(self, node)?;
                }
            }
        }
        for node in &destroyed {
            Scene::detach_destroyed(node);
            self.records.remove(&key(node));
        }
        self.records.retain(|_, r| r.node.strong_count() > 0);
        Ok(())
    }
    /// Camera, visibility, world transforms, shading and opacity follow ancestors.
    /// Callbacks receive an explicit draw context; no Python thread-local context is used.
    pub fn draw(&mut self, viewport: [f32; 4], target: Option<&RcImage>) -> Result<(), String> {
        let _busy = Busy::enter(&self.busy)?;
        let camera =
            Node::effective_camera(&self.root).ok_or("Node draw requires an effective camera")?;
        let target = target.cloned().unwrap_or_else(super::resource::screen);
        super::graphics3d::render_3d_viewport(&target, &camera, viewport, |ctx| {
            for node in subtree(&self.root) {
                if !contains(&self.root, &node) || !Node::effective_visible(&node) {
                    continue;
                }
                ctx.dither_alpha = 1.;
                ctx.depth_test = true;
                ctx.depth_write = true;
                ctx.depth_offset = 0.;
                ctx.decal_distance = None;
                ctx.shaded = true;
                let hook = self.records.get(&key(&node)).and_then(|r| r.draw.clone());
                if let Some(hook) = hook {
                    (hook.borrow_mut())(self, &node, ctx)?;
                }
                let drawable = self
                    .records
                    .get(&key(&node))
                    .and_then(|r| r.drawable.clone());
                if let Some((primitive, material)) = drawable {
                    draw_node_primitive(ctx, &node, &primitive.borrow(), &material)?;
                }
            }
            Ok(())
        })?
    }
}
/// Issue a custom draw call with the node's inherited shading and opacity.
pub fn with_node_draw_state<R>(
    ctx: &mut DrawContext,
    node: &RcNode,
    f: impl FnOnce(&mut DrawContext, &Mat4, DrawState<'_>) -> R,
) -> R {
    let shading = Node::effective_shading(node);
    let shading = shading.as_ref().map(|s| s.borrow());
    let world = Node::world_transform_value(node);
    let state = DrawState {
        shaded: ctx.shaded,
        dither_alpha: ctx.dither_alpha * Node::effective_opacity(node),
        depth_test: ctx.depth_test,
        depth_write: ctx.depth_write,
        billboard: draw::BILLBOARD_OFF,
        shading: shading.as_deref(),
    };
    let alpha = ctx.dither_alpha;
    let result = f(ctx, &world, state);
    ctx.dither_alpha = alpha;
    result
}
pub fn draw_node_primitive(
    ctx: &mut DrawContext,
    node: &RcNode,
    p: &Primitive,
    material: &Material,
) -> Result<(), String> {
    let (color, image) = material.col_img.as_flat_and_image();
    with_node_draw_state(ctx, node, |ctx, world, state| {
        draw::prim(
            ctx,
            world,
            p.mode,
            p.cull,
            &p.positions,
            (!p.indices.is_empty()).then_some(p.indices.as_slice()),
            (!p.normals.is_empty()).then_some(p.normals.as_slice()),
            (!p.uvs.is_empty()).then_some(p.uvs.as_slice()),
            color,
            image.as_ref(),
            material.colkey,
            state,
        )
        .map_err(str::to_owned)
    })
}
