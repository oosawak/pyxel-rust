#!/usr/bin/env node
/**
 * encrypt_video.js
 * 動画ファイルをAES-256-CTRで暗号化してWASMプレイヤー用に出力する
 *
 * 使い方:
 *   node scripts/encrypt_video.js <input.mp4> <output_dir>
 *
 * 出力:
 *   <output_dir>/video.enc   - 暗号化済み動画
 *   <output_dir>/key.json    - キーとIV（本番ではSymbolチェーンに移行）
 */
const crypto = require('crypto');
const fs     = require('fs');
const path   = require('path');

const inputFile = process.argv[2];
const outputDir = process.argv[3] || '.';

if (!inputFile) {
  console.error('使い方: node encrypt_video.js <input.mp4> [output_dir]');
  process.exit(1);
}

// ランダムなキー(32byte)とIV(16byte)を生成
const key = crypto.randomBytes(32);
const iv  = crypto.randomBytes(16);

// 動画を読み込んで暗号化
const inputData  = fs.readFileSync(inputFile);
const cipher     = crypto.createCipheriv('aes-256-ctr', key, iv);
const encrypted  = Buffer.concat([cipher.update(inputData), cipher.final()]);

// 出力
fs.mkdirSync(outputDir, { recursive: true });

const encPath  = path.join(outputDir, 'video.enc');
const keyPath  = path.join(outputDir, 'key.json');

fs.writeFileSync(encPath, encrypted);
fs.writeFileSync(keyPath, JSON.stringify({
  key: key.toString('hex'),
  iv:  iv.toString('hex'),
  originalName: path.basename(inputFile),
  mimeType: 'video/mp4',
  createdAt: new Date().toISOString()
}, null, 2));

console.log(`✅ 暗号化完了`);
console.log(`   入力: ${inputFile} (${(inputData.length / 1024).toFixed(1)} KB)`);
console.log(`   出力: ${encPath} (${(encrypted.length / 1024).toFixed(1)} KB)`);
console.log(`   キー: ${keyPath}`);
console.log(`   ⚠️  key.json は公開リポジトリにコミットしないこと`);
