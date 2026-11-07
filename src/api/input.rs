/// Input API - Keyboard and mouse

#[cfg(feature = "wasm-backend")]
fn s() -> &'static mut crate::backend::wasm_backend::WasmState {
    crate::backend::wasm_backend::state()
}
#[cfg(feature = "wgpu-backend")]
fn s() -> &'static mut crate::backend::wgpu_backend::WgpuState {
    crate::backend::wgpu_backend::state()
}

macro_rules! dispatch {
    (soft: $soft:expr, pyxel: $p:expr) => {{
        #[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))]
        {
            $soft
        }
        #[cfg(feature = "pyxel-core-backend")]
        {
            $p
        }
    }};
}

pub fn btn(key: u32) -> bool {
    dispatch!(soft: s().input.btn(key), pyxel: pyxel::pyxel().is_button_down(key))
}
pub fn btnp(key: u32) -> bool {
    dispatch!(soft: s().input.btnp(key), pyxel: pyxel::pyxel().is_button_pressed(key, None, None))
}
pub fn btnp_hold(key: u32, _hold: u32, _repeat: u32) -> bool {
    dispatch!(
        soft:  s().input.btnp_hold(key, _hold, _repeat),
        pyxel: pyxel::pyxel().is_button_pressed(key, Some(_hold), Some(_repeat))
    )
}
pub fn btnr(key: u32) -> bool {
    dispatch!(soft: s().input.btnr(key), pyxel: pyxel::pyxel().is_button_released(key))
}
pub fn mouse_x() -> i32 {
    dispatch!(soft: s().input.mouse_x, pyxel: *pyxel::mouse_x())
}
pub fn mouse_y() -> i32 {
    dispatch!(soft: s().input.mouse_y, pyxel: *pyxel::mouse_y())
}
pub fn mouse_wheel() -> i32 {
    dispatch!(soft: s().input.value(crate::api::MOUSE_WHEEL_Y), pyxel: *pyxel::mouse_wheel())
}
pub fn set_mouse_visible(visible: bool) {
    dispatch!(soft: { s().input.mouse_visible = visible; }, pyxel: pyxel::pyxel().set_mouse_visible(visible));
}

/// Read an analog input (mouse coordinates/wheel or gamepad axis).
pub fn btnv(key: u32) -> i32 {
    dispatch!(soft: s().input.value(key), pyxel: pyxel::pyxel().button_value(key))
}
pub fn set_btn(key: u32, down: bool) {
    dispatch!(soft: s().input.set_key(key, down), pyxel: pyxel::pyxel().set_button_state(key, down));
}
pub fn set_btnv(key: u32, value: i32) {
    dispatch!(soft: s().input.set_value(key, value), pyxel: pyxel::pyxel().set_button_value(key, value));
}
pub fn input_keys() -> Vec<u32> {
    dispatch!(soft: s().input.input_keys.clone(), pyxel: pyxel::input_keys().clone())
}
/// In a browser this changes the logical coordinates; browsers cannot warp the OS cursor.
pub fn set_mouse_pos(x: f32, y: f32) {
    dispatch!(soft: { s().input.mouse_x=x as i32; s().input.mouse_y=y as i32; }, pyxel: pyxel::pyxel().set_mouse_position(x,y));
}
/// Official spelling for cursor visibility.
pub fn mouse(visible: bool) {
    set_mouse_visible(visible);
}
