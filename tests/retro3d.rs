#![cfg(feature = "pyxel-core-backend")]

use pyxel_rust::cube::{draw, Camera, Mat4, Vec3};
use pyxel_rust::{api::render_3d, Image};

fn rectangle(ctx: &mut pyxel_rust::cube::scene::DrawContext, z: f32, color: i32) {
    let transform = Mat4::from_translation(&Vec3 { x: 0.0, y: 0.0, z });
    draw::rect(
        ctx,
        &transform.borrow(),
        2.0,
        2.0,
        color,
        draw::DrawState::unshaded(),
    );
}

#[test]
fn renders_cube_with_perspective_and_resets_depth_between_frames() {
    let target = Image::new(64, 64);
    let camera = Camera::new();
    camera.borrow_mut().clear_color = Some(1);
    render_3d(&target, &camera, |ctx| {
        let transform = Mat4::from_translation(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: -4.0,
        });
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
    });
    assert_eq!(target.borrow().pixel(32.0, 32.0), 10);
    assert_eq!(target.borrow().pixel(0.0, 0.0), 1);
    assert!(camera.borrow().depth.iter().any(|depth| depth.is_finite()));

    // A farther object must appear in the next frame despite the previous depth.
    render_3d(&target, &camera, |ctx| rectangle(ctx, -6.0, 8));
    assert_eq!(target.borrow().pixel(32.0, 32.0), 8);
}

#[test]
fn occlusion_is_independent_of_draw_order() {
    let target = Image::new(64, 64);
    let camera = Camera::new();
    for near_first in [true, false] {
        render_3d(&target, &camera, |ctx| {
            if near_first {
                rectangle(ctx, -3.0, 8);
                rectangle(ctx, -6.0, 10);
            } else {
                rectangle(ctx, -6.0, 10);
                rectangle(ctx, -3.0, 8);
            }
        });
        assert_eq!(target.borrow().pixel(32.0, 32.0), 8);
    }
}

#[test]
fn preserves_2d_background_and_resizes_camera_buffers() {
    let camera = Camera::new();
    for (width, height) in [(64, 64), (96, 48)] {
        let target = Image::new(width, height);
        target.borrow_mut().clear(12);
        render_3d(&target, &camera, |ctx| rectangle(ctx, -4.0, 7));
        assert_eq!(target.borrow().pixel(0.0, 0.0), 12);
        assert_eq!(
            target
                .borrow()
                .pixel((width / 2) as f32, (height / 2) as f32),
            7
        );
        assert_eq!(camera.borrow().depth.len(), (width * height) as usize);
    }
}

#[test]
fn camera_remains_usable_after_callback_panic() {
    let target = Image::new(64, 64);
    let camera = Camera::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        render_3d(&target, &camera, |_| panic!("drawing failed"));
    }));
    assert!(result.is_err());
    render_3d(&target, &camera, |ctx| rectangle(ctx, -4.0, 7));
    assert_eq!(target.borrow().pixel(32.0, 32.0), 7);
}
