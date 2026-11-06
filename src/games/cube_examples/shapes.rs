use super::*;

pub struct Hello {
    pub frame: u32,
    camera: RcCamera,
    shade: RcShading,
    image: RcImage,
    cubes: Vec<Mat4>,
}
impl Hello {
    pub fn new() -> Self {
        let camera = Camera::new();
        camera.borrow_mut().clear_color = Some(0);
        super::camera(&camera, v(0., 78., 104.), v(0., 0., 0.));
        Self {
            frame: 0,
            camera,
            shade: shading(),
            image: Image::new(200, 150),
            cubes: vec![identity(); 6],
        }
    }
    pub fn update(&mut self) {
        let f = self.frame as f32;
        for (i, m) in self.cubes.iter_mut().enumerate() {
            let phase = i as f32 * 60.;
            let orbit = phase + f * 2.;
            *m = mm(
                tr(v(
                    cos(orbit) * 52.,
                    sin(phase + f * 4.) * 13. + 10.4,
                    sin(orbit) * 52.,
                )),
                rot(v(f * 3., f * 5., 0.)),
            );
        }
        self.frame += 1;
    }
    pub fn draw(&self) -> RcImage {
        let shade = self.shade.borrow();
        render_3d(&self.image, &self.camera, |ctx| {
            let s = state(Some(&shade));
            draw::text(
                ctx,
                &identity(),
                &v(0., 0., 0.),
                "Hello, Pyxel Cube!",
                (self.frame.saturating_sub(1) % 16) as i32,
                None,
                s,
            );
            for (m, col) in self.cubes.iter().zip([8, 9, 10, 11, 12, 14]) {
                draw::box_solid(ctx, m, &v(15.6, 15.6, 15.6), col, None, None, s);
            }
        });
        self.image.clone()
    }
}

