//! Rust ports of the six MIT-licensed Pyxel 3.0 Cube examples.
//! See assets/UPSTREAM.md for provenance and third-party model attribution.
mod collision;
mod motion;
mod physics;
mod shapes;

use crate::api::{draw_mesh_3d, render_3d, MeshAnimationPlayer};
use crate::cube::{
    draw, scene::DrawContext, Camera, Collider, Mat4, Mesh, Node, Primitive, RcCamera, RcCollider,
    RcMat4, RcMesh, RcNode, RcPrimitive, RcShading, Scene, Shading, Vec3,
};
use crate::Image;
use std::{cell::RefCell, rc::Rc};
#[cfg(feature = "wasm-backend")]
use wasm_bindgen::prelude::*;
type RcImage = Rc<RefCell<Image>>;

pub(super) fn v(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}
pub(super) fn add(a: Vec3, b: Vec3) -> Vec3 {
    v(a.x + b.x, a.y + b.y, a.z + b.z)
}
pub(super) fn sub(a: Vec3, b: Vec3) -> Vec3 {
    v(a.x - b.x, a.y - b.y, a.z - b.z)
}
pub(super) fn mul(a: Vec3, b: f32) -> Vec3 {
    v(a.x * b, a.y * b, a.z * b)
}
pub(super) fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
pub(super) fn length(a: Vec3) -> f32 {
    dot(a, a).sqrt()
}
pub(super) fn norm(a: Vec3) -> Vec3 {
    let d = length(a);
    if d > 1e-8 {
        mul(a, 1.0 / d)
    } else {
        v(0., 0., 0.)
    }
}
pub(super) fn lerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    add(a, mul(sub(b, a), t))
}
pub(super) fn cross(a: Vec3, b: Vec3) -> Vec3 {
    v(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}
pub(super) fn sin(a: f32) -> f32 {
    a.to_radians().sin()
}
pub(super) fn cos(a: f32) -> f32 {
    a.to_radians().cos()
}
pub(super) fn mat(m: RcMat4) -> Mat4 {
    *m.borrow()
}
pub(super) fn tr(a: Vec3) -> Mat4 {
    mat(Mat4::from_translation(&a))
}
pub(super) fn rot(a: Vec3) -> Mat4 {
    mat(Mat4::from_euler(&a))
}
pub(super) fn mm(a: Mat4, b: Mat4) -> Mat4 {
    a.mul_mat_value(&b)
}
pub(super) fn identity() -> Mat4 {
    Mat4::identity_value()
}
pub(super) fn pos(n: &RcNode) -> Vec3 {
    n.borrow().transform.borrow().pos_value()
}
pub(super) fn set_mat(n: &RcNode, m: Mat4) {
    *n.borrow().transform.borrow_mut() = m;
}
pub(super) fn world(n: &RcNode) -> Mat4 {
    Node::world_transform_value(n)
}
pub(super) fn camera(cam: &RcCamera, eye: Vec3, target: Vec3) {
    *cam.borrow().transform.borrow_mut() = mat(Mat4::look_at(&eye, &target, &v(0., 1., 0.)));
}
const PALETTE: [u32; 16] = [
    0x000000, 0x2b335f, 0x7e2072, 0x19959c, 0x8b4852, 0x395c98, 0xa9c1ff, 0xeeeeee, 0xd4186c,
    0xd38441, 0xe9c35b, 0x70c6a9, 0x7696de, 0xa3a3a3, 0xff9798, 0xedc7b0,
];
pub(super) fn shading() -> RcShading {
    Shading::new(&PALETTE)
}
pub(super) fn state(shade: Option<&Shading>) -> draw::DrawState<'_> {
    draw::DrawState {
        shaded: true,
        shading: shade,
        ..draw::DrawState::unshaded()
    }
}
pub(super) fn primitive(
    ctx: &mut DrawContext,
    m: Mat4,
    p: &Primitive,
    col: i32,
    img: Option<&RcImage>,
    key: Option<i32>,
    s: draw::DrawState<'_>,
) {
    draw::prim(
        ctx,
        &m,
        p.mode,
        p.cull,
        &p.positions,
        (!p.indices.is_empty()).then_some(p.indices.as_slice()),
        (!p.normals.is_empty()).then_some(p.normals.as_slice()),
        (!p.uvs.is_empty()).then_some(p.uvs.as_slice()),
        col,
        img,
        key,
        s,
    )
    .expect("valid example geometry");
}
pub(super) fn hud(image: &RcImage, x: f32, y: f32, s: &str, col: u8) {
    image.borrow_mut().draw_text(x, y, s, col, None);
}
pub(super) fn collider(
    size: Vec3,
    radius: f32,
    mass: f32,
    mesh: Option<RcMesh>,
    trigger: bool,
    rolls: bool,
) -> RcCollider {
    Collider::new(
        Vec3::new(size.x, size.y, size.z),
        radius,
        mesh,
        trigger,
        rolls,
        mass,
        0.,
        0.5,
        Vec3::zero(),
        Vec3::zero(),
        9.8,
        Vec3::down(),
        0.1,
        0.1,
    )
}
pub(super) fn load(name: &str) -> Result<RcMesh, String> {
    #[cfg(feature = "pyxel-core-backend")]
    {
        let file = match name {
            "gazelle" => "gazelle.glb",
            "stage" => "garden_stage.glb",
            "stage_collision" => "garden_stage_collision.glb",
            "platform" => "garden_platform.glb",
            "platform_collision" => "garden_platform_collision.glb",
            "player" => "garden_player.glb",
            "coin" => "garden_coin.glb",
            _ => return Err(format!("Unknown example asset {name}")),
        };
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/games/cube_examples/assets")
            .join(file);
        Mesh::from_glb(path.to_str().ok_or("Asset path is not UTF-8")?, None, 30.)
    }
    #[cfg(not(feature = "pyxel-core-backend"))]
    {
        let bytes: &[u8] = match name {
            "gazelle" => include_bytes!("assets/gazelle.glb"),
            "stage" => include_bytes!("assets/garden_stage.glb"),
            "stage_collision" => include_bytes!("assets/garden_stage_collision.glb"),
            "platform" => include_bytes!("assets/garden_platform.glb"),
            "platform_collision" => include_bytes!("assets/garden_platform_collision.glb"),
            "player" => include_bytes!("assets/garden_player.glb"),
            "coin" => include_bytes!("assets/garden_coin.glb"),
            _ => return Err(format!("Unknown example asset {name}")),
        };
        Mesh::from_glb_bytes(bytes, None, 30.)
    }
}
pub(super) fn texture(bytes: &[u8], w: u32, h: u32) -> RcImage {
    let image = Image::new(w, h);
    for y in 0..h {
        for x in 0..w {
            image
                .borrow_mut()
                .set_pixel(x as f32, y as f32, bytes[(y * w + x) as usize]);
        }
    }
    image
}

