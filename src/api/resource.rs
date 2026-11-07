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

#[cfg(feature = "wasm-backend")]
pub use pyxel::memory_resource::ResourceOptions;
#[cfg(feature = "wasm-backend")]
pub fn load_bytes(bytes: &[u8], options: ResourceOptions) -> Result<(), String> {
    pyxel::memory_resource::load(bytes, options)
}
#[cfg(feature = "wasm-backend")]
pub fn save_bytes(options: ResourceOptions) -> Result<Vec<u8>, String> {
    pyxel::memory_resource::save(options)
}
#[cfg(feature = "wasm-backend")]
pub fn load_pal_text(text: &str) -> Result<(), String> {
    pyxel::memory_resource::load_palette(text)
}
#[cfg(feature = "wasm-backend")]
pub fn save_pal_text() -> String {
    pyxel::memory_resource::save_palette()
}
#[cfg(feature = "pyxel-core-backend")]
pub fn load(
    filename: &str,
    exclude_images: bool,
    exclude_tilemaps: bool,
    exclude_sounds: bool,
    exclude_musics: bool,
) -> Result<(), String> {
    pyxel::pyxel().load_resource(
        filename,
        Some(exclude_images),
        Some(exclude_tilemaps),
        Some(exclude_sounds),
        Some(exclude_musics),
    )
}
#[cfg(feature = "pyxel-core-backend")]
pub fn save(
    filename: &str,
    exclude_images: bool,
    exclude_tilemaps: bool,
    exclude_sounds: bool,
    exclude_musics: bool,
) -> Result<(), String> {
    pyxel::pyxel().save_resource(
        filename,
        Some(exclude_images),
        Some(exclude_tilemaps),
        Some(exclude_sounds),
        Some(exclude_musics),
    )
}
#[cfg(feature = "pyxel-core-backend")]
pub fn load_pal(filename: &str) -> Result<(), String> {
    pyxel::pyxel().load_palette(filename)
}
#[cfg(feature = "pyxel-core-backend")]
pub fn save_pal(filename: &str) -> Result<(), String> {
    pyxel::pyxel().save_palette(filename)
}

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn colors() -> std::cell::RefMut<'static, Vec<u32>> {
    pyxel::colors()
}

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn with_images<T>(edit: impl FnOnce(&mut Vec<RcImage>) -> T) -> T {
    edit(&mut pyxel::images())
}

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn with_tilemaps<T>(edit: impl FnOnce(&mut Vec<RcTilemap>) -> T) -> T {
    edit(&mut pyxel::tilemaps())
}

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn with_sounds<T>(edit: impl FnOnce(&mut Vec<RcSound>) -> T) -> T {
    edit(&mut pyxel::sounds())
}

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn with_musics<T>(edit: impl FnOnce(&mut Vec<RcMusic>) -> T) -> T {
    edit(&mut pyxel::musics())
}

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn with_channels<T>(edit: impl FnOnce(&mut Vec<RcChannel>) -> T) -> T {
    edit(&mut pyxel::channels())
}

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn with_tones<T>(edit: impl FnOnce(&mut Vec<RcTone>) -> T) -> T {
    edit(&mut pyxel::tones())
}
#[cfg(feature = "pyxel-core-backend")]
pub fn screenshot(filename: Option<&str>, scale: Option<u32>) -> Result<(), String> {
    pyxel::pyxel().save_screenshot(filename, scale)
}
