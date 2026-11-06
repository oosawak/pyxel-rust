use super::*;
include!("assets/tufts.rs");
pub struct MotionDemo {
    pub frame: u32,
    pub selected: usize,
    pub model: Model,
    camera: RcCamera,
    shade: RcShading,
    image: RcImage,
    yaw: f32,
    pitch: f32,
    offset: f32,
    lit: bool,
    joints: Vec<RcNode>,
    clips: Vec<usize>,
}
impl MotionDemo {
    pub fn new() -> Result<Self, String> {
        let mut model = Model::new(load("gazelle")?)?;
        set_mat(
            &model.node,
            mm(
                tr(v(0.45591, 2.1, 12.73412)),
                mat(Mat4::from_scale(&v(84.0189, 84.0189, 84.0189))),
            ),
        );
        let clips = model.player.clips();
        let clips: Vec<usize> = ["move", "run", "eat"]
            .iter()
            .map(|name| {
                clips
                    .iter()
                    .position(|(n, _)| n == name)
                    .ok_or(format!("Missing motion {name}"))
            })
            .collect::<Result<_, _>>()?;
        model.player.select_clip(Some(clips[0]))?;
        model.sync();
        let joints = ["rightFronLeg", "leftFronLeg", "rightBackLeg", "leftBackLeg"]
            .iter()
            .map(|n| model.find(n))
            .collect::<Result<_, _>>()?;
        let camera = Camera::new();
        {
            let mut c = camera.borrow_mut();
            c.near = 7.;
            c.far = 70000.;
            c.clear_color = Some(12);
            c.ortho_size = Some(245.);
        }
        let shade = shading();
        shade.borrow_mut().direction = Vec3::new(-0.5, -1., 0.8);
        let mut d = Self {
            frame: 0,
            selected: 0,
            model,
            camera,
            shade,
            image: Image::new(320, 240),
            yaw: 148.,
            pitch: (119.0_f32 / 420.).atan().to_degrees(),
            offset: 0.,
            lit: false,
            joints,
            clips,
        };
        d.move_camera();
        Ok(d)
    }
    fn move_camera(&mut self) {
        camera(
            &self.camera,
            v(
                420. * sin(self.yaw),
                59.5 + 420. * self.pitch.to_radians().tan(),
                420. * cos(self.yaw),
            ),
            v(0., 59.5, 0.),
        );
        let extent = 59.5 * cos(self.pitch) + 154. * sin(self.pitch);
        self.camera.borrow_mut().ortho_size = Some(245.0_f32.max(extent * 2. + 14.));
    }
    pub fn update(&mut self, i: &Controls) {
        if let Some(index) = i.motion {
            self.selected = index;
            self.model
                .player
                .select_clip(Some(self.clips[index]))
                .unwrap();
        }
        if i.shade {
            self.lit = !self.lit;
        }
        self.yaw += 0.4 + 2. * ((i.right as i32 - i.left as i32) as f32);
        self.pitch = (self.pitch + (i.up as i32 - i.down as i32) as f32).clamp(5., 60.);
        self.move_camera();
        self.offset = (self.offset + [29.4, 175., 0.][self.selected] / 30.) % 294.;
        self.model.player.advance(1. / 30.).unwrap();
        self.model.sync();
        self.frame += 1;
    }
    pub fn draw(&self) -> RcImage {
        let shade = self.shade.borrow();
        render_3d(&self.image, &self.camera, |ctx| {
            let rotation = rot(v(-90., 0., 0.));
            let s = draw::DrawState::unshaded();
            draw::elli(ctx, &rotation, 308., 308., 5, s);
            draw::elli(ctx, &mm(tr(v(0., 0.7, 0.)), rotation), 297.5, 297.5, 3, s);
            let center = mul(
                self.joints
                    .iter()
                    .map(|j| world(j).pos_value())
                    .fold(v(0., 0., 0.), add),
                0.25,
            );
            ctx.decal_distance = Some(160.);
            draw::elli(ctx, &mm(tr(center), rotation), 52.5, 101.5, 5, s);
            ctx.decal_distance = None;
            let cam = mat(self.camera.borrow().transform.clone());
            let right = norm(v(cam.data[0][0], 0., cam.data[2][0]));
            let inverse = cam.inverse_value();
            let scale = 240. / self.camera.borrow().ortho_size.unwrap();
            for &(x, start_z, height, mirror) in TUFTS {
                let z = (start_z + self.offset + 147.).rem_euclid(294.) - 147.;
                if x * x + z * z > 148.75 * 148.75 {
                    continue;
                }
                let base = v(x, 1.4, z);
                let tips = [(-4.55, 0.7), (0.35, 1.), (4.2, 0.55)].map(|(dx, dy)| {
                    add(add(base, mul(right, dx * mirror)), v(0., height * dy, 0.))
                });
                if std::iter::once(base).chain(tips).all(|p| {
                    let p = inverse.mul_vec_value(&p);
                    let x = 160. + p.x * scale;
                    let y = 120. - p.y * scale;
                    (1.0..319.).contains(&x) && (26.0..239.).contains(&y)
                }) {
                    for tip in tips {
                        draw::line(ctx, &identity(), &base, &tip, 11, s);
                    }
                }
            }
            self.model.draw(ctx, state(self.lit.then_some(&shade)));
        });
        hud(
            &self.image,
            8.,
            8.,
            "Arrows: Rotate  S: Shading  Q: Quit",
            7,
        );
        hud(&self.image, 8., 18., "1: Walk  2: Run  3: Eat", 7);
        self.image.clone()
    }
}
