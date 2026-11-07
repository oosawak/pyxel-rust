/// Game modules (WASM builds only)
#[cfg(feature = "wasm-backend")]
pub mod arisa;

pub mod aobacastle;
#[cfg(feature = "wasm-backend")]
pub mod nanoteras;

#[cfg(feature = "wasm-backend")]
pub mod retro3d;

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub mod cube_examples;

#[cfg(feature = "wasm-backend")]
pub mod api_coverage;
