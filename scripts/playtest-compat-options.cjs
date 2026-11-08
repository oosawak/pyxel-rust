const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const {chromium} = require('playwright');
const base = process.env.PYXEL_TEST_URL || 'http://127.0.0.1:8765';
const output = process.env.PYXEL_TEST_OUTPUT || '/private/tmp/pyxel-compat-options';
const info = page => page.evaluate(() => JSON.parse(demo.browser_demo_info()));
async function download(page, id, name) {
  const pending = page.waitForEvent('download');
  await page.locator(id).click();
  const file = await pending;
  await file.saveAs(`${output}/${name}`);
  return fs.readFile(`${output}/${name}`);
}
(async () => {
  await fs.mkdir(output,{recursive:true});
  const browser = await chromium.launch();
  try {
    const errors=[];
    const page=await browser.newPage({viewport:{width:1100,height:1100}});
    page.on('pageerror',e=>errors.push(e.message));
    await page.goto(`${base}/examples/browser-system/?config=1&display=3&capture=3&record=1`);
    await page.waitForFunction(()=>window.demo && JSON.parse(demo.browser_demo_info()).frame>3);
    let current=await info(page);
    assert.equal(current.options.display_scale,3);
    assert.equal(current.options.capture_scale,3);
    assert.ok(current.options.recorded>0,'recording starts at initialization');
    assert.equal(await page.locator('canvas').getAttribute('width'),'960');
    const image=await download(page,'#image-png','image-3x.png');
    assert.equal(image.readUInt32BE(16),24);assert.equal(image.readUInt32BE(20),24);
    const pixels=await page.evaluate(async bytes=>{
      const bitmap=await createImageBitmap(new Blob([new Uint8Array(bytes)],{type:'image/png'}));
      const c=document.createElement('canvas');c.width=bitmap.width;c.height=bitmap.height;
      const ctx=c.getContext('2d');ctx.drawImage(bitmap,0,0);bitmap.close();
      return Array.from(ctx.getImageData(0,0,c.width,c.height).data);
    },Array.from(image));
    for(let y=0;y<24;y++)for(let x=0;x<24;x++) {
      const expected=(x<12)===(y<12)?[212,24,108,255]:[112,198,169,255];
      assert.deepEqual(pixels.slice((y*24+x)*4,(y*24+x+1)*4),expected);
    }
    const png=await download(page,'#configured-png','screen-3x.png');
    assert.equal(png.readUInt32BE(16),960);assert.equal(png.readUInt32BE(20),576);
    const gif=await download(page,'#configured-gif','recording-3x.gif');
    assert.equal(gif.subarray(0,6).toString(),'GIF89a');
    assert.equal(gif.readUInt16LE(6),960);assert.equal(gif.readUInt16LE(8),576);
    assert.equal(await page.evaluate(()=>{try{demo.browser_demo_image_png(0);return false}catch{return true}}),true);
    assert.equal(await page.evaluate(()=>{try{demo.start_browser_system_with_options(3,0,1,false,0);return false}catch{return true}}),true);
    assert.equal((await info(page)).options.capture_scale,3);
    await page.locator('#bgm').click();
    await page.waitForFunction(()=>JSON.parse(demo.browser_demo_info()).playing);
    await page.locator('canvas').focus();await page.keyboard.press('q');
    await page.waitForFunction(()=>!JSON.parse(demo.browser_demo_info()).options.running);
    assert.equal((await info(page)).playing,false,'quit stops audio too');
    const stopped=(await info(page)).frame;await page.waitForTimeout(100);assert.equal((await info(page)).frame,stopped);
    await page.evaluate(async()=>{demo.browser_demo_shapes();await demo.browser_flip()});
    const centers=await page.locator('canvas').evaluate(c=>{
      const ctx=c.getContext('2d');
      return [[16,36],[40,36],[64,36],[88,36]].map(([x,y])=>Array.from(ctx.getImageData(x*3,y*3,1,1).data));
    });
    assert.deepEqual(centers,[[212,24,108,255],[43,51,95,255],[112,198,169,255],[43,51,95,255]]);
    await page.screenshot({path:`${output}/desktop.png`,fullPage:true});
    await page.setViewportSize({width:390,height:844});
    assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));
    await page.screenshot({path:`${output}/mobile.png`,fullPage:true});
    const headless=await browser.newPage();headless.on('pageerror',e=>errors.push(e.message));
    await headless.goto(`${base}/examples/browser-system/?config=1&headless=1&capture=3&record=1`);
    await headless.waitForFunction(()=>window.demo);
    current=await info(headless);assert.equal(current.options.headless,true);assert.equal(current.options.running,false);assert.equal(current.frame,0);
    assert.equal(await headless.locator('canvas').isVisible(),false);
    const offscreen=await download(headless,'#configured-png','headless-3x.png');
    assert.equal(offscreen.readUInt32BE(16),960);assert.equal(offscreen.readUInt32BE(20),576);
    assert.deepEqual(errors,[]);
    console.log(JSON.stringify({result:'passed',checks:['display scale','automatic recording','configured PNG/GIF scale','image PNG nearest-neighbor pixels','invalid settings preserve state','custom quit key','official filled/outline semantics','headless rendering/export','desktop/mobile layout'],screenshots:output}));
  } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1});
