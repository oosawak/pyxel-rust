# pyxel-rust の WASM 対応修正

このドキュメントは、pyxel-rust を WASM ゲーム開発に対応させるために実施した修正の履歴です。

## 修正概要

### 1. Cargo.toml - Feature フラグの整理

**ファイル**: `Cargo.toml`

**変更内容**:
```toml
[features]
default = ["pyxel-core-backend"]

pyxel-core-backend = ["dep:pyxel-core", "pyxel-core/sdl2_static"]
sdl2_static = ["dep:pyxel-core", "pyxel-core/sdl2_static", "pyxel-core-backend"]
sdl2_dynamic = ["dep:pyxel-core", "pyxel-core/sdl2_dynamic", "pyxel-core-backend"]
wgpu-backend = ["dep:pixels", "dep:winit"]
wasm-backend = ["dep:wasm-bindgen", "dep:wasm-bindgen-futures", "dep:js-sys", "dep:web-sys"]
```

**理由**:
- WASM ターゲットでは `default` フィーチャ（pyxel-core-backend）を使用してはいけない
- pyxel-core は SDL2 を CMake でコンパイルするため、WASM では失敗する
- WASM ゲームは `--features wasm-backend` で明示的にビルドしなければならない

### 2. src/lib.rs - sendai_* モジュール削除

**ファイル**: `src/lib.rs`

**変更内容**:
- `pub mod sendai_game;`
- `pub mod sendai_ui;`
- `pub mod sendai_bindings;`

を削除。

**理由**:
- 仙台大観音ゲームは pyxel-rust 内に含めるべきではない
- 独立した WASM ゲームライブラリとして `/Workspace/sendai_daikannon/` で管理
- pyxel-rust は基盤エンジンのみを提供

### 3. Workspace 設定

**ファイル**: `/Workspace/Cargo.toml`

**変更内容**:
```toml
[workspace]
members = [
    "pyxel-rust",
    "cubeboy_rust",
    "lineboy_rust",
    "rpg_rust",
    "arisa_quest",
    "sendai_daikannon",  // 追加
]
```

**理由**:
- WASM ゲームクレートをワークスペースに登録して一括ビルド可能に
- 依存関係を適切に管理

## ビルドコマンド

### デスクトップアプリ（ネイティブ）

```bash
# デフォルト（SDL2 静的リンク）
cargo build --release

# SDL2 動的リンク
cargo build --release --features sdl2_dynamic

# Wgpu バックエンド
cargo build --release --features wgpu-backend
```

### WASM ゲーム

```bash
cd /home/oosawak/Workspace/<game_name>

# WASM バイナリをビルド
cargo build --target wasm32-unknown-unknown --release --features wasm-backend

# wasm-bindgen で JavaScript 生成
~/.cargo/bin/wasm-bindgen \
  /home/oosawak/Workspace/target/wasm32-unknown-unknown/release/<game_name>.wasm \
  --out-dir /path/to/output \
  --target web
```

## エラー対応

### エラー: "wasm-ld-18 doesn't exist"

**症状**:
```
clang: error: unable to execute command: Executable "wasm-ld-18" doesn't exist!
```

**原因**: 
- `cargo build --target wasm32-unknown-unknown --release` で `default` フィーチャ（pyxel-core-backend）が有効になっている
- pyxel-core が SDL2 を CMake でコンパイルしようとして失敗

**解決策**:
```bash
# ❌ NG: デフォルト feature でビルド
cargo build --target wasm32-unknown-unknown --release

# ✅ OK: WASM feature で明示的にビルド
cargo build --target wasm32-unknown-unknown --release --features wasm-backend
```

### エラー: "CMake Error"

**症状**:
```
CMakeError occurred when trying to build: platform "wasm32-unknown-unknown"
```

**原因**: 上記と同じ

**対応**: 上記を参照

## チェックリスト

新しい WASM ゲームを追加する際：

- [ ] `/Workspace/<game_name>/` ディレクトリを作成
- [ ] `Cargo.toml` に `wasm-backend` feature を定義
- [ ] `src/lib.rs` に `#[wasm_bindgen]` で public API を実装
- [ ] `/Workspace/Cargo.toml` の `members` に新しいゲームを追加
- [ ] `cargo build --target wasm32-unknown-unknown --release --features wasm-backend` で成功を確認
- [ ] wasm-bindgen で `.js` と `.wasm` を生成
- [ ] HTML に `import` と初期化コードを追加

---

## 歴史

| 日時 | 修正内容 |
|------|---------|
| 2025-05-05 | Cargo.toml の feature 整理、sendai_* モジュール削除 |
