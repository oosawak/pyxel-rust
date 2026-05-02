//! WasmBackend — wasm-bindgen + canvas 2D rendering
//!
//! Rendering pipeline:
//!   1. Game calls drawing primitives → writes color indices into `pixel_buffer`
//!   2. Each frame: map indices through palette → RGBA → ImageData
//!   3. `context.put_image_data()` renders to an HTML canvas element
//!
//! Entry: the browser calls main() via wasm-bindgen, which calls init() then run().

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, ImageData,
    KeyboardEvent, MouseEvent,
};
use js_sys::Uint8ClampedArray;
use std::rc::Rc;
use std::cell::RefCell;

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

const DISPLAY_SCALE: u32 = 4;
const KEY_COUNT: usize = 256;

// --------------------------------------------------------------------------
// InputState
// --------------------------------------------------------------------------
pub struct InputState {
    held:     [bool; KEY_COUNT],
    pressed:  [bool; KEY_COUNT],
    released: [bool; KEY_COUNT],
    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            held: [false; KEY_COUNT], pressed: [false; KEY_COUNT],
            released: [false; KEY_COUNT], mouse_x: 0, mouse_y: 0,
        }
    }
    pub fn tick(&mut self) {
        self.pressed  = [false; KEY_COUNT];
        self.released = [false; KEY_COUNT];
    }
    pub fn set_key(&mut self, idx: u32, down: bool) {
        let i = idx as usize;
        if i >= KEY_COUNT { return; }
        if down && !self.held[i]  { self.pressed[i] = true; }
        if !down && self.held[i]  { self.released[i] = true; }
        self.held[i] = down;
    }
    pub fn btn(&self, idx: u32)  -> bool { let i = idx as usize; i < KEY_COUNT && self.held[i] }
    pub fn btnp(&self, idx: u32) -> bool { let i = idx as usize; i < KEY_COUNT && self.pressed[i] }
    pub fn btnr(&self, idx: u32) -> bool { let i = idx as usize; i < KEY_COUNT && self.released[i] }
}

// --------------------------------------------------------------------------
// WasmState
// --------------------------------------------------------------------------
pub struct WasmState {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub pixel_buffer: Vec<u8>,
    pub rgba_buffer: Vec<u8>,   // pre-allocated RGBA output
    pub palette: [[u8; 4]; 16],
    pub color_map: [u8; 16],
    pub clip_rect: Option<[i32; 4]>,
    pub camera_x: f32,
    pub camera_y: f32,
    pub dither_alpha: f32,
    pub input: InputState,
    pub should_quit: bool,
    pub frame_count: u32,
}

impl WasmState {
    fn new(width: u32, height: u32, fps: u32) -> Self {
        let mut color_map = [0u8; 16];
        for i in 0..16u8 { color_map[i as usize] = i; }
        let pixels = (width * height) as usize;
        Self {
            width, height, fps,
            pixel_buffer: vec![0u8; pixels],
            rgba_buffer: vec![0u8; pixels * 4],
            palette: DEFAULT_PALETTE, color_map,
            clip_rect: None, camera_x: 0.0, camera_y: 0.0, dither_alpha: 1.0,
            input: InputState::new(), should_quit: false, frame_count: 0,
        }
    }

    fn plot(&mut self, x: i32, y: i32, col: u8) {
        if let Some([cx, cy, cw, ch]) = self.clip_rect {
            if x < cx || y < cy || x >= cx+cw || y >= cy+ch { return; }
        }
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 { return; }
        if self.dither_alpha < 1.0 {
            let threshold = (x + y) & 1;
            if self.dither_alpha <= 0.5 && threshold == 0 { return; }
        }
        self.pixel_buffer[(y as u32 * self.width + x as u32) as usize] = col & 0x0f;
    }

    fn cam(&self, x: f32, y: f32) -> (i32, i32) {
        ((x - self.camera_x) as i32, (y - self.camera_y) as i32)
    }

    pub fn clear(&mut self, col: u8) {
        let c = col & 0x0f;
        for px in self.pixel_buffer.iter_mut() { *px = c; }
    }

    pub fn pset(&mut self, x: f32, y: f32, col: u8) {
        let (px, py) = self.cam(x, y);
        self.plot(px, py, col);
    }

