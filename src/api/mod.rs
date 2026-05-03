/// Pyxel API — backend-agnostic public interface

pub mod system;
pub mod graphics;
pub mod input;
pub mod math;
pub mod audio;
pub mod resource;
pub mod constants;

// Re-export all public functions
pub use system::{init, run, quit, width, height, frame_count};
pub use graphics::{cls, pset, pget, line, rect, rectfill, circ, circfill, text, blt, bltm,
                   clip, clip_reset, camera, camera_reset, pal, pal_reset, dither,
                   set_palette_color, get_palette_color,
                   image_rect, image_pset, image_line, image_tri,
                   tilemap_cls, tilemap_pget, tilemap_pset};
pub use input::{btn, btnp, btnr, btnp_hold, mouse_x, mouse_y, set_mouse_visible};
pub use math::{sgn, abs, rnd, rnd_int, rndf, clamp, mid, max, min, sin, cos, sqrt, floor, rseed};
pub use audio::{play, playm, stop, is_playing};
pub use constants::*;
