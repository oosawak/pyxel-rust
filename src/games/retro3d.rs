//! Browser demo of the official Cube renderer through the Canvas backend.
use crate::cube::{draw, Camera, Mat4, Mesh, RcMesh, Vec3};
use crate::prelude::*;
use crate::Image;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
struct Controls {
    angle: f32,
    distance: f32,
    paused: bool,
    started: bool,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            angle: 25.0,
            distance: 7.0,
            paused: false,
            started: false,
        }
    }
}

#[derive(Clone)]
struct LoadedModel {
    mesh: RcMesh,
    fit: crate::cube::RcMat4,
}

thread_local! {
    static MODEL: RefCell<Option<LoadedModel>> = const { RefCell::new(None) };
    static CONTROLS: RefCell<Controls> = RefCell::new(Controls::default());
}

/// Import a self-contained GLB from browser-provided bytes. Failed imports retain
/// the previous model so a malformed selection cannot blank the current scene.
#[wasm_bindgen]
pub fn load_retro3d_glb(bytes: &[u8]) -> Result<u32, JsValue> {
    if bytes.len() > 16 * 1024 * 1024 {
        return Err(JsValue::from_str("GLB must be at most 16 MiB"));
    }
    let mesh = Mesh::from_glb_bytes(bytes, None, 30.0).map_err(|e| JsValue::from_str(&e))?;
    let (bounds, triangles) = {
        let mesh = mesh.borrow();
        let triangles: usize = mesh
            .primitives
            .iter()
            .flatten()
            .map(|p| {
                let p = p.borrow();
                if p.indices.is_empty() {
                    p.positions.len() / 9
                } else {
                    p.indices.len() / 3
                }
            })
            .sum();
        (mesh.local_aabb(), triangles)
    };
    let min = [bounds.min.x, bounds.min.y, bounds.min.z];
    let max = [bounds.max.x, bounds.max.y, bounds.max.z];
    if triangles == 0 || min.iter().chain(max.iter()).any(|n| !n.is_finite()) {
        return Err(JsValue::from_str(
            "GLB must contain finite triangle geometry",
        ));
    }
    let extent = (0..3)
        .map(|i| f64::from(max[i]) - f64::from(min[i]))
        .fold(0.0, f64::max);
    let scale = (2.4 / extent) as f32;
    if extent <= 0.0 || !scale.is_finite() || scale <= 0.0 {
        return Err(JsValue::from_str(
            "GLB bounds cannot be fitted into the view",
        ));
    }
    let center: [f32; 3] =
        std::array::from_fn(|i| ((f64::from(min[i]) + f64::from(max[i])) * 0.5) as f32);
    let fit = Mat4::from_rows([
        [scale, 0.0, 0.0, -center[0] * scale],
        [0.0, scale, 0.0, -center[1] * scale],
        [0.0, 0.0, scale, -center[2] * scale],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    MODEL.with(|model| *model.borrow_mut() = Some(LoadedModel { mesh, fit }));
    Ok(triangles as u32)
}

#[wasm_bindgen]
pub fn clear_retro3d_model() {
    MODEL.with(|model| *model.borrow_mut() = None);
}

#[wasm_bindgen]
pub fn retro3d_has_model() -> bool {
    MODEL.with(|model| model.borrow().is_some())
}

#[wasm_bindgen]
pub fn set_retro3d_paused(paused: bool) {
    CONTROLS.with(|c| c.borrow_mut().paused = paused);
}

#[wasm_bindgen]
pub fn retro3d_paused() -> bool {
    CONTROLS.with(|c| c.borrow().paused)
}

#[wasm_bindgen]
pub fn retro3d_angle() -> f32 {
    CONTROLS.with(|c| c.borrow().angle)
}

#[wasm_bindgen]
pub fn retro3d_distance() -> f32 {
    CONTROLS.with(|c| c.borrow().distance)
}

#[wasm_bindgen]
pub fn rotate_retro3d(degrees: f32) {
    if degrees.is_finite() {
        CONTROLS.with(|c| {
            let mut controls = c.borrow_mut();
            controls.angle = (controls.angle + degrees).rem_euclid(360.0);
        });
    }
}

#[wasm_bindgen]
pub fn set_retro3d_distance(distance: f32) {
    if distance.is_finite() {
        CONTROLS.with(|c| c.borrow_mut().distance = distance.clamp(5.0, 10.0));
    }
}

#[wasm_bindgen]
pub fn reset_retro3d() {
    CONTROLS.with(|c| {
        let started = c.borrow().started;
        *c.borrow_mut() = Controls {
            started,
            ..Controls::default()
        };
    });
}

#[wasm_bindgen]
pub fn start_retro3d() {
    let already_started = CONTROLS.with(|c| {
        let mut controls = c.borrow_mut();
        let started = controls.started;
        controls.started = true;
        started
    });
    if already_started {
        return;
    }
    init(160, 120, "Pyxel Cube / Browser", 30);
    let target = Image::new(160, 120);
    let camera = Camera::new();
    camera.borrow_mut().clear_color = Some(1);
    run(
        || {
            CONTROLS.with(|c| {
                let mut controls = c.borrow_mut();
                if btnp(KEY_SPACE) {
                    controls.paused = !controls.paused;
                }
                if btnp(KEY_R) {
                    *controls = Controls {
                        started: true,
                        ..Controls::default()
                    };
                }
                let manual = if btn(KEY_LEFT) {
                    -3.0
                } else if btn(KEY_RIGHT) {
                    3.0
                } else {
                    0.0
                };
                if !controls.paused || manual != 0.0 {
                    controls.angle = (controls.angle + if manual != 0.0 { manual } else { 1.0 })
                        .rem_euclid(360.0);
                }
            });
        },
        move || {
            let controls = CONTROLS.with(|c| *c.borrow());
            render_3d(&target, &camera, |ctx| {
                let state = draw::DrawState::unshaded();
                let identity = Mat4::identity_value();
                for x in -6..=6 {
                    draw::line(
                        ctx,
                        &identity,
                        &Vec3 {
                            x: x as f32,
                            y: -1.2,
                            z: -3.0,
                        },
                        &Vec3 {
                            x: x as f32,
                            y: -1.2,
                            z: -13.0,
                        },
                        5,
                        state,
                    );
                }
                for z in -13..=-3 {
                    draw::line(
                        ctx,
                        &identity,
                        &Vec3 {
                            x: -6.0,
                            y: -1.2,
                            z: z as f32,
                        },
                        &Vec3 {
                            x: 6.0,
                            y: -1.2,
                            z: z as f32,
                        },
                        5,
                        state,
                    );
                }
                let model = MODEL.with(|model| model.borrow().clone());
                if let Some(model) = model {
                    let transform = Mat4::from_translation(&Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: -controls.distance,
                    });
                    let transform = transform.borrow().rotate_y(controls.angle);
                    let transform = transform.borrow().mul_mat(&model.fit.borrow());
                    draw_mesh_3d(ctx, &model.mesh.borrow(), &transform.borrow(), state)
                        .expect("GLB import validates primitive buffers");
                } else {
                    for (x, color, offset) in [(-2.1, 12, -30.0), (0.0, 10, 0.0), (2.1, 14, 30.0)] {
                        let transform = Mat4::from_translation(&Vec3 {
                            x,
                            y: 0.0,
                            z: -controls.distance - x.abs() * 0.35,
                        });
                        let transform = transform.borrow().rotate_y(controls.angle + offset);
                        let transform = transform.borrow().rotate_x(controls.angle * 0.6);
                        let size = Vec3 {
                            x: 1.5,
                            y: 1.5,
                            z: 1.5,
                        };
                        draw::box_solid(ctx, &transform.borrow(), &size, color, None, None, state);
                        draw::boxb(ctx, &transform.borrow(), &size, 7, state);
                    }
                }
            });
            blit_3d(&target, 0.0, 0.0);
            text(5.0, 5.0, "PYXEL CUBE 3.0", 7);
            text(5.0, 108.0, "RUST + WASM", 6);
        },
    );
}
