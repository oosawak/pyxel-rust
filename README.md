# pyxel-rust

> ⚠️ **このリポジトリは現在開発中です。**  
> API・仕様は予告なく変更される可能性があります。

**pyxel-rust** は、[Pyxel](https://github.com/kitao/pyxel) にインスパイアされた Rust 製の 2D ゲームエンジンです。  
WebAssembly (WASM) ターゲットをサポートし、ブラウザ上でゲームを動作させることができます。

---

## 特徴

- 🦀 **Rust 製コア** — 型安全で高速なゲームエンジン
- 🌐 **WASM 対応** — `wasm-pack` でブラウザ向けにビルド可能
- 🎮 **Pyxel 互換 API** — スプライト・BGM・入力など Pyxel ライクな API
- 🖼️ **Canvas API 統合** — ブラウザの Canvas 2D / WebGL で描画
- 🖥️ **デスクトップ対応** — ネイティブウィンドウでの実行もサポート

---

## リポジトリ構成

```
pyxel-rust/
├── src/                  ← pyxel-rust エンジン (Rust)
│   ├── lib.rs
│   ├── api/              ← graphics, input, audio, system ...
│   └── backend/
│       └── wasm_backend/ ← Canvas API 統合 (WASM)
├── docs/
│   └── examples/         ← サンプル・デモ
│       ├── arisa-quest/  ← GPS連動RPGデモ
│       ├── cubeboy/      ← プラットフォーマーデモ
│       └── lineboy/      ← ラインアクションデモ
└── ROADMAP.md
```

---

## ビルド方法

### WASM ビルド

```bash
# wasm-pack のインストール
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# ビルド例（arisa-quest パッケージ）
wasm-pack build . --target web --out-dir docs/examples/arisa-quest/pkg -- -p arisa_quest
```

### デスクトップビルド

```bash
cargo build
cargo run --example cubeboy
```

### ローカル確認（GitHub Pages 相当）

```bash
python3 -m http.server 8000 --directory docs
# → http://localhost:8000/examples/arisa-quest/
```

---

## デプロイ

GitHub Actions で `main` ブランチへの push 時に `docs/` を GitHub Pages として自動公開。

---

## 参考

- [Pyxel 公式](https://github.com/kitao/pyxel)
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)
- [Leaflet.js](https://leafletjs.com/)

## ライセンス

MIT License
