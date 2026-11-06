# Official Cube example ports

Source: Pyxel 3.0.0, commit e0b528f9a6f98bfc92bb47ebe1c7623cb3f609fd.
Original Python: https://github.com/kitao/pyxel/tree/v3.0.0/python/pyxel/examples/cube

The six examples c01 through c06 are translated to Rust in the parent directory.
The camera, geometry, model transforms, animation names, physics constants,
collision response, shooting/lock-on logic and stage layout follow that release.
The existing wasm-bindgen Canvas backend presents the official software renderer.
Node update/draw/contact phases are explicitly orchestrated by these examples;
this does not introduce a general Python-style Node callback API to pyxel-rust.

The original GLB assets are copied byte-for-byte. The MIT license below applies
to sample code and first-party assets. Gazelle is separately CC BY 4.0: see
[gazelle.md](gazelle.md) for monkus's attribution and upstream modifications.
No additional geometry or animation edits were made here.

cat_16x16.bin and laser.bin contain decoded Pyxel color indices from the original
PNGs; regenerate with scripts/prepare-cube-textures.py --upstream /path/to/pyxel.
The tufts.rs list records Python random.Random(8) / random.Random(42) placement
from c04, so grass placement does not depend on a different Rust PRNG.

sounds.json contains the original effect MML strings. examples/cube_audio.rs
renders finite WAV effects using official pyxel-core Sound; the browser plays
these samples on their original channels after the user enables sound.
Regenerate with cargo run --locked --example cube_audio.

Browser adaptations: touch controls, explicit pause, explicit reset for c05,
optional sound activation, and browser gamepad polling. Q pauses a Rust demo
instead of closing a browser window. Switching the comparison view starts a new
instance; it does not synchronize time between Python and Rust runtimes.

The UI links the live official Python pages for reference; these may advance
beyond the pinned release. This is a functional port, not a claim of pixel-perfect
or timing-identical output across runtimes. Rust uses f32 core math while Python
expressions can evaluate at double precision.
