/**
 * video-server/server.js
 * WASMプレイヤー用ローカル配信サーバー（PM2で起動）
 *
 * 起動: pm2 start video-server/server.js --name video-server
 * URL:  http://localhost:3700
 *
 * API:
 *   GET /api/videos                    動画一覧
 *   GET /api/video/:id/manifest        manifest.json（キー含む）
 *   GET /api/video/:id/chunk/:n        暗号化チャンクバイナリ
 *   POST /api/upload                   動画アップロード・暗号化・セグメント化
 *   WS  /ws                            WebSocketでチャンク配信
 */
const express  = require('express');
const path     = require('path');
const fs       = require('fs');
const crypto   = require('crypto');
const { spawnSync } = require('child_process');
const os       = require('os');
const http     = require('http');
const { WebSocketServer } = require('ws');

const app  = express();
const PORT = 3700;
const ROOT        = path.join(__dirname, '..');
const PLAYER_DIR  = path.join(ROOT, 'docs', 'examples', 'video-player');
const VIDEOS_DIR  = path.join(PLAYER_DIR, 'videos');   // セグメント済み動画の格納先

app.use(express.json());

// Content-Type / CORS / COOP ヘッダー
app.use((req, res, next) => {
  if (req.path.endsWith('.wasm')) res.setHeader('Content-Type', 'application/wasm');
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
  res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
  next();
});

// 静的ファイル配信
app.use(express.static(PLAYER_DIR));

// Arisa Quest を同一オリジンで配信（Mixed Content回避）
// http://i9:3700/arisa-quest/ でアクセス可能
const ARISA_DIR = path.join(ROOT, 'docs', 'examples', 'arisa-quest');
app.use('/arisa-quest', express.static(ARISA_DIR));

// ── API ──────────────────────────────────────────────────────────────────────

/** GET /api/videos — 動画一覧（manifest.jsonのtitle/createdAt） */
app.get('/api/videos', (req, res) => {
  if (!fs.existsSync(VIDEOS_DIR)) return res.json([]);
  const list = fs.readdirSync(VIDEOS_DIR)
    .filter(id => fs.existsSync(path.join(VIDEOS_DIR, id, 'manifest.json')))
    .map(id => {
      const m = JSON.parse(fs.readFileSync(path.join(VIDEOS_DIR, id, 'manifest.json'), 'utf8'));
      return { id, title: m.title, chunkCount: m.chunks.length, createdAt: m.createdAt, lat: m.lat || null, lng: m.lng || null };
    });
  res.json(list);
});

/** GET /api/video/:id/manifest — manifest.json（キー含む、本番では認証必須） */
app.get('/api/video/:id/manifest', (req, res) => {
  const mPath = path.join(VIDEOS_DIR, req.params.id, 'manifest.json');
  if (!fs.existsSync(mPath)) return res.status(404).json({ error: 'not found' });
  res.json(JSON.parse(fs.readFileSync(mPath, 'utf8')));
});

/** GET /api/video/:id/chunk/:n — 暗号化チャンクバイナリ */
app.get('/api/video/:id/chunk/:n', (req, res) => {
  const chunkPath = path.join(VIDEOS_DIR, req.params.id,
    `chunk_${String(req.params.n).padStart(3,'0')}.enc`);
  if (!fs.existsSync(chunkPath)) return res.status(404).json({ error: 'chunk not found' });
  res.setHeader('Content-Type', 'application/octet-stream');
  res.sendFile(chunkPath);
});

