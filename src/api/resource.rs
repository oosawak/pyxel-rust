/// Resource Management API

#[cfg(feature = "pyxel-core-backend")]
pub type Image = pyxel::Image;
#[cfg(feature = "pyxel-core-backend")]
pub type Tilemap = pyxel::Tilemap;

// wgpu-backend stubs (needed so `use pyxel_rust::prelude::*` compiles)
#[cfg(not(feature = "pyxel-core-backend"))]
pub struct Image;
#[cfg(not(feature = "pyxel-core-backend"))]
pub struct Tilemap;