/// An independent node hierarchy for an imported mesh; geometry may be shared.
pub(super) struct Model {
    pub node: RcNode,
    pub mesh: RcMesh,
    pub parts: Vec<RcNode>,
    pub player: MeshAnimationPlayer,
}
impl Model {
    pub fn new(mesh: RcMesh) -> Result<Self, String> {
        let node = Node::new();
        let m = mesh.borrow();
        let mut parts = Vec::new();
        for (i, t) in m.transforms.iter().enumerate() {
            let n = Node::new();
            set_mat(&n, *t.borrow());
            n.borrow_mut().name = m.names.get(i).cloned().unwrap_or_default();
            parts.push(n);
        }
        for (i, n) in parts.iter().enumerate() {
            let parent = m.parents[i];
            Node::add_child(
                if parent < 0 {
                    &node
                } else {
                    &parts[parent as usize]
                },
                n,
            );
        }
        drop(m);
        let player = MeshAnimationPlayer::new(mesh.clone(), 30.)?;
        Ok(Self {
            node,
            mesh,
            parts,
            player,
        })
    }
    pub fn sync(&self) {
        for (n, t) in self.parts.iter().zip(&self.mesh.borrow().transforms) {
            set_mat(n, *t.borrow());
        }
    }
    pub fn find(&self, name: &str) -> Result<RcNode, String> {
        Node::find_by_name(&self.node, name)
            .into_iter()
            .next()
            .ok_or(format!("Model node {name} is missing"))
    }
    pub fn draw(&self, ctx: &mut DrawContext, s: draw::DrawState<'_>) {
        if Node::effective_visible(&self.node) {
            draw_mesh_3d(ctx, &self.mesh.borrow(), &world(&self.node), s)
                .expect("valid example mesh");
        }
    }
}

#[derive(Clone, Default)]
pub struct Controls {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub action: bool,
    pub pressed: bool,
    pub released: bool,
    pub shade: bool,
    pub reset: bool,
    pub motion: Option<usize>,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub pointer: bool,
    pub sound_events: Vec<u8>,
}

pub enum CubeExample {
    Hello(shapes::Hello),
    Shapes(shapes::Shapes),
    Custom(shapes::Custom),
    Motion(motion::MotionDemo),
    Collision(collision::CollisionDemo),
    Physics(physics::PhysicsDemo),
}
impl CubeExample {
    pub fn new(id: usize) -> Result<Self, String> {
        Ok(match id {
            0 => Self::Hello(shapes::Hello::new()),
            1 => Self::Shapes(shapes::Shapes::new()),
            2 => Self::Custom(shapes::Custom::new()),
            3 => Self::Motion(motion::MotionDemo::new()?),
            4 => Self::Collision(collision::CollisionDemo::new()?),
            5 => Self::Physics(physics::PhysicsDemo::new()),
            _ => return Err("Example index must be 0..5".into()),
        })
    }
    pub fn size(&self) -> (u32, u32) {
        match self {
            Self::Hello(_) => (200, 150),
            Self::Shapes(_) => (240, 240),
            Self::Custom(_) => (256, 192),
            _ => (320, 240),
        }
    }
    pub fn update(&mut self, input: &mut Controls) {
        match self {
            Self::Hello(d) => d.update(),
            Self::Shapes(d) => d.update(input),
            Self::Custom(d) => d.update(input),
            Self::Motion(d) => d.update(input),
            Self::Collision(d) => d.update(input),
            Self::Physics(d) => d.update(input),
        }
    }
    pub fn draw(&mut self) -> RcImage {
        match self {
            Self::Hello(d) => d.draw(),
            Self::Shapes(d) => d.draw(),
            Self::Custom(d) => d.draw(),
            Self::Motion(d) => d.draw(),
            Self::Collision(d) => d.draw(),
            Self::Physics(d) => d.draw(),
        }
    }
    pub fn info(&self) -> serde_json::Value {
        match self {
            Self::Hello(d) => serde_json::json!({"frame":d.frame}),
            Self::Shapes(d) => serde_json::json!({"frame":d.frame,"wire":d.wire}),
            Self::Custom(d) => {
                serde_json::json!({"frame":d.frame,"locked":d.locked,"shots":d.targets.len(),"age":d.age,"centers":d.screen_centers()})
            }
            Self::Motion(d) => {
                serde_json::json!({"frame":d.frame,"motion":d.selected,"time":d.model.player.elapsed()})
            }
            Self::Collision(d) => d.info(),
            Self::Physics(d) => d.info(),
        }
    }
}

