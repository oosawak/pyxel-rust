/// Constants — color indices and key codes

// Color constants
pub const COLOR_BLACK:      u8 = 0;
pub const COLOR_NAVY:       u8 = 1;
pub const COLOR_PURPLE:     u8 = 2;
pub const COLOR_GREEN:      u8 = 3;
pub const COLOR_BROWN:      u8 = 4;
pub const COLOR_DARK_BLUE:  u8 = 5;
pub const COLOR_LIGHT_BLUE: u8 = 6;
pub const COLOR_WHITE:      u8 = 7;
pub const COLOR_RED:        u8 = 8;
pub const COLOR_ORANGE:     u8 = 9;
pub const COLOR_YELLOW:     u8 = 10;
pub const COLOR_LIME:       u8 = 11;
pub const COLOR_CYAN:       u8 = 12;
pub const COLOR_GRAY:       u8 = 13;
pub const COLOR_PINK:       u8 = 14;
pub const COLOR_PEACH:      u8 = 15;

// ---------------------------------------------------------------------------
// Software-renderer key codes (wgpu-backend and wasm-backend share indices)
// Indices match keycode_to_u32()/code_to_idx() in each backend's mod.rs
// ---------------------------------------------------------------------------
#[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))]
mod keys_soft {
    pub const KEY_ESCAPE:    u32 = 0;
    pub const KEY_SPACE:     u32 = 1;
    pub const KEY_RETURN:    u32 = 2;
    pub const KEY_BACKSPACE: u32 = 3;
    pub const KEY_TAB:       u32 = 4;
    pub const KEY_UP:        u32 = 5;
    pub const KEY_DOWN:      u32 = 6;
    pub const KEY_LEFT:      u32 = 7;
    pub const KEY_RIGHT:     u32 = 8;
    pub const KEY_LSHIFT:    u32 = 9;
    pub const KEY_RSHIFT:    u32 = 10;
    pub const KEY_LCTRL:     u32 = 11;
    pub const KEY_RCTRL:     u32 = 12;
    pub const KEY_LALT:      u32 = 13;
    pub const KEY_RALT:      u32 = 14;
    pub const KEY_A: u32 = 20; pub const KEY_B: u32 = 21; pub const KEY_C: u32 = 22;
    pub const KEY_D: u32 = 23; pub const KEY_E: u32 = 24; pub const KEY_F: u32 = 25;
    pub const KEY_G: u32 = 26; pub const KEY_H: u32 = 27; pub const KEY_I: u32 = 28;
    pub const KEY_J: u32 = 29; pub const KEY_K: u32 = 30; pub const KEY_L: u32 = 31;
    pub const KEY_M: u32 = 32; pub const KEY_N: u32 = 33; pub const KEY_O: u32 = 34;
    pub const KEY_P: u32 = 35; pub const KEY_Q: u32 = 36; pub const KEY_R: u32 = 37;
    pub const KEY_S: u32 = 38; pub const KEY_T: u32 = 39; pub const KEY_U: u32 = 40;
    pub const KEY_V: u32 = 41; pub const KEY_W: u32 = 42; pub const KEY_X: u32 = 43;
    pub const KEY_Y: u32 = 44; pub const KEY_Z: u32 = 45;
    pub const KEY_0: u32 = 50; pub const KEY_1: u32 = 51; pub const KEY_2: u32 = 52;
    pub const KEY_3: u32 = 53; pub const KEY_4: u32 = 54; pub const KEY_5: u32 = 55;
    pub const KEY_6: u32 = 56; pub const KEY_7: u32 = 57; pub const KEY_8: u32 = 58;
    pub const KEY_9: u32 = 59;
    pub const KEY_F1:  u32 = 60; pub const KEY_F2:  u32 = 61; pub const KEY_F3:  u32 = 62;
    pub const KEY_F4:  u32 = 63; pub const KEY_F5:  u32 = 64; pub const KEY_F6:  u32 = 65;
    pub const KEY_F7:  u32 = 66; pub const KEY_F8:  u32 = 67; pub const KEY_F9:  u32 = 68;
    pub const KEY_F10: u32 = 69; pub const KEY_F11: u32 = 70; pub const KEY_F12: u32 = 71;
    // Mouse button stubs
    pub const MOUSE_BUTTON_LEFT:   u32 = 200;
    pub const MOUSE_BUTTON_RIGHT:  u32 = 201;
    pub const MOUSE_BUTTON_MIDDLE: u32 = 202;
    pub const MOUSE_POS_X:   u32 = 0;
    pub const MOUSE_POS_Y:   u32 = 0;
    pub const MOUSE_WHEEL_X: u32 = 0;
    pub const MOUSE_WHEEL_Y: u32 = 0;
}

#[cfg(feature = "pyxel-core-backend")]
mod keys_pyxel {
    pub use pyxel::{
        KEY_ESCAPE, KEY_RETURN, KEY_SPACE, KEY_BACKSPACE, KEY_TAB,
        KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT,
        KEY_LSHIFT, KEY_RSHIFT, KEY_LCTRL, KEY_RCTRL, KEY_LALT, KEY_RALT,
        KEY_A, KEY_B, KEY_C, KEY_D, KEY_E, KEY_F, KEY_G, KEY_H, KEY_I, KEY_J,
        KEY_K, KEY_L, KEY_M, KEY_N, KEY_O, KEY_P, KEY_Q, KEY_R, KEY_S, KEY_T,
        KEY_U, KEY_V, KEY_W, KEY_X, KEY_Y, KEY_Z,
        KEY_0, KEY_1, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, KEY_7, KEY_8, KEY_9,
        KEY_F1, KEY_F2, KEY_F3, KEY_F4, KEY_F5, KEY_F6,
        KEY_F7, KEY_F8, KEY_F9, KEY_F10, KEY_F11, KEY_F12,
        MOUSE_BUTTON_LEFT, MOUSE_BUTTON_RIGHT, MOUSE_BUTTON_MIDDLE,
        MOUSE_POS_X, MOUSE_POS_Y, MOUSE_WHEEL_X, MOUSE_WHEEL_Y,
    };
}

#[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))]
pub use keys_soft::*;
#[cfg(feature = "pyxel-core-backend")]
pub use keys_pyxel::*;