    pub fn pget(&self, x: f32, y: f32) -> u8 {
        let (px, py) = ((x - self.camera_x) as i32, (y - self.camera_y) as i32);
        if px < 0 || py < 0 || px >= self.width as i32 || py >= self.height as i32 { return 0; }
        self.pixel_buffer[(py as u32 * self.width + px as u32) as usize]
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, col: u8) {
        let (mut x0, mut y0) = self.cam(x1, y1);
        let (x1i, y1i) = self.cam(x2, y2);
        let dx = (x1i - x0).abs(); let dy = -(y1i - y0).abs();
        let sx = if x0 < x1i { 1 } else { -1 };
        let sy = if y0 < y1i { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.plot(x0, y0, col);
            if x0 == x1i && y0 == y1i { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 += sx; }
            if e2 <= dx { err += dx; y0 += sy; }
        }
    }

    pub fn draw_rect_border(&mut self, x: f32, y: f32, w: f32, h: f32, col: u8) {
        let (x0, y0) = self.cam(x, y);
        let (w, h) = (w as i32, h as i32);
        for i in 0..w { self.plot(x0+i, y0, col); self.plot(x0+i, y0+h-1, col); }
        for i in 0..h { self.plot(x0, y0+i, col); self.plot(x0+w-1, y0+i, col); }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, col: u8) {
        let (x0, y0) = self.cam(x, y);
        for dy in 0..h as i32 {
            for dx in 0..w as i32 { self.plot(x0+dx, y0+dy, col); }
        }
    }

    pub fn draw_circle_border(&mut self, x: f32, y: f32, r: f32, col: u8) {
        let (cx, cy, ri) = (self.cam(x, y).0, self.cam(x, y).1, r as i32);
        let (mut xi, mut yi, mut d) = (0i32, ri, 3 - 2*ri);
        while xi <= yi {
            for (a, b) in [(xi,yi),(yi,xi)] {
                for (sa,sb) in [(1,1),(1,-1),(-1,1),(-1,-1)] {
                    self.plot(cx+sa*a, cy+sb*b, col);
                }
                let _ = b;
            }
            if d < 0 { d += 4*xi + 6; } else { d += 4*(xi-yi) + 10; yi -= 1; }
            xi += 1;
        }
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, r: f32, col: u8) {
        let (cx, cy, ri) = (self.cam(x, y).0, self.cam(x, y).1, r as i32);
        for dy in -ri..=ri {
            let half = ((ri*ri - dy*dy) as f32).sqrt() as i32;
            for dx in -half..=half { self.plot(cx+dx, cy+dy, col); }
        }
    }

