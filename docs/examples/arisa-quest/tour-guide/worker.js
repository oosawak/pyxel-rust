import init, { Model } from './pkg/rullama.js';

let wasmReady = null;
let model = null;

async function ensureWasm() {
  if (!wasmReady) wasmReady = init('./pkg/rullama_bg.wasm');
  return wasmReady;
}

function postStatus(state, text) {
  self.postMessage({ type: 'status', state, text });
}

function decodeToken(token) {
  return String(token ?? '').replaceAll('▁', ' ');
}

async function loadModel(file, maxContext, textOnly) {
  await ensureWasm();
  if (model) {
    try { model.free?.(); } catch {}
    model = null;
  }
  const size = file.size;
  const readFn = async (offset, length) => {
    const slice = file.slice(Number(offset), Number(offset) + Number(length));
    return new Uint8Array(await slice.arrayBuffer());
  };
  postStatus('loading', 'モデルを読み込み中');
  model = textOnly
    ? await Model.loadFromOpfsTextOnly(readFn, size, maxContext || 512)
    : await Model.loadFromOpfs(readFn, size, maxContext || 512);
  postStatus('ready', '準備完了');
  return {
    vocabSize: model.vocabSize,
    hasVision: model.hasVision,
    hasAudio: model.hasAudio,
    maxContext: model.maxContext,
  };
}

async function generate({ systemPrompt, query, context, prompt, maxTokens }) {
  if (!model) throw new Error('model not loaded');
  const m = model;
  m.reset();
  try {
    m.setSampling({
      temperature: 0.2,
      top_k: 40,
      top_p: 0.9,
      repetition_penalty: 1.05,
      seed: 0,
    });
  } catch {}

  const messages = [
    { role: 'system', content: systemPrompt },
    { role: 'user', content: prompt ?? `### 観光コンテキスト\n${context}\n\n### 質問\n${query}` },
  ];
  const rendered = m.renderChat(messages, false);
  const ids = m.encode(rendered);
  let next = 0;
  for (const id of ids) {
    next = await m.step(id);
  }

  let answer = '';
  for (let i = 0; i < (maxTokens || 128); i += 1) {
    const piece = decodeToken(m.tokenStr(next));
    answer += piece;
    if (m.isEos(next)) break;
    next = await m.step(next);
  }

  return { answer: answer.trim(), prompt: rendered };
}

self.addEventListener('message', async (ev) => {
  const msg = ev.data || {};
  const { requestId, type } = msg;
  if (type === 'init') {
    try {
      await ensureWasm();
      postStatus('idle', '未読込');
      self.postMessage({ requestId, ok: true, result: true });
    } catch (e) {
      self.postMessage({ requestId, ok: false, error: e?.message || String(e) });
    }
    return;
  }
  try {
    if (type === 'load-model') {
      const result = await loadModel(msg.file, Number(msg.maxContext || 512), !!msg.textOnly);
      self.postMessage({ requestId, ok: true, result });
      return;
    }
    if (type === 'generate') {
      const result = await generate(msg);
      self.postMessage({ requestId, ok: true, result });
      return;
    }
    throw new Error(`unknown RPC type: ${type}`);
  } catch (e) {
    self.postMessage({ requestId, ok: false, error: e?.message || String(e) });
    postStatus('error', e?.message || String(e));
  }
});
