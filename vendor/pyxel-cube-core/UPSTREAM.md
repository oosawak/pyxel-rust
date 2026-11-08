# Upstream source

Source: https://github.com/kitao/pyxel/tree/e0b528f9a6f98bfc92bb47ebe1c7623cb3f609fd/crates/pyxel-core
Version: Pyxel 3.0.0 (MIT; see LICENSE).

This copy keeps the official Cube geometry, rasterizer, scene graph, image,
and font implementations. Local memory-I/O and streaming entry points are listed below. Cube depends on internal image/canvas/resource
types, so the core source is retained together rather than reimplementing its
rasterizer. The SDL2 desktop backend continues to use the original Git dependency.

Local adaptations:
- Standalone package manifest and build script, with no SDL2 build or linkage.
- platform/mod.rs routes to portable.rs. Platform operations fail explicitly;
  lifecycle, input, audio, and Canvas output belong to pyxel-rust's WASM backend.
- graphics.rs uses glow::Texture so its unused GL code also type-checks on WASM.
- cube/mesh.rs adds Mesh::from_glb_bytes; cube/glb_parser.rs shares the existing
  importer between file and memory input, rejecting image URIs before decoder I/O.
- audio.rs exposes a platform-free AudioRenderer for Web Audio. Sound/Music expose
  WAV bytes using the same official renderer; no sound algorithm is replaced.
- Image adds from_image_bytes/load_bytes/png_bytes, Font adds from_bytes for
  BDF/TTF/OTF, and Sound adds load_pcm_bytes through the shared PCM decoder.
  Image quantization and font/PCM decoding reuse the original implementations.
- ResourceData factors global bank access out of runtime access; memory_resource
  reads/writes the official ZIP/TOML archive and palette text with validation before
  bank mutation, plus manifest/cell allocation limits. Archive contents remain the
  official classic image/tilemap/sound/music format (no MML/PCM/Tone/Font/Node).
- TMX parsing adds from_tmx_text/load_tmx_text, sharing the official CSV parser.
- The pure official BGM MML generator is re-exported for the browser audio adapter.
- Image exposes read-only logical pixels and a validated canvas resize helper so the
  browser uses a live screen Image instead of a second pixel buffer.
- Screencast exposes memory GIF output and captured-frame count through the same
  official encoder. Browser recording is opt-in and bounded by its outer adapter.
- Browser RNG backends are enabled for dependencies using getrandom.

For upstream updates, compare the source with the pinned commit, reapply these
adapter changes, then run both native and wasm-backend rendering tests and the
browser smoke test. Browser GLB input comes from byte slices with resources embedded
in the BIN chunk. Other filesystem-based imports/exports still need browser-specific
loading; this adapter does not provide a virtual filesystem or Python callbacks.
