const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const { chromium } = require('playwright');
const base = process.env.PYXEL_TEST_URL || 'http://127.0.0.1:8765';
const output = process.env.PYXEL_TEST_OUTPUT || '/private/tmp/pyxel-cube-comparison';
const slugs = ['c01-hello-cube','c02-basic-shapes','c03-custom-shapes','c04-mesh-and-motion','c05-3d-collision','c06-3d-physics'];
async function ready(page) {
  await page.waitForFunction(() => document.querySelector('#load-state').textContent.startsWith('Rust版を実行中'));
  const rust = page.frames().find(f => f.url().includes('/cube/rust.html'));
  await rust.waitForFunction(() => JSON.parse(demo.cube_example_info()).frame >= 3);
  const official = page.frameLocator('#official-frame');
  await official.locator('#pyxel-prompt').waitFor({ timeout:60000 });
  await official.locator('#canvas').click();
  const reference = page.frames().find(f => f.url().startsWith('https://kitao.github.io/'));
  await reference.waitForFunction(() => {try {return window.pyxelContext?.initialized && window.pyxelContext.pyodide.runPython('import pyxel; pyxel.frame_count') >= 3;} catch {return false;}}, null, { timeout:60000 });
  return rust;
}
(async () => {
  await fs.mkdir(output,{recursive:true});
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({viewport:{width:1440,height:1000}});
    const errors=[]; page.on('pageerror',e=>errors.push(e.message));
    for (let id=0;id<6;id++) {
      await page.goto(`${base}/examples/cube/?demo=${slugs[id]}`);
      const rust = await ready(page);
      assert.equal(await page.locator('iframe').count(),2);
      assert.equal(await page.locator('#show-compare').getAttribute('aria-pressed'),'true');
      const left=await page.locator('#rust-frame').boundingBox(), right=await page.locator('#official-frame').boundingBox();
      assert.ok(left.x+left.width <= right.x+2, 'Desktop frames are side by side');
      const rustCanvas=await rust.locator('canvas').boundingBox();
      const officialCanvas=await page.frameLocator('#official-frame').locator('#canvas').boundingBox();
      assert.ok(Math.abs(rustCanvas.y-officialCanvas.y)<3, 'Canvas tops align');
      assert.ok(Math.abs(rustCanvas.width-officialCanvas.width)<3, 'Canvas display widths match');
      await rust.locator('#pause').click();
      const before=await rust.evaluate(()=>JSON.parse(demo.cube_example_info()).frame);
      await page.waitForTimeout(150);
      assert.equal(await rust.evaluate(()=>JSON.parse(demo.cube_example_info()).frame),before);
      if ([0,3,5].includes(id)) await page.screenshot({path:`${output}/desktop-${slugs[id]}.png`,fullPage:true});
      console.log('Parallel running:',slugs[id]);
    }
    // Changing render resolution restarts only Rust, keeping the official scene alive.
    for (const scale of [2,4,1]) {
      const reference=page.frames().find(f=>f.url().startsWith('https://kitao.github.io/'));
      await reference.evaluate(()=>window.comparisonSentinel='alive');
      await page.locator('#resolution').selectOption(String(scale));
      await page.waitForFunction(()=>document.querySelector('#load-state').textContent.startsWith('Rust版を実行中'));
      const rust=page.frames().find(f=>f.url().includes('/cube/rust.html'));
      await rust.waitForFunction(()=>JSON.parse(demo.cube_example_info()).frame>=3);
      const info=await rust.evaluate(()=>JSON.parse(demo.cube_example_info()));
      assert.equal(info.resolution,scale);
      assert.equal(info.render_width,320*scale);
      assert.equal(info.render_height,240*scale);
      assert.equal(await reference.evaluate(()=>window.comparisonSentinel),'alive');
      await rust.locator('#pause').click();
      if (scale===4) await page.screenshot({path:`${output}/high-resolution-4x.png`,fullPage:true});
    }
    await page.locator('#show-rust').click();
    await page.waitForFunction(()=>document.querySelector('#load-state').textContent.startsWith('Rust版を実行中'));
    assert.equal(await page.locator('iframe').count(),1);
    assert.equal(await page.locator('#demo-frame').count(),1);
    await page.locator('#show-official').click();
    assert.equal(await page.locator('iframe').count(),1);
    await page.locator('#show-compare').click(); await ready(page);
    const before=await page.locator('#rust-frame').getAttribute('src');
    await page.locator('#restart').click(); await ready(page);
    assert.equal(await page.locator('#rust-frame').getAttribute('src'),before);
    // An official load completion must not clear a Rust startup error.
    await page.route('**/rust.html*',route=>route.fulfill({contentType:'text/html',body:`<script>parent.postMessage({type:'cube-error',message:'test startup error'},location.origin)</script>`}));
    await page.locator('#restart').click();
    await page.waitForSelector('#load-state[role="alert"]');
    await page.waitForFunction(()=>document.querySelectorAll('.frame-status')[1].textContent.startsWith('公式ページ'));
    assert.match(await page.locator('#load-state').textContent(),/test startup error/);
    await page.unroute('**/rust.html*');
    const mobile=await browser.newPage({viewport:{width:390,height:844},isMobile:true,hasTouch:true});mobile.on('pageerror',e=>errors.push(e.message));
    await mobile.goto(`${base}/examples/cube/?demo=c01-hello-cube`); await ready(mobile);
    const a=await mobile.locator('#rust-frame').boundingBox(), b=await mobile.locator('#official-frame').boundingBox();
    assert.ok(a.y+a.height <= b.y, 'Mobile frames stack vertically');
    assert.ok(await mobile.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));
    await mobile.screenshot({path:`${output}/mobile.png`,fullPage:true});
    assert.deepEqual(errors,[]);
    console.log(JSON.stringify({result:'passed',checks:['six pairs run together','desktop side by side','mobile stacked/no overflow','pause independently','single modes','restart both','per-frame errors preserved','real 1x/2x/4x resolution','official survives resolution changes'],screenshots:output}));
  } finally {await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
