import { examples, officialUrl } from './examples.js';
const select = document.getElementById('example');
const host = document.getElementById('frame-host');
const status = document.getElementById('load-state');
const rust = document.getElementById('show-rust');
const official = document.getElementById('show-official');
let mode = new URLSearchParams(location.search).get('version') === 'official' ? 'official' : 'rust';
for (const [id, example] of examples.entries()) {
  const option = new Option(`${String(id+1).padStart(2,'0')}  ${example.title}`, String(id));
  select.add(option);
  const item = document.createElement('li');
  const button = document.createElement('button');
  button.textContent = `${example.title} — ${example.description}`;
  button.onclick = () => { select.value = String(id); load(); document.getElementById('example').scrollIntoView(); };
  item.append(button); document.getElementById('catalog').append(item);
}
const initial = new URLSearchParams(location.search).get('demo');
select.value = String(Math.max(0, examples.findIndex(item => item.slug === initial)));
let currentFrame;
let loaded = false;
let timeout;
function load() {
  const id = Number(select.value);
  const example = examples[id];
  document.getElementById('description').textContent = example.description;
  document.getElementById('help').textContent = example.help;
  document.getElementById('viewer-title').textContent = example.title;
  document.getElementById('version').textContent = mode === 'rust' ? 'Rust / wasm-bindgen' : 'Pyxel公式 / Python';
  rust.setAttribute('aria-pressed', String(mode === 'rust')); official.setAttribute('aria-pressed', String(mode === 'official'));
  document.getElementById('official-link').href = officialUrl(example);
  document.getElementById('python-source').href = `https://github.com/kitao/pyxel/blob/v3.0.0/python/pyxel/examples/cube/${example.source}`;
  const params = new URLSearchParams({ demo: example.slug, version: mode });
  history.replaceState(null,'',`?${params}`);
  loaded = false;
  clearTimeout(timeout);
  const frame = document.createElement('iframe');
  frame.title = `${example.title} — ${mode === 'rust' ? 'Rust移植版' : 'Pyxel公式版'}`;
  frame.allow = 'autoplay; fullscreen; gamepad';
  frame.allowFullscreen = true;
  frame.src = mode === 'rust' ? `./rust.html?id=${id}` : officialUrl(example);
  frame.id = 'demo-frame';
  status.textContent = `${mode === 'rust' ? 'Rust版' : '公式版'}を読み込み中…`;
  status.setAttribute('role', 'status');
  frame.onload = () => {
    if (currentFrame !== frame) return;
    if (mode === 'official') {
      loaded = true; clearTimeout(timeout);
      status.textContent = '公式ページを読み込みました。画面をクリック／タップして操作してください。実行状態は公式画面内で確認できます。';
    }
  };
  currentFrame = frame;
  host.replaceChildren(frame);
  timeout = setTimeout(() => {
    if (loaded || currentFrame !== frame) return;
    status.textContent = '読み込みに時間がかかっています。「最初から」で再試行するか、別タブで公式版を開いてください。';
    status.setAttribute('role', 'alert');
  }, 45000);
}
window.addEventListener('message', event => {
  if (event.origin !== location.origin || event.source !== currentFrame?.contentWindow || mode !== 'rust') return;
  if (event.data?.type === 'cube-ready') { loaded = true; clearTimeout(timeout); status.textContent = 'Rust版を実行中。画面をクリック／タップして操作してください。'; }
  if (event.data?.type === 'cube-error') { clearTimeout(timeout); status.textContent = `Rust版を起動できませんでした：${event.data.message}`; status.setAttribute('role','alert'); }
});
select.onchange = load;
rust.onclick = () => { mode='rust'; load(); };
official.onclick = () => { mode='official'; load(); };
document.getElementById('restart').onclick = load;
load();