pub struct Shapes {
    pub frame: u32,
    pub wire: bool,
    camera: RcCamera,
    shade: RcShading,
    image: RcImage,
    cat: RcImage,
    yaw: f32,
    pitch: f32,
    previous: Option<(f32, f32)>,
}
impl Shapes {
    pub fn new() -> Self {
        let cam = Camera::new();
        cam.borrow_mut().clear_color = Some(1);
        Self {
            frame: 0,
            wire: false,
            camera: cam,
            shade: shading(),
            image: Image::new(240, 240),
            cat: texture(include_bytes!("assets/cat_16x16.bin"), 16, 16),
            yaw: 0.,
            pitch: 0.,
            previous: None,
        }
    }
    pub fn update(&mut self, i: &Controls) {
        self.wire = i.action;
        if i.pointer {
            if let Some((x, y)) = self.previous {
                self.yaw -= (i.mouse_x - x) * 0.5;
                self.pitch += (i.mouse_y - y) * 0.5;
            }
            self.previous = Some((i.mouse_x, i.mouse_y));
        }
        self.yaw += 2. * ((i.left as i32 - i.right as i32) as f32);
        self.pitch += 2. * ((i.down as i32 - i.up as i32) as f32);
        self.yaw = self.yaw.clamp(-45., 45.);
        self.pitch = self.pitch.clamp(-30., 30.);
        let target = v(0., 7., 0.);
        camera(
            &self.camera,
            add(
                target,
                v(
                    210. * sin(self.yaw) * cos(self.pitch),
                    210. * sin(self.pitch),
                    210. * cos(self.yaw) * cos(self.pitch),
                ),
            ),
            target,
        );
        self.frame += 1;
    }
    pub fn draw(&self) -> RcImage {
        let shade = self.shade.borrow();
        render_3d(&self.image, &self.camera, |ctx| {
            let s = state(Some(&shade));
            for j in 0..10 {
                let a = j as f32 * 36.;
                let m = mm(
                    tr(v(87.5 * sin(a), 87.5 * cos(a), 0.)),
                    rot(v(0., (self.frame.saturating_sub(1) * 2) as f32, 0.)),
                );
                let uv = ((0., 0.), (1., 0.), (0., 1.), (1., 1.));
                let label = match j {
                    0 => {
                        draw::pset(ctx, &m, &v(0., 0., 0.), 8, s);
                        "pset"
                    }
                    1 => {
                        draw::line(ctx, &m, &v(-17.5, 0., 0.), &v(17.5, 0., 0.), 14, s);
                        "line"
                    }
                    2 => {
                        if self.wire {
                            draw::trib(
                                ctx,
                                &m,
                                &v(0., 17.5, 0.),
                                &v(-17.5, -12.25, 0.),
                                &v(17.5, -12.25, 0.),
                                9,
                                s,
                            );
                            "trib"
                        } else {
                            draw::tri(
                                ctx,
                                &m,
                                &v(0., 17.5, 0.),
                                &v(-17.5, -12.25, 0.),
                                &v(17.5, -12.25, 0.),
                                9,
                                s,
                            );
                            "tri"
                        }
                    }
                    3 => {
                        if self.wire {
                            draw::rectb(ctx, &m, 35., 24.5, 10, s);
                            "rectb"
                        } else {
                            draw::rect(ctx, &m, 35., 24.5, 10, s);
                            "rect"
                        }
                    }
                    4 => {
                        if self.wire {
                            draw::circb(ctx, &m, &v(0., 0., 0.), 17.5, 3, s);
                            "circb"
                        } else {
                            draw::circ(ctx, &m, &v(0., 0., 0.), 17.5, 3, s);
                            "circ"
                        }
                    }
                    5 => {
                        if self.wire {
                            draw::ellib(ctx, &m, 35., 24.5, 11, s);
                            "ellib"
                        } else {
                            draw::elli(ctx, &m, 35., 24.5, 11, s);
                            "elli"
                        }
                    }
                    6 => {
                        draw::plane(ctx, &m, &self.cat, uv, 35., 35., None, s);
                        "plane"
                    }
                    7 => {
                        draw::sprite(
                            ctx,
                            &m,
                            &v(0., 0., 0.),
                            &self.cat,
                            uv,
                            35.,
                            35.,
                            None,
                            0.,
                            s,
                        );
                        "sprite"
                    }
                    8 => {
                        if self.wire {
                            draw::boxb(ctx, &m, &v(29.75, 29.75, 29.75), 12, s);
                            "boxb"
                        } else {
                            draw::box_solid(
                                ctx,
                                &m,
                                &v(29.75, 29.75, 29.75),
                                7,
                                Some(&self.cat),
                                None,
                                s,
                            );
                            "box"
                        }
                    }
                    _ => {
                        if self.wire {
                            draw::sphereb(ctx, &m, &v(0., 0., 0.), 17.5, 2, s);
                            "sphereb"
                        } else {
                            draw::sphere(
                                ctx,
                                &m,
                                &v(0., 0., 0.),
                                17.5,
                                7,
                                Some(&self.cat),
                                None,
                                s,
                            );
                            "sphere"
                        }
                    }
                };
                ctx.depth_offset = -35.;
                draw::text(ctx, &m, &v(0., 24.5, 0.), label, 7, None, s);
                ctx.depth_offset = 0.;
            }
        });
        self.image.borrow_mut().draw_rect(85., 110., 70., 20., 0);
        hud(&self.image, 88., 113., "Mouse: Rotate", 7);
        hud(&self.image, 88., 121., "Space: Wireframe", 7);
        self.image.clone()
    }
}

