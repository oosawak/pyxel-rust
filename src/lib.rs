//! pyxel-rust - Rust game engine with Pyxel-compatible API
//!
//! Backends:
//!   - `pyxel-core-backend` (default): wraps pyxel-core / SDL2
//!   - `wgpu-backend`: pure Rust / wgpu + winit, supports native + WASM

#[cfg(feature = "pyxel-core-backend")]
extern crate pyxel;

#[cfg(all(feature = "wasm-backend", not(feature = "pyxel-core-backend")))]
extern crate pyxel_cube_core as pyxel;

pub mod api;
pub mod backend;
pub mod games;

/// Official Pyxel 3.0 Cube types, scene graph, collisions, and software renderer.
/// Available on SDL2 and the Canvas/WASM backend.
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub use pyxel::cube;
#[cfg(feature = "pyxel-core-backend")]
pub use pyxel::screen;

// Re-export pyxel-core types when using pyxel-core-backend
#[cfg(feature = "pyxel-core-backend")]
pub use pyxel::Color;
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub use pyxel::{
    Channel, Font, Image, ImageSource, Music, RcChannel, RcFont, RcImage, RcMusic, RcSound,
    RcTilemap, RcTone, Sound, Tilemap, Tone, ToneMode,
};

// Generic Color type for wgpu-backend
#[cfg(not(feature = "pyxel-core-backend"))]
pub type Color = u8;

/// Prelude — `use pyxel_rust::prelude::*;` for all API functions
pub mod prelude {
    pub use crate::api::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {}
}

#[cfg(feature="wasm-backend")]
pub use api::screen;
