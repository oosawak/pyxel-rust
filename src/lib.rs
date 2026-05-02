//! pyxel-rust - Rust implementation of Pyxel game engine
//!
//! A complete Rust implementation of the Pyxel retro game engine.
//! Built on Pyxel-core with CLI and GUI support.

// Re-export from pyxel-core
pub use pyxel::*;

/// Common prelude for Pyxel-rust games
pub mod prelude {
    pub use crate::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cli() {
        // CLI tests will be added
    }
}