pub struct Custom {
    pub frame: u32,
    pub locked: Vec<usize>,
    pub targets: Vec<(usize, Vec3)>,
    pub age: u32,
    centers: Vec<Vec3>,
    flashes: Vec<u32>,
    camera: RcCamera,
    shade: RcShading,
    image: RcImage,
    body: RcPrimitive,
    base: Vec<f32>,
    strip: RcPrimitive,
    laser: RcImage,
    mouse: (f32, f32),
}
impl Custom {
    pub fn new() -> Self {
        let camera = Camera::new();
        camera.borrow_mut().clear_color = Some(0);
        let shade = shading();
        shade.borrow_mut().direction = Vec3::new(0.4, -1., -0.5);
        let body = Primitive::sphere(1.);
        let base = body.borrow().positions.clone();
        let strip = Primitive::new();
        {
            let mut p = strip.borrow_mut();
            p.cull = 0;
            for j in 0..24 {
                p.uvs.extend([0., j as f32 / 23., 1., j as f32 / 23.]);
                if j < 23 {
                    let k = j * 2;
                    p.indices.extend([k, k + 1, k + 2, k + 1, k + 3, k + 2]);
                }
            }
        }
        Self {
            frame: 0,
            locked: vec![],
            targets: vec![],
            age: 0,
            centers: vec![v(0., 0., 0.); 5],
            flashes: vec![0; 5],
            camera,
            shade,
            image: Image::new(256, 192),
            body,
            base,
            strip,
            laser: texture(include_bytes!("assets/laser.bin"), 16, 16),
            mouse: (128., 96.),
        }
    }
    pub fn screen_centers(&self) -> Vec<(f32, f32)> {
        self.centers.iter().map(|p| self.project(*p)).collect()
    }
    fn project(&self, p: Vec3) -> (f32, f32) {
        let cam = self.camera.borrow();
        let local = cam.transform.borrow().inverse_value().mul_vec_value(&p);
        let scale = 192. / (-2. * local.z * (cam.fov.to_radians() / 2.).tan());
        (128. + local.x * scale, 96. - local.y * scale)
    }
    pub fn update(&mut self, i: &mut Controls) {
        self.mouse = (i.mouse_x, i.mouse_y);
        let offset = i.mouse_y / 192. - 0.5;
        camera(
            &self.camera,
            v(0., 132. - offset * 44., 110.),
            v(0., 11., 0.),
        );
        let f = self.frame as f32;
        for j in 0..5 {
            self.flashes[j] = self.flashes[j].saturating_sub(1);
            let orbit = j as f32 * std::f32::consts::TAU / 5. + f * 0.01;
            self.centers[j] = v(
                38.5 * orbit.cos(),
                27.5 + 5.5 * (f * 0.02 + j as f32 * 1.3).sin(),
                38.5 * orbit.sin(),
            );
        }
        self.frame += 1;
        if !self.targets.is_empty() {
            self.age += 1;
            let hit = self.age == 14 || (i.pressed && self.age < 14);
            if hit {
                for (j, _) in &self.targets {
                    self.flashes[*j] = 12;
                }
                i.sound_events.push(2);
            }
            if i.pressed || self.age > 22 {
                self.targets.clear();
            } else {
                return;
            }
        }
        if i.pressed {
            self.locked.clear();
        }
        if i.action {
            for (j, (x, y)) in self.screen_centers().into_iter().enumerate() {
                if (i.mouse_x - x).hypot(i.mouse_y - y) < 13. && !self.locked.contains(&j) {
                    self.locked.push(j);
                    i.sound_events.push(0);
                }
            }
        } else if i.released && !self.locked.is_empty() {
            self.fire();
            i.sound_events.extend([1, 3]);
        }
    }
    fn fire(&mut self) {
        self.age = 1;
        self.targets.clear();
        let cam = mat(self.camera.borrow().transform.clone());
        let right = cam.mul_dir_value(&v(1., 0., 0.));
        let center = (self.locked.len() as f32 - 1.) * 0.5;
        for (index, j) in self.locked.iter().enumerate() {
            let lock_side = (index as f32 - center) / center.max(1.);
            let mut side = dot(sub(self.centers[*j], v(0., 11., 0.)), right);
            if side.abs() <= 4.4 {
                side = if lock_side != 0. { lock_side } else { 1. };
            }
            let fan = side.signum() * 1.2 * (0.7 + 0.3 * lock_side.abs());
            let direction = norm(sub(self.centers[*j], v(0., 5.5, 110.)));
            self.targets
                .push((*j, norm(add(direction, mul(right, fan)))));
        }
        self.locked.clear();
    }
    fn path(&self, center: Vec3, mut direction: Vec3, progress: f32) -> Vec<Vec3> {
        let origin = v(0., 5.5, 110.);
        let step = length(sub(center, origin)) * 1.15 / 23.;
        let mut p = origin;
        let mut path = vec![p];
        for j in 1..24 {
            let ratio = (j - 1) as f32 / 22.;
            let t = ((ratio - 0.3) / 0.7).max(0.);
            let steer = t * t * (3. - 2. * t) * 0.35;
            direction = norm(lerp(direction, norm(sub(center, p)), steer));
            p = add(p, mul(direction, step));
            path.push(p);
        }
        let correction = sub(center, path[23]);
        for (j, p) in path.iter_mut().enumerate() {
            *p = add(*p, mul(correction, (j as f32 / 23.).powf(2.5)));
        }
        path[23] = center;
        (0..24)
            .map(|j| {
                let p = progress * j as f32;
                let k = (p as usize).min(22);
                lerp(path[k], path[k + 1], p - k as f32)
            })
            .collect()
    }
    pub fn draw(&mut self) -> RcImage {
        let shade = self.shade.borrow();
        render_3d(&self.image, &self.camera, |ctx| {
            let s = state(Some(&shade));
            for j in 0..13 {
                let p = -77. + 154. * j as f32 / 12.;
                draw::line(ctx, &identity(), &v(p, 0., -77.), &v(p, 0., 77.), 5, s);
                draw::line(ctx, &identity(), &v(-77., 0., p), &v(77., 0., p), 5, s);
            }
            let time = self.frame.saturating_sub(1) as f32 * 0.06;
            for (j, col) in [8, 9, 10, 12, 14].into_iter().enumerate() {
                let phase = j as f32 * 1.3;
                let mut body = self.body.borrow_mut();
                body.positions.clear();
                for xyz in self.base.chunks_exact(3) {
                    let [x, y, z] = [xyz[0], xyz[1], xyz[2]];
                    let r = 11.
                        + 2.75 * (3. * x + 1.7 * time + phase).sin()
                        + 2.42 * (3.5 * y + 1.3 * time + phase * 1.7).sin()
                        + 2.2 * (4. * z + 2.1 * time + phase * 0.6).sin();
                    body.positions.extend([x * r, y * r, z * r]);
                }
                body.compute_normals();
                let color = if self.flashes[j] > 0 && self.flashes[j] % 2 == 0 {
                    7
                } else {
                    col
                };
                primitive(ctx, tr(self.centers[j]), &body, color, None, None, s);
            }
            ctx.depth_test = false;
            let unlit = draw::DrawState {
                depth_test: false,
                ..draw::DrawState::unshaded()
            };
            let cam = mat(self.camera.borrow().transform.clone());
            let right = cam.mul_dir_value(&v(1., 0., 0.));
            for (j, direction) in &self.targets {
                let path = self.path(
                    self.centers[*j],
                    *direction,
                    (self.age as f32 / 14.).min(1.),
                );
                let mut strip = self.strip.borrow_mut();
                strip.positions.clear();
                for (k, p) in path.iter().enumerate() {
                    let tangent = sub(path[(k + 1).min(23)], path[k.saturating_sub(1)]);
                    let side = cross(tangent, sub(*p, cam.pos_value()));
                    let side = if length(side) > 1e-6 {
                        norm(side)
                    } else {
                        right
                    };
                    let a = sub(*p, mul(side, 2.42));
                    let b = add(*p, mul(side, 2.42));
                    strip.positions.extend([a.x, a.y, a.z, b.x, b.y, b.z]);
                }
                primitive(
                    ctx,
                    identity(),
                    &strip,
                    7,
                    Some(&self.laser),
                    Some(0),
                    unlit,
                );
            }
        });
        for j in &self.locked {
            let (x, y) = self.project(self.centers[*j]);
            for (r, c) in [(4., 8), (5., 7), (6., 8)] {
                self.image.borrow_mut().draw_rect_border(
                    x.round() - r,
                    y.round() - r,
                    r * 2. + 1.,
                    r * 2. + 1.,
                    c,
                );
            }
        }
        for r in [7., 9.] {
            self.image
                .borrow_mut()
                .draw_circle_border(self.mouse.0, self.mouse.1, r, 7);
        }
        hud(&self.image, 76., 5., "Drag: Lock   Release: Fire", 7);
        self.image.clone()
    }
}
