# pyxel-rust Development Roadmap

## プロジェクト概要

[Pyxel](https://github.com/kitao/pyxel) にインスパイアされた Rust 製 2D ゲームライブラリ。  
`wasm-bindgen` を使用してブラウザ（WASM）でも動作します（Emscripten は使用していません）。

---

## 現在の状態

### ✅ 完了済み

**コア実装**
- Rust プロジェクト基盤・Cargo.toml 設定
- Pyxel フォーク（pyxel-core）統合
- System API: `init` / `run` / `quit` / `width` / `height` / `frame_count`
- Graphics API: `cls` / `pset` / `pget` / `line` / `rect` / `rectfill` / `circ` / `circfill` / `text` / `blt` / `bltm` / `clip` / `camera` / `pal` / `dither`
- Input API: `btn` / `btnp` / `btnr` / `mouse_x` / `mouse_y` / `mouse_wheel`
- Math API: `sin` / `cos` / `sqrt` / `rnd` など
- Constants: `KEY_*` / `COLOR_*`
- Audio API: `play` / `playm` / `stop`（バックエンド依存）
- Resource API: Image / Tilemap / Sound / Music

**バックエンド**
- `pyxel-core-backend`（SDL2 静的リンク、デフォルト）
- `wgpu-backend`（pixels + winit、デスクトップ代替）
- `wasm-backend`（wasm-bindgen + web-sys Canvas 2D、ブラウザ）

**ゲーム移植・デモ**
- Cubeboy（プラットフォーマー）— Rust 移植完了
- Lineboy（ラインアクション）— Rust 移植完了
- RPG サンプル — Rust 実装済み
- Arisa Quest（GPS 連動 RPG デモ）— ブラウザデモ公開中

**インフラ・ツール**
- Python CLI ツール（`pyxel_rust_cli.py`）— Pyxel CLI 拡張、`rust_run` / `rust_package` / `rust_app2wasm` など
- ビルド依存インストーラー（`install-deps.sh`）
- CI スクリプト（`scripts/ci.sh`）— fmt / clippy / build / test
- GitHub Pages 公開（`docs/` ディレクトリ）

---

## 今後の課題

### 🔧 未整備・改善余地あり

- **WASM ビルド手順の整備** — wasm-pack のインストールから公開までのフローを簡略化したい
- **GitHub Actions** — CI / GitHub Pages 自動デプロイのワークフロー未設定
- **Audio（WASM）** — ブラウザでのオーディオ再生は未検証
- **Tilemap / Image ロード** — `.pyxres` リソースファイルの WASM 対応
- **API カバレッジ拡充** — Pyxel 全 API との差分を埋める

### 🎮 ゲーム・デモ

- **Arisa Quest** — GPS 連動 RPG デモの継続開発（バトルシステム・地図機能）
- **追加サンプル** — より多くのゲームジャンルのデモ

### 📚 ドキュメント

- API リファレンスの整備
- チュートリアル・使用例の充実

---

## リソース・参考

- **Pyxel-core**: `./pyxel_fork/crates/pyxel-core/`
- **参考**: [Pyxel 公式](https://github.com/kitao/pyxel)
- **WASM ビルド**: [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)