#[cfg(feature = "wasm-backend")]
thread_local! {static EXAMPLE:RefCell<Option<CubeExample>>=const{RefCell::new(None)};static INPUT:RefCell<Controls>=RefCell::new(Controls::default());static PAUSED:RefCell<bool>=const{RefCell::new(false)};static EVENTS:RefCell<Vec<u8>>=const{RefCell::new(Vec::new())};}
#[cfg(feature = "wasm-backend")]
#[wasm_bindgen]
pub fn start_cube_example(id: usize) -> Result<(), JsValue> {
    if EXAMPLE.with(|e| e.borrow().is_some()) {
        return Err(JsValue::from_str(
            "Reload the demo frame to start another example",
        ));
    }
    let example = CubeExample::new(id).map_err(|e| JsValue::from_str(&e))?;
    let (w, h) = example.size();
    EXAMPLE.with(|e| *e.borrow_mut() = Some(example));
    crate::api::init(w, h, "Official Cube example — Rust", 30);
    crate::api::run(
        || {
            let paused = PAUSED.with(|p| *p.borrow());
            if !paused {
                INPUT.with(|i| {
                    EXAMPLE.with(|e| {
                        let mut input = i.borrow_mut();
                        if let Some(d) = e.borrow_mut().as_mut() {
                            d.update(&mut input);
                        }
                        EVENTS.with(|events| {
                            events.borrow_mut().extend(input.sound_events.drain(..))
                        });
                        input.pressed = false;
                        input.released = false;
                        input.shade = false;
                        input.reset = false;
                        input.motion = None;
                    })
                });
            }
        },
        || {
            EXAMPLE.with(|e| {
                if let Some(d) = e.borrow_mut().as_mut() {
                    let image = d.draw();
                    crate::api::blit_3d(&image, 0., 0.);
                }
            });
        },
    );
    Ok(())
}
#[cfg(feature = "wasm-backend")]
#[wasm_bindgen]
pub fn cube_example_input(json: &str) -> Result<(), JsValue> {
    let data: serde_json::Value =
        serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    INPUT.with(|i| {
        let mut i = i.borrow_mut();
        macro_rules! flag {
            ($field:ident) => {
                if let Some(b) = data.get(stringify!($field)).and_then(|x| x.as_bool()) {
                    i.$field = b;
                }
            };
        }
        flag!(left);
        flag!(right);
        flag!(up);
        flag!(down);
        flag!(action);
        flag!(pointer);
        for name in ["pressed", "released", "shade", "reset"] {
            if data.get(name).and_then(|x| x.as_bool()) == Some(true) {
                match name {
                    "pressed" => i.pressed = true,
                    "released" => i.released = true,
                    "shade" => i.shade = true,
                    _ => i.reset = true,
                }
            }
        }
        if let Some(n) = data.get("motion").and_then(|x| x.as_u64()) {
            i.motion = Some((n as usize).min(2));
        }
        if let Some(x) = data.get("mouse_x").and_then(|x| x.as_f64()) {
            i.mouse_x = x as f32;
        }
        if let Some(y) = data.get("mouse_y").and_then(|x| x.as_f64()) {
            i.mouse_y = y as f32;
        }
    });
    Ok(())
}
#[cfg(feature = "wasm-backend")]
#[wasm_bindgen]
pub fn pause_cube_example(paused: bool) {
    PAUSED.with(|p| *p.borrow_mut() = paused);
}
#[cfg(feature = "wasm-backend")]
#[wasm_bindgen]
pub fn cube_example_info() -> String {
    EXAMPLE.with(|e| {
        e.borrow()
            .as_ref()
            .map(|d| d.info().to_string())
            .unwrap_or("null".into())
    })
}
#[cfg(feature = "wasm-backend")]
#[wasm_bindgen]
pub fn cube_example_sounds() -> String {
    EVENTS.with(|e| serde_json::to_string(&std::mem::take(&mut *e.borrow_mut())).unwrap())
}
