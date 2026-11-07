//! Shared resource handles. Editing a handle edits the bank used by drawing/audio.
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub use crate::{
    Channel, Font, Image, ImageSource, Music, RcChannel, RcFont, RcImage, RcMusic, RcSound,
    RcTilemap, RcTone, Sound, Tilemap, Tone, ToneMode,
};
#[cfg(not(any(feature = "pyxel-core-backend", feature = "wasm-backend")))]
pub struct Image;
#[cfg(not(any(feature = "pyxel-core-backend", feature = "wasm-backend")))]
pub struct Tilemap;

/// Snapshot of bank handles. Contents are shared; replacing entries in this Vec is local.
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn images() -> Vec<RcImage> {
    pyxel::images().clone()
}

/// Snapshot of bank handles. Contents are shared; replacing entries in this Vec is local.
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn tilemaps() -> Vec<RcTilemap> {
    pyxel::tilemaps().clone()
}

/// Snapshot of bank handles. Contents are shared; replacing entries in this Vec is local.
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn sounds() -> Vec<RcSound> {
    pyxel::sounds().clone()
}

/// Snapshot of bank handles. Contents are shared; replacing entries in this Vec is local.
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn musics() -> Vec<RcMusic> {
    pyxel::musics().clone()
}

/// Snapshot of bank handles. Contents are shared; replacing entries in this Vec is local.
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn channels() -> Vec<RcChannel> {
    pyxel::channels().clone()
}

/// Snapshot of bank handles. Contents are shared; replacing entries in this Vec is local.
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn tones() -> Vec<RcTone> {
    pyxel::tones().clone()
}
