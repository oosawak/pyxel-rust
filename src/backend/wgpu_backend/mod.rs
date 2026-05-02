/// WgpuBackend — wgpu + winit 0.28 + pixels 0.12

pub mod drawing;
pub mod font;
pub mod input;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
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
        for i in 0..16u8 { color_map[i as usize] = i; }
        Self {
            width, height, fps, title: title.to_string(),
            pixel_buffer: vec![0u8; (width * height) as usize],
            palette: DEFAULT_PALETTE, color_map,
            clip_rect: None, camera_x: 0.0, camera_y: 0.0, dither_alpha: 1.0,
            input: InputState::new(), should_quit: false, frame_count: 0,
        }
    }
}

pub fn init(width: u32, height: u32, title: &str, fps: u32) {
    unsafe { STATE = Some(WgpuState::new(width, height, title, fps)); }
}

fn vk_to_idx(vk: VirtualKeyCode) -> u32 {
    match vk {
        VirtualKeyCode::Escape     => 0,  VirtualKeyCode::Space      => 1,
        VirtualKeyCode::Return     => 2,  VirtualKeyCode::Back       => 3,
        VirtualKeyCode::Tab        => 4,
        VirtualKeyCode::Up         => 5,  VirtualKeyCode::Down       => 6,
        VirtualKeyCode::Left       => 7,  VirtualKeyCode::Right      => 8,
        VirtualKeyCode::LShift     => 9,  VirtualKeyCode::RShift     => 10,
        VirtualKeyCode::LControl   => 11, VirtualKeyCode::RControl   => 12,
        VirtualKeyCode::LAlt       => 13, VirtualKeyCode::RAlt       => 14,
        VirtualKeyCode::A => 20, VirtualKeyCode::B => 21, VirtualKeyCode::C => 22,
        VirtualKeyCode::D => 23, VirtualKeyCode::E => 24, VirtualKeyCode::F => 25,
        VirtualKeyCode::G => 26, VirtualKeyCode::H => 27, VirtualKeyCode::I => 28,
        VirtualKeyCode::J => 29, VirtualKeyCode::K => 30, VirtualKeyCode::L => 31,
        VirtualKeyCode::M => 32, VirtualKeyCode::N => 33, VirtualKeyCode::O => 34,
        VirtualKeyCode::P => 35, VirtualKeyCode::Q => 36, VirtualKeyCode::R => 37,
        VirtualKeyCode::S => 38, VirtualKeyCode::T => 39, VirtualKeyCode::U => 40,
        VirtualKeyCode::V => 41, VirtualKeyCode::W => 42, VirtualKeyCode::X => 43,
        VirtualKeyCode::Y => 44, VirtualKeyCode::Z => 45,
        VirtualKeyCode::Key0 => 50, VirtualKeyCode::Key1 => 51, VirtualKeyCode::Key2 => 52,
        VirtualKeyCode::Key3 => 53, VirtualKeyCode::Key4 => 54, VirtualKeyCode::Key5 => 55,
        VirtualKeyCode::Key6 => 56, VirtualKeyCode::Key7 => 57, VirtualKeyCode::Key8 => 58,
        VirtualKeyCode::Key9 => 59,
        VirtualKeyCode::F1  => 60, VirtualKeyCode::F2  => 61, VirtualKeyCode::F3  => 62,
        VirtualKeyCode::F4  => 63, VirtualKeyCode::F5  => 64, VirtualKeyCode::F6  => 65,
        VirtualKeyCode::F7  => 66, VirtualKeyCode::F8  => 67, VirtualKeyCode::F9  => 68,
        VirtualKeyCode::F10 => 69, VirtualKeyCode::F11 => 70, VirtualKeyCode::F12 => 71,
        _ => 255,
    }
}

pub fn run(mut update: Box<dyn FnMut()>, mut draw: Box<dyn FnMut()>) {
    let (width, height, title, fps) = {
        let s = state();
        (s.width, s.height, s.title.clone(), s.fps)
    };

    let event_loop = EventLoop::new();
    let win_w = width * DISPLAY_SCALE;
    let win_h = height * DISPLAY_SCALE;

    let window = WindowBuilder::new()
        .with_title(&title)
        .with_inner_size(LogicalSize::new(win_w, win_h))
        .with_resizable(false)
        .build(&event_loop)
        .expect("Failed to create window");

    let mut pixels = {
        let surface = SurfaceTexture::new(win_w, win_h, &window);
        Pixels::new(width, height, surface).expect("Failed to create pixel surface")
    };

    let frame_duration = std::time::Duration::from_secs_f64(1.0 / fps as f64);
    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. }, ..
            } => {
                if let Some(vk) = input.virtual_keycode {
                    let pressed = input.state == ElementState::Pressed;
                    state().input.set_key(vk_to_idx(vk), pressed);
                    if vk == VirtualKeyCode::Escape {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. }, ..
            } => {
                let s = state();
                s.input.mouse_x = (position.x / DISPLAY_SCALE as f64) as i32;
                s.input.mouse_y = (position.y / DISPLAY_SCALE as f64) as i32;
            }

            Event::MainEventsCleared => {
                let now = std::time::Instant::now();
                if now.duration_since(last_frame) >= frame_duration {
                    last_frame = now;
                    {
                        let s = state();
                        s.frame_count += 1;
                        s.input.tick();
                    }

                    update();
                    draw();

                    if state().should_quit {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    {
                        let s = state();
                        let frame = pixels.frame_mut();
                        for (i, &idx) in s.pixel_buffer.iter().enumerate() {
                            let mapped = s.color_map[idx as usize & 0x0f];
                            let rgba = s.palette[mapped as usize];
                            let base = i * 4;
                            frame[base..base + 4].copy_from_slice(&rgba);
                        }
                    }

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                    }
                    window.request_redraw();
                }
            }

            _ => {}
        }
    });
}
