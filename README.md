# pyxel-rust / アリサクエスト

> ⚠️ **このリポジトリは現在開発中です。**  
> API・仕様は予告なく変更される可能性があります。

---

**アリサクエスト**は、GPS位置情報と連動するブラウザ動作のターン制RPGデモです。  
現実の地図上を歩くと敵とエンカウントし、WebAssembly製のバトルエンジンで戦闘が始まります。

🎮 **プレイはこちら**: https://oosawak.github.io/pyxel-rust/examples/arisa-quest/

---

## 特徴

- 🗺️ **GPS連動マップ** — OpenStreetMap (Leaflet.js) で現在地を表示
- ⚔️ **WASMバトルエンジン** — Rust製バトルロジックをWebAssemblyで実行
- 📡 **サイバー空間演出** — GPS取得前は戦国武将の辞世の句によるマトリックス雨
- 📍 **距離感応エンカウント** — 敵との実距離に応じて遠近感のある表示
- 👆 **タッチ対応** — スマートフォンで背景スワイプ、タップ操作に対応
- 🏯 **山形城フォールバック** — GPS取得不可時は山形城（霞城公園）を起点に

---

## 技術構成

```
┌─────────────────────────────────────────────┐
│              ブラウザ (HTML/JS)              │
│                                             │
│  ┌──────────────┐    ┌───────────────────┐  │
│  │  Leaflet.js  │    │  bg-canvas (JS)   │  │
│  │  OpenStreet  │    │  スプライト描画    │  │
│  │  Map表示     │    │  背景・キャラ・UI  │  │
│  └──────────────┘    └───────────────────┘  │
│         ↑ GPS情報              ↑             │
│  ┌──────────────┐    ┌───────────────────┐  │
│  │ Geolocation  │    │  WASM (Rust)      │  │
│  │    API       │    │  バトルロジック    │  │
│  │  (ブラウザ)  │    │  HP/MP/状態遷移   │  │
│  └──────────────┘    └───────────────────┘  │
└─────────────────────────────────────────────┘
```

### WASM (Rust) が担当する領域

| 機能 | 詳細 |
|------|------|
| バトルロジック | HP/MP管理、コマンド処理、状態遷移 |
| バトル状態エクスポート | `get_game_state()` `get_player_hp()` など |
| 描画エンジン基盤 | pyxel-rust カスタムエンジン（Canvas API 経由） |

### JavaScript が担当する領域

| 機能 | 詳細 |
|------|------|
| GPS・位置情報 | Geolocation API（JS必須） |
| 地図表示 | Leaflet.js + OpenStreetMap |
| スプライト描画 | `bg-canvas` でキャラ・背景を合成 |
| UI・アニメーション | CSS アニメーション、Canvas 2D |
| タッチ操作 | Touch Events API |
| サイバー演出 | マトリックス雨、GPSロックオン演出 |

> **設計原則**: WASMはゲームのコア（演算・ルール）、JSは現実世界とのブリッジ（GPS・地図・UI）。  
> GPS処理をWASMに持たせることはブラウザAPI制約上できないため、この分担は変えないこと。

---

## リポジトリ構成

```
pyxel-rust/
├── src/                          ← pyxel-rust エンジン (Rust)
│   ├── lib.rs
│   ├── api/                      (graphics, input, audio, system...)
│   └── backend/
│       └── wasm_backend/mod.rs   ← Canvas API 統合
├── docs/
│   └── examples/
│       ├── arisa-quest/          ← 🎯 メインゲーム
│       │   ├── index.html        ← すべての実装（CSS+HTML+JS）
│       │   └── pkg/              ← WASM ビルド成果物
│       ├── cubeboy/              ← プラットフォーマーデモ
│       └── lineboy/              ← ラインアクションデモ
├── ARISA_QUEST_HANDOVER.md       ← 詳細な引き継ぎ資料
└── ROADMAP.md
```

---

## ビルド方法

### WASM ビルド（GitHub Pages 向け）

```bash
# wasm-pack が必要
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# ビルド（arisa_quest パッケージ）
cd docs/examples/arisa-quest
wasm-pack build ../../.. --target web --out-dir docs/examples/arisa-quest/pkg -- -p arisa_quest
```

### ローカル確認

```bash
# 簡易 HTTP サーバー（Python）
python3 -m http.server 8000 --directory docs
# → http://localhost:8000/examples/arisa-quest/
```

### Rust エンジン単体ビルド（デスクトップ）

```bash
cargo build
cargo run --example cubeboy
```

---

## デプロイ

GitHub Actions で `main` ブランチへの push 時に自動デプロイ。  
`docs/` フォルダを GitHub Pages のルートとして公開。

手動デプロイ:
```bash
git add docs/examples/arisa-quest/pkg/
git commit -m "Update WASM build"
git push origin main
```

---

## ドキュメント

- [引き継ぎ資料 (詳細)](./ARISA_QUEST_HANDOVER.md) — エンジニア向け設計・実装詳細
- [ロードマップ](./ROADMAP.md)

---

## 参考

- [Pyxel 公式](https://github.com/kitao/pyxel)
- [Leaflet.js](https://leafletjs.com/)
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)

## ライセンス

MIT License
