//! WasmBackend — wasm-bindgen + canvas 2D rendering
//!
//! Rendering pipeline:
//!   1. Game calls drawing primitives → writes color indices into `pixel_buffer`
//!   2. Each frame: map indices through palette → RGBA → ImageData
//!   3. `context.put_image_data()` renders to an HTML canvas element
//!
//! Entry: the browser calls main() via wasm-bindgen, which calls init() then run().

use js_sys::Uint8ClampedArray;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};
pub mod audio;
pub mod system;
pub mod text_files;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, KeyboardEvent, MouseEvent};

// ── Game state atomics (set each frame by game logic, read by JS) ────────────
// get_game_state: 0=other, 1=victory, 2=battle-in-progress
static GAME_STATE: AtomicI32 = AtomicI32::new(0);
static PLAYER_HP: AtomicI32 = AtomicI32::new(128);
static PLAYER_MAX_HP: AtomicI32 = AtomicI32::new(128);
static PLAYER_MP: AtomicI32 = AtomicI32::new(54);
static PLAYER_MAX_MP: AtomicI32 = AtomicI32::new(54);
static PLAYER_LEVEL: AtomicI32 = AtomicI32::new(1);
static PLAYER_FLASH: AtomicI32 = AtomicI32::new(0);
static ENEMY_IDX: AtomicI32 = AtomicI32::new(-1);
static ENEMY_FLASH: AtomicI32 = AtomicI32::new(0);
static ENEMY_HP: AtomicI32 = AtomicI32::new(0);
static ENEMY_MAX_HP: AtomicI32 = AtomicI32::new(0);
static BG_IDX: AtomicI32 = AtomicI32::new(0);
/// JS calls start_battle_from_js(idx) → sets this; game picks it up next frame
static BATTLE_REQUEST: AtomicI32 = AtomicI32::new(-1);

