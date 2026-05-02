//! pyxel-rust - Rust game engine with Pyxel-compatible API
//!
//! Backends:
//!   - `pyxel-core-backend` (default): wraps pyxel-core / SDL2
//!   - `wgpu-backend`: pure Rust / wgpu + winit, supports native + WASM

#[cfg(feature = "pyxel-core-backend")]
extern crate pyxel;

pub mod backend;
pub mod api;

// Re-export pyxel-core types when using pyxel-core-backend
#[cfg(feature = "pyxel-core-backend")]
pub use pyxel::{Color, Image, Tilemap, Sound, Music, Channel};

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
