# pyxel-rust

> ⚠️ **このリポジトリは現在開発中です。**  
> API・仕様は予告なく変更される可能性があります。

**pyxel-rust** は、[Pyxel](https://github.com/kitao/pyxel) にインスパイアされた Rust 製のゲームライブラリです。
`wasm-bindgen` を使用して WebAssembly ターゲットをサポートし、ブラウザ上でゲームを動作させることができます。  
（Emscripten は使用していません。元の Python 版 Pyxel が Emscripten を使用しています。）

---

## 特徴

- 🧊 **Pyxel Cube 3D（デスクトップ・ブラウザ）** — 公式 Pyxel 3.0.0 のソフトウェア3D描画・シーングラフ・衝突判定を利用
- 🦀 **Rust 製コア** — 型安全で高速なゲームエンジン
- 🌐 **WASM 対応** — `wasm-bindgen` / `wasm-pack` でブラウザ向けにビルド（`wasm32-unknown-unknown`）
- 🖼️ **Canvas 2D 統合** — ブラウザでは `web-sys` 経由で Canvas 2D API を使用
- 🖥️ **デスクトップ対応** — pyxel-core (SDL2) または wgpu バックエンドで実行
- 🎮 **Pyxel 互換 API** — `init` / `run` / `cls` / `blt` / `text` など主要 API を実装
- 🐍 **Python CLI ツール** — `pyxel_rust_cli.py`（Python 製）で Rust ゲームのビルド・実行を管理

---

## バックエンド構成

| バックエンド | 対象 | 使用ライブラリ | feature フラグ |
|---|---|---|---|
| `pyxel-core-backend` | デスクトップ（デフォルト） | pyxel-core (SDL2) | `pyxel-core-backend` |
| `wgpu-backend` | デスクトップ（代替） | pixels + winit | `wgpu-backend` |
| `wasm-backend` | ブラウザ | wasm-bindgen + web-sys | `wasm-backend` |

---

## リポジトリ構成

```
pyxel-rust/
├── src/                    ← pyxel-rust エンジン (Rust)
│   ├── lib.rs
│   ├── api/                ← graphics, input, audio, system, math ...
│   └── backend/
│       ├── wasm_backend/   ← Canvas 2D API (ブラウザ / wasm-bindgen)
│       └── wgpu_backend/   ← wgpu バックエンド (デスクトップ代替)
├── examples/               ← Rust サンプルソース
│   ├── cubeboy.rs          ← プラットフォーマー
│   ├── lineboy.rs          ← ラインアクション
│   └── rpg.rs              ← RPG
├── docs/
│   └── examples/           ← ブラウザで動作するデモ (GitHub Pages)
│       ├── arisa-quest/    ← GPS連動RPGデモ
│       ├── cubeboy/        ← プラットフォーマーデモ
│       └── lineboy/        ← ラインアクションデモ
├── scripts/
│   └── ci.sh               ← CI スクリプト (fmt / clippy / build / test)
├── pyxel_rust_cli.py       ← Python CLI (Pyxel CLI 拡張)
├── pyproject.toml          ← Python パッケージ設定
└── install-deps.sh         ← ビルド依存ライブラリインストーラー
```

---

## セットアップ

### 依存ライブラリのインストール（デスクトップビルド用）

[rustup](https://rustup.rs/) を使用してください。`rust-toolchain.toml` は公式Pyxel 3.0.0と同じ
`nightly-2026-11-05`（WASMターゲット・rustfmt・Clippy付き）を指定します。

デフォルトの静的SDL2ビルドにはCMake、C/C++コンパイラ、libclangが必要です。
macOSではXcode Command Line ToolsとCMakeを準備します：

```bash
xcode-select --install  # 未導入の場合
brew install cmake
```

Debian / Ubuntuでは以下のスクリプトで依存ライブラリを導入できます：

```bash
sudo bash install-deps.sh
```

### Python CLI のインストール

```bash
pip install -e .
# → pyxel-rust コマンドが使用可能になります
```

---

## ビルド方法

### WASM ビルド（ブラウザ向け）

```bash
# wasm-pack のインストール（未インストールの場合）
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# ビルド（wasm-backend feature を指定）
wasm-pack build --target web --out-dir docs/examples/arisa-quest/pkg \
  -- --no-default-features --features wasm-backend
```

### デスクトップビルド

```bash
# デフォルト（pyxel-core / SDL2 バックエンド）
cargo build
cargo run --example cubeboy

# wgpu バックエンドを使う場合
cargo build --no-default-features --features wgpu-backend
```

### Pyxel 3.0.0 / レトロ3D

デフォルトのSDL2バックエンドは、公式Pyxel **3.0.0** のリリースコミット
`e0b528f9a6f98bfc92bb47ebe1c7623cb3f609fd` に固定しています。
CargoがGitHubから直接取得するため、`pyxel_fork` の手動配置は不要です。
初回は依存関係とSDL2ソースのダウンロードにネットワーク接続が必要です。

```bash
# 回転する立方体（SDL2静的リンク）
cargo run --example retro3d

# システムのSDL2を使用する場合
cargo run --no-default-features --features sdl2_dynamic --example retro3d

# macOS + Homebrew SDL2（brew install sdl2）の場合
BINDGEN_EXTRA_CLANG_ARGS="-I$(brew --prefix sdl2)/include/SDL2" \
LIBRARY_PATH="$(brew --prefix sdl2)/lib" \
cargo run --no-default-features --features sdl2_dynamic --example retro3d

# ウィンドウ不要の3D描画テスト
cargo test --locked --test retro3d

# wgpuの画像・タイルマップ互換性テスト
cargo test --locked --no-default-features --features wgpu-backend --test wgpu_compat
```

`pyxel_rust::cube` は公式の `Vec3` / `Mat4` / `Quat` / `Camera` /
`Mesh` / `Node` / `Scene` / 衝突判定と描画関数を公開します。
`prelude::render_3d(&target, &camera, |ctx| { ... })` で画面やオフスクリーン画像に描画できます。
SDL2の画面は `pyxel_rust::screen()`、各バックエンドのオフスクリーン画像は `Image::new(width, height)` で取得します。
使用例は [`examples/retro3d.rs`](examples/retro3d.rs) を参照してください。

`render_3d` はフレームごとに深度をリセットし、カメラのバッファを再利用します。
`camera.clear_color = None` なら描画先画像の既存ピクセルを保持します。
SDL2の画面に直接描く場合は既存の2D背景を保持し、描画後に `text` などでHUDを重ねられます。
公式Python版のNodeコールバックはこのRustラッパーでは自動実行しません。
公式Python API全体との互換性を保証するものではありません。
`wasm-backend` でも同じ `cube` / `Image` / `render_3d` を利用できます。
描画した画像を `blit_3d(&target, x, y)` でCanvas用フレームバッファへ転送し、
転送は画像全体を上書きするため、その後に2Dの文字やUIを重ねます。`wgpu-backend` のCube接続は未対応です。

### ブラウザ3Dデモ

```bash
wasm-pack build --target web --out-dir docs/examples/retro3d/pkg \
  -- --locked --no-default-features --features wasm-backend
python3 -m http.server 8765 --bind 127.0.0.1 --directory docs
# http://127.0.0.1:8765/examples/retro3d/
```

3つの立方体を回転し、一時停止・距離変更できます。
左右キー / Space / R、または画面のタッチボタンで操作します。
実装は [`src/games/retro3d.rs`](src/games/retro3d.rs)、
公開ページは [`docs/examples/retro3d/`](docs/examples/retro3d/) です。
ビルド済みWASM/JSを同梱しているため、プレビューだけならHTTPサーバーの起動で確認できます。

ブラウザ版は公式3.0.0の固定ソースを
[`vendor/pyxel-cube-core`](vendor/pyxel-cube-core/UPSTREAM.md) に同梱しています。
Cubeの描画・数学・衝突判定を維持し、SDL2との接続を切り離して既存のCanvasバックエンドへ接続します。
ブラウザでは `cube::Mesh::from_glb_bytes(&bytes, colkey, fps)` でGLBのバイト列を読み込み、
`draw_mesh_3d(ctx, &mesh.borrow(), &transform, state)` で階層変換とマテリアルを反映して描画できます。
ファイルシステムやPythonのNodeコールバックは使用しません。

デモ下部の「ファイルを選ぶ」で自分のGLBを表示できます。「サンプルを表示」では
テクスチャ付きピラミッドを読み込みます。「動くサンプル」には上下移動と回転の2クリップを用意しています。
選択したファイルはブラウザ内で処理します。
表示位置と倍率は自動調整し、読み込みに失敗しても現在のモデルを保持します。

対応範囲：
- デモのファイル上限は16 MiB。画像・バッファをGLBのBINチャンクに埋め込んでください。
- 埋め込みテクスチャ、ノードの親子変換、パートごとのマテリアルに対応します。
- ノードの移動・回転・拡大縮小アニメーションを、クリップ選択・再生/停止・シーク・ループ・速度変更で確認できます。
- スキニング、モーフターゲット、外部画像/バッファは未対応です。表示位置と倍率は初期姿勢を基準に調整します。
- サンプルは外部アセットを使わず生成しています。`python3 scripts/generate-retro3d-model.py` で再生成できます。

`MeshAnimationPlayer` はSDL2とWASMの共通APIです。GLB読込時と同じFPSを指定し、
ゲームの `update` から `advance(経過秒)` を呼び出します。選択時に元の姿勢を復元するため、
異なるノードを動かすクリップに切り替えても前の姿勢が残りません。

```rust
let mut player = MeshAnimationPlayer::new(mesh.clone(), 30.0)?;
player.select_clip(Some(0))?;
player.advance(1.0 / 30.0)?; // ゲーム更新ごとのシミュレーション時間
player.set_playing(false);
player.seek(1.0)?;          // 秒単位でシーク
player.select_clip(None)?;  // 元の姿勢へ戻す
```

デモの「一時停止」は自動回転とアニメーションを両方停止します。
アニメーション欄の再生ボタンはモデルの動きを再開し、シークはアニメーションを停止して指定位置を表示します。
アニメーション付きモデルの読込時は、動きを見やすくするため自動回転をオフにします。

```bash
# ブラウザ用のプラットフォーム非依存レンダラーも同じ描画テストを実行
cargo test --locked --no-default-features --features wasm-backend --test retro3d --test glb_browser --test animation_browser

# ブラウザで表示・操作・モバイルレイアウトを検証（HTTPサーバーを起動した状態）
npm install --no-save playwright@1.62.1
npx playwright install chromium
node scripts/playtest-retro3d.cjs
```

スクリーンショットは一時ディレクトリの `pyxel-retro3d-smoke` に保存します。
`PYXEL_TEST_URL` と `PYXEL_TEST_OUTPUT` でテスト先と出力先を指定できます。

公式3.0.0では旧MML構文と2.0以前のリソース読込が削除されています。
古いデスクトップ用アセットは移行が必要です。
詳細: [公式変更履歴](https://github.com/kitao/pyxel/blob/v3.0.0/CHANGELOG.md)

### CLI コマンド（Python CLI 経由）

```bash
pyxel-rust rust_run <PROJECT_NAME>       # cargo run で実行
pyxel-rust rust_package <PROJECT_NAME>  # .pyxapp にパッケージング
pyxel-rust rust_app2wasm <PROJECT_NAME> # WASM ビルド＆ローカルサーバー起動
```

### ローカル確認（GitHub Pages 相当）

```bash
python3 -m http.server 8000 --directory docs
# → http://localhost:8000/examples/arisa-quest/
```

### CI（フォーマット・Lint・テスト）

```bash
bash scripts/ci.sh
```

---

## デプロイ

`docs/` ディレクトリを GitHub Pages として公開しています。  
自動デプロイは未設定のため、手動で `docs/` を更新後プッシュしてください。

---

## 参考

- [Pyxel 公式](https://github.com/kitao/pyxel) — API 設計の参考元
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) — Rust ↔ JS バインディング
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) — WASM ビルドツール
- [Leaflet.js](https://leafletjs.com/) — arisa-quest の地図表示に使用
- [web-sys](https://docs.rs/web-sys) — Canvas 2D API バインディング

---

## ライセンス

MIT License
