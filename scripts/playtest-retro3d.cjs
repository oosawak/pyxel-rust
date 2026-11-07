// Requires Playwright. Serve docs/ locally before running this script.
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');
const { chromium } = require('playwright');

const base = process.env.PYXEL_TEST_URL || 'http://127.0.0.1:8765';
const output = process.env.PYXEL_TEST_OUTPUT || path.join(os.tmpdir(), 'pyxel-retro3d-smoke');

async function pixels(page) {
  return page.locator('canvas').evaluate(canvas => {
    const data = canvas.getContext('2d').getImageData(0, 0, canvas.width, canvas.height).data;
    const colors = {};
    let hash = 2166136261;
    for (let i = 0; i < data.length; i += 4) {
      const color = `${data[i]},${data[i + 1]},${data[i + 2]}`;
      colors[color] = (colors[color] || 0) + 1;
      hash = Math.imul(hash ^ data[i] ^ (data[i + 1] << 8) ^ (data[i + 2] << 16), 16777619);
    }
    return { hash, colors };
  });
}

(async () => {
  await fs.mkdir(output, { recursive: true });
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ viewport: { width: 1040, height: 980 } });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    const response = await page.goto(`${base}/examples/retro3d/`);
    assert.equal(response.status(), 200);
    await page.waitForFunction(() => document.querySelector('#status').textContent === '実行中');
    await page.evaluate(async () => {
      window.demo = await import(document.body.dataset.wasmModule);
    });
    const angle = await page.evaluate(() => demo.retro3d_angle());
    await page.waitForFunction(start => demo.retro3d_angle() !== start, angle);
    await page.locator('#pause').click();
    await page.waitForFunction(() => document.querySelector('#status').textContent === '一時停止中');
    const paused = await pixels(page);
    // All three differently colored cubes must reach the actual Canvas output.
    for (const color of ['233,195,91', '118,150,222', '255,151,152']) {
      assert.ok(paused.colors[color] > 100, `Missing cube color ${color}`);
    }
    await page.waitForTimeout(400);
    assert.equal((await pixels(page)).hash, paused.hash, 'Pause must freeze the rendered image');
    const beforeButton = await page.evaluate(() => demo.retro3d_angle());
    await page.locator('#right').click();
    await page.waitForFunction(start => demo.retro3d_angle() !== start, beforeButton);
    await page.waitForTimeout(100);
    assert.notEqual((await pixels(page)).hash, paused.hash, 'Rotation must change the rendered image');
    await page.locator('#distance').evaluate(input => {
      input.value = '5';
      input.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await page.waitForFunction(() => demo.retro3d_distance() === 5);
    await page.locator('canvas').focus();
    await page.keyboard.press('r');
    await page.waitForFunction(() => demo.retro3d_distance() === 7 && !demo.retro3d_paused());
    await page.keyboard.press('Space');
    await page.waitForFunction(() => demo.retro3d_paused());
    const beforeKey = await page.evaluate(() => demo.retro3d_angle());
    await page.keyboard.down('ArrowRight');
    await page.waitForFunction(start => demo.retro3d_angle() !== start, beforeKey);
    await page.keyboard.up('ArrowRight');
    await page.waitForFunction(() => document.querySelector('#status').textContent === '一時停止中');
    await page.screenshot({ path: path.join(output, 'desktop.png'), fullPage: true });

    // Load the built-in sample, then exercise the actual file selection path.
    await page.locator('#load-sample').click();
    await page.waitForFunction(() => document.querySelector('#model-status').textContent.includes('6 三角形'));
    assert.ok(await page.evaluate(() => demo.retro3d_has_model()));
    const samplePixels = await pixels(page);
    for (const color of ['233,195,91', '118,150,222']) {
      assert.ok(samplePixels.colors[color] > 100, `Missing imported texture color ${color}`);
    }
    await page.screenshot({ path: path.join(output, 'desktop-model.png'), fullPage: true });
    const fixture = await fs.readFile(path.join(__dirname, '../docs/examples/retro3d/assets/pyramid.glb'));
    await page.locator('#model-file').setInputFiles({ name: 'my-pyramid.glb', mimeType: 'model/gltf-binary', buffer: fixture });
    await page.waitForFunction(() => document.querySelector('#model-status').textContent.includes('my-pyramid.glb — 6 三角形'));
    await page.waitForTimeout(100);
    const importedPixels = await pixels(page);
    await page.locator('#model-file').setInputFiles({ name: 'broken.glb', mimeType: 'model/gltf-binary', buffer: Buffer.from('invalid GLB') });
    await page.waitForFunction(() => document.querySelector('#model-status').getAttribute('role') === 'alert');
    assert.ok(await page.evaluate(() => demo.retro3d_has_model()), 'Invalid import must retain the valid model');
    assert.equal((await pixels(page)).hash, importedPixels.hash, 'Invalid import must preserve the rendered scene');
    await page.locator('#clear-model').click();
    await page.waitForFunction(() => !demo.retro3d_has_model());
    await page.waitForTimeout(100);
    assert.ok((await pixels(page)).colors['255,151,152'] > 100, 'Clear must restore the pink cube');

    const mobile = await browser.newPage({ viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true });
    mobile.on('pageerror', error => errors.push(error.message));
    await mobile.goto(`${base}/examples/retro3d/`);
    await mobile.waitForFunction(() => document.querySelector('#status').textContent === '実行中');
    await mobile.evaluate(async () => { window.demo = await import(document.body.dataset.wasmModule); });
    await mobile.locator('#pause').tap();
    await mobile.waitForFunction(() => demo.retro3d_paused());
    const beforeTouch = await mobile.evaluate(() => demo.retro3d_angle());
    await mobile.locator('#left').tap();
    await mobile.waitForFunction(start => demo.retro3d_angle() !== start, beforeTouch);
    assert.ok(await mobile.evaluate(() => document.documentElement.scrollWidth <= innerWidth), 'Mobile page must not overflow');
    await mobile.waitForFunction(() => document.querySelector('#status').textContent === '一時停止中');
    await mobile.screenshot({ path: path.join(output, 'mobile.png'), fullPage: true });
    await mobile.locator('#load-sample').tap();
    await mobile.waitForFunction(() => demo.retro3d_has_model());
    await mobile.waitForTimeout(100);
    await mobile.screenshot({ path: path.join(output, 'mobile-model.png'), fullPage: true });
    assert.deepEqual(errors, [], 'Browser runtime errors');

    // Animation must move mesh vertices while automatic model rotation is off.
    await page.locator('#load-animated-sample').click();
    await page.waitForFunction(() => !document.querySelector('#animation-panel').hidden);
    assert.deepEqual(await page.evaluate(() => JSON.parse(demo.retro3d_animation_names())), ['Bounce', 'Turn']);
    assert.equal(await page.evaluate(() => demo.retro3d_auto_rotate()), false);
    await page.locator('#animation-play').click();
    await page.waitForFunction(() => demo.retro3d_animation_playing());
    const animationStart = await page.evaluate(() => demo.retro3d_animation_time());
    await page.waitForFunction(start => demo.retro3d_animation_time() !== start, animationStart);
    await page.waitForFunction(() => document.querySelector('#animation-play').textContent === 'アニメーション停止');
    await page.locator('#animation-play').click();
    await page.waitForFunction(() => !demo.retro3d_animation_playing());
    const stoppedTime = await page.evaluate(() => demo.retro3d_animation_time());
    await page.waitForTimeout(300);
    assert.equal(await page.evaluate(() => demo.retro3d_animation_time()), stoppedTime);
    const seek = async seconds => {
      await page.locator('#animation-seek').evaluate((input, value) => {
        input.value = String(value);
        input.dispatchEvent(new Event('input', { bubbles: true }));
      }, seconds);
      await page.waitForFunction(value => Math.abs(demo.retro3d_animation_time() - value) < 0.001, seconds);
      await page.waitForTimeout(100);
    };
    await seek(0);
    const restFrame = await pixels(page);
    await seek(1);
    await page.locator('#animation-seek').focus();
    await page.keyboard.press('ArrowRight');
    await page.waitForFunction(() => demo.retro3d_animation_time() > 1);
    await seek(1);
    assert.notEqual((await pixels(page)).hash, restFrame.hash, 'Clip sampling must move the rendered mesh');
    await page.screenshot({ path: path.join(output, 'desktop-animation.png'), fullPage: true });
    await page.locator('#animation-clip').selectOption('1');
    await page.waitForFunction(() => demo.retro3d_animation_index() === 1);
    await seek(1);
    await page.locator('#animation-clip').selectOption('-1');
    await page.waitForFunction(() => demo.retro3d_animation_index() === -1);
    await page.waitForTimeout(100);
    assert.equal((await pixels(page)).hash, restFrame.hash, 'Rest pose must clear previous clip transforms');
    await page.locator('#animation-clip').selectOption('0');
    await page.locator('#animation-loop').uncheck();
    await page.locator('#animation-speed').selectOption('2');
    await seek(1.9);
    await page.waitForFunction(() => !document.querySelector('#animation-play').disabled);
    await page.locator('#animation-play').click();
    await page.waitForFunction(() => demo.retro3d_animation_time() === 2 && !demo.retro3d_animation_playing());
    await page.locator('#clear-model').click();
    assert.ok(await page.locator('#animation-panel').isHidden());

    await mobile.locator('#load-animated-sample').tap();
    await mobile.waitForFunction(() => !document.querySelector('#animation-panel').hidden);
    await mobile.locator('#animation-play').tap();
    await mobile.waitForFunction(() => demo.retro3d_animation_playing());
    await mobile.waitForTimeout(200);
    await mobile.waitForFunction(() => document.querySelector('#animation-play').textContent === 'アニメーション停止');
    await mobile.locator('#animation-play').tap();
    await mobile.waitForFunction(() => !demo.retro3d_animation_playing());
    assert.ok(await mobile.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
    await mobile.screenshot({ path: path.join(output, 'mobile-animation.png'), fullPage: true });

    // An unavailable WASM binary must display a recoverable error, not hang on loading.
    const failurePage = await browser.newPage();
    await failurePage.route(/\/pyxel_rust_bg\.wasm(?:\?.*)?$/, route => route.abort());
    await failurePage.goto(`${base}/examples/retro3d/`);
    await failurePage.waitForFunction(() => document.querySelector('#status').getAttribute('role') === 'alert');
    assert.ok(await failurePage.locator('#pause').isDisabled());
    console.log(JSON.stringify({ result: 'passed', checks: ['load', 'canvas colors', 'animation', 'pause', 'rotation', 'zoom', 'keyboard reset', 'keyboard input', 'mobile touch and layout', 'GLB sample', 'embedded textures', 'GLB file selection', 'failed import preserves model', 'clear model', 'mobile model', 'animation playback/pause', 'animation seek', 'clip switch/rest pose', 'nonloop endpoint', 'mobile animation', 'load error'], screenshots: output }));
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
