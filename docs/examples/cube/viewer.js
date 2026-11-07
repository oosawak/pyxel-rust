import { examples, officialUrl } from './examples.js';
const select = document.getElementById('example');
const resolutionSelect = document.getElementById('resolution');
let resolution = Number(new URLSearchParams(location.search).get('resolution')) || 1;
if (![1,2,4].includes(resolution)) resolution = 1;
const host = document.getElementById('frame-host');
const status = document.getElementById('load-state');
const buttons = {
  compare: document.getElementById('show-compare'),
  rust: document.getElementById('show-rust'),
  official: document.getElementById('show-official'),
};
const requested = new URLSearchParams(location.search).get('version');
let mode = ['rust', 'official'].includes(requested) ? requested : 'compare';
let frames = new Map();
for (const [id, example] of examples.entries()) {
  select.add(new Option(`${String(id+1).padStart(2,'0')}  ${example.title}`, String(id)));
  const item = document.createElement('li');
  const button = document.createElement('button');
  button.textContent = `${example.title} — ${example.description}`;
  button.onclick = () => { select.value = String(id); load(); select.scrollIntoView(); };
  item.append(button); document.getElementById('catalog').append(item);
}
const initial = new URLSearchParams(location.search).get('demo');
select.value = String(Math.max(0, examples.findIndex(item => item.slug === initial)));
function updateStatus() {
  const states = [...frames.values()];
  status.textContent = states.map(state => state.text).join(' ');
  status.setAttribute('role', states.some(state => state.error) ? 'alert' : 'status');
}
function setState(state, text, { error = false, done = false } = {}) {
  state.text = text;
  state.error = error;
  state.output.textContent = text;
  state.output.setAttribute('role', error ? 'alert' : 'status');
  if (done) clearTimeout(state.timer);
  updateStatus();
}
function createFrame(kind, id, example) {
  const frame = document.createElement('iframe');
  const panel = document.createElement('section');
  panel.className = 'frame-panel';
  panel.setAttribute('aria-label', kind === 'rust' ? 'Rust移植版' : 'Pyxel公式版');
  const heading = document.createElement('h2');
  heading.className = 'frame-label';
  heading.textContent = kind === 'rust' ? `Rust移植版 · ${example.size[0]*resolution}×${example.size[1]*resolution}` : 'Pyxel公式版 / Python';
  const output = document.createElement('p');
  output.className = 'frame-status';
  frame.title = `${example.title} — ${heading.textContent}`;
  frame.allow = 'autoplay; fullscreen; gamepad';
  frame.allowFullscreen = true;
  frame.id = mode === 'compare' ? `${kind}-frame` : 'demo-frame';
  const state = { frame, output, text: '', error: false, timer: null };
  frames.set(kind, state);
  setState(state, `${kind === 'rust' ? 'Rust版' : '公式版'}を読み込み中…`);
  frame.onload = () => {
    if (frames.get(kind) !== state) return;
    if (kind === 'official') setState(state, '公式ページを読み込みました。画面をクリック／タップして開始してください。', { done: true });
  };
  state.timer = setTimeout(() => {
    if (frames.get(kind) !== state) return;
    setState(state, `${kind === 'rust' ? 'Rust版' : '公式版'}の読み込みに時間がかかっています。「最初から」で再試行できます。`, { error: true });
  }, 45000);
  frame.src = kind === 'rust' ? `./rust.html?id=${id}&resolution=${resolution}${mode === 'compare' ? '&compare=1' : ''}` : officialUrl(example);
  panel.append(heading, frame, output);
  return panel;
}
function updateResolution() {
  const example = examples[Number(select.value)];
  resolutionSelect.replaceChildren(...[1,2,4].map(scale => new Option(`${scale === 1 ? "標準" : `${scale}倍`} · ${example.size[0]*scale}×${example.size[1]*scale}`, String(scale))));
  resolutionSelect.value = String(resolution);
  resolutionSelect.disabled = mode === "official";
  history.replaceState(null, "", `?${new URLSearchParams({ demo: example.slug, version: mode, resolution })}`);
}
resolutionSelect.onchange = () => {
  resolution = Number(resolutionSelect.value);
  const old = frames.get("rust");
  if (old) {
    clearTimeout(old.timer);
    const panel = old.frame.parentElement;
    panel.replaceWith(createFrame("rust", Number(select.value), examples[Number(select.value)]));
  }
  updateResolution();
};
function load() {
  for (const state of frames.values()) clearTimeout(state.timer);
  frames = new Map();
  const id = Number(select.value);
  const example = examples[id];
  document.body.dataset.mode = mode;
  document.getElementById('description').textContent = example.description;
  document.getElementById('help').textContent = example.help;
  document.getElementById('viewer-title').textContent = example.title;
  document.getElementById('version').textContent = mode === 'compare' ? 'Rust ↔ Pyxel公式' : mode === 'rust' ? 'Rust / wasm-bindgen' : 'Pyxel公式 / Python';
  for (const [kind, button] of Object.entries(buttons)) button.setAttribute('aria-pressed', String(mode === kind));
  document.getElementById('official-link').href = officialUrl(example);
  document.getElementById('python-source').href = `https://github.com/kitao/pyxel/blob/v3.0.0/python/pyxel/examples/cube/${example.source}`;
  updateResolution();
  host.classList.toggle('compare', mode === 'compare');
  host.style.setProperty('--demo-aspect', `${example.size[0]} / ${example.size[1]}`);
  const kinds = mode === 'compare' ? ['rust', 'official'] : [mode];
  host.replaceChildren(...kinds.map(kind => createFrame(kind, id, example)));
  updateStatus();
}
window.addEventListener('message', event => {
  const state = frames.get('rust');
  if (event.origin !== location.origin || !state || event.source !== state.frame.contentWindow) return;
  if (event.data?.type === 'cube-resize' && mode === 'compare' && Number.isFinite(event.data.height)) { state.frame.style.height = `${Math.max(100, Math.min(2000,event.data.height))}px`; }
  if (event.data?.type === 'cube-ready') setState(state, 'Rust版を実行中。画面をクリック／タップして操作してください。', { done: true });
  if (event.data?.type === 'cube-error') setState(state, `Rust版を起動できませんでした：${event.data.message}`, { error: true, done: true });
});
select.onchange = load;
for (const [kind, button] of Object.entries(buttons)) button.onclick = () => { mode = kind; load(); };
document.getElementById('restart').onclick = load;
load();
