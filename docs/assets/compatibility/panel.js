// Implementation audit and live browser smoke checks are intentionally separate.
const panel = document.getElementById('compatibility');
const labels = { supported: '実装あり', partial: '一部対応', missing: '未対応', unreviewed: '未確認' };
const backends = { wasm: 'ブラウザ / WASM', sdl2: 'デスクトップ / SDL2', wgpu: 'デスクトップ / wgpu' };
const repo = 'https://github.com/oosawak/pyxel-rust/blob/';
const pageSize = 30;
let data;
let pageIndex = 0;

function element(tag, text, className) {
  const node = document.createElement(tag);
  if (text !== undefined) node.textContent = text;
  if (className) node.className = className;
  return node;
}
function link(text, href) {
  const node = element('a', text);
  node.href = href;
  node.target = '_blank';
  node.rel = 'noopener';
  return node;
}
function select(label, id, options) {
  const field = element('label', label);
  const control = element('select');
  control.id = id;
  for (const [value, text] of Object.entries(options)) {
    const option = element('option', text);
    option.value = value;
    control.append(option);
  }
  field.append(control);
  return [field, control];
}
function buildPanel() {
  const body = document.getElementById('compatibility-body');
  body.replaceChildren();
  const verdict = element('p', '全機能対応ではありません', 'compat-verdict');
  verdict.id = 'compat-verdict';
  body.append(verdict, element('p', '「実装あり」はRustから使える相当機能があることを示します。公式Python APIとの完全互換や、全条件でのテスト成功を意味しません。空処理・固定値を返すスタブは「未対応」にしています。'));
  body.append(element('p', `基準：Pyxel ${data.version}。${data.scope}`));
  const provenance = element('p', undefined, 'compat-meta');
  provenance.append('実装の確認基準：', link(data.engineRevision.slice(0, 8), repo + data.engineRevision), '。確認基準に固定したスナップショットです。現在のブラウザ確認とは別に表示します。');
  body.append(provenance);
  const exercise = element('p');
  exercise.append(link('入力・基本描画・数学の動作確認を開く ↗', new URL('../api-coverage/', location.href).href));
  exercise.append(' · ', link('Nodeアニメーションを試す ↗', new URL('../node-motion/', location.href).href));
  exercise.append(' · ', link('画面・録画・Nodeを試す ↗', new URL('../screen-tools/', location.href).href));
  exercise.append(' · ', link('画面・文字・ファイルを試す ↗', new URL('../browser-system/', location.href).href));
  exercise.append(' · ', link('画像・タイルマップ・音声を試す ↗', new URL('../resources-audio/', location.href).href));
  body.append(exercise);
  const filters = element('div', undefined, 'compat-filters');
  const [backendField, backend] = select('バックエンド', 'compat-backend', backends);
  const categories = Object.fromEntries([...new Set(data.entries.map(row => row.category))].map(value => [value, value]));
  const [categoryField, category] = select('分類', 'compat-category', { all: 'すべての分類', ...categories });
  const [statusField, status] = select('実装状況', 'compat-status', { all: 'すべての状況', ...labels });
  const [kindField, kind] = select('表示対象', 'compat-kind', { all: 'API・定数・Rust拡張', api: 'APIのみ（定数・拡張を除く）', constants: '定数のみ', extension: 'Rust拡張のみ' });
  const searchField = element('label', 'API名・制限を検索', 'compat-search');
  const search = element('input');
  search.type = 'search';
  search.id = 'compat-search';
  search.placeholder = '例：audio、playm、Node.update';
  searchField.append(search);
  filters.append(backendField, categoryField, statusField, kindField, searchField);
  body.append(filters);
  const stats = element('div', undefined, 'compat-stats');
  stats.id = 'compat-stats';
  const count = element('p');
  count.id = 'compat-count';
  count.setAttribute('role', 'status');
  const rows = element('ul', undefined, 'compat-rows');
  rows.id = 'compat-rows';
  rows.setAttribute('aria-label', '機能ごとの対応状況');
  const paging = element('div', undefined, 'compat-paging');
  const prev = element('button', '前の30件');
  prev.id = 'compat-prev';
  const pageLabel = element('p');
  pageLabel.id = 'compat-page';
  const next = element('button', '次の30件');
  next.id = 'compat-next';
  paging.append(prev, pageLabel, next);
  body.append(stats, count, rows, paging);
  const paint = () => {
    // Counters cover the selected backend and kind, before search/status filters.
    const isConstant = row => /^[A-Z][A-Z0-9_.]*$/.test(row.name.split('.').at(-1));
    const scoped = data.entries.filter(row => kind.value === 'all' || (kind.value === 'extension' ? row.group === 'Rust拡張' : row.group !== 'Rust拡張' && (kind.value === 'constants' ? isConstant(row) : !isConstant(row))));
    stats.replaceChildren();
    for (const [value, text] of Object.entries(labels)) {
      const stat = element('div', undefined, 'compat-stat');
      stat.dataset.support = value;
      stat.append(element('strong', String(scoped.filter(row => row.backends[backend.value].status === value).length)), text);
      stats.append(stat);
    }
    const counts = scoped.map(row => row.backends[backend.value].status);
    verdict.textContent = counts.some(value => value !== 'supported') ? '全機能対応ではありません' : '選択した範囲には実装があります（動作検証とは別）';
    const query = search.value.trim().toLowerCase();
    const filtered = scoped.filter(row => {
      const assessment = row.backends[backend.value];
      return (category.value === 'all' || category.value === row.category)
        && (status.value === 'all' || status.value === assessment.status)
        && `${row.name} ${row.signature} ${row.category} ${row.group} ${assessment.note}`.toLowerCase().includes(query);
    });
    const pages = Math.max(1, Math.ceil(filtered.length / pageSize));
    pageIndex = Math.min(pageIndex, pages - 1);
    count.textContent = `${backends[backend.value]}：${filtered.length} / ${scoped.length} 項目（全体 ${data.entries.length} 項目）。集計はバックエンドと表示対象に連動します。`;
    rows.replaceChildren();
    if (!filtered.length) rows.append(element('li', '条件に一致する機能はありません。', 'compat-row'));
    for (const row of filtered.slice(pageIndex * pageSize, (pageIndex + 1) * pageSize)) {
      const assessment = row.backends[backend.value];
      const item = element('li', undefined, 'compat-row');
      item.dataset.api = row.name;
      const head = element('div', undefined, 'compat-row-head');
      const badge = element('span', labels[assessment.status], 'compat-badge');
      badge.dataset.support = assessment.status;
      head.append(element('code', row.name), badge);
      const links = element('div', undefined, 'compat-links');
      links.append(link('実装の根拠', assessment.source));
      if (row.reference) links.append(link('公式API', row.reference));
      item.append(head, element('p', `${row.group} / ${row.category}`), element('p', assessment.note), links);
      const test = element('p', '個別動作：未検証。ソース確認による表示です。');
      if (assessment.verification) {
        test.textContent = '関連テストあり（一部の条件）。';
        const path = { glb:'tests/glb_browser.rs', animation:'tests/animation_browser.rs', math:'tests/math_compatibility.rs', drawing:'scripts/playtest-api-coverage.cjs', 'browser-input':'scripts/playtest-api-coverage.cjs', resources:'tests/browser_resources.rs', 'resources-audio':'scripts/playtest-resources-audio.cjs', 'browser-system':'scripts/playtest-browser-system.cjs', 'screen-tools':'scripts/playtest-screen-tools.cjs', 'scene-lifecycle':'tests/scene_lifecycle.rs', 'node-motion':'tests/node_motion.rs', 'matrix-views':'tests/matrix_views.rs' }[assessment.verification];
        test.append(' ', link('検証内容を見る', repo + data.engineRevision + '/' + path));
      }
      item.append(test);
      rows.append(item);
    }
    prev.disabled = pageIndex === 0;
    next.disabled = pageIndex + 1 >= pages;
    pageLabel.textContent = `${pageIndex + 1} / ${pages} ページ`;
  };
  for (const control of [backend, category, status, kind, search]) {
    control.addEventListener(control === search ? 'input' : 'change', () => { pageIndex = 0; paint(); });
  }
  prev.onclick = () => { pageIndex--; paint(); };
  next.onclick = () => { pageIndex++; paint(); };
  paint();
  addLiveChecks(body);
}

