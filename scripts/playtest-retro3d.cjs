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
      window.demo = await import('./pkg/pyxel_rust.js');
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

    const mobile = await browser.newPage({ viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true });
    mobile.on('pageerror', error => errors.push(error.message));
    await mobile.goto(`${base}/examples/retro3d/`);
    await mobile.waitForFunction(() => document.querySelector('#status').textContent === '実行中');
    await mobile.evaluate(async () => { window.demo = await import('./pkg/pyxel_rust.js'); });
    await mobile.locator('#pause').tap();
    await mobile.waitForFunction(() => demo.retro3d_paused());
    const beforeTouch = await mobile.evaluate(() => demo.retro3d_angle());
    await mobile.locator('#left').tap();
    await mobile.waitForFunction(start => demo.retro3d_angle() !== start, beforeTouch);
    assert.ok(await mobile.evaluate(() => document.documentElement.scrollWidth <= innerWidth), 'Mobile page must not overflow');
    await mobile.waitForFunction(() => document.querySelector('#status').textContent === '一時停止中');
    await mobile.screenshot({ path: path.join(output, 'mobile.png'), fullPage: true });
    assert.deepEqual(errors, [], 'Browser runtime errors');

    // An unavailable WASM binary must display a recoverable error, not hang on loading.
    const failurePage = await browser.newPage();
    await failurePage.route('**/pyxel_rust_bg.wasm', route => route.abort());
    await failurePage.goto(`${base}/examples/retro3d/`);
    await failurePage.waitForFunction(() => document.querySelector('#status').getAttribute('role') === 'alert');
    assert.ok(await failurePage.locator('#pause').isDisabled());
    console.log(JSON.stringify({ result: 'passed', checks: ['load', 'canvas colors', 'animation', 'pause', 'rotation', 'zoom', 'keyboard reset', 'keyboard input', 'mobile touch and layout', 'load error'], screenshots: output }));
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
