/// Input API - Keyboard and mouse input handling
/// 
/// Matches Python Pyxel interface for input.

use pyxel::Key;

/// Check if key is held down this frame
pub fn btn(key: Key) -> bool {
    pyxel::pyxel().is_button_down(key)
}

/// Check if key was pressed this frame (one shot)
pub fn btnp(key: Key) -> bool {
    pyxel::pyxel().is_button_pressed(key, None, None)
}

/// Check if key with hold frame
pub fn btnp_hold(key: Key, hold: u32, repeat: u32) -> bool {
    pyxel::pyxel().is_button_pressed(key, Some(hold), Some(repeat))
}

/// Check if key was released this frame
pub fn btnr(key: Key) -> bool {
    pyxel::pyxel().is_button_released(key)
}

/// Get mouse X position
pub fn mouse_x() -> i32 {
    *pyxel::mouse_x()
}

/// Get mouse Y position
pub fn mouse_y() -> i32 {
    *pyxel::mouse_y()
}

/// Get mouse wheel value
pub fn mouse_wheel() -> i32 {
    *pyxel::mouse_wheel()
}

/// Set mouse visibility
pub fn set_mouse_visible(visible: bool) {
    pyxel::pyxel().set_mouse_visible(visible);
}
