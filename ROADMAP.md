# pyxel-rust Development Roadmap

## プロジェクト目標

✅ Pyxel ゲームエンジンの完全な Rust 実装
✅ Emscripten WASM 対応で、ブラウザで動作
✅ Cubeboy・Lineboy を Python → Rust に移植
✅ Nantaraquad の NQuad とは独立（将来の連携は可能）

## フェーズ

### Phase 1: プロジェクト基盤 (1-2日)
- [x] pyxel-rust プロジェクト作成
- [x] Pyxel フォーク統合
- [x] Cargo.toml 設定
- [ ] Git 初期化 & リモート設定
- [ ] ドキュメント基盤
- [ ] TODO 管理体制構築

### Phase 2: コア API 実装 (3-5日)
- [ ] System API (init, run, quit, clip, camera)
- [ ] Graphics API (pset, pget, line, rect, circ, etc)
- [ ] Input API (btn, btnp, btnr, mouse)
- [ ] Math utilities (sin, cos, sqrt, rnd)
- [ ] Constants (KEY_*, COLOR_*)

### Phase 3: 高度な描画 (2-3日)
- [ ] Image/Tilemap support
- [ ] Text rendering
- [ ] blt (sprite blitting)
- [ ] bltm (tilemap rendering)

### Phase 4: オーディオ実装 (2-3日)
- [ ] Audio API (play, playm, stop)
- [ ] Sound class
- [ ] Music class

### Phase 5: Cubeboy 移植 (3-5日)
- [ ] Cubeboy Python コード解析
- [ ] Rust への翻訳
- [ ] テスト・デバッグ

### Phase 6: Lineboy 移植 (2-3日)
- [ ] Lineboy Python コード解析
- [ ] Rust への翻訳
- [ ] テスト・デバッグ

### Phase 7: WASM 統合 (2-3日)
- [ ] Emscripten セットアップ
- [ ] WASM ビルド検証
- [ ] ブラウザテスト

### Phase 8: ドキュメント & 最適化 (2-3日)
- [ ] API ドキュメント整備
- [ ] 使用例・チュートリアル
- [ ] パフォーマンス最適化

## 進捗追跡

進捗は `TODO.md` で管理します。

### ステータス
- `pending` - 未開始
- `in_progress` - 実施中
- `done` - 完了
- `blocked` - ブロック中

## リソース

- **Pyxel-core**: `./pyxel_fork/crates/pyxel-core/`
- **参考実装**: `/home/oosawak/Workspace/Nantaraquad/` (NQuad)
- **Discord 通知**: 進捗自動配信

## 依存プロジェクト

- ❌ **影響を受けない**: Nantaraquad (完全独立)
- ✅ **将来の統合**: NQuad を pyxel-rust に組み込む可能性あり

## 注意点

- NQuad は変更しない（参考のみ）
- Cubeboy・Lineboy の移植は新しいプロジェクト内で行う
- Pyxel フォークは定期的に上流と同期する
