# video-server — 暗号化動画配信サーバー

ローカル i9 マシンで動作する動画配信サーバーです。  
動画を AES-256-CTR で暗号化してセグメント分割し、WebSocket でブラウザへ配信します。  
ブラウザ側では WASM で復号しながら MSE（Media Source Extensions）で早期再生します。

---

## 起動方法

```bash
# PM2 で起動（推奨）
cd /home/oosawak/Workspace/pyxel-rust
pm2 start video-server/server.js --name video-server
pm2 save   # 設定を保存

# 状態確認
pm2 list
pm2 logs video-server

# 停止・再起動
pm2 stop video-server
pm2 restart video-server
```

起動後のアクセス先：

| 用途 | URL |
|------|-----|
| 動画プレイヤー | http://i9:3700/ |
| 管理画面（アップロード） | http://i9:3700/admin.html |
| アリサクエスト（同一オリジン配信） | http://i9:3700/arisa-quest/ |

---

## アーキテクチャ

```
[動画ファイル]
    ↓ POST /api/upload
[server.js]
    FFmpeg → fMP4変換
    → box分割 (chunk_000: ftyp+moov, chunk_001+: moof+mdat)
    → AES-256-CTR 暗号化 → .enc ファイル保存
    → manifest.json 生成（キー・IV・チャンク数・mimeType）

[ブラウザ]
    WebSocket (ws://i9:3700/ws)
    ← meta (manifest情報)
    ← chunk-start → binary → chunk-end (×チャンク数)
    ← done
    WASM で AES-256-CTR 復号 → MSE appendBuffer → 2チャンク目で再生開始
```

### fMP4 構造

| チャンク | 内容 | 役割 |
|---------|------|------|
| `chunk_000.enc` | ftyp + moov | 初期化セグメント（コーデック情報） |
| `chunk_001.enc` 〜 | moof + mdat | メディアセグメント（映像・音声データ） |

---

## REST API

### `GET /api/videos`
登録済み動画の一覧を返す。

```json
[
  {
    "id": "e483b749e9eba920",
    "title": "動画タイトル",
    "chunkCount": 12,
    "createdAt": "2025-01-01T00:00:00.000Z",
    "lat": 38.252,
    "lng": 140.861
  }
]
```

### `GET /api/video/:id/manifest`
指定 ID の manifest.json を返す（暗号化キー含む）。

```json
{
  "id": "e483b749e9eba920",
  "title": "動画タイトル",
  "mimeType": "video/mp4; codecs=\"avc1.640028,mp4a.40.2\"",
  "isFragmented": true,
  "chunkCount": 12,
  "key": "hex文字列(64文字)",
  "iv": "hex文字列(32文字)",
  "lat": 38.252,
  "lng": 140.861
}
```

### `GET /api/video/:id/chunk/:n`
暗号化チャンクバイナリを返す（`n` は 0 始まり）。

### `POST /api/upload`
動画をアップロードして暗号化・セグメント分割する。  
リクエストボディは動画ファイルのバイナリ（`Content-Type` は任意）。

クエリパラメータ：

| パラメータ | 説明 |
|-----------|------|
| `title` | 動画タイトル（省略可） |
| `lat` | 撮影地点の緯度（省略可） |
| `lng` | 撮影地点の経度（省略可） |

---

## WebSocket プロトコル

接続先：`ws://i9:3700/ws`

### 再生フロー

```
Client → Server:  "play:<id>"

Server → Client:  JSON  { type: "meta", manifest: {...} }
Server → Client:  JSON  { type: "chunk-start", index: 0, size: 12345 }
Server → Client:  Binary ArrayBuffer (チャンクデータ)
Server → Client:  JSON  { type: "chunk-end", index: 0 }
... (チャンク数分繰り返し)
Server → Client:  JSON  { type: "done" }
```

- バイナリ受信の判定：`data instanceof ArrayBuffer`
- チャンク 0（初期化セグメント）受信後に MSE `addSourceBuffer()` を実行
- **チャンク 2 以降で `video.play()` を呼び出し早期再生開始**（全受信待ち不要）

---

## 暗号化仕様

| 項目 | 内容 |
|------|------|
| アルゴリズム | AES-256-CTR |
| キー長 | 256 bit（32 bytes）|
| IV | 128 bit（16 bytes）|
| チャンク独立性 | 各チャンクは CTR カウンタ [0] から独立して暗号化 |
| キー保管 | manifest.json（サーバーローカル）|

> ⚠️ manifest.json にはキーが平文で含まれます。  
> `/api/video/:id/manifest` エンドポイントはローカル LAN 内からのみアクセスしてください。

---

## ファイル構成

```
video-server/
├── server.js          ← サーバー本体（Express + WebSocket）
└── package.json       ← 依存パッケージ（express, ws）

docs/examples/video-player/
├── index.html         ← WASMプレイヤー（WebSocket受信・MSE再生）
├── admin.html         ← 管理画面（アップロード・動画一覧）
├── video-index.json   ← アリサクエスト用マーカー情報（GitHub Pages配信）
├── slime-chan.png      ← ローディング用猫スライム画像
├── pkg/               ← WASM バイナリ（AES復号）
└── videos/
    └── <id>/
        ├── manifest.json   ← キー・チャンク情報
        ├── chunk_000.enc   ← 初期化セグメント（暗号化）
        ├── chunk_001.enc   ← メディアセグメント（暗号化）
        └── ...
```

---

## video-index.json の更新

アリサクエストのマップマーカーに動画を追加する場合は手動で更新します。

```bash
# 登録済み動画一覧を確認
curl http://i9:3700/api/videos | python3 -m json.tool

# video-index.json を手動更新
vim docs/examples/video-player/video-index.json
git add docs/examples/video-player/video-index.json
git commit -m "Add new video marker"
git push
```

`video-index.json` のフォーマット：

```json
[
  {
    "id": "e483b749e9eba920",
    "title": "動画タイトル",
    "lat": 38.252,
    "lng": 140.861
  }
]
```

---

## トラブルシューティング

### PM2 再起動後にプロセスがない

```bash
pm2 resurrect        # 保存済みプロセスリストを復元
# または
pm2 start video-server/server.js --name video-server
```

### WebSocket 接続できない

```bash
pm2 logs video-server   # エラーログ確認
pm2 restart video-server
```

### アップロードが失敗する

FFmpeg がインストールされていることを確認：

```bash
ffmpeg -version
```

### GitHub Pages（HTTPS）から ws:// に接続できない

`window.open('http://i9:3700/?id=xxx', '_blank')` で HTTP コンテキストの別タブを開くことで  
Mixed Content を回避しています（`ws://` は HTTP ページからは使用可能）。
