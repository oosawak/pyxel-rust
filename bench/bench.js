// 3-way benchmark x 2 games: Pyxel(Python) vs pyxel-rust(WASM) vs plain JS
// Metrics: transferred bytes, time-to-first-moving-frame (screenshot diff),
// rAF FPS over a window, CDP TaskDuration/ScriptDuration deltas, longtask count.
const puppeteer = require('puppeteer-core');
const crypto = require('crypto');
const fs = require('fs');

const GAMES = [
  { game: 'CubeBoy', impl: 'pyxel-python', label: 'Pyxel (Python/Emscripten)', url: 'http://127.0.0.1:8917/html-player/games/cube-boy-pyx/index.html' },
  { game: 'CubeBoy', impl: 'pyxel-rust',   label: 'pyxel-rust (Rust/WASM)',    url: 'http://127.0.0.1:8917/pyxel-rust/examples/cubeboy/' },
  { game: 'CubeBoy', impl: 'plain-js',     label: 'Plain JavaScript',          url: 'http://127.0.0.1:8917/html-player/games/cube-boy/index.html' },
  { game: 'LINE BOY', impl: 'pyxel-python', label: 'Pyxel (Python/Emscripten)', url: 'http://127.0.0.1:8917/html-player/games/line-boy-pyx/index.html' },
  { game: 'LINE BOY', impl: 'pyxel-rust',   label: 'pyxel-rust (Rust/WASM)',    url: 'http://127.0.0.1:8917/pyxel-rust/examples/lineboy/' },
  { game: 'LINE BOY', impl: 'plain-js',     label: 'Plain JavaScript',          url: 'http://127.0.0.1:8917/html-player/games/line-boy/index.html' },
];
const SAMPLE_MS = 20000;

async function measure(browser, target) {
  const page = await browser.newPage();
  await page.setViewport({ width: 960, height: 720 });
  const client = await page.target().createCDPSession();
  await client.send('Network.enable');
  await client.send('Network.setCacheDisabled', { cacheDisabled: true });
  await client.send('Performance.enable');
  let bytes = 0;
  client.on('Network.loadingFinished', (e) => { bytes += e.encodedDataLength || 0; });

  const t0 = Date.now();
  await page.goto(target.url, { waitUntil: 'load', timeout: 60000 }).catch(() => {});
  // Wait for a canvas to appear (engine booted).
  let hasCanvas = false;
  for (let i = 0; i < 60; i++) {
    hasCanvas = await page.evaluate(() => document.querySelectorAll('canvas').length > 0).catch(() => false);
    if (hasCanvas) break;
    await new Promise(r => setTimeout(r, 500));
  }
  if (!hasCanvas) { await page.close(); return { game: target.game, impl: target.impl, label: target.label, url: target.url, error: 'no-canvas' }; }
  // Start the game (click-to-start screens, audio unlock).
  await page.mouse.click(480, 360).catch(() => {});
  await page.keyboard.press('Space').catch(() => {});

  let firstFrameMs = null;
  let prevHash = null;
  const deadline = Date.now() + 45000;
  while (Date.now() < deadline) {
    const shot = await page.screenshot({ type: 'png' }).catch(() => null);
    if (shot) {
      const h = crypto.createHash('md5').update(shot).digest('hex');
      if (prevHash && h !== prevHash) { firstFrameMs = Date.now() - t0; break; }
      prevHash = h;
    }
    await new Promise(r => setTimeout(r, 200));
  }

  await page.keyboard.press('Enter').catch(() => {});

  await page.evaluate(() => {
    window.__frames = 0; window.__longtasks = 0;
    const loop = () => { window.__frames++; requestAnimationFrame(loop); };
    requestAnimationFrame(loop);
    try {
      new PerformanceObserver((list) => { window.__longtasks += list.getEntries().length; })
        .observe({ entryTypes: ['longtask'] });
    } catch (e) {}
  });
  const m0 = await client.send('Performance.getMetrics');
  const f0 = await page.evaluate(() => window.__frames);

  const sampleStart = Date.now();
  let toggle = false;
  while (Date.now() - sampleStart < SAMPLE_MS) {
    await page.keyboard.down(toggle ? 'ArrowLeft' : 'ArrowRight').catch(() => {});
    await page.screenshot({ type: 'png' }).catch(() => {}); // keep compositor producing frames equally for all targets
    await new Promise(r => setTimeout(r, 1200));
    await page.keyboard.up(toggle ? 'ArrowLeft' : 'ArrowRight').catch(() => {});
    await page.keyboard.press('Space').catch(() => {});
    toggle = !toggle;
  }

  const m1 = await client.send('Performance.getMetrics');
  const f1 = await page.evaluate(() => window.__frames);
  const longtasks = await page.evaluate(() => window.__longtasks).catch(() => null);
  const get = (m, k) => (m.metrics.find(x => x.name === k) || {}).value || 0;
  const taskDelta = get(m1, 'TaskDuration') - get(m0, 'TaskDuration');
  const scriptDelta = get(m1, 'ScriptDuration') - get(m0, 'ScriptDuration');

  const shotFinal = await page.screenshot({ type: 'png' }).catch(() => null);
  fs.writeFileSync(`shot-${target.game.replace(/\s/g, '')}-${target.impl}.png`, shotFinal || Buffer.alloc(0));

  await page.close();
  return {
    game: target.game, impl: target.impl, label: target.label, url: target.url,
    bytes, firstFrameMs,
    fps: (f1 - f0) / (SAMPLE_MS / 1000),
    taskBusyPct: taskDelta / (SAMPLE_MS / 1000) * 100,
    scriptBusyPct: scriptDelta / (SAMPLE_MS / 1000) * 100,
    longtasks,
  };
}

(async () => {
  const browser = await puppeteer.launch({
    executablePath: process.env.CHROME_PATH || '/home/hatch/workspace/pyxel-bench/chrome-linux64/chrome',
    headless: 'new',
    args: ['--no-sandbox', '--mute-audio', '--autoplay-policy=no-user-gesture-required',
           '--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--window-size=960,720'],
  });
  const results = [];
  for (const t of GAMES) {
    if (process.env.ONLY && t.impl !== process.env.ONLY) continue;
    for (let run = 1; run <= 2; run++) {
      try {
        const r = await measure(browser, t);
        r.run = run;
        results.push(r);
        console.log(JSON.stringify(r));
      } catch (e) {
        console.log(JSON.stringify({ game: t.game, impl: t.impl, run, error: String(e).slice(0, 300) }));
      }
    }
  }
  await browser.close();
  fs.writeFileSync('results.json', JSON.stringify(results, null, 2));
  console.log('DONE');
})();
