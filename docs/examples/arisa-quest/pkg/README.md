# pyxel-rust

> ⚠️ **このリポジトリは現在開発中です。**  
> API・仕様は予告なく変更される可能性があります。

**pyxel-rust** は、[Pyxel](https://github.com/kitao/pyxel) にインスパイアされた Rust 製の 2D ゲームライブラリです。  
`wasm-bindgen` を使用して WebAssembly ターゲットをサポートし、ブラウザ上でゲームを動作させることができます。  
（Emscripten は使用していません。元の Python 版 Pyxel が Emscripten を使用しています。）

---

## 特徴

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

SDL2 等のシステムライブラリが必要です：

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
