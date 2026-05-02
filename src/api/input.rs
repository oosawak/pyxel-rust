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
        #[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))] { $soft }
        #[cfg(feature = "pyxel-core-backend")] { $p }
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
        soft:  s().input.btnp(key),
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
    dispatch!(soft: 0, pyxel: *pyxel::mouse_wheel())
}
pub fn set_mouse_visible(visible: bool) {
    dispatch!(soft: { let _ = visible; }, pyxel: pyxel::pyxel().set_mouse_visible(visible));
}
