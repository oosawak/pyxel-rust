use super::event::Event;
use glow::Context;

#[derive(PartialEq)]
pub enum GlProfile {
    None,
    Gl,
    Gles,
}

#[allow(unused_variables)]
pub fn is_sigint_received() -> bool {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn init(headless: bool) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn quit() {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn ticks() -> u32 {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn export_browser_file(filename: &str) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn init_window(title: &str, width: u32, height: u32) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn window_pos() -> (i32, i32) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn set_window_pos(x: i32, y: i32) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn window_size() -> (u32, u32) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn set_window_size(width: u32, height: u32) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn set_window_title(title: &str) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn set_window_icon(width: u32, height: u32, rgba: &[u8]) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn is_fullscreen() -> bool {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn set_fullscreen(enabled: bool) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn set_mouse_pos(x: i32, y: i32) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn set_mouse_visible(visible: bool) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn display_size() -> (u32, u32) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn start_audio<F: FnMut(&mut [i16]) + 'static>(
    sample_rate: u32,
    buffer_size: u32,
    callback: F,
) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn pause_audio(paused: bool) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn close_audio() {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn lock_audio() {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn unlock_audio() {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn run_frame_loop<F: FnMut(f32)>(fps: u32, callback: F) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn step_frame(fps: u32) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub(super) fn swap_window() {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn poll_events(events: &mut Vec<Event>) {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn gl_profile() -> GlProfile {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}

#[allow(unused_variables)]
pub fn with_gl_context<T>(f: impl FnOnce(&mut Context) -> T) -> T {
    panic!("Use pyxel-rust backend APIs for lifecycle, audio, and window operations");
}
