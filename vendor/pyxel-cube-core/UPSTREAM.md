# Upstream source

Source: https://github.com/kitao/pyxel/tree/e0b528f9a6f98bfc92bb47ebe1c7623cb3f609fd/crates/pyxel-core
Version: Pyxel 3.0.0 (MIT; see LICENSE).

This copy keeps the official Cube geometry, rasterizer, scene graph, image,
font, and model code unchanged. Cube depends on internal image/canvas/resource
types, so the core source is retained together rather than reimplementing its
rasterizer. The SDL2 desktop backend continues to use the original Git dependency.

Local adaptations:
- Standalone package manifest and build script, with no SDL2 build or linkage.
- platform/mod.rs routes to portable.rs. Platform operations fail explicitly;
  lifecycle, input, audio, and Canvas output belong to pyxel-rust's WASM backend.
- graphics.rs uses glow::Texture so its unused GL code also type-checks on WASM.
- Browser RNG backends are enabled for dependencies using getrandom.

For upstream updates, compare the source with the pinned commit, reapply these
adapter changes, then run both native and wasm-backend rendering tests and the
browser smoke test. Filesystem-based imports/exports still need browser-specific
loading; this adapter does not provide a virtual filesystem or Python callbacks.