    pub fn draw_text(&mut self, x: f32, y: f32, s: &str, col: u8) {
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

    pub fn draw_image(&mut self, _x: f32, _y: f32, _img: u32, _sx: f32, _sy: f32, _sw: f32, _sh: f32, _colkey: Option<u8>) {
        // stub: image system not yet implemented
    }

    pub fn draw_tilemap(&mut self, _x: f32, _y: f32, _tm: u32, _mx: f32, _my: f32, _mw: f32, _mh: f32, _colkey: Option<u8>) {
        // stub: tilemap system not yet implemented
    }

    fn compose_rgba(&mut self) {
        for (i, &idx) in self.pixel_buffer.iter().enumerate() {
            let mapped = self.color_map[idx as usize & 0x0f];
            let rgba = self.palette[mapped as usize];
            let base = i * 4;
            self.rgba_buffer[base..base+4].copy_from_slice(&rgba);
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

pub fn init(width: u32, height: u32, _title: &str, fps: u32) {
    unsafe { STATE = Some(WasmState::new(width, height, fps)); }

    // Set canvas size
    if let Some(canvas) = get_canvas() {
        canvas.set_width(width * DISPLAY_SCALE);
        canvas.set_height(height * DISPLAY_SCALE);
    }

    // Register keyboard input
    register_keyboard_events();
}

fn get_canvas() -> Option<HtmlCanvasElement> {
    let document = web_sys::window()?.document()?;
    let el = document.get_element_by_id("pyxel_canvas")
        .or_else(|| {
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
        .get_context("2d").ok()??
        .dyn_into::<CanvasRenderingContext2d>().ok()
}

/// Convert a JS KeyboardEvent.code string to our InputState index
fn code_to_idx(code: &str) -> u32 {
    match code {
        "Escape"       => 0,  "Space"        => 1,
        "Enter"        => 2,  "Backspace"    => 3,
        "Tab"          => 4,
        "ArrowUp"      => 5,  "ArrowDown"    => 6,
        "ArrowLeft"    => 7,  "ArrowRight"   => 8,
        "ShiftLeft"    => 9,  "ShiftRight"   => 10,
        "ControlLeft"  => 11, "ControlRight" => 12,
        "AltLeft"      => 13, "AltRight"     => 14,
        "KeyA" => 20, "KeyB" => 21, "KeyC" => 22,
        "KeyD" => 23, "KeyE" => 24, "KeyF" => 25,
        "KeyG" => 26, "KeyH" => 27, "KeyI" => 28,
        "KeyJ" => 29, "KeyK" => 30, "KeyL" => 31,
        "KeyM" => 32, "KeyN" => 33, "KeyO" => 34,
        "KeyP" => 35, "KeyQ" => 36, "KeyR" => 37,
        "KeyS" => 38, "KeyT" => 39, "KeyU" => 40,
        "KeyV" => 41, "KeyW" => 42, "KeyX" => 43,
        "KeyY" => 44, "KeyZ" => 45,
        "Digit0" => 50, "Digit1" => 51, "Digit2" => 52,
        "Digit3" => 53, "Digit4" => 54, "Digit5" => 55,
        "Digit6" => 56, "Digit7" => 57, "Digit8" => 58,
        "Digit9" => 59,
        "F1"  => 60, "F2"  => 61, "F3"  => 62, "F4"  => 63,
        "F5"  => 64, "F6"  => 65, "F7"  => 66, "F8"  => 67,
        "F9"  => 68, "F10" => 69, "F11" => 70, "F12" => 71,
        _ => 255,
    }
}

fn register_keyboard_events() {
    let window = match web_sys::window() { Some(w) => w, None => return };

    let keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(|e: KeyboardEvent| {
        let idx = code_to_idx(&e.code());
        state().input.set_key(idx, true);
        // Prevent default for arrow keys and space to avoid page scroll
        match idx { 1 | 5 | 6 | 7 | 8 => e.prevent_default(), _ => {} }
    });
    let keyup = Closure::<dyn FnMut(KeyboardEvent)>::new(|e: KeyboardEvent| {
        let idx = code_to_idx(&e.code());
        state().input.set_key(idx, false);
    });

    let _ = window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref());
    let _ = window.add_event_listener_with_callback("keyup",   keyup.as_ref().unchecked_ref());
    keydown.forget();
    keyup.forget();

    // Mouse events on canvas
    if let Some(canvas) = get_canvas() {
        let scale = DISPLAY_SCALE;
        let mousemove = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
            let s = state();
            s.input.mouse_x = (e.offset_x() / scale as i32).max(0);
            s.input.mouse_y = (e.offset_y() / scale as i32).max(0);
        });
        let _ = canvas.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref());
        mousemove.forget();
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

            {
                let s = state();
                s.frame_count += 1;
                s.input.tick();
            }

            update();
            draw();

            // Render pixel_buffer → canvas
            if let Some(ctx) = get_context() {
                let s = state();
                s.compose_rgba();
                let w = s.width;
                let h = s.height;
                let scale = DISPLAY_SCALE;
                let rgba = &s.rgba_buffer;

                let arr = unsafe {
                    Uint8ClampedArray::view(rgba)
                };
                if let Ok(img_data) = ImageData::new_with_u8_clamped_array_and_sh(
                    wasm_bindgen::Clamped(rgba), w, h
                ) {
                    let _ = ctx.put_image_data(&img_data, 0.0, 0.0);
                    ctx.set_image_smoothing_enabled(false);
                    // Scale up: draw the small canvas scaled to display size
                    let _ = ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &get_canvas().unwrap(), 0.0, 0.0,
                        w as f64, h as f64, 0.0, 0.0,
                        (w * scale) as f64, (h * scale) as f64,
                    );
                    drop(arr);
                }
            }

            if state().should_quit { return; }
        }

        let _ = window.request_animation_frame(
            f.borrow().as_ref().unwrap().as_ref().unchecked_ref()
        );
    }));

    let window = web_sys::window().unwrap();
    let _ = window.request_animation_frame(
        g.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    );
}
