/// Resource Management API
/// 
/// Convenient wrapper types for managing images and tilemaps.
/// These provide a more idiomatic Rust interface over pyxel-core resources.

/// Image wrapper - represents a sprite or drawable image
/// 
/// Note: pyxel-core uses raw pointers for images.
/// This wrapper is provided for convenience and future extensibility.
pub type Image = pyxel::Image;

/// Tilemap wrapper - represents a tilemap
/// 
/// Note: pyxel-core uses raw pointers for tilemaps.
/// This wrapper is provided for convenience and future extensibility.
pub type Tilemap = pyxel::Tilemap;