function addLiveChecks(body) {
  const section = element('section', undefined, 'compat-checks');
  const heading = element('h3', '現在のブラウザを確認');
  heading.style.fontSize = '16px';
  const button = element('button', 'ブラウザ確認を実行');
  button.id = 'compat-run';
  const result = element('p', 'まだ実行していません。WASM起動・Canvas描画・デモの状態を確認します。');
  result.id = 'compat-live-result';
  result.setAttribute('role', 'status');
  const list = element('ul');
  list.id = 'compat-live-checks';
  section.append(heading, element('p', 'この確認は表示中のデモだけが対象です。音声・全API・SDL2・wgpuの動作は判定しません。対応表の実装状況は変更しません。'), button, result, list);
  body.append(section);
  button.onclick = async () => {
    button.disabled = true;
    list.replaceChildren();
    result.textContent = 'ブラウザを確認中…';
    let passed = 0;
    let total = 0;
    const check = async (name, run) => {
      total++;
      let detail = '';
      try { await run(); passed++; detail = '確認できました'; }
      catch (error) { detail = `確認できません：${error.message || error}`; }
      list.append(element('li', `${name} — ${detail}`));
    };
    try {
      let demo;
      await check('WASM起動', async () => {
        demo = await import(document.body.dataset.wasmModule || '../../examples/retro3d/pkg/pyxel_rust.js');
        // Do not initialize a second WASM instance or restart the active game loop.
        if (!document.getElementById('status').textContent.match(/^(実行中|一時停止中)$/)) throw new Error('デモの起動が完了していません');
        if (!Number.isFinite(demo.retro3d_angle())) throw new Error('状態を取得できません');
      });
      await check('Canvasの描画', () => {
        const canvas = document.getElementById('pyxel_canvas');
        const ctx = canvas.getContext('2d');
        const { data: pixels } = ctx.getImageData(0, 0, canvas.width, canvas.height);
        let different = false;
        for (let i = 4; i < pixels.length; i += 4) {
          if (pixels[i] !== pixels[0] || pixels[i + 1] !== pixels[1] || pixels[i + 2] !== pixels[2]) { different = true; break; }
        }
        if (!different) throw new Error('単色のため描画結果を判定できません');
      });
      await check('3Dデモの状態取得', () => {
        if (!demo || !Number.isFinite(demo.retro3d_distance()) || typeof demo.retro3d_paused() !== 'boolean') throw new Error('状態を取得できません');
      });
      await check('GLB・アニメーションのAPI接続', () => {
        for (const key of ['load_retro3d_glb', 'clear_retro3d_model', 'select_retro3d_animation', 'seek_retro3d_animation']) {
          if (!demo || typeof demo[key] !== 'function') throw new Error(`${key}がありません`);
        }
        if (!Array.isArray(JSON.parse(demo.retro3d_animation_names()))) throw new Error('クリップ情報が不正です');
      });
      result.textContent = `${passed} / ${total} 件を確認しました。${passed === total ? 'このデモの起動・描画・状態取得を確認できました。全機能対応の判定ではありません。' : '確認できない項目があります。上の結果を確認してください。'}`;
    } finally { button.disabled = false; }
  };
}

async function load() {
  try {
    const response = await fetch(new URL('./data.json', import.meta.url), { cache:'no-cache' });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    data = await response.json();
    if (data.schema !== 1 || !Array.isArray(data.entries) || !data.entries.length
      || data.entries.some(row => !row.name || !row.id || !row.category
        || Object.keys(backends).some(backend => !labels[row.backends?.[backend]?.status]
          || typeof row.backends[backend].note !== 'string'
          || !row.backends[backend].source?.startsWith('https://github.com/')))) {
      throw new Error('対応表データが不正です');
    }
    buildPanel();
  } catch (error) {
    const status = element('p', `対応状況を読み込めませんでした：${error.message}。対応済みとは判定していません。`);
    status.id = 'compatibility-loading';
    status.setAttribute('role', 'alert');
    const retry = element('button', '読み込み直す');
    retry.onclick = () => { retry.remove(); status.textContent = '対応状況を読み込み中…'; load(); };
    document.getElementById('compatibility-body').replaceChildren(status, retry);
  }
}
if (panel) load();
