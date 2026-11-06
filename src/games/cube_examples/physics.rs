use super::*;

struct Body {
    node: RcNode,
    primitive: RcPrimitive,
    paint: Option<RcImage>,
    color: i32,
    transforms: Vec<Mat4>,
    floor: bool,
    shot: bool,
}
fn box_primitive(size: Vec3, cell: f32) -> RcPrimitive {
    let p = Primitive::r#box(&size);
    {
        let mut p = p.borrow_mut();
        for vertex in 0..24 {
            let axes = [(0, 1), (0, 2), (2, 1)][vertex / 8];
            for (uv, axis) in [axes.0, axes.1].into_iter().enumerate() {
                let extent = [size.x, size.y, size.z][axis];
                p.uvs[vertex * 2 + uv] =
                    (p.positions[vertex * 3 + axis] + extent / 2.) / (16. * cell);
            }
        }
    }
    p
}
impl Body {
    fn new(
        parent: &RcNode,
        position: Vec3,
        size: Vec3,
        colors: (i32, Option<i32>),
        mass: f32,
        radius: f32,
    ) -> Self {
        let node = Node::new();
        set_mat(&node, tr(position));
        node.borrow_mut().collider = Some(collider(size, radius, mass, None, false, mass > 0.));
        Node::add_child(parent, &node);
        let primitive = if radius > 0. {
            Primitive::capsule(size.y, radius)
        } else {
            box_primitive(
                size,
                if mass > 0. {
                    size.x.max(size.y).max(size.z) / 2.
                } else {
                    96.
                },
            )
        };
        let accent = match colors.0 {
            3 => 11,
            6 => 12,
            8 => 14,
            9 => 10,
            10 => 15,
            11 => 12,
            12 => 6,
            14 => 15,
            _ => 7,
        };
        let mut paint = None;
        if mass > 0. || colors.1.is_some() {
            let img = Image::new(16, 16);
            for y in 0..16 {
                for x in 0..16 {
                    let color = if mass > 0. && radius > 0. {
                        if size.y > 0. {
                            if (x < 8) == (y < 8) {
                                colors.0
                            } else {
                                accent
                            }
                        } else if x % 8 < 2 {
                            15
                        } else {
                            colors.0
                        }
                    } else if (x + y) % 2 == 0 {
                        colors.0
                    } else {
                        colors.1.unwrap_or(accent)
                    };
                    img.borrow_mut().set_pixel(x as f32, y as f32, color as u8);
                }
            }
            paint = Some(img);
        }
        Self {
            node,
            primitive,
            paint,
            color: colors.0,
            transforms: vec![identity()],
            floor: false,
            shot: false,
        }
    }
    fn mass(&self) -> f32 {
        self.node.borrow().collider.as_ref().unwrap().borrow().mass
    }
    fn contact(&self, c: &crate::cube::RcContact) {
        if self.mass() <= 0. {
            return;
        }
        let c = c.borrow();
        let normal = *c.normal.borrow();
        let previous = *self.node.borrow().transform.borrow();
        set_mat(&self.node, mm(tr(mul(normal, c.depth)), previous));
        let coll = self.node.borrow().collider.clone().unwrap();
        let col = coll.borrow_mut();
        let vel = add(*col.velocity.borrow(), *c.delta_velocity.borrow());
        *col.velocity.borrow_mut() = vel;
        let spin = add(
            *col.angular_velocity.borrow(),
            *c.delta_angular_velocity.borrow(),
        );
        *col.angular_velocity.borrow_mut() = spin;
    }
    fn draw(&self, ctx: &mut DrawContext, s: draw::DrawState<'_>) {
        if !Node::effective_visible(&self.node) {
            return;
        }
        let m = world(&self.node);
        for t in &self.transforms {
            primitive(
                ctx,
                mm(m, *t),
                &self.primitive.borrow(),
                self.color,
                self.paint.as_ref(),
                None,
                s,
            );
        }
        if self.floor {
            ctx.depth_write = false;
            ctx.depth_offset = -0.5;
            let grid = draw::DrawState {
                depth_write: false,
                ..s
            };
            for offset in (-2400..=2400).step_by(400) {
                let o = offset as f32;
                draw::line(ctx, &m, &v(o, 12., -2400.), &v(o, 12., 2400.), 5, grid);
                draw::line(ctx, &m, &v(-2400., 12., o), &v(2400., 12., o), 5, grid);
            }
            ctx.depth_write = true;
            ctx.depth_offset = 0.;
        }
    }
}

