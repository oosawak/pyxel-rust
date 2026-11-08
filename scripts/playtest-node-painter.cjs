const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const { chromium } = require('playwright');
const base = process.env.PYXEL_TEST_URL || 'http://127.0.0.1:8765';
const output = process.env.PYXEL_TEST_OUTPUT || '/private/tmp/pyxel-node-painter';
(async () => {
  await fs.mkdir(output, { recursive: true });
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ viewport: { width: 1100, height: 1100 } });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto(`${base}/examples/screen-tools/`);
    await page.waitForFunction(() => window.demo);
    await page.locator('#painter').click();
    await page.waitForFunction(() => JSON.parse(demo.screen_demo_info()).node_children === 18);
    await page.waitForTimeout(100);
    const rust = await page.evaluate(() => {
      const pixels = [];
      for (let y = 24; y < 152; y++) for (let x = 0; x < 256; x++) {
        const c = demo.screen_demo_pixel(x, y);
        pixels.push([8, 10, 11].includes(c) ? c : 0);
      }
      return pixels;
    });
    const cells = Array.from({ length: 18 }, (_, index) => {
      let n = 0;
      const cx = (index % 6 + .5) * 256 / 6, cy = 16 + Math.floor(index / 6) * 128 / 3;
      for (let y = Math.floor(cy - 12); y < cy + 12; y++)
        for (let x = Math.floor(cx - 15); x < cx + 15; x++) if (rust[y * 256 + x]) n++;
      return n;
    });
    assert.ok(cells.every(n => n > 0), `all 18 commands must render: ${cells}`);
    await page.screenshot({ path: `${output}/desktop.png`, fullPage: true });
    await page.evaluate(() => demo.screen_demo_node_flags(true, true, 0));
    await page.waitForTimeout(100);
    assert.equal(await page.evaluate(() => {
      let n = 0;
      for (let y = 24; y < 152; y++) for (let x = 0; x < 256; x++)
        if ([8,10,11].includes(demo.screen_demo_pixel(x,y))) n++;
      return n;
    }), 0, 'parent opacity must hide all commands');
    await page.evaluate(() => demo.screen_demo_node_flags(true, true, 1));
    await page.waitForFunction(() => demo.screen_demo_pixel(21, 40) === 10);
    await page.setViewportSize({ width: 390, height: 844 });
    assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
    await page.screenshot({ path: `${output}/mobile.png`, fullPage: true });
    // Run equivalent commands against the actual official Python binding, offscreen.
    const official = await browser.newPage();
    await official.goto('https://kitao.github.io/pyxel/web/showcase/examples/c01-hello-cube.html');
    await official.locator('#pyxel-prompt').waitFor({ timeout: 60000 });
    await official.locator('#canvas').click();
    await official.waitForFunction(() => {
      try { return pyxelContext?.initialized && !pyxelContext.hasFatalError && pyxelContext.pyodide.runPython('import pyxel; pyxel.frame_count') > 3; }
      catch { return false; }
    }, null, { timeout: 60000 });
    const comparison = await official.evaluate(() => JSON.parse(pyxelContext.pyodide.runPython(`
import json as _np_json
from pyxel.cube import Node as _NPNode, Mat4 as _NPMat4, Vec3 as _NPVec3, Camera as _NPCamera, Primitive as _NPPrimitive
_np_texture = pyxel.Image(8,8)
_np_texture.cls(11)
_np_texture.rect(0,0,4,4,8)
_np_texture.rect(4,4,4,4,8)
class _NPPainter(_NPNode):
    def __init__(self, i):
        super().__init__()
        self.i = i
        self.transform = _NPMat4.from_translation(_NPVec3(i%6*4-10,4.5-i//6*4,-8))
    def on_draw(self):
        self.shaded(False)
        m = _NPMat4()
        z = _NPVec3(0,0,0)
        a,b,c = _NPVec3(-.9,-.8,0),_NPVec3(.9,-.8,0),_NPVec3(0,.9,0)
        uv = ((0,0),(8,0),(0,8),(8,8))
        i = self.i
        if i == 0: self.pset(z,10)
        elif i == 1: self.line(a,c,10)
        elif i == 2: self.tri(a,b,c,8)
        elif i == 3: self.trib(a,b,c,11)
        elif i == 4: self.rect(m,1.8,1.6,8)
        elif i == 5: self.rectb(m,1.8,1.6,11)
        elif i == 6: self.circ(z,.9,8)
        elif i == 7: self.circb(z,.9,11)
        elif i == 8: self.elli(m,1.8,1.3,8)
        elif i == 9: self.ellib(m,1.8,1.3,11)
        elif i == 10: self.box(_NPMat4.from_euler(_NPVec3(20,30,0)),_NPVec3(1.4,1.4,1.4),_np_texture)
        elif i == 11: self.boxb(_NPMat4.from_euler(_NPVec3(20,30,0)),_NPVec3(1.4,1.4,1.4),11)
        elif i == 12: self.sphere(z,.9,_np_texture)
        elif i == 13: self.sphereb(z,.9,11)
        elif i == 14: self.plane(m,_np_texture,uv,1.8,1.6)
        elif i == 15: self.sprite(z,_np_texture,uv,1.8,1.6,rotate=20)
        elif i == 16: self.prim(m,_NPPrimitive.plane(1.8,1.6),_np_texture)
        else: self.text(z,'3D',10)
_np_root = _NPNode()
_np_root.camera = _NPCamera()
_np_root.camera.ortho_size = 12
for _np_i in range(18): _np_root.add_child(_NPPainter(_np_i))
_np_target = pyxel.Image(256,128)
_np_target.cls(1)
_np_root.draw(0,0,256,128,_np_target)
_np_json.dumps({'version':pyxel.VERSION,'pixels':[c if c in (8,10,11) else 0 for y in range(128) for x in range(256) for c in [_np_target.pget(x,y)]]})
`)));
    const differences = rust.reduce((n, value, i) => n + (value !== comparison.pixels[i]), 0);
    assert.equal(differences, 0, '18 commands must match official offscreen pixels');
    assert.deepEqual(errors, []);
    console.log(JSON.stringify({ result: 'passed', cells, official: comparison.version, differences, checks: ['18 rendered commands', 'official pixel comparison', 'inherited opacity', 'desktop/mobile layout'], screenshots: output }));
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
