//! pyxel-rust - Rust implementation of Pyxel game engine
//!
//! A complete Rust implementation of the Pyxel retro game engine.
//! Built on Pyxel-core with CLI and GUI support.

// Import pyxel-core (extern crate) - it's aliased to `pyxel` in its Cargo.toml
extern crate pyxel;

/// API wrapper modules - Python-like interface to pyxel-core
pub mod api;

// Re-export commonly used types from pyxel
pub use pyxel::{
    Color, Image, Tilemap, Sound, Music, Channel,
};

/// Common prelude for Pyxel-rust games
/// 
/// Use this for convenient access to all Pyxel APIs:
/// ```ignore
/// use pyxel_rust::prelude::*;
/// 
/// fn main() {
///     init(160, 120, "My Game", 60);
///     run(update, draw);
/// }
/// ```
pub mod prelude {
    pub use crate::api::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cli() {
        // CLI tests will be added
    }
}
