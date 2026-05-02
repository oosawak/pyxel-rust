/// Backend abstraction for pyxel-rust
///
/// Each backend implements the same public API:
///   - `pyxel-core-backend`: wraps pyxel-core / SDL2 (native only)
///   - `wgpu-backend`:        wgpu + winit  (native + WASM)
///
/// Game code never touches this module directly — use `prelude::*`.

#[cfg(feature = "wgpu-backend")]
pub mod wgpu_backend;

#[cfg(feature = "wasm-backend")]
pub mod wasm_backend;

/// Shared 4×6 bitmap font used by all software renderers
#[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))]
pub mod font;

/// Core trait every backend must implement.
/// Used for documentation and future dynamic dispatch.
pub trait Backend {
    // --- System ---
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn frame_count(&self) -> u32;
    fn quit(&mut self);

    // --- Graphics ---
    fn clear(&mut self, col: u8);
    fn pset(&mut self, x: f32, y: f32, col: u8);
    fn pget(&self, x: f32, y: f32) -> u8;
    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, col: u8);
    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, col: u8);
    fn rectfill(&mut self, x: f32, y: f32, w: f32, h: f32, col: u8);
    fn circ(&mut self, x: f32, y: f32, r: f32, col: u8);
    fn circfill(&mut self, x: f32, y: f32, r: f32, col: u8);
    fn text(&mut self, x: f32, y: f32, s: &str, col: u8);
    fn blt(&mut self, x: f32, y: f32, img: u32, sx: f32, sy: f32, sw: f32, sh: f32, colkey: Option<u8>);
    fn bltm(&mut self, x: f32, y: f32, tm: u32, mx: f32, my: f32, mw: f32, mh: f32, colkey: Option<u8>);
    fn clip(&mut self, x: f32, y: f32, w: f32, h: f32);
    fn clip_reset(&mut self);
    fn pal(&mut self, col1: u8, col2: u8);
    fn pal_reset(&mut self);
    fn dither(&mut self, alpha: f32);
    fn camera(&mut self, x: f32, y: f32);
    fn camera_reset(&mut self);

    // --- Input ---
    fn btn(&self, key: u32) -> bool;
    fn btnp(&self, key: u32) -> bool;
    fn btnr(&self, key: u32) -> bool;
    fn mouse_x(&self) -> i32;
    fn mouse_y(&self) -> i32;
    fn set_mouse_visible(&mut self, visible: bool);

    // --- Audio ---
    fn play(&mut self, ch: u32, snd: u32, sec: Option<f32>, loop_: bool, resume: bool);
    fn playm(&mut self, msc: u32, sec: Option<f32>, loop_: bool);
    fn stop(&mut self, ch: Option<u32>);
    fn is_playing(&self, ch: u32) -> bool;
}
