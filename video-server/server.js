/**
 * video-server/server.js
 * WASMプレイヤー用ローカル配信サーバー（PM2で起動）
 *
 * 起動: pm2 start video-server/server.js --name video-server
 * 停止: pm2 stop video-server
 * URL:  http://localhost:3700
 */
const express = require('express');
const path    = require('path');
const fs      = require('fs');

const app  = express();
const PORT = 3700;
const PLAYER_DIR = path.join(__dirname, '..', 'docs', 'examples', 'video-player');

// WASMのContent-Typeを正しく設定（ブラウザが要求）
app.use((req, res, next) => {
  if (req.path.endsWith('.wasm')) {
    res.setHeader('Content-Type', 'application/wasm');
  }
  next();
});

// CORSヘッダー（将来の外部サーバー移行時も安全に）
app.use((req, res, next) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
  res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
  next();
});

// 静的ファイル配信（video-player フォルダ全体）
app.use(express.static(PLAYER_DIR));

// /key エンドポイント: key.jsonを返す（本番ではSymbol認証に置き換え）
app.get('/key', (req, res) => {
  const keyPath = path.join(PLAYER_DIR, 'encrypted', 'key.json');
  if (!fs.existsSync(keyPath)) {
    return res.status(404).json({ error: 'key not found' });
  }
  res.json(JSON.parse(fs.readFileSync(keyPath, 'utf8')));
});

app.listen(PORT, () => {
  console.log(`🎬 video-server running at http://localhost:${PORT}`);
  console.log(`   player: http://localhost:${PORT}/index.html`);
  console.log(`   key API: http://localhost:${PORT}/key`);
});
