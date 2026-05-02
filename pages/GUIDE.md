# pyxel-rust 使用ガイド

## インストール・ビルド

### 環境要件
- Rust 1.70+ (`rustup` でインストール)
- Cargo (Rust に付属)
- Git

### セットアップ

```bash
cd /home/oosawak/Workspace/pyxel-rust

# 依存関係のダウンロード
cargo fetch

# ビルド検証
cargo build
```

## プロジェクト構造

```
pyxel-rust/
├── src/
│   ├── lib.rs              ← メイン公開 API
│   └── api/                ← API モジュール（実装予定）
├── examples/               ← 使用例
├── docs/                   ← ドキュメント
├── scripts/
│   └── notify-discord.sh   ← Discord 通知
└── ROADMAP.md              ← 開発ロードマップ
```

## 開発フロー

### 1. タスク確認

```bash
# TODO リストを確認
sqlite3 ~/.copilot/session-state/*/session.db \
  "SELECT id, title, status FROM todos WHERE id LIKE 'pyxel-rust%'"
```

### 2. タスク開始

```bash
sqlite3 ~/.copilot/session-state/*/session.db \
  "UPDATE todos SET status = 'in_progress' WHERE id = 'pyxel-rust-api-system'"
```

### 3. コード実装

```bash
# lib.rs に API を実装
vim src/lib.rs

# テスト実行
cargo test
```

### 4. Git コミット

```bash
git add src/lib.rs
git commit -m "feat(api): Implement system functions

- init(width, height, title, fps)
- run(update_fn, draw_fn)
- quit()

Closes: pyxel-rust-api-system"
```

### 5. Discord 通知

```bash
export DISCORD_WEBHOOK_URL="https://discordapp.com/api/webhooks/..."
./scripts/notify-discord.sh "System API implemented ✅" 3066993
```

### 6. タスク完了

```bash
sqlite3 ~/.copilot/session-state/*/session.db \
  "UPDATE todos SET status = 'done' WHERE id = 'pyxel-rust-api-system'"
```

## API 実装順序

推奨する実装順:

1. **System API** - 基本的な初期化・終了
   - `init()`, `run()`, `quit()`
   - `width`, `height`, `frame_count`

2. **Graphics API** - 描画基本
   - `cls()`, `pset()`, `pget()`
   - `line()`, `rect()`, `circ()`

3. **Input API** - ユーザー入力
   - `btn()`, `btnp()`, `btnr()`
   - `mouse_x`, `mouse_y`

4. **Math Utilities** - 数値処理
   - `sin()`, `cos()`, `sqrt()`, `rnd()`

5. **Advanced Graphics** - 高度な描画
   - `blt()`, `bltm()` (スプライト・タイルマップ)
   - `text()` (テキスト描画)

6. **Audio** - 音声
   - `play()`, `playm()`, `stop()`

## トラブルシューティング

### `cargo build` が失敗する

```bash
# キャッシュをクリア
cargo clean
cargo build
```

### pyxel_fork のコンパイルエラー

```bash
# pyxel_fork を最新版に更新
cd pyxel_fork
git pull origin main
cd ..
cargo build
```

### Discord 通知が送信されない

```bash
# webhook URL を確認
echo $DISCORD_WEBHOOK_URL

# テスト送信
./scripts/notify-discord.sh "Test message"
```

## リソース

- **Pyxel API リファレンス**: https://github.com/kitao/pyxel
- **pyxel-core (Rust 実装)**: `./pyxel_fork/crates/pyxel-core/`
- **開発プラン**: `ROADMAP.md`
- **TODO 管理**: SQL で `pyxel-rust-*` 始まりのタスク

## よくある質問

**Q: Nantaraquad と何が違うのか？**
A: 完全に独立したプロジェクト。Nantaraquad の NQuad は変更なし。将来、NQuad を組み込む可能性あり。

**Q: Cubeboy・Lineboy のポートはいつ？**
A: System・Graphics・Input API の実装後。ROADMAP.md を参照。

**Q: WASM ビルドはどうする？**
A: System API 実装後に Emscripten セットアップ。スクリプトで自動化予定。

**Q: どれくらいで完成する？**
A: 基本的な API なら 1-2 週間。フル実装は 3-4 週間程度。