// ── JS → Rust: exported getters ──────────────────────────────────────────────
#[wasm_bindgen]
pub fn get_game_state() -> i32 {
    GAME_STATE.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_player_hp() -> i32 {
    PLAYER_HP.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_player_max_hp() -> i32 {
    PLAYER_MAX_HP.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_player_mp() -> i32 {
    PLAYER_MP.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_player_max_mp() -> i32 {
    PLAYER_MAX_MP.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_player_level() -> i32 {
    PLAYER_LEVEL.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_player_flash() -> i32 {
    PLAYER_FLASH.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_enemy_idx() -> i32 {
    ENEMY_IDX.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_enemy_flash() -> i32 {
    ENEMY_FLASH.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_enemy_hp() -> i32 {
    ENEMY_HP.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_enemy_max_hp() -> i32 {
    ENEMY_MAX_HP.load(Ordering::Relaxed)
}
#[wasm_bindgen]
pub fn get_bg_idx() -> i32 {
    BG_IDX.load(Ordering::Relaxed)
}

/// Called from JS to start a battle against enemy at given index.
/// The game loop picks this up next frame and transitions to Battle state.
#[wasm_bindgen]
pub fn start_battle_from_js(idx: i32) {
    BATTLE_REQUEST.store(idx, Ordering::Relaxed);
}

/// WASM entry point — JS calls `main()` after `await init()` to start the Arisa Quest game.
#[wasm_bindgen]
pub fn main() {
    crate::games::arisa::start();
}

/// WASM entry point for NanoTerras light-ring game.
/// Separate from main() — arisa state is completely unaffected.
#[wasm_bindgen]
pub fn main_nanoteras() {
    crate::games::nanoteras::start();
}

/// WASM entry point for Aoba Castle ninja climbing game.
#[wasm_bindgen]
pub fn main_aobacastle() {
    crate::games::aobacastle::start();
}

// ── Rust → JS: internal setters (called each frame from game draw/update) ────
pub fn set_game_state(v: i32) {
    GAME_STATE.store(v, Ordering::Relaxed);
}
pub fn set_player_hp(v: i32) {
    PLAYER_HP.store(v, Ordering::Relaxed);
}
pub fn set_player_max_hp(v: i32) {
    PLAYER_MAX_HP.store(v, Ordering::Relaxed);
}
pub fn set_player_mp(v: i32) {
    PLAYER_MP.store(v, Ordering::Relaxed);
}
pub fn set_player_max_mp(v: i32) {
    PLAYER_MAX_MP.store(v, Ordering::Relaxed);
}
pub fn set_player_level(v: i32) {
    PLAYER_LEVEL.store(v, Ordering::Relaxed);
}
pub fn set_player_flash(v: i32) {
    PLAYER_FLASH.store(v, Ordering::Relaxed);
}
pub fn set_enemy_idx(v: i32) {
    ENEMY_IDX.store(v, Ordering::Relaxed);
}
pub fn set_enemy_flash(v: i32) {
    ENEMY_FLASH.store(v, Ordering::Relaxed);
}
pub fn set_enemy_hp(v: i32) {
    ENEMY_HP.store(v, Ordering::Relaxed);
}
pub fn set_enemy_max_hp(v: i32) {
    ENEMY_MAX_HP.store(v, Ordering::Relaxed);
}
pub fn set_bg_idx(v: i32) {
    BG_IDX.store(v, Ordering::Relaxed);
}
pub fn take_battle_request() -> i32 {
    BATTLE_REQUEST.swap(-1, Ordering::Relaxed)
}

// Pyxel 16-color palette
pub const DEFAULT_PALETTE: [[u8; 4]; 16] = [
    [0x00, 0x00, 0x00, 0xff],
    [0x2b, 0x33, 0x5f, 0xff],
    [0x7e, 0x20, 0x72, 0xff],
    [0x19, 0x95, 0x9c, 0xff],
    [0x8b, 0x48, 0x52, 0xff],
    [0x39, 0x5c, 0x98, 0xff],
    [0xa9, 0xc1, 0xff, 0xff],
    [0xee, 0xee, 0xee, 0xff],
    [0xd4, 0x18, 0x6c, 0xff],
    [0xd3, 0x84, 0x41, 0xff],
    [0xe9, 0xc3, 0x5b, 0xff],
    [0x70, 0xc6, 0xa9, 0xff],
    [0x76, 0x96, 0xde, 0xff],
    [0xa3, 0xa3, 0xa3, 0xff],
    [0xff, 0x97, 0x98, 0xff],
    [0xed, 0xc7, 0xb0, 0xff],
];

/// 表示サイズが約512pxになるようなスケールを計算
fn compute_scale(w: u32, h: u32) -> u32 {
    let max_dim = w.max(h);
    (512u32 / max_dim).max(1)
}

// --------------------------------------------------------------------------
// InputState
// --------------------------------------------------------------------------
pub use super::input::InputState;

// --------------------------------------------------------------------------
// WasmState
// --------------------------------------------------------------------------
pub struct WasmState {
    pub width: u32,
    pub height: u32,
    pub display_scale: u32,
    pub fps: u32,
    pub pixel_buffer: Vec<u8>,
    pub rgba_buffer: Vec<u8>, // pre-allocated RGBA output
    pub palette: [[u8; 4]; 256],
    pub color_map: [u8; 256],
    pub clip_rect: Option<[i32; 4]>,
    pub camera_x: f32,
    pub camera_y: f32,
    pub dither_alpha: f32,
    pub input: InputState,
    pub should_quit: bool,
    pub frame_count: u32,
    pub integer_scale: bool,
    pub perf_monitor: bool,
    pub actual_fps: f64,
    pub update_ms: f64,
    pub draw_ms: f64,
    pub dropped_bytes: Vec<(String, Vec<u8>)>,
    pub text_overlay: Vec<(i32, i32, String, u8)>, // non-ASCII text drawn via fillText
}

impl WasmState {
    fn new(width: u32, height: u32, fps: u32) -> Self {
        let color_map = std::array::from_fn(|i| i as u8);
        let mut palette = [[0, 0, 0, 255]; 256];
        palette[..16].copy_from_slice(&DEFAULT_PALETTE);
        let pixels = (width * height) as usize;
        Self {
            width,
            height,
            display_scale: compute_scale(width, height),
            fps,
            pixel_buffer: vec![0u8; pixels],
            rgba_buffer: vec![0u8; pixels * 4],
            palette,
            color_map,
            clip_rect: None,
            camera_x: 0.0,
            camera_y: 0.0,
            dither_alpha: 1.0,
            input: InputState::new(),
            should_quit: false,
            frame_count: 0,
            integer_scale: false,
            perf_monitor: false,
            actual_fps: 0.,
            update_ms: 0.,
            draw_ms: 0.,
            dropped_bytes: Vec::new(),
            text_overlay: Vec::new(),
        }
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) -> Result<(), String> {
        let count = width
            .checked_mul(height)
            .filter(|n| *n > 0 && *n <= 16 * 1024 * 1024)
            .ok_or("Invalid screen dimensions")? as usize;
        self.width = width;
        self.height = height;
        self.display_scale = compute_scale(width, height);
        self.pixel_buffer = vec![0; count];
        self.rgba_buffer = vec![0; count * 4];
        self.clip_rect = None;
        self.camera_x = 0.;
        self.camera_y = 0.;
        self.dither_alpha = 1.;
        self.text_overlay.clear();
        Ok(())
    }
    /// Reuse the official 2D rasterizer, preserving the software screen's state.
    pub(crate) fn raster_core(&mut self, draw: impl FnOnce(&mut crate::Image)) {
        let image = crate::Image::new(self.width, self.height);
        let mut image = image.borrow_mut();
        // data_ptr covers exactly width*height color indices, and image owns this allocation.
        let pixels =
            unsafe { std::slice::from_raw_parts_mut(image.data_ptr(), self.pixel_buffer.len()) };
        pixels.copy_from_slice(&self.pixel_buffer);
        if let Some([x, y, w, h]) = self.clip_rect {
            image.set_clip_rect(x as f32, y as f32, w as f32, h as f32);
        }
        image.set_camera(self.camera_x, self.camera_y);
        image.set_dithering(self.dither_alpha);
        for (from, to) in self.color_map.iter().enumerate() {
            image.map_color(from as u8, *to);
        }
        draw(&mut image);
        let pixels =
            unsafe { std::slice::from_raw_parts(image.data_ptr(), self.pixel_buffer.len()) };
        self.pixel_buffer.copy_from_slice(pixels);
    }
    fn plot(&mut self, x: i32, y: i32, col: u8) {
        if let Some([cx, cy, cw, ch]) = self.clip_rect {
            if x < cx || y < cy || x >= cx + cw || y >= cy + ch {
                return;
            }
        }
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        if self.dither_alpha < 1.0 {
            let threshold = (x + y) & 1;
            if self.dither_alpha <= 0.5 && threshold == 0 {
                return;
            }
        }
        self.pixel_buffer[(y as u32 * self.width + x as u32) as usize] =
            self.color_map[col as usize];
    }

    fn cam(&self, x: f32, y: f32) -> (i32, i32) {
        ((x - self.camera_x) as i32, (y - self.camera_y) as i32)
    }

    pub fn clear(&mut self, col: u8) {
        let c = self.color_map[col as usize];
        for px in self.pixel_buffer.iter_mut() {
            *px = c;
        }
    }

    pub fn pset(&mut self, x: f32, y: f32, col: u8) {
        let (px, py) = self.cam(x, y);
        self.plot(px, py, col);
    }

    pub fn pget(&self, x: f32, y: f32) -> u8 {
        let (px, py) = ((x - self.camera_x) as i32, (y - self.camera_y) as i32);
        if px < 0 || py < 0 || px >= self.width as i32 || py >= self.height as i32 {
            return 0;
        }
        self.pixel_buffer[(py as u32 * self.width + px as u32) as usize]
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, col: u8) {
        let (mut x0, mut y0) = self.cam(x1, y1);
        let (x1i, y1i) = self.cam(x2, y2);
        let dx = (x1i - x0).abs();
        let dy = -(y1i - y0).abs();
        let sx = if x0 < x1i { 1 } else { -1 };
        let sy = if y0 < y1i { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.plot(x0, y0, col);
            if x0 == x1i && y0 == y1i {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_rect_border(&mut self, x: f32, y: f32, w: f32, h: f32, col: u8) {
        let (x0, y0) = self.cam(x, y);
        let (w, h) = (w as i32, h as i32);
        for i in 0..w {
            self.plot(x0 + i, y0, col);
            self.plot(x0 + i, y0 + h - 1, col);
        }
        for i in 0..h {
            self.plot(x0, y0 + i, col);
            self.plot(x0 + w - 1, y0 + i, col);
        }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, col: u8) {
        let (x0, y0) = self.cam(x, y);
        for dy in 0..h as i32 {
            for dx in 0..w as i32 {
                self.plot(x0 + dx, y0 + dy, col);
            }
        }
    }

    pub fn draw_circle_border(&mut self, x: f32, y: f32, r: f32, col: u8) {
        let (cx, cy, ri) = (self.cam(x, y).0, self.cam(x, y).1, r as i32);
        let (mut xi, mut yi, mut d) = (0i32, ri, 3 - 2 * ri);
        while xi <= yi {
            for (a, b) in [(xi, yi), (yi, xi)] {
                for (sa, sb) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    self.plot(cx + sa * a, cy + sb * b, col);
                }
                let _ = b;
            }
            if d < 0 {
                d += 4 * xi + 6;
            } else {
                d += 4 * (xi - yi) + 10;
                yi -= 1;
            }
            xi += 1;
        }
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, r: f32, col: u8) {
        let (cx, cy, ri) = (self.cam(x, y).0, self.cam(x, y).1, r as i32);
        for dy in -ri..=ri {
            let half = ((ri * ri - dy * dy) as f32).sqrt() as i32;
            for dx in -half..=half {
                self.plot(cx + dx, cy + dy, col);
            }
        }
    }

    pub fn draw_text(&mut self, x: f32, y: f32, s: &str, col: u8) {
        // Non-ASCII (Japanese etc.) → defer to Canvas fillText overlay
        if s.chars().any(|c| c as u32 > 127) {
            let (cx, cy) = self.cam(x, y);
            self.text_overlay
                .push((cx, cy, s.to_string(), self.color_map[col as usize]));
            return;
        }
        let (mut cx, cy) = self.cam(x, y);
        for ch in s.chars() {
            let code = ch as usize;
            if code >= 32 && code < 128 {
                let glyph = &crate::backend::font::FONT[code - 32];
                for (row, &bits) in glyph.iter().enumerate() {
                    for col_bit in 0..4u8 {
                        if bits & (0x80 >> col_bit) != 0 {
                            self.plot(cx + col_bit as i32, cy + row as i32, col);
                        }
                    }
                }
            }
            cx += 5;
        }
    }

    pub fn draw_image(
        &mut self,
        x: f32,
        y: f32,
        img: u32,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
        colkey: Option<u8>,
    ) {
        let image = pyxel::images().get(img as usize).cloned();
        if let Some(image) = image {
            self.raster_core(|dst| {
                dst.draw_image(x, y, &image, sx, sy, sw, sh, colkey, None, None)
            });
        }
    }
    pub fn draw_tilemap(
        &mut self,
        x: f32,
        y: f32,
        tm: u32,
        mx: f32,
        my: f32,
        mw: f32,
        mh: f32,
        colkey: Option<u8>,
    ) {
        let tilemap = pyxel::tilemaps().get(tm as usize).cloned();
        if let Some(tilemap) = tilemap {
            self.raster_core(|dst| {
                dst.draw_tilemap(x, y, &tilemap, mx, my, mw, mh, colkey, None, None)
            });
        }
    }
    pub fn image_plot(&mut self, bank: u32, x: i32, y: i32, col: u8) {
        self.image_pset(bank, x as f32, y as f32, col);
    }
    pub fn image_pset(&mut self, bank: u32, x: f32, y: f32, col: u8) {
        if let Some(image) = pyxel::images().get(bank as usize) {
            image.borrow_mut().set_pixel(x, y, col);
        }
    }
    pub fn image_rect(&mut self, bank: u32, x: f32, y: f32, w: f32, h: f32, col: u8) {
        if let Some(image) = pyxel::images().get(bank as usize) {
            image.borrow_mut().draw_rect(x, y, w, h, col);
        }
    }
    pub fn image_line(&mut self, bank: u32, x1: f32, y1: f32, x2: f32, y2: f32, col: u8) {
        if let Some(image) = pyxel::images().get(bank as usize) {
            image.borrow_mut().draw_line(x1, y1, x2, y2, col);
        }
    }
    pub fn image_tri(
        &mut self,
        bank: u32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        col: u8,
    ) {
        if let Some(image) = pyxel::images().get(bank as usize) {
            image
                .borrow_mut()
                .draw_triangle(x1, y1, x2, y2, x3, y3, col);
        }
    }
    pub fn tilemap_cls(&mut self, tm: u32, tile: (u8, u8)) {
        if let Some(map) = pyxel::tilemaps().get(tm as usize) {
            map.borrow_mut().clear((tile.0.into(), tile.1.into()));
        }
    }
    pub fn tilemap_pget(&self, tm: u32, x: u32, y: u32) -> (u8, u8) {
        pyxel::tilemaps().get(tm as usize).map_or((0, 0), |m| {
            let t = m.borrow().tile(x as f32, y as f32);
            (t.0 as u8, t.1 as u8)
        })
    }
    pub fn tilemap_pset(&mut self, tm: u32, x: u32, y: u32, tile: (u8, u8)) {
        if let Some(map) = pyxel::tilemaps().get(tm as usize) {
            map.borrow_mut()
                .set_tile(x as f32, y as f32, (tile.0.into(), tile.1.into()));
        }
    }

    fn compose_rgba(&mut self) {
        let colors = pyxel::colors();
        for i in 0..256 {
            let rgb = colors.get(i).copied().unwrap_or(0);
            self.palette[i] = [(rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8, 255];
        }
        for (i, &idx) in self.pixel_buffer.iter().enumerate() {
            let rgba = self.palette[idx as usize];
            let base = i * 4;
            self.rgba_buffer[base..base + 4].copy_from_slice(&rgba);
        }
    }
}

// --------------------------------------------------------------------------
// Global state (single-threaded WASM is safe)
// --------------------------------------------------------------------------
static mut STATE: Option<WasmState> = None;

pub(crate) fn state() -> &'static mut WasmState {
    unsafe { STATE.as_mut().expect("wasm-backend not initialized") }
}

/// タッチ UI から直接呼べるキー状態セット関数
#[wasm_bindgen]
pub fn set_key(idx: u32, down: bool) {
    if unsafe { STATE.is_some() } {
        state().input.set_key(idx, down);
    }
}

/// キーインデックスの定数マップを返す (JS から参照用)
#[wasm_bindgen]
pub fn key_index(code: &str) -> u32 {
    code_to_idx(code)
}

pub fn init(width: u32, height: u32, _title: &str, fps: u32) {
    unsafe {
        STATE = Some(WasmState::new(width, height, fps));
    }
    let scale = state().display_scale;

    // Set canvas size
    if let Some(canvas) = get_canvas() {
        canvas.set_width(width * scale);
        canvas.set_height(height * scale);
    }

    // Register keyboard input
    register_keyboard_events();
    text_files::register();
    system::register();
    system::title(_title);
}

fn get_canvas() -> Option<HtmlCanvasElement> {
    let document = web_sys::window()?.document()?;
    let el = document.get_element_by_id("pyxel_canvas").or_else(|| {
        // Create canvas if not present
        let canvas = document.create_element("canvas").ok()?;
        canvas.set_id("pyxel_canvas");
        document.body()?.append_child(&canvas).ok()?;
        Some(canvas)
    })?;
    el.dyn_into::<HtmlCanvasElement>().ok()
}

fn get_context() -> Option<CanvasRenderingContext2d> {
    get_canvas()?
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}

/// Convert a JS KeyboardEvent.code string to our InputState index
use super::keymap::code_to_idx;

fn register_keyboard_events() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(|e: KeyboardEvent| {
        // Form controls retain native arrow/space behavior for timeline and clip UI.
        let is_editor = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|e| e.id() == "pyxel_text_input");
        if !is_editor
            && e.target()
                .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                .and_then(|target| {
                    target
                        .closest("input, select, textarea, button, [contenteditable]")
                        .ok()
                        .flatten()
                })
                .is_some()
        {
            return;
        }
        let idx = code_to_idx(&e.code());
        state().input.set_key(idx, true);
        if !is_editor
            && !e.is_composing()
            && !e.ctrl_key()
            && !e.meta_key()
            && e.key().chars().count() == 1
        {
            state().input.input_text.push_str(&e.key());
        }
        if is_editor {
            return;
        }

        // Prevent default for arrow keys and space to avoid page scroll
        match idx {
            1 | 5 | 6 | 7 | 8 => e.prevent_default(),
            _ => {}
        }
    });
    let keyup = Closure::<dyn FnMut(KeyboardEvent)>::new(|e: KeyboardEvent| {
        let idx = code_to_idx(&e.code());
        state().input.set_key(idx, false);
    });

    let _ = window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref());
    let _ = window.add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref());
    keydown.forget();
    keyup.forget();

    let blur = Closure::<dyn FnMut(web_sys::Event)>::new(|_| state().input.release_all());
    let _ = window.add_event_listener_with_callback("blur", blur.as_ref().unchecked_ref());
    blur.forget();
    if let Some(canvas) = get_canvas() {
        for name in ["mousemove", "mousedown", "mouseup"] {
            let event_name = name.to_string();
            let callback = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
                let Some(canvas) = get_canvas() else {
                    return;
                };
                let rect = canvas.get_bounding_client_rect();
                if rect.width() <= 0. || rect.height() <= 0. {
                    return;
                }
                let s = state();
                s.input.mouse_x = ((e.client_x() as f64 - rect.left()) * s.width as f64
                    / rect.width())
                .floor() as i32;
                s.input.mouse_y = ((e.client_y() as f64 - rect.top()) * s.height as f64
                    / rect.height())
                .floor() as i32;
                if event_name != "mousemove" {
                    use crate::api::constants::*;
                    let key = match e.button() {
                        0 => MOUSE_BUTTON_LEFT,
                        1 => MOUSE_BUTTON_MIDDLE,
                        2 => MOUSE_BUTTON_RIGHT,
                        3 => MOUSE_BUTTON_X1,
                        4 => MOUSE_BUTTON_X2,
                        _ => KEY_UNKNOWN,
                    };
                    s.input.set_key(key, event_name == "mousedown");
                }
            });
            // Release remains observable even when the pointer leaves the canvas.
            if name == "mouseup" {
                let _ = window
                    .add_event_listener_with_callback(name, callback.as_ref().unchecked_ref());
            } else {
                let _ = canvas
                    .add_event_listener_with_callback(name, callback.as_ref().unchecked_ref());
            }
            callback.forget();
        }
        let wheel = Closure::<dyn FnMut(web_sys::WheelEvent)>::new(|e: web_sys::WheelEvent| {
            use crate::api::constants::*;
            let input = &mut state().input;
            // SDL/Pyxel reports positive Y when scrolling up. Normalize browser pixel/line modes to steps.
            for (key, delta) in [(MOUSE_WHEEL_X, e.delta_x()), (MOUSE_WHEEL_Y, -e.delta_y())] {
                let step = if delta == 0. {
                    0
                } else {
                    delta.signum() as i32
                };
                input.set_value(key, input.value(key).saturating_add(step));
            }
        });
        let _ = canvas.add_event_listener_with_callback("wheel", wheel.as_ref().unchecked_ref());
        wheel.forget();
        let context =
            Closure::<dyn FnMut(web_sys::Event)>::new(|e: web_sys::Event| e.prevent_default());
        let _ = canvas
            .add_event_listener_with_callback("contextmenu", context.as_ref().unchecked_ref());
        context.forget();
    }
}

fn poll_gamepads() {
    use super::gamepad_keys::{AXES, BUTTONS};
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(pads) = window.navigator().get_gamepads() else {
        return;
    };
    for index in 0..4 {
        let pad = pads
            .get(index as u32)
            .dyn_into::<web_sys::Gamepad>()
            .ok()
            .filter(|p| p.connected() && p.mapping() == web_sys::GamepadMappingType::Standard);
        for (button, key) in BUTTONS[index].iter().enumerate() {
            if let Some(key) = key {
                let down = pad
                    .as_ref()
                    .and_then(|p| {
                        p.buttons()
                            .get(button as u32)
                            .dyn_into::<web_sys::GamepadButton>()
                            .ok()
                    })
                    .is_some_and(|b| b.pressed());
                state().input.set_key(*key, down);
            }
        }
        for axis in 0..6 {
            let value = pad.as_ref().map_or(0., |p| {
                if axis < 4 {
                    p.axes()
                        .get(axis as u32)
                        .as_f64()
                        .unwrap_or(0.)
                        .clamp(-1., 1.)
                } else {
                    p.buttons()
                        .get((axis + 2) as u32)
                        .dyn_into::<web_sys::GamepadButton>()
                        .ok()
                        .map_or(0., |b| b.value())
                        .clamp(0., 1.)
                }
            });
            let scale = if value < 0. { 32768. } else { 32767. };
            state()
                .input
                .set_value(AXES[index][axis], (value * scale).round() as i32);
        }
    }
}

pub fn run(mut update: Box<dyn FnMut()>, mut draw: Box<dyn FnMut()>) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let fps = state().fps;
    let frame_ms = 1000.0 / fps as f64;
    let last_time: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));

    *g.borrow_mut() = Some(Closure::new(move || {
        let window = web_sys::window().unwrap();
        let perf = window.performance().unwrap();
        let now = perf.now();

        let elapsed = now - *last_time.borrow();
        if elapsed >= frame_ms {
            *last_time.borrow_mut() = now;

            state().frame_count += 1;

            poll_gamepads();
            if let Some(canvas) = get_canvas() {
                let _ = canvas.style().set_property(
                    "cursor",
                    if state().input.mouse_visible {
                        "default"
                    } else {
                        "none"
                    },
                );
            }
            let start = perf.now();
            update();
            let after_update = perf.now();
            draw();
            let after_draw = perf.now();
            state().actual_fps = 1000. / elapsed;
            state().update_ms = after_update - start;
            state().draw_ms = after_draw - after_update;
            system::update_monitor();

            // Render pixel_buffer → canvas
            if let Some(ctx) = get_context() {
                let s = state();
                s.compose_rgba();
                let w = s.width;
                let h = s.height;
                let scale = s.display_scale;
                let rgba = s.rgba_buffer.clone();
                let overlays: Vec<(i32, i32, String, u8)> = s.text_overlay.drain(..).collect();
                let palette = s.palette;

                if let Ok(img_data) =
                    ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&rgba), w, h)
                {
                    if scale == 1 {
                        // No scaling needed — write directly
                        let _ = ctx.put_image_data(&img_data, 0.0, 0.0);
                    } else {
                        // Write to a temp offscreen canvas, then scale onto main canvas
                        let document = web_sys::window().unwrap().document().unwrap();
                        let off = document
                            .create_element("canvas")
                            .unwrap()
                            .dyn_into::<HtmlCanvasElement>()
                            .unwrap();
                        off.set_width(w);
                        off.set_height(h);
                        let off_ctx = off
                            .get_context("2d")
                            .unwrap()
                            .unwrap()
                            .dyn_into::<CanvasRenderingContext2d>()
                            .unwrap();
                        let _ = off_ctx.put_image_data(&img_data, 0.0, 0.0);
                        ctx.set_image_smoothing_enabled(false);
                        let _ = ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            &off, 0.0, 0.0,
                            w as f64, h as f64,
                            0.0, 0.0,
                            (w * scale) as f64, (h * scale) as f64,
                        );
                    }
                }

                // Draw non-ASCII text (Japanese) via Canvas fillText
                if !overlays.is_empty() {
                    let font_px = scale * 7;
                    ctx.set_font(&format!(
                        "{}px 'DotGothic16', 'M PLUS Rounded 1c', monospace",
                        font_px
                    ));
                    ctx.set_text_baseline("top");
                    for (x, y, text, col) in &overlays {
                        let [r, g, b, _] = palette[*col as usize];
                        ctx.set_fill_style(&JsValue::from_str(&format!("rgb({},{},{})", r, g, b)));
                        let _ =
                            ctx.fill_text(text, *x as f64 * scale as f64, *y as f64 * scale as f64);
                    }
                }
            }

            if state().should_quit {
                return;
            }

            // tick() at end of frame so touch/key events fired before rAF are visible in update()
            state().input.tick();
        }

        let _ =
            window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }));

    let window = web_sys::window().unwrap();
    let _ = window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());
}

