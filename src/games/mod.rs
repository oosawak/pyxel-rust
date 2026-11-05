/// Game modules (WASM builds only)
#[cfg(feature = "wasm-backend")]
pub mod arisa;

#[cfg(feature = "wasm-backend")]
pub mod nanoteras;
pub mod aobacastle;

#[cfg(feature = "wasm-backend")]
pub mod retro3d;
