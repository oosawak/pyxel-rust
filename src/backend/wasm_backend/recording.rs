//! Opt-in bounded recording of logical screen pixels using the official GIF encoder.
use super::state;
use wasm_bindgen::prelude::*;
const LIMIT: u64 = 32 * 1024 * 1024;
pub fn configure_screencast(seconds: u32) -> Result<(), String> {
    let s = state();
    if seconds == 0 {
        s.recorder = None;
        return Ok(());
    }
    let bytes = u64::from(s.width) * u64::from(s.height) * u64::from(s.fps) * u64::from(seconds);
    if seconds > 30 || bytes > LIMIT {
        return Err("Recording must fit 32 MiB and 1..30 seconds; reduce size or duration".into());
    }
    s.recorder = Some(pyxel::Screencast::new(s.fps, seconds));
    Ok(())
}
pub fn reset_screencast() {
    if let Some(r) = &mut state().recorder {
        r.reset();
    }
}
pub fn screencast_gif(scale: u32) -> Result<Option<Vec<u8>>, String> {
    state()
        .recorder
        .as_mut()
        .ok_or("Enable recording with configure_screencast first")?
        .gif_bytes(scale)
}
pub fn capture() {
    let s = state();
    if let Some(r) = &mut s.recorder {
        r.capture(
            s.width,
            s.height,
            s.screen.borrow().pixels(),
            &pyxel::colors(),
            s.frame_count,
        );
    }
}
#[wasm_bindgen]
pub fn browser_recording(seconds: u32) -> Result<(), JsValue> {
    configure_screencast(seconds).map_err(|e| JsValue::from_str(&e))
}
#[wasm_bindgen]
pub fn browser_reset_screencast() {
    reset_screencast();
}
#[wasm_bindgen]
pub fn browser_screencast(scale: u32) -> Result<Option<Vec<u8>>, JsValue> {
    screencast_gif(scale).map_err(|e| JsValue::from_str(&e))
}
#[wasm_bindgen]
pub fn browser_recorded_frames() -> u32 {
    state()
        .recorder
        .as_ref()
        .map_or(0, |r| r.frame_count() as u32)
}

/// Export with the initialization capture scale unless explicitly overridden.
pub fn configured_screencast_gif(scale: Option<u32>) -> Result<Option<Vec<u8>>, String> {
    screencast_gif(scale.unwrap_or_else(|| state().capture_scale))
}
#[wasm_bindgen]
pub fn browser_capture_gif(scale: Option<u32>) -> Result<Option<Vec<u8>>, JsValue> {
    configured_screencast_gif(scale).map_err(|e| JsValue::from_str(&e))
}