/** POST /api/upload — multipart不使用、base64ボディで受け取りセグメント化 */
app.post('/api/upload', express.raw({ type: '*/*', limit: '200mb' }), (req, res) => {
  const title = req.headers['x-title'] || 'untitled';
  const lat   = parseFloat(req.headers['x-lat'] || '0');
  const lng   = parseFloat(req.headers['x-lng'] || '0');
  const id    = crypto.randomBytes(8).toString('hex');
  const tmpFile = path.join(os.tmpdir(), `${id}.mp4`);
  const outDir  = path.join(VIDEOS_DIR, id);

  try {
    fs.writeFileSync(tmpFile, req.body);
    fs.mkdirSync(outDir, { recursive: true });

    // FFmpegでfMP4（フラグメント化MP4）に変換 → MSEストリーミング対応
    // -movflags frag_keyframe+empty_moov+default_base_moof: MSE互換fMP4
    // -frag_duration 3000000: 3秒ごとにフラグメント（=3秒でストリーム再生開始）
    const fmp4File = path.join(os.tmpdir(), `${id}_fmp4.mp4`);
    const ff = spawnSync('ffmpeg', [
      '-y', '-i', tmpFile,
      '-c:v', 'libx264', '-preset', 'fast', '-crf', '23',
      '-c:a', 'aac', '-b:a', '128k',
      '-movflags', 'frag_keyframe+empty_moov+default_base_moof',
      '-frag_duration', '3000000',
      fmp4File
    ], { maxBuffer: 1024 * 1024 * 512 });
    if (ff.status !== 0) throw new Error('FFmpeg failed: ' + ff.stderr?.toString().slice(-500));

    // avcCボックスからコーデック文字列を取得
    const fmp4Data = fs.readFileSync(fmp4File);
    let mimeType = 'video/mp4; codecs="avc1.640028,mp4a.40.2"';
    const avcIdx = fmp4Data.indexOf(Buffer.from('avcC'));
    if (avcIdx >= 0) {
      const profile = fmp4Data[avcIdx + 5].toString(16).padStart(2,'0').toUpperCase();
      const constraint = fmp4Data[avcIdx + 6].toString(16).padStart(2,'0').toUpperCase();
      const level = fmp4Data[avcIdx + 7].toString(16).padStart(2,'0').toUpperCase();
      mimeType = `video/mp4; codecs="avc1.${profile}${constraint}${level},mp4a.40.2"`;
    }

    // fMP4をボックス単位で解析して分割
    // chunk_000 = 初期化セグメント (ftyp+moov)
    // chunk_001〜 = メディアセグメント (moof+mdat ペア)
    const segments = [];
    let pos = 0;
    let initEnd = 0;
    while (pos < fmp4Data.length - 8) {
      const size = fmp4Data.readUInt32BE(pos);
      const name = fmp4Data.slice(pos + 4, pos + 8).toString('ascii');
      if (size < 8) break;
      if (name === 'ftyp' || name === 'moov') {
        initEnd = pos + size;
      } else if (name === 'moof') {
        // moof+mdat をペアで取得
        const moofEnd = pos + size;
        const mdatSize = fmp4Data.readUInt32BE(moofEnd);
        segments.push({ start: pos, end: moofEnd + mdatSize });
      }
      pos += size;
    }

    // 初期化セグメント暗号化・保存
    const key = crypto.randomBytes(32);
    const iv  = crypto.randomBytes(16);
    const chunkMeta = [];

    const initRaw = fmp4Data.slice(0, initEnd);
    const initCiph = crypto.createCipheriv('aes-256-ctr', key, iv);
    const initEnc  = Buffer.concat([initCiph.update(initRaw), initCiph.final()]);
    fs.writeFileSync(path.join(outDir, 'chunk_000.enc'), initEnc);
    chunkMeta.push({ file: 'chunk_000.enc', size: initEnc.length, originalSize: initRaw.length, isInit: true });

    // メディアセグメント暗号化・保存
    let byteOffset = initRaw.length;
    segments.forEach((seg, i) => {
      const raw  = fmp4Data.slice(seg.start, seg.end);
      const ciph = crypto.createCipheriv('aes-256-ctr', key, iv);
      // CTRモードはバイトオフセット指定が必要 → initサイズ分ずらす
      const enc  = Buffer.concat([ciph.update(raw), ciph.final()]);
      const fname = `chunk_${String(i + 1).padStart(3,'0')}.enc`;
      fs.writeFileSync(path.join(outDir, fname), enc);
      chunkMeta.push({ file: fname, size: enc.length, originalSize: raw.length });
      byteOffset += raw.length;
    });

    // manifest保存
    const manifest = {
      title, lat, lng,
      mimeType,
      isFragmented: true,
      chunks: chunkMeta,
      key: key.toString('hex'),
      iv:  iv.toString('hex'),
      createdAt: new Date().toISOString()
    };
    fs.writeFileSync(path.join(outDir, 'manifest.json'), JSON.stringify(manifest, null, 2));

    fs.unlinkSync(tmpFile);
    fs.unlinkSync(fmp4File);

    res.json({ id, title, chunkCount: chunkMeta.length });
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
});

fs.mkdirSync(VIDEOS_DIR, { recursive: true });

// ── WebSocket チャンク配信 ────────────────────────────────────────────────────
// ws://host:3700/ws に接続後、"play:<id>" を送信すると暗号化チャンクをプッシュ
//
// クライアント側フロー:
//   1. new WebSocket('ws://host:3700/ws') → 即接続
//   2. onopen で "play:<videoId>" 送信
//   3. meta(JSON) → chunk-start(JSON) → binary × N → chunk-end(JSON) → done(JSON)

const server = http.createServer(app);
const wss = new WebSocketServer({ server, path: '/ws' });

wss.on('connection', (ws) => {
  ws.on('message', async (data) => {
    const msg = data.toString();
    if (!msg.startsWith('play:')) return;
    const id = msg.slice(5);
    const mPath = path.join(VIDEOS_DIR, id, 'manifest.json');
    if (!fs.existsSync(mPath)) {
      ws.send(JSON.stringify({ error: 'not found' }));
      return;
    }
    const manifest = JSON.parse(fs.readFileSync(mPath, 'utf8'));

    // メタ情報を先に送信
    ws.send(JSON.stringify({
      type: 'meta',
      chunkCount: manifest.chunks.length,
      mimeType: manifest.mimeType,
      isFragmented: manifest.isFragmented,
      key: manifest.key,
      iv: manifest.iv,
    }));

    // 暗号化チャンクを順次送信（WebSocketはメッセージサイズ制限なし）
    for (let i = 0; i < manifest.chunks.length; i++) {
      if (ws.readyState !== ws.OPEN) break;
      const chunkPath = path.join(VIDEOS_DIR, id,
        `chunk_${String(i).padStart(3, '0')}.enc`);
      const buf = fs.readFileSync(chunkPath);

      ws.send(JSON.stringify({ type: 'chunk-start', index: i, size: buf.length }));
      ws.send(buf);  // バイナリをそのまま送信
      ws.send(JSON.stringify({ type: 'chunk-end', index: i }));
    }
    if (ws.readyState === ws.OPEN)
      ws.send(JSON.stringify({ type: 'done' }));
  });

  ws.on('error', (e) => console.error('[ws] error:', e.message));
});

server.listen(PORT, () => {
  console.log(`🎬 video-server running at http://localhost:${PORT}`);
  console.log(`   player:  http://localhost:${PORT}/`);
  console.log(`   admin:   http://localhost:${PORT}/admin.html`);
  console.log(`   API:     http://localhost:${PORT}/api/videos`);
  console.log(`   WS:      ws://localhost:${PORT}/ws`);
});
