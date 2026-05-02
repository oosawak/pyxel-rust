# GitHub Pages Configuration for pyxel-rust

This directory contains the web-based games powered by pyxel-rust.

## Games

- **[Cubeboy](./cubeboy/)** - A platformer game written in Rust and compiled to WebAssembly

## Building Games for Web

To build a game for web:

```bash
pyxel-rust app2html cubeboy
```

This will:
1. Compile the Rust game to WebAssembly using `wasm-pack`
2. Generate the necessary JavaScript bindings
3. Start a local HTTP server to test the game

## Deploying to GitHub Pages

1. Build the game with `pyxel-rust app2html <game_name>`
2. Copy the generated files from `web/pkg/` to `docs/<game_name>/`
3. Commit and push to GitHub
4. Enable GitHub Pages in repository settings (set source to `docs/` folder)
5. Access your game at `https://<username>.github.io/pyxel-rust/<game_name>/`

## Current Status

- [ ] Cubeboy WASM build
- [ ] Lineboy WASM build
- [ ] Deploy to GitHub Pages
