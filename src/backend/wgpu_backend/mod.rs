/// WgpuBackend — wgpu + winit 0.30 + pixels 0.17
pub mod drawing;
pub mod font;
pub mod input;

use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use input::InputState;

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

static mut STATE: Option<WgpuState> = None;

pub(crate) fn state() -> &'static mut WgpuState {
    unsafe { STATE.as_mut().expect("wgpu-backend not initialized") }
}

pub struct WgpuState {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub title: String,
    pub pixel_buffer: Vec<u8>,
    pub image_banks: [Vec<u8>; 3],
    pub tilemaps: [Vec<(u8, u8)>; 8],
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

impl WgpuState {
    pub fn new(width: u32, height: u32, title: &str, fps: u32) -> Self {
        let mut color_map = [0u8; 16];
        for i in 0..16u8 {
            color_map[i as usize] = i;
        }
        Self {
            width,
            height,
            fps,
            title: title.to_string(),
            pixel_buffer: vec![0u8; (width * height) as usize],
            image_banks: std::array::from_fn(|_| vec![0; 256 * 256]),
            tilemaps: std::array::from_fn(|_| vec![(0, 0); 256 * 256]),
            palette: DEFAULT_PALETTE,
            color_map,
            clip_rect: None,
            camera_x: 0.0,
            camera_y: 0.0,
            dither_alpha: 1.0,
            input: InputState::new(),
            should_quit: false,
            frame_count: 0,
        }
    }
}

pub fn init(width: u32, height: u32, title: &str, fps: u32) {
    unsafe {
        STATE = Some(WgpuState::new(width, height, title, fps));
    }
}

fn vk_to_idx(key: KeyCode) -> u32 {
    let name = format!("{key:?}");
    super::keymap::code_to_idx(match name.as_str() {
        "SuperLeft" => "MetaLeft",
        "SuperRight" => "MetaRight",
        _ => &name,
    })
}

struct WgpuApp {
    update: Box<dyn FnMut()>,
    draw: Box<dyn FnMut()>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    next_frame: Instant,
    frame_duration: Duration,
}

impl ApplicationHandler for WgpuApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let s = state();
        let attributes = Window::default_attributes()
            .with_title(&s.title)
            .with_inner_size(LogicalSize::new(
                s.width * DISPLAY_SCALE,
                s.height * DISPLAY_SCALE,
            ))
            .with_resizable(false);
        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("Failed to create window"),
        );
        window.set_ime_allowed(true);
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window.clone());
        self.pixels =
            Some(Pixels::new(s.width, s.height, surface).expect("Failed to create pixel surface"));
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let Some(text) = &event.text {
                        state().input.input_text.push_str(text);
                    }
                }
                if let PhysicalKey::Code(key) = event.physical_key {
                    let pressed = event.state == ElementState::Pressed;
                    state().input.set_key(vk_to_idx(key), pressed);
                    if pressed && key == KeyCode::Escape {
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let s = state();
                if let Some(window) = &self.window {
                    let size = window.inner_size();
                    s.input.mouse_x =
                        (position.x * s.width as f64 / size.width.max(1) as f64) as i32;
                    s.input.mouse_y =
                        (position.y * s.height as f64 / size.height.max(1) as f64) as i32;
                }
            }
            WindowEvent::MouseInput {
                state: button_state,
                button,
                ..
            } => {
                use crate::api::constants::*;
                use winit::event::MouseButton;
                let key = match button {
                    MouseButton::Left => MOUSE_BUTTON_LEFT,
                    MouseButton::Middle => MOUSE_BUTTON_MIDDLE,
                    MouseButton::Right => MOUSE_BUTTON_RIGHT,
                    MouseButton::Back => MOUSE_BUTTON_X1,
                    MouseButton::Forward => MOUSE_BUTTON_X2,
                    _ => KEY_UNKNOWN,
                };
                state()
                    .input
                    .set_key(key, button_state == ElementState::Pressed);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                use crate::api::constants::*;
                let (x, y) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                    winit::event::MouseScrollDelta::PixelDelta(p) => (p.x, p.y),
                };
                let input = &mut state().input;
                for (key, value) in [(MOUSE_WHEEL_X, x), (MOUSE_WHEEL_Y, y)] {
                    input.set_value(
                        key,
                        input.value(key).saturating_add(if value == 0. {
                            0
                        } else {
                            value.signum() as i32
                        }),
                    );
                }
            }
            WindowEvent::Ime(winit::event::Ime::Commit(text)) => {
                state().input.input_text.push_str(&text)
            }
            WindowEvent::DroppedFile(path) => state()
                .input
                .dropped_files
                .push(path.to_string_lossy().into_owned()),
            WindowEvent::Focused(false) => state().input.release_all(),
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                if let Some(pixels) = &mut self.pixels {
                    if pixels.resize_surface(size.width, size.height).is_err() {
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if Instant::now() < self.next_frame {
                    return;
                }
                let Some(pixels) = &mut self.pixels else {
                    return;
                };
                state().frame_count += 1;
                if let Some(window) = &self.window {
                    window.set_cursor_visible(state().input.mouse_visible);
                }
                (self.update)();
                (self.draw)();
                // Keep key edges available to callbacks before clearing them.
                state().input.tick();
                if state().should_quit {
                    event_loop.exit();
                    return;
                }
                let s = state();
                let frame = pixels.frame_mut();
                for (i, &index) in s.pixel_buffer.iter().enumerate() {
                    let mapped = s.color_map[index as usize & 0x0f];
                    frame[i * 4..i * 4 + 4].copy_from_slice(&s.palette[mapped as usize]);
                }
                if pixels.render().is_err() {
                    event_loop.exit();
                }
                self.next_frame = Instant::now() + self.frame_duration;
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if Instant::now() >= self.next_frame {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

pub fn run(update: Box<dyn FnMut()>, draw: Box<dyn FnMut()>) {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = WgpuApp {
        update,
        draw,
        window: None,
        pixels: None,
        next_frame: Instant::now(),
        frame_duration: Duration::from_secs_f64(1.0 / state().fps as f64),
    };
    event_loop.run_app(&mut app).expect("Event loop failed");
}
