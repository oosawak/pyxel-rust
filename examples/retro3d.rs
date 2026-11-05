//! Rotating solid cube using the official Pyxel 3.0 software renderer.
use pyxel_rust::cube::{draw, Camera, Mat4, Vec3};
use pyxel_rust::prelude::*;

fn main() {
    init(160, 120, "pyxel-rust / Pyxel Cube 3.0", 30);
    let camera = Camera::new();
    camera.borrow_mut().clear_color = Some(1);
    run(
        || {},
        move || {
            let angle = frame_count() as f32 * 2.0;
            let transform = Mat4::from_translation(&Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            });
            let transform = transform.borrow().rotate_y(angle);
            let transform = transform.borrow().rotate_x(angle * 0.6);
            render_3d(&pyxel_rust::screen(), &camera, |ctx| {
                draw::box_solid(
                    ctx,
                    &transform.borrow(),
                    &Vec3 {
                        x: 2.0,
                        y: 2.0,
                        z: 2.0,
                    },
                    10,
                    None,
                    None,
                    draw::DrawState::unshaded(),
                );
                draw::boxb(
                    ctx,
                    &transform.borrow(),
                    &Vec3 {
                        x: 2.0,
                        y: 2.0,
                        z: 2.0,
                    },
                    7,
                    draw::DrawState::unshaded(),
                );
            });
            text(4.0, 4.0, "PYXEL CUBE 3.0", 7);
        },
    );
}
