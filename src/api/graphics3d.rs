//! Pyxel Cube rendering for SDL2 and Canvas/WASM backends.
//!
//! Draw commands use the official `crate::cube::draw` API. The target can be
//! the screen or an offscreen image. Camera buffers are reused between frames.

use crate::cube::{raster, scene::DrawContext, RcCamera};
use pyxel::RcImage;

// Return the camera's scratch buffers even if a drawing callback unwinds.
struct RenderFrame(DrawContext);

impl Drop for RenderFrame {
    fn drop(&mut self) {
        let mut camera = self.0.camera.borrow_mut();
        camera.depth = std::mem::take(&mut self.0.depth);
        camera.vertex_scratch = std::mem::take(&mut self.0.vertex_cache);
    }
}

/// Render a full image using a Cube camera, resetting depth for this frame.
///
/// The camera's `clear_color` controls whether the image is cleared. With
/// `None`, existing 2D drawing is preserved. Draw 2D HUD elements afterwards.
/// Release any `RefCell` borrows of the camera and target before calling.
pub fn render_3d(target: &RcImage, camera: &RcCamera, draw: impl FnOnce(&mut DrawContext)) {
    let (width, height) = {
        let image = target.borrow();
        (image.width(), image.height())
    };
    let (vp, clip_row, depth, vertex_cache, clear_color) = {
        let mut cam = camera.borrow_mut();
        cam.ensure_depth(width, height);
        cam.clear_depth();
        let view = raster::view_matrix(&cam);
        (
            raster::matmul(
                &raster::projection_matrix(&cam, width as f32, height as f32),
                &view,
            ),
            raster::camera_clip_row(&view),
            std::mem::take(&mut cam.depth),
            std::mem::take(&mut cam.vertex_scratch),
            cam.clear_color,
        )
    };
    if let Some(color) = clear_color {
        target.borrow_mut().clear(color as u8);
    }
    let context = DrawContext {
        target: target.clone(),
        camera: camera.clone(),
        vp,
        clip_row,
        vp_x: 0.0,
        vp_y: 0.0,
        vp_w: width as f32,
        vp_h: height as f32,
        clip: raster::compute_clip_rect(0.0, 0.0, width as f32, height as f32, width, height),
        depth,
        depth_w: width,
        depth_h: height,
        vertex_cache,
        dither_alpha: 1.0,
        depth_test: true,
        depth_write: true,
        depth_offset: 0.0,
        decal_distance: None,
        shaded: false,
    };
    let mut frame = RenderFrame(context);
    draw(&mut frame.0);
}

/// Copy a rendered image onto the active backend, then draw 2D HUD elements.
/// Uses the backend's camera and clipping rules, like other 2D drawing calls.
pub fn blit_3d(target: &RcImage, x: f32, y: f32) {
    let image = target.borrow();
    for row in 0..image.height() {
        for col in 0..image.width() {
            crate::api::pset(
                x + col as f32,
                y + row as f32,
                image.pixel(col as f32, row as f32),
            );
        }
    }
}
