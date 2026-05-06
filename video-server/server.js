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
 */
const express  = require('express');
const path     = require('path');
const fs       = require('fs');
const crypto   = require('crypto');
const { spawnSync } = require('child_process');
const os       = require('os');
const { RTCPeerConnection } = require('werift');

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

    // FFmpegでセグメント分割
    const tmpChunkDir = fs.mkdtempSync(path.join(os.tmpdir(), 'seg-'));
    const ff = spawnSync('ffmpeg', [
      '-y', '-i', tmpFile, '-c', 'copy',
      '-f', 'segment', '-segment_time', '2', '-reset_timestamps', '1',
      path.join(tmpChunkDir, 'chunk_%03d.mp4')
    ]);
    if (ff.status !== 0) throw new Error('FFmpeg failed');

    // 暗号化
    const key = crypto.randomBytes(32);
    const iv  = crypto.randomBytes(16);
    const chunkFiles = fs.readdirSync(tmpChunkDir).filter(f => f.endsWith('.mp4')).sort();
    const chunkMeta = chunkFiles.map((f, idx) => {
      const raw  = fs.readFileSync(path.join(tmpChunkDir, f));
      const ciph = crypto.createCipheriv('aes-256-ctr', key, iv);
      const enc  = Buffer.concat([ciph.update(raw), ciph.final()]);
      fs.writeFileSync(path.join(outDir, `chunk_${String(idx).padStart(3,'0')}.enc`), enc);
      return { file: `chunk_${String(idx).padStart(3,'0')}.enc`, size: enc.length, originalSize: raw.length };
    });

    // manifest保存
    const manifest = { title, lat, lng, mimeType: 'video/mp4', chunkDuration: 2,
      chunks: chunkMeta, key: key.toString('hex'), iv: iv.toString('hex'),
      createdAt: new Date().toISOString() };
    fs.writeFileSync(path.join(outDir, 'manifest.json'), JSON.stringify(manifest, null, 2));

    fs.rmSync(tmpChunkDir, { recursive: true });
    fs.unlinkSync(tmpFile);

    res.json({ id, title, chunkCount: chunkMeta.length });
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
});

fs.mkdirSync(VIDEOS_DIR, { recursive: true });

// ── WebRTC DataChannel ───────────────────────────────────────────────────────
// シグナリング: POST /rtc/signal  (SDPオファー受信 → アンサー返却)
// データ転送:   DataChannel "video" 経由でバイナリチャンクをプッシュ
//
// クライアント側フロー:
//   1. RTCPeerConnection作成 → DataChannel "video" 作成 → SDP offer生成
//   2. POST /rtc/signal にofferを送信 → answerを受け取る
//   3. DataChannelが開いたら "play:<videoId>" を送信
//   4. サーバーが暗号化チャンクを順次バイナリで送信
//   5. 最後に JSON文字列 {"done":true,...} を送信

app.post('/rtc/signal', express.json(), async (req, res) => {
  try {
    const { offer } = req.body;
    const pc = new RTCPeerConnection({
      iceServers: [],
      // UDPポートを固定範囲に（ファイアウォール開放用: UDP 3701-3710）
      icePortRange: [3701, 3710],
    });

    pc.ondatachannel = ({ channel }) => {
      channel.onmessage = async ({ data }) => {
        if (typeof data !== 'string' || !data.startsWith('play:')) return;
        const id = data.slice(5);
        const mPath = path.join(VIDEOS_DIR, id, 'manifest.json');
        if (!fs.existsSync(mPath)) {
          channel.send(JSON.stringify({ error: 'not found' }));
          return;
        }
        const manifest = JSON.parse(fs.readFileSync(mPath, 'utf8'));
        const FRAG_SIZE = 16384; // 16KB — DataChannel安全サイズ

        // メタ情報を先に送信
        channel.send(JSON.stringify({
          type: 'meta',
          chunkCount: manifest.chunks.length,
          mimeType: manifest.mimeType,
          key: manifest.key,
          iv: manifest.iv,
        }));

        // 暗号化チャンクをフラグメント分割して送信
        for (let i = 0; i < manifest.chunks.length; i++) {
          const chunkPath = path.join(VIDEOS_DIR, id,
            `chunk_${String(i).padStart(3, '0')}.enc`);
          const buf = fs.readFileSync(chunkPath);
          const fragCount = Math.ceil(buf.length / FRAG_SIZE);

          // チャンク開始通知
          channel.send(JSON.stringify({ type: 'chunk-start', index: i, size: buf.length, fragments: fragCount }));

          for (let f = 0; f < fragCount; f++) {
            const slice = buf.slice(f * FRAG_SIZE, (f + 1) * FRAG_SIZE);
            channel.send(slice);
            // バッファ詰まり防止
            await new Promise(r => setTimeout(r, 5));
          }

          // チャンク終了通知
          channel.send(JSON.stringify({ type: 'chunk-end', index: i }));
        }
        channel.send(JSON.stringify({ type: 'done' }));
      };
    };

    await pc.setRemoteDescription(offer);
    const answer = await pc.createAnswer();
    await pc.setLocalDescription(answer);

    // ICE収集が完了するまで待つ（LAN内は即座に完了、最大1500ms）
    await new Promise(resolve => {
      if (pc.iceGatheringState === 'complete') return resolve();
      pc.onicegatheringstatechange = () => {
        if (pc.iceGatheringState === 'complete') resolve();
      };
      setTimeout(resolve, 1500);
    });

    res.json({ answer: pc.localDescription });
  } catch (e) {
    console.error('[rtc] signal error:', e);
    res.status(500).json({ error: e.message });
  }
});

app.listen(PORT, () => {
  console.log(`🎬 video-server running at http://localhost:${PORT}`);
  console.log(`   player:  http://localhost:${PORT}/`);
  console.log(`   admin:   http://localhost:${PORT}/admin.html`);
  console.log(`   API:     http://localhost:${PORT}/api/videos`);
  console.log(`   WebRTC:  POST http://localhost:${PORT}/rtc/signal`);
});
