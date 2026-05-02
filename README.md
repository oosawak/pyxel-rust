# pyxel-rust

> ⚠️ **このリポジトリは現在開発中です。**  
> API・ディレクトリ構成は予告なく変更される可能性があります。  
> 現時点では一般利用を想定していません。  
> **This repository is under active development and not ready for general use.**

---

Rust implementation of the Pyxel game engine, based on official Pyxel-core.

## 概要

**pyxel-rust** は、Pyxel ゲームエンジンの完全な Rust 実装です。

- 🎮 **完全な API 互換性**: Pyxel の 140+ API をすべて Rust で実装
- 🌐 **WASM対応**: Emscripten を使用してブラウザで動作
- 🔧 **簡潔な API**: Pyxel と同じシンプルで直感的なインターフェース
- 🎨 **豊富な描画機能**: 図形描画、テキスト、スプライト、タイルマップ対応

## クイックスタート

### インストール

```bash
cd /home/oosawak/Workspace/pyxel-rust
cargo build
```

### ゲーム作成例

```rust
use pyxel_rust::prelude::*;

fn main() {
    init(128, 128, "My Game", 60);
    
    run(update, draw);
}

fn update() {
    if btn(KEY_Q) {
        quit();
    }
}

fn draw() {
    cls(COLOR_BLACK);
    circ(64, 64, 10, COLOR_WHITE);
}
```

## 構造

```
pyxel-rust/
├── src/
│   ├── lib.rs              ← メイン API
│   ├── api/                ← API モジュール
│   │   ├── system.rs       (init, run, quit)
│   │   ├── graphics.rs     (pset, rect, circ)
│   │   ├── input.rs        (btn, btnp)
│   │   ├── audio.rs        (play, playm)
│   │   └── constants.rs    (KEY_*, COLOR_*)
│   └── wasm/               ← WASM 統合
├── examples/
│   ├── cubeboy.rs
│   └── lineboy.rs
└── pyxel_fork/             ← Pyxel 公式フォーク
```

## ビルド・実行

### デバッグビルド
```bash
cargo build
cargo test
```

### リリースビルド
```bash
cargo build --release
```

### Pyxel-rust CLI

完全な Rust 実装の CLI ツール：

```bash
# ゲーム例を実行
pyxel-rust run cubeboy

# ゲーム例を HTML/WASM に変換してブラウザで実行
pyxel-rust app2html cubeboy
pyxel-rust app2html cubeboy -p 3000  # カスタムポート指定

# 新しいプロジェクトを作成
pyxel-rust new my_game

# 他のコマンド
pyxel-rust editor
pyxel-rust build --release
pyxel-rust version
```

**注意**: CLI は Python 依存なし、完全な Rust 実装です。

### サンプルゲームの実行

#### Cubeboy (プラットフォーマー)
```bash
pyxel-rust run cubeboy
# または
cargo run --example cubeboy
```
プレイヤーを移動・ジャンプ・ダッシュさせるプラットフォーマーゲーム。
- **操作**: 矢印キーで移動、SPACE でジャンプ、X でダッシュ

### WASM ビルド
```bash
./scripts/build-wasm.sh --release
```

## ドキュメント

- [使用ガイド](./docs/GUIDE.md)
- [API リファレンス](./docs/API.md)
- [開発ロードマップ](./ROADMAP.md)

## ライセンス

MIT License - See LICENSE file

## 参考資料

- [Pyxel 公式](https://github.com/kitao/pyxel)
- [Pyxel-core (Rust実装)](./pyxel_fork/crates/pyxel-core/)
