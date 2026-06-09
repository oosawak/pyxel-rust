# アリサクエスト

> pyxel-rust エンジンを使ったサンプルデモです。

**アリサクエスト**は、GPS位置情報と連動するブラウザ動作のターン制RPGデモです。  
現実の地図上を歩くと敵とエンカウントし、WebAssembly製のバトルエンジンで戦闘が始まります。

🎮 **プレイはこちら**: https://oosawak.github.io/pyxel-rust/examples/arisa-quest/

---

## 特徴

- 🗺️ **GPS連動マップ** — OpenStreetMap (Leaflet.js) で現在地を表示
- ⚔️ **WASMバトルエンジン** — Rust製バトルロジックをWebAssemblyで実行
- 📡 **サイバー空間演出** — GPS取得前はマトリックス雨演出
- 📍 **距離感応エンカウント** — 敵との実距離に応じて遠近感のある表示
- 👆 **タッチ対応** — スマートフォンでのプレイに対応
- 🏯 **山形城フォールバック** — GPS取得不可時は山形城（霞城公園）を起点
- 🗺️ **追加マーカー** — 観光案内 (WASM) と船予約の入口を地図上に追加

---

## 技術構成

```
ブラウザ (index.html)
├── Leaflet.js + OpenStreetMap  ← GPS・地図表示
├── Canvas 2D (bg-canvas)       ← スプライト描画・バトルUI
└── WASM (Rust / pyxel-rust)    ← バトルロジック・HP/MP管理
```

### WASM (Rust) が担当する領域

| 機能 | 詳細 |
|------|------|
| バトルロジック | HP/MP管理、コマンド処理、状態遷移 |
| 状態エクスポート | `get_game_state()` `get_player_hp()` など |

### JavaScript が担当する領域

| 機能 | 詳細 |
|------|------|
| GPS・位置情報 | Geolocation API |
| 地図表示 | Leaflet.js + OpenStreetMap |
| スプライト描画 | Canvas 2D でキャラ・背景を合成 |
| UI・アニメーション | CSS アニメーション |
| サイバー演出 | マトリックス雨、GPSロックオン演出 |

> **設計原則**: WASMはゲームコア（演算・ルール）、JSは現実世界とのブリッジ（GPS・地図・UI）。

---

## ファイル構成

```
arisa-quest/
├── index.html   ← すべての実装（CSS + HTML + JS）が1ファイルに集約
└── pkg/         ← WASM ビルド成果物（wasm-pack 生成）
```

---

## ビルド方法

```bash
# リポジトリルートで実行
wasm-pack build . --target web --out-dir docs/examples/arisa-quest/pkg -- -p arisa_quest

# 観光案内 WASM を更新する場合
wasm-pack build tour_guide_wasm --target web --out-dir docs/examples/arisa-quest/tour-guide/wasm
```

## ローカル確認

```bash
python3 -m http.server 8000 --directory docs
# → http://localhost:8000/examples/arisa-quest/
```
