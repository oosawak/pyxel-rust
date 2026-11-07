use super::*;
use crate::cube::{Quat, RcQuat};

pub struct CollisionDemo {
    pub frame: u32,
    root: RcNode,
    camera: RcCamera,
    shade: RcShading,
    pub(super) image: Surface,
    stage: Model,
    platform: Model,
    platform_start: Vec3,
    platform_end: Vec3,
    delta: Vec3,
    player: RcNode,
    actor: Model,
    start: Vec3,
    coins: Vec<(RcNode, Model)>,
    goal: RcNode,
    coins_taken: u32,
    on_floor: bool,
    floor_normal: Vec3,
    on_platform: bool,
    jumps: u32,
    move_input: (i32, i32),
    movement: Vec3,
    heading: f32,
    rotation: RcQuat,
    motion: Option<String>,
    motion_frame: f32,
    yaw: f32,
    target: Vec3,
    camera_offset: Option<Vec3>,
    lit: bool,
}
impl CollisionDemo {
    pub fn new() -> Result<Self, String> {
        let root = Node::new();
        let stage = Model::new(load("stage")?)?;
        stage.node.borrow_mut().collider = Some(collider(
            v(0., 0., 0.),
            0.,
            0.,
            Some(load("stage_collision")?),
            false,
            false,
        ));
        stage.node.borrow_mut().tags.insert("ground".into());
        Node::add_child(&root, &stage.node);
        let coin_mesh = load("coin")?;
        let markers = Node::children(&stage.find("Coins")?);
        let mut coins = vec![];
        for n in markers {
            let model = Model::new(coin_mesh.clone())?;
            Node::add_child(&n, &model.node);
            n.borrow_mut().collider = Some(collider(v(0., 0., 0.), 5., 0., None, true, false));
            n.borrow_mut().tags.insert("coin".into());
            coins.push((n, model));
        }
        let goal = stage.find("Goal")?;
        goal.borrow_mut().collider = Some(collider(v(24., 16., 24.), 0., 0., None, true, false));
        let platform_start = world(&stage.find("PlatformStart")?).pos_value();
        let platform_end = world(&stage.find("PlatformEnd")?).pos_value();
        let platform = Model::new(load("platform")?)?;
        set_mat(&platform.node, tr(platform_start));
        platform.node.borrow_mut().collider = Some(collider(
            v(0., 0., 0.),
            0.,
            0.,
            Some(load("platform_collision")?),
            false,
            false,
        ));
        platform.node.borrow_mut().tags.insert("ground".into());
        Node::add_child(&root, &platform.node);
        let start = add(world(&stage.find("Start")?).pos_value(), v(0., 8., 0.));
        let player = Node::new();
        let coll = collider(v(0., 0., 0.), 8., 1., None, false, false);
        {
            let mut c = coll.borrow_mut();
            c.gravity = 0.;
            c.linear_damp = 0.;
            c.angular_damp = 0.;
        }
        player.borrow_mut().collider = Some(coll);
        Node::add_child(&root, &player);
        let actor = Model::new(load("player")?)?;
        Node::add_child(&player, &actor.node);
        let camera = Camera::new();
        camera.borrow_mut().near = 2.;
        let shade = shading();
        shade.borrow_mut().direction = Vec3::new(-0.5, -1., -0.8);
        let mut d = Self {
            frame: 0,
            root,
            camera,
            shade,
            image: Surface::new(320, 240),
            stage,
            platform,
            platform_start,
            platform_end,
            delta: v(0., 0., 0.),
            player,
            actor,
            start,
            coins,
            goal,
            coins_taken: 0,
            on_floor: true,
            floor_normal: v(0., 1., 0.),
            on_platform: false,
            jumps: 0,
            move_input: (0, 0),
            movement: v(0., 0., 0.),
            heading: 180.,
            rotation: Quat::from_euler(&v(0., 180., 0.)),
            motion: None,
            motion_frame: 0.,
            yaw: 28.,
            target: start,
            camera_offset: None,
            lit: false,
        };
        d.reset_player();
        d.update_camera();
        Ok(d)
    }
    fn reset_player(&mut self) {
        set_mat(&self.player, tr(self.start));
        let c = self.player.borrow().collider.clone().unwrap();
        *c.borrow().velocity.borrow_mut() = v(0., 0., 0.);
        self.on_floor = true;
        self.floor_normal = v(0., 1., 0.);
        self.on_platform = false;
        self.jumps = 0;
        self.move_input = (0, 0);
        self.movement = v(0., 0., 0.);
        self.heading = 180.;
        self.rotation = Quat::from_euler(&v(0., 180., 0.));
        self.motion = None;
        self.yaw = 28.;
        self.target = self.start;
        self.camera_offset = None;
    }
    fn clip_camera(&self, focus: Vec3, offset: Vec3) -> Vec3 {
        Scene::raycast(
            &self.root,
            focus,
            offset,
            length(offset),
            false,
            Some(&["ground".into()]),
        )
        .map(|hit| mul(norm(offset), (hit.distance - 6.).max(1.)))
        .unwrap_or(offset)
    }
    fn update_camera(&mut self) {
        let focus = pos(&self.player);
        self.target = lerp(self.target, focus, 0.15);
        let back = v(sin(self.yaw), 0., cos(self.yaw));
        let mut side = v(back.z, 0., -back.x);
        if self
            .camera_offset
            .is_some_and(|offset| dot(offset, side) < 0.)
        {
            side = mul(side, -1.);
        }
        let mut offset = v(0., 0., 0.);
        for candidate in [
            add(
                add(sub(self.target, focus), mul(back, 100.)),
                v(0., 63., 0.),
            ),
            add(mul(back, 60.), v(0., 60., 0.)),
            add(mul(back, 30.), v(0., 60., 0.)),
            add(mul(back, 8.), v(0., 60., 0.)),
            mul(back, 80.),
            mul(add(back, side), 56.),
            mul(sub(back, side), 56.),
            mul(back, -80.),
        ] {
            let candidate = self.clip_camera(focus, candidate);
            if length(candidate) > length(offset) {
                offset = candidate;
            }
            if length(offset) >= 48. {
                break;
            }
        }
        if let Some(previous) = self.camera_offset {
            let turn = Quat::from_two_vectors(&previous, &offset);
            let unit = Quat::identity();
            let angle = unit.borrow().angle_to(&turn.borrow());
            let turn = unit
                .borrow()
                .slerp(&turn.borrow(), 0.25_f32.min(8. / angle.max(0.001)));
            let distance = length(previous) + (length(offset) - length(previous)) * 0.25;
            offset = mul(
                *turn.borrow().mul_vec(&norm(previous)).borrow(),
                distance.max(48.),
            );
            let clipped = self.clip_camera(focus, offset);
            if length(clipped) >= 48. {
                offset = clipped;
            } else {
                let mut eye = add(focus, offset);
                let start = mat(self.camera.borrow().transform.clone()).pos_value();
                for _ in 0..2 {
                    let delta = sub(eye, start);
                    let hit = Scene::raycast(
                        &self.root,
                        start,
                        delta,
                        length(delta),
                        false,
                        Some(&["ground".into()]),
                    );
                    if let Some(hit) = hit {
                        eye = add(
                            eye,
                            mul(hit.normal, 4. - dot(sub(eye, hit.point), hit.normal)),
                        );
                    } else {
                        break;
                    }
                }
                offset = sub(eye, focus);
            }
        }
        self.camera_offset = Some(offset);
        camera(&self.camera, add(focus, offset), focus);
    }
    fn animate(&mut self, walking: bool) {
        let desired = Quat::from_euler(&v(0., self.heading, 0.));
        let rotation = self.rotation.borrow().slerp(&desired.borrow(), 0.25);
        self.rotation = rotation;
        let rotation = mat(Mat4::from_quat(&self.rotation.borrow()));
        set_mat(&self.actor.node, mat(rotation.translate(&v(0., -8., 0.))));
        let motion = if !self.on_floor {
            "jump"
        } else if walking {
            "walk"
        } else {
            "idle"
        };
        if self.motion.as_deref() != Some(motion) {
            self.motion = Some(motion.into());
            self.motion_frame = 0.;
            let index = self
                .actor
                .player
                .clips()
                .iter()
                .position(|(n, _)| n == motion)
                .expect("official player motion");
            self.actor.player.select_clip(Some(index)).unwrap();
        }
        let duration = self.actor.player.duration();
        if duration > 0. {
            self.actor
                .player
                .seek((self.motion_frame / 30.).rem_euclid(duration))
                .unwrap();
            self.actor.sync();
        }
        let speed = (1.8 - self.motion_frame / 40.).max(1.);
        self.motion_frame += if motion == "jump" { speed } else { 1. };
    }
    pub fn update(&mut self, i: &mut Controls) {
        if i.shade {
            self.lit = !self.lit;
        }
        if i.reset {
            self.reset_player();
        }
        for (_, coin) in &self.coins {
            set_mat(&coin.node, rot(v(0., self.frame as f32 * 6., 0.)));
        }
        let platform_pos = lerp(
            self.platform_start,
            self.platform_end,
            (1. - cos(self.frame as f32 * 1.5 + 120.)) / 2.,
        );
        self.delta = sub(platform_pos, pos(&self.platform.node));
        set_mat(&self.platform.node, tr(platform_pos));
        if pos(&self.player).y < -160. {
            self.reset_player();
        }
        if self.on_platform {
            let p = add(pos(&self.player), self.delta);
            set_mat(&self.player, tr(p));
        }
        self.on_platform = false;
        let input = (i.right as i32 - i.left as i32, i.up as i32 - i.down as i32);
        if input != self.move_input {
            let camera = mat(self.camera.borrow().transform.clone());
            let right = norm(v(camera.data[0][0], 0., camera.data[2][0]));
            let forward = norm(v(-camera.data[0][2], 0., -camera.data[2][2]));
            self.movement = add(mul(right, input.0 as f32), mul(forward, input.1 as f32));
            self.move_input = input;
        }
        let movement = mul(norm(self.movement), 2.);
        if length(movement) > 0. {
            self.heading = movement.x.atan2(movement.z).to_degrees();
        }
        let coll = self.player.borrow().collider.clone().unwrap();
        let mut vy = (coll.borrow().velocity.borrow().y - 0.45).max(-10.);
        if self.on_floor {
            vy = (-dot(movement, self.floor_normal) / self.floor_normal.y).min(0.) - 0.45;
        }
        if i.pressed && self.jumps < 2 {
            vy = 6.5;
            if self.jumps == 0 {
                self.yaw = self.heading + 180.;
            }
            self.jumps += 1;
            self.motion = None;
            self.on_floor = false;
            i.sound_events.push(0);
        }
        *coll.borrow().velocity.borrow_mut() = v(movement.x, vy, movement.z);
        self.animate(length(movement) > 0.);
        self.on_floor = false;
        Scene::integrate_motion(&self.root, 1. / 30.);
        let contacts = Scene::detect_contacts(&self.root);
        for pair in contacts {
            let (other, c) = if Rc::ptr_eq(&pair.node_a, &self.player) {
                (pair.node_b, pair.contact_a)
            } else if Rc::ptr_eq(&pair.node_b, &self.player) {
                (pair.node_a, pair.contact_b)
            } else {
                continue;
            };
            if other.borrow().tags.contains("coin") {
                if other.borrow().active {
                    let mut n = other.borrow_mut();
                    n.active = false;
                    n.visible = false;
                    self.coins_taken += 1;
                    i.sound_events.push(1);
                }
                continue;
            }
            if Rc::ptr_eq(&other, &self.goal) {
                if other.borrow().active {
                    other.borrow_mut().active = false;
                    i.sound_events.push(2);
                }
                continue;
            }
            let c = c.borrow();
            let normal = *c.normal.borrow();
            let p = add(pos(&self.player), mul(normal, c.depth));
            set_mat(&self.player, tr(p));
            if normal.y > 0.45 && coll.borrow().velocity.borrow().y <= 0. {
                self.on_floor = true;
                self.floor_normal = normal;
                self.jumps = 0;
                coll.borrow().velocity.borrow_mut().y = 0.;
                if Rc::ptr_eq(&other, &self.platform.node) {
                    self.on_platform = true;
                }
            }
        }
        if self.on_floor && length(self.movement) > 0. {
            let turn = (self.heading + 360. - self.yaw).rem_euclid(360.) - 180.;
            self.yaw += (turn * 0.05).clamp(-2., 2.);
        }
        self.update_camera();
        self.frame += 1;
    }
    pub fn draw(&self) -> RcImage {
        self.image.borrow_mut().clear(6);
        let shade = self.shade.borrow();
        self.image.render(&self.camera, |ctx| {
            let s = state(self.lit.then_some(&shade));
            self.stage.draw(ctx, s);
            self.platform.draw(ctx, s);
            for (_, coin) in &self.coins {
                coin.draw(ctx, s);
            }
            let p = pos(&self.player);
            if let Some(hit) = Scene::raycast(
                &self.root,
                p,
                v(0., -1., 0.),
                f32::INFINITY,
                false,
                Some(&["ground".into()]),
            ) {
                ctx.decal_distance = Some(hit.distance + 8.);
                let transform = mm(tr(p), rot(v(-90., 0., 0.)));
                draw::elli(
                    ctx,
                    &transform,
                    14.,
                    14.,
                    1,
                    draw::DrawState {
                        dither_alpha: 0.5,
                        ..draw::DrawState::unshaded()
                    },
                );
                draw::elli(ctx, &transform, 9., 9., 1, draw::DrawState::unshaded());
                ctx.decal_distance = None;
            }
            self.actor.draw(ctx, s);
        });
        let caption = format!("COINS {}/{}", self.coins_taken, self.coins.len());
        self.outlined_text(8., 8., &caption);
        if !self.goal.borrow().active {
            self.outlined_text(252., 8., "SUMMIT REACHED!");
        }
        self.image.borrow_mut().draw_rect(0., 228., 320., 12., 0);
        hud(
            &self.image,
            64.,
            231.,
            "Arrows: Move  Space: Jump x2  S: Shading  Q: Quit",
            7,
        );
        self.image.finish()
    }
    fn outlined_text(&self, x: f32, y: f32, text: &str) {
        for dy in -1..=1 {
            for dx in -1..=1 {
                hud(&self.image, x + dx as f32, y + dy as f32, text, 0);
            }
        }
        hud(&self.image, x, y, text, 7);
    }
    pub fn info(&self) -> serde_json::Value {
        let p = pos(&self.player);
        let platform = pos(&self.platform.node);
        serde_json::json!({"frame":self.frame,"player":[p.x,p.y,p.z],"on_floor":self.on_floor,"jumps":self.jumps,"coins":self.coins_taken,"coin_total":self.coins.len(),"goal":!self.goal.borrow().active,"platform":[platform.x,platform.y,platform.z],"motion":self.motion})
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn coins_and_goal_are_delivered_once_after_contact_detection() {
        let mut game = CollisionDemo::new().unwrap();
        let coin = game.coins[0].0.clone();
        set_mat(&game.player, tr(world(&coin).pos_value()));
        game.update(&mut Controls::default());
        assert_eq!(game.coins_taken, 1);
        assert!(!coin.borrow().visible && !coin.borrow().active);
        game.update(&mut Controls::default());
        assert_eq!(game.coins_taken, 1);
        set_mat(&game.player, tr(world(&game.goal).pos_value()));
        game.update(&mut Controls::default());
        assert!(!game.goal.borrow().active);
    }
}
