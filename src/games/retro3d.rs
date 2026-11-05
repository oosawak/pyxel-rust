//! Browser demo of the official Cube renderer through the Canvas backend.
use crate::cube::{draw, Camera, Mat4, Vec3};
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

thread_local! {
    static CONTROLS: RefCell<Controls> = RefCell::new(Controls::default());
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
            });
            blit_3d(&target, 0.0, 0.0);
            text(5.0, 5.0, "PYXEL CUBE 3.0", 7);
            text(5.0, 108.0, "RUST + WASM", 6);
        },
    );
}
