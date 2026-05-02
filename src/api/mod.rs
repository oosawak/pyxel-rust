/// Pyxel API wrapper modules
/// 
/// This module re-exports all pyxel-core APIs with a Python-like interface.
/// All functions are designed to match the original Python Pyxel behavior.

pub mod system;
pub mod graphics;
pub mod input;
pub mod math;
pub mod audio;
pub mod resource;
pub mod constants;

// Re-export commonly used types
pub use pyxel::{
    Color, Channel,
};
pub use resource::{Image, Tilemap};

// Re-export all public functions
pub use system::{init, run, quit, width, height, frame_count};
pub use graphics::{cls, pset, pget, line, rect, rectfill, circ, circfill, text, blt, bltm, clip, clip_reset, camera, camera_reset, pal, pal_reset, dither};
pub use input::{btn, btnp, btnr, mouse_x, mouse_y, set_mouse_visible};
pub use math::{sgn, abs, rnd, rnd_int, clamp, mid, max, min};
pub use audio::{play, playm, stop, is_playing};
pub use constants::*;
