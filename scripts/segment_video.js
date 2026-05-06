#!/usr/bin/env node
/**
 * scripts/segment_video.js
 * 動画をFFmpegで2秒チャンクに分割→各チャンクをAES-256-CTR暗号化→manifest.json出力
 *
 * 使い方:
 *   node scripts/segment_video.js <input.mp4> <output_dir> [title]
 *
 * 出力 (<output_dir>/):
 *   chunk_000.enc, chunk_001.enc, ...  暗号化済みチャンク
 *   manifest.json                      再生に必要なメタ情報（チャンク数・キー・IV）
 */
const { execSync, spawnSync } = require('child_process');
const crypto = require('crypto');
const fs     = require('fs');
const path   = require('path');
const os     = require('os');

const inputFile = process.argv[2];
const outputDir = process.argv[3] || './output';
const title     = process.argv[4] || path.basename(inputFile, path.extname(inputFile));

if (!inputFile || !fs.existsSync(inputFile)) {
  console.error('使い方: node segment_video.js <input.mp4> <output_dir> [title]');
  process.exit(1);
}

// 一時ディレクトリに生チャンクを出力
const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'vidseg-'));
fs.mkdirSync(outputDir, { recursive: true });

console.log(`📂 入力: ${inputFile}`);
console.log(`📦 出力: ${outputDir}`);
console.log(`✂️  FFmpegでセグメント分割中（2秒チャンク）...`);

// FFmpegでMP4をセグメント分割（各2秒、再エンコードなし）
const ffmpegResult = spawnSync('ffmpeg', [
  '-y', '-i', inputFile,
  '-c', 'copy',
  '-f', 'segment',
  '-segment_time', '2',
  '-reset_timestamps', '1',
  path.join(tmpDir, 'chunk_%03d.mp4')
], { encoding: 'utf8' });

if (ffmpegResult.status !== 0) {
  console.error('FFmpegエラー:', ffmpegResult.stderr);
  process.exit(1);
}

// チャンクファイル一覧
const chunks = fs.readdirSync(tmpDir)
  .filter(f => f.endsWith('.mp4'))
  .sort();

if (chunks.length === 0) {
  console.error('チャンクが生成されませんでした');
  process.exit(1);
}

console.log(`   ${chunks.length}チャンクを生成`);
console.log(`🔑 AES-256-CTRで暗号化中...`);

// 全チャンク共通のキーとIVを生成
const key = crypto.randomBytes(32);
const iv  = crypto.randomBytes(16);

// 各チャンクを暗号化して出力
const chunkMeta = [];
chunks.forEach((chunkFile, idx) => {
  const raw       = fs.readFileSync(path.join(tmpDir, chunkFile));
  const cipher    = crypto.createCipheriv('aes-256-ctr', key, iv);
  const encrypted = Buffer.concat([cipher.update(raw), cipher.final()]);
  const outName   = `chunk_${String(idx).padStart(3, '0')}.enc`;
  fs.writeFileSync(path.join(outputDir, outName), encrypted);
  chunkMeta.push({ file: outName, size: encrypted.length, originalSize: raw.length });
  process.stdout.write(`   [${idx + 1}/${chunks.length}] ${outName} (${(encrypted.length / 1024).toFixed(1)}KB)\n`);
});

// manifest.json
const manifest = {
  title,
  mimeType: 'video/mp4',
  chunkDuration: 2,
  chunks: chunkMeta,
  key: key.toString('hex'),
  iv:  iv.toString('hex'),
  createdAt: new Date().toISOString()
};
fs.writeFileSync(path.join(outputDir, 'manifest.json'), JSON.stringify(manifest, null, 2));

// 一時ファイル削除
fs.rmSync(tmpDir, { recursive: true });

console.log(`\n✅ 完了`);
console.log(`   チャンク数: ${chunks.length}`);
console.log(`   manifest: ${path.join(outputDir, 'manifest.json')}`);
console.log(`   ⚠️  manifest.json（キー含む）は公開リポジトリにコミットしないこと`);
