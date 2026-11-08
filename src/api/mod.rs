pub mod audio;
pub mod constants;
pub mod graphics;
pub mod input;
pub mod math;
pub mod resource;
/// Pyxel API — backend-agnostic public interface
pub mod system;

// Re-export all public functions
pub use audio::{is_playing, play, play_pos, playm, stop, try_play, try_playm};
pub use constants::*;
pub use graphics::{
    blt, bltm, camera, camera_reset, circ, circb, circfill, clip, clip_reset, cls, dither,
    get_palette_color, image_line, image_pset, image_rect, image_tri, line, pal, pal_reset, pget,
    pset, rect, rectb, rectfill, set_palette_color, text, tilemap_cls, tilemap_pget, tilemap_pset,
};
pub use input::{
    btn, btnp, btnp_hold, btnr, btnv, input_keys, mouse, mouse_wheel, mouse_x, mouse_y, set_btn,
    set_btnv, set_mouse_pos, set_mouse_visible,
};
pub use math::{
    abs, atan2, ceil, clamp, cos, floor, max, mid, min, rnd, rnd_int, rndf, rndi, rseed, sgn, sin,
    sqrt,
};
pub use system::{frame_count, height, init, quit, run, width};

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub mod graphics3d;
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub use graphics3d::{blit_3d, draw_mesh_3d, render_3d};

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub mod animation3d;
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub use animation3d::MeshAnimationPlayer;

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub use math::{noise, nseed};

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub use graphics::{blt_transform, bltm_transform, elli, ellib, fill, text_with_font, tri, trib};

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub use resource::*;

#[cfg(feature = "wasm-backend")]
pub use input::dropped_file_bytes;
pub use input::{dropped_files, input_text, set_dropped_files, set_input_text};

#[cfg(feature = "wasm-backend")]
pub use crate::backend::wasm_backend::system::browser_screenshot as screenshot_png;
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub use audio::gen_bgm;
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub use system::{fullscreen, icon, integer_scale, perf_monitor, resize, title};

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub use graphics::{blt3d, bltm3d};
#[cfg(feature = "pyxel-core-backend")]
pub use system::set_reset_handler;
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub use system::{flip, reset, screen_mode, show};

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub mod scene3d;
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub use graphics3d::render_3d_viewport;
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub use scene3d::{
    draw_node_primitive, with_node_draw_state, NodeMotionState, NodePainter, NodeScene,
};

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub use system::{init_with_options, InitOptions};
