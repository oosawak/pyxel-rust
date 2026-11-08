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
    let image = target.borrow();
    let (w, h) = (image.width(), image.height());
    drop(image);
    render_3d_viewport(target, camera, [0., 0., w as f32, h as f32], draw)
        .expect("Invalid viewport");
}

/// Render into a viewport; camera scratch buffers are returned even during unwinding.
pub fn render_3d_viewport<R>(
    target: &RcImage,
    camera: &RcCamera,
    viewport: [f32; 4],
    draw: impl FnOnce(&mut DrawContext) -> R,
) -> Result<R, String> {
    let [x, y, vw, vh] = viewport;
    if !viewport.iter().all(|v| v.is_finite()) || vw <= 0. || vh <= 0. {
        return Err("Invalid viewport".into());
    }
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
            raster::matmul(&raster::projection_matrix(&cam, vw, vh), &view),
            raster::camera_clip_row(&view),
            std::mem::take(&mut cam.depth),
            std::mem::take(&mut cam.vertex_scratch),
            cam.clear_color,
        )
    };
    if let Some(color) = clear_color {
        let clip = raster::compute_clip_rect(x, y, vw, vh, width, height);
        raster::clear_viewport(&mut target.borrow_mut(), &clip, color as u8);
    }
    let context = DrawContext {
        target: target.clone(),
        camera: camera.clone(),
        vp,
        clip_row,
        vp_x: x,
        vp_y: y,
        vp_w: vw,
        vp_h: vh,
        clip: raster::compute_clip_rect(x, y, vw, vh, width, height),
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
    Ok(draw(&mut frame.0))
}

/// Copy a rendered image onto the active backend, then draw 2D HUD elements.
/// Uses the backend's camera and clipping rules, like other 2D drawing calls.
pub fn blit_3d(target: &RcImage, x: f32, y: f32) {
    let (width, height) = {
        let image = target.borrow();
        (image.width(), image.height())
    };
    super::resource::screen().borrow_mut().draw_image(
        x,
        y,
        target,
        0.,
        0.,
        width as f32,
        height as f32,
        None,
        None,
        None,
    );
}

/// Draw a static mesh, preserving its node transforms and per-part materials.
/// Imported animation tracks can be applied to the mesh before drawing.
pub fn draw_mesh_3d(
    context: &mut DrawContext,
    mesh: &crate::cube::Mesh,
    transform: &crate::cube::Mat4,
    state: crate::cube::draw::DrawState<'_>,
) -> Result<(), String> {
    mesh.validate()?;
    let transforms = mesh.compose_world_transforms(transform);
    for (index, primitive) in mesh.primitives.iter().enumerate() {
        let Some(primitive) = primitive else {
            continue;
        };
        let primitive = primitive.borrow();
        let material = mesh.material_for_part(index);
        let (color, image) = material.col_img.as_flat_and_image();
        crate::cube::draw::prim(
            context,
            &transforms[index],
            primitive.mode,
            primitive.cull,
            &primitive.positions,
            (!primitive.indices.is_empty()).then_some(primitive.indices.as_slice()),
            (!primitive.normals.is_empty()).then_some(primitive.normals.as_slice()),
            (!primitive.uvs.is_empty()).then_some(primitive.uvs.as_slice()),
            color,
            image.as_ref(),
            material.colkey,
            state,
        )
        .map_err(str::to_owned)?;
    }
    Ok(())
}