pub struct PhysicsDemo {
    pub frame: u32,
    root: RcNode,
    camera: RcCamera,
    shade: RcShading,
    image: RcImage,
    bodies: Vec<Body>,
    angle: f32,
    pitch: f32,
    shot_count: u32,
    mouse: (f32, f32),
    repeat: u32,
}
impl PhysicsDemo {
    pub fn new() -> Self {
        let root = Node::new();
        let camera = Camera::new();
        camera.borrow_mut().far = 6000.;
        let mut d = Self {
            frame: 0,
            root,
            camera,
            shade: shading(),
            image: Image::new(320, 240),
            bodies: vec![],
            angle: 0.,
            pitch: 30.,
            shot_count: 0,
            mouse: (160., 120.),
            repeat: 0,
        };
        d.static_scene();
        d.reset();
        d.move_camera();
        d
    }
    fn body(&mut self, p: Vec3, size: Vec3, color: i32, mass: f32, radius: f32) -> usize {
        let b = Body::new(&self.root, p, size, (color, None), mass, radius);
        self.bodies.push(b);
        self.bodies.len() - 1
    }
    fn static_body(&mut self, p: Vec3, size: Vec3, color: (i32, i32)) -> usize {
        let b = Body::new(&self.root, p, size, (color.0, Some(color.1)), 0., 0.);
        self.bodies.push(b);
        self.bodies.len() - 1
    }
    fn static_scene(&mut self) {
        let floor = self.body(v(0., -12., 0.), v(6000., 24., 6000.), 1, 0., 0.);
        self.bodies[floor].floor = true;
        for (x, y, z, w, d) in [
            (-260.0_f32, 540., -25., 160., 270.),
            (320., 370., -90., 180., 180.),
            (-340., 320., 220., 220., 160.),
            (250., 480., 195., 150., 270.),
        ] {
            let px = x + x.signum() * (w - 64.) / 2.;
            self.static_body(v(px, (y - 10.) / 2., z), v(64., y - 10., 64.), (1, 5));
            self.static_body(v(x, y, z), v(w, 20., d), (1, 5));
        }
        for z in [100., 340.] {
            self.static_body(v(-450., 280., z), v(90., 560., 40.), (1, 5));
        }
        for (z, h, ledge, dep) in [(-195., 570., -125., 180.), (355., 600., 275., 200.)] {
            self.static_body(v(0., (h - 10.) / 2., z), v(40., h - 10., 40.), (1, 5));
            self.static_body(v(0., h, ledge), v(200., 20., dep), (1, 5));
        }
        self.static_body(v(0., 330., 80.), v(24., 24., 550.), (1, 5));
        for (position, pitch, span, w, dep) in [
            (v(0., 330., 80.), 0., 440., 24., 336.),
            (v(0., 16., 80.), 90., 480., 40., 32.),
        ] {
            for angle in [0., 90.] {
                let j = self.static_body(position, v(span, w, dep), (2, 4));
                let b = &mut self.bodies[j];
                set_mat(
                    &b.node,
                    mm(
                        mm(tr(position), rot(v(pitch, 0., 0.))),
                        rot(v(0., 0., angle)),
                    ),
                );
                *b.node
                    .borrow()
                    .collider
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .angular_velocity
                    .borrow_mut() = v(0., 0., -1.5);
                if angle != 0. {
                    b.primitive = box_primitive(v((span - w) / 2., w, dep), 96.);
                    let shift = tr(v((span + w) / 4., 0., 0.));
                    b.transforms = vec![shift, shift.inverse_value()];
                }
            }
        }
    }
    fn reset(&mut self) {
        self.bodies.retain(|b| {
            if b.mass() > 0. {
                Node::detach(&b.node);
                false
            } else {
                true
            }
        });
        self.shot_count = 0;
        for (x, y, z, layers, palette) in [
            (-235., 540., -25., 2, [8, 14, 9]),
            (200., 480., 195., 1, [11, 12, 6]),
        ] {
            for row in 0..5 {
                let color = palette[row / 2];
                for col in 0..5 - row {
                    for layer in 0..layers {
                        let dx = (layer as f32 - (layers - 1) as f32 / 2.) * 50.;
                        let dz = -100. + row as f32 * 25. + col as f32 * 50.;
                        self.body(
                            v(x + dx, y + 35. + row as f32 * 50., z + dz),
                            v(50., 50., 50.),
                            color,
                            5.,
                            0.,
                        );
                    }
                }
            }
        }
        for (x, y, z, floors, palette) in [
            (290., 380., -80., 3, [9, 10, 14]),
            (0., 580., -125., 2, [3, 11, 12]),
            (0., 610., 275., 1, [8, 14, 9]),
        ] {
            for floor in 0..floors {
                let base = y + floor as f32 * 80.;
                for (dx, dz) in [(-50., -50.), (50., -50.), (0., 50.)] {
                    self.body(
                        v(x + dx, base + 32.5, z + dz),
                        v(30., 65., 30.),
                        palette[0],
                        3.,
                        0.,
                    );
                }
                self.body(v(x, base + 72.5, z), v(180., 15., 160.), palette[1], 8., 0.);
                self.body(v(x, base + 20., z), v(100., 40., 50.), palette[2], 4., 0.);
            }
        }
        let j = self.body(v(-450., 570., 220.), v(280., 20., 90.), 9, 14., 0.);
        set_mat(
            &self.bodies[j].node,
            mm(tr(v(-450., 570., 220.)), rot(v(0., 90., 0.))),
        );
        let j = self.body(v(-450., 615., 220.), v(0., 90., 0.), 11, 25., 35.);
        set_mat(
            &self.bodies[j].node,
            mm(tr(v(-450., 615., 220.)), rot(v(90., 0., 0.))),
        );
        for (x, y, z, c) in [(290., 675., -80., 8), (0., 745., 275., 10)] {
            self.body(v(x, y, z), v(0., 0., 0.), c, 60., 55.);
        }
        for (row, color) in [9, 10, 14].into_iter().enumerate() {
            for side in [-1., 1.] {
                let offset = if row != 1 {
                    v(side * 50., 0., 0.)
                } else {
                    v(0., 0., side * 45.)
                };
                let p = add(v(-340., 365. + row as f32 * 70., 220.), offset);
                let j = self.body(p, v(0., 110., 0.), color, 12., 35.);
                set_mat(
                    &self.bodies[j].node,
                    mm(tr(p), rot(v(90., if row == 1 { 90. } else { 0. }, 0.))),
                );
            }
        }
        for floor in 0..4 {
            let y = 490. + floor as f32 * 78.;
            for z in [135., 255.] {
                self.body(v(280., y + 32., z), v(18., 64., 18.), 8, 1., 0.);
            }
            self.body(v(280., y + 71., 195.), v(64., 14., 180.), 9, 3., 0.);
        }
    }
    fn move_camera(&self) {
        let target = v(0., 470., 0.);
        let rotation = rot(v(-self.pitch, self.angle, 0.));
        camera(
            &self.camera,
            add(target, rotation.mul_vec_value(&v(0., 0., 1100.))),
            target,
        );
    }
    fn shoot(&mut self, center: bool) {
        let cam = self.camera.borrow();
        let scale = 2. * (cam.fov.to_radians() / 2.).tan() / 240.;
        let (x, y) = if center {
            (0., 0.)
        } else {
            ((self.mouse.0 - 160.) * scale, (120. - self.mouse.1) * scale)
        };
        let matrix = mat(cam.transform.clone());
        drop(cam);
        let direction = norm(matrix.mul_dir_value(&norm(v(x, y, -1.))));
        let origin = matrix.pos_value();
        let hits = Scene::raycast_all(&self.root, origin, direction, f32::INFINITY, false, None);
        let target = hits
            .into_iter()
            .find(|h| {
                !self
                    .bodies
                    .iter()
                    .any(|b| b.shot && Rc::ptr_eq(&b.node, &h.node))
            })
            .map(|h| h.point)
            .unwrap_or(add(origin, mul(direction, 1000.)));
        let p = sub(add(origin, mul(direction, 80.)), v(0., 24., 0.));
        let flight = (length(sub(target, p)) / 30.).max(1.);
        let kind = self.shot_count as usize % 3;
        self.shot_count += 1;
        let j = self.body(
            p,
            [v(0., 0., 0.), v(0., 36., 0.), v(58., 58., 58.)][kind],
            [8, 14, 10][kind],
            40.,
            [36., 24., 0.][kind],
        );
        let b = &mut self.bodies[j];
        b.shot = true;
        if kind == 1 {
            b.primitive = Primitive::sphere(24.);
            b.transforms = vec![mat(Mat4::from_scale(&v(1., 1.75, 1.)))];
        }
        let collider = b.node.borrow().collider.clone().unwrap();
        let c = collider.borrow();
        let acceleration = mul(norm(*c.gravity_direction.borrow()), c.gravity * 100. / 900.);
        let drag = c.linear_damp / 30.;
        let travel = if drag == 0. {
            flight
        } else {
            (1. - drag) * (1. - (1. - drag).powf(flight)) / drag
        };
        let fall = if drag == 0. {
            flight * (flight + 1.) / 2.
        } else {
            (flight - travel) / drag
        };
        *c.velocity.borrow_mut() = mul(sub(sub(target, p), mul(acceleration, fall)), 1. / travel);
        *c.angular_velocity.borrow_mut() = v(3., 5., 2.);
        drop(c);
        if self.bodies.iter().filter(|b| b.shot).count() > 16 {
            if let Some(j) = self.bodies.iter().position(|b| b.shot) {
                Node::destroy(&self.bodies[j].node);
            }
        }
    }
    pub fn update(&mut self, i: &mut Controls) {
        if i.reset {
            self.reset();
        }
        self.angle += 2. * (i.right as i32 - i.left as i32) as f32;
        self.pitch = (self.pitch + 2. * (i.up as i32 - i.down as i32) as f32).clamp(-12., 75.);
        self.move_camera();
        self.mouse = (i.mouse_x, i.mouse_y);
        if i.action {
            self.repeat += 1;
        } else {
            self.repeat = 0;
        }
        if i.pressed || (i.action && self.repeat > 10 && self.repeat % 10 == 0) {
            self.shoot(!i.pointer);
            i.sound_events.push(0);
        }
        Scene::integrate_motion(&self.root, 1. / 30.);
        let pairs = Scene::detect_contacts(&self.root);
        for p in pairs {
            for b in &self.bodies {
                if Rc::ptr_eq(&b.node, &p.node_a) {
                    b.contact(&p.contact_a);
                }
                if Rc::ptr_eq(&b.node, &p.node_b) {
                    b.contact(&p.contact_b);
                }
            }
        }
        for n in Scene::collect_destroyed_post_order(&self.root) {
            Scene::take_destroy_notification(&n);
            Scene::detach_destroyed(&n);
        }
        self.bodies.retain(|b| !b.node.borrow().destroyed);
        self.frame += 1;
    }
    pub fn draw(&self) -> RcImage {
        self.image.borrow_mut().clear(12);
        let shade = self.shade.borrow();
        render_3d(&self.image, &self.camera, |ctx| {
            let s = state(Some(&shade));
            for b in &self.bodies {
                b.draw(ctx, s);
            }
        });
        self.image
            .borrow_mut()
            .draw_circle_border(self.mouse.0, self.mouse.1, 4., 7);
        self.image
            .borrow_mut()
            .set_pixel(self.mouse.0, self.mouse.1, 7);
        hud(
            &self.image,
            8.,
            8.,
            "Mouse: Aim  Click/Space: Fire  Arrows: Rotate  R: Reset",
            7,
        );
        self.image.clone()
    }
    pub fn info(&self) -> serde_json::Value {
        serde_json::json!({"frame":self.frame,"bodies":self.bodies.len(),"shots":self.shot_count,"dynamic_y":self.bodies.iter().filter(|b|b.mass()>0.).map(|b|pos(&b.node).y).collect::<Vec<_>>(),"angle":self.angle})
    }
}