#[cfg(test)]
mod coverage_tests {
    use super::*;
    #[test]
    fn extended_palette_and_resize_preserve_state_on_invalid_requests() {
        let mut screen = WasmState::new(8, 8, 30);
        let old = pyxel::colors().clone();
        {
            let mut colors = pyxel::colors();
            colors.resize(256, 0);
            colors[200] = 0x123456;
        }
        screen.color_map[8] = 200;
        screen.pset(2., 2., 8);
        screen.color_map[8] = 8;
        assert_eq!(
            screen.pixel_buffer[18], 200,
            "pal maps at draw time, not presentation time"
        );
        screen.compose_rgba();
        assert_eq!(&screen.rgba_buffer[72..76], &[0x12, 0x34, 0x56, 255]);
        assert!(screen.resize(0, 8).is_err());
        assert_eq!((screen.width, screen.height), (8, 8));
        assert_eq!(screen.pixel_buffer[18], 200);
        screen.resize(10, 6).unwrap();
        assert_eq!(screen.pixel_buffer.len(), 60);
        assert!(screen.pixel_buffer.iter().all(|&p| p == 0));
        *pyxel::colors() = old;
    }
    #[test]
    fn official_shapes_preserve_background_clip_camera_and_dither() {
        let mut screen = WasmState::new(80, 60, 30);
        screen.clear(1);
        screen.camera_x = 3.;
        screen.camera_y = -2.;
        screen.clip_rect = Some([5, 4, 65, 48]);
        screen.dither_alpha = 0.5;
        let expected = crate::Image::new(80, 60);
        let mut expected = expected.borrow_mut();
        expected.clear(1);
        expected.set_camera(3., -2.);
        expected.set_clip_rect(5., 4., 65., 48.);
        expected.set_dithering(0.5);
        fn shapes(i: &mut crate::Image) {
            i.draw_ellipse(-4., 2., 35., 22., 8);
            i.draw_ellipse_border(28., 9., 22., 32., 10);
            i.draw_triangle(10., 30., 30., 55., 60., 32., 11);
            i.draw_triangle_border(20., 3., 75., 40., 65., 5., 7);
            i.flood_fill(8., 8., 3);
        }
        screen.raster_core(shapes);
        shapes(&mut expected);
        expected.reset_clip_rect();
        for y in 0..60 {
            for x in 0..80 {
                assert_eq!(
                    screen.pixel_buffer[y * 80 + x],
                    expected.pixel(x as f32, y as f32)
                );
            }
        }
        assert_eq!(screen.pixel_buffer[0], 1);
        assert!(screen.pixel_buffer.iter().any(|&c| c == 8));
    }
}
