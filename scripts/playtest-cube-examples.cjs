const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const os=require('node:os');
const {chromium}=require('playwright');
const base=process.env.PYXEL_TEST_URL||'http://127.0.0.1:8765';
const resolution=Number(process.env.PYXEL_TEST_RESOLUTION||1);
const output=process.env.PYXEL_TEST_OUTPUT||path.join(os.tmpdir(),'pyxel-official-cube');
const slugs=['c01-hello-cube','c02-basic-shapes','c03-custom-shapes','c04-mesh-and-motion','c05-3d-collision','c06-3d-physics'];
const info=f=>f.evaluate(()=>JSON.parse(demo.cube_example_info()));
const waitFrames=(f,start,delta=2)=>f.waitForFunction(({start,delta})=>JSON.parse(demo.cube_example_info()).frame>=start+delta,{start,delta});
const hash=f=>f.locator('canvas').evaluate(c=>{const p=c.getContext('2d').getImageData(0,0,c.width,c.height).data;let h=2166136261;for(let i=0;i<p.length;i+=4)h=Math.imul(h^p[i]^(p[i+1]<<8)^(p[i+2]<<16),16777619);return h;});
(async()=>{await fs.mkdir(output,{recursive:true});const b=await chromium.launch();try{const page=await b.newPage({viewport:{width:1050,height:1000}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
  for(let id=0;id<6;id++){
    await page.goto(`${base}/examples/cube/?demo=${slugs[id]}&version=rust&resolution=${resolution}`);
    await page.waitForFunction(()=>document.querySelector('#load-state').textContent.startsWith('Rust版を実行中'));
    const f=page.frames().find(f=>f.url().includes('/cube/rust.html'));assert.ok(f);
    await waitFrames(f,0,3);
    const colors=await f.locator('canvas').evaluate(c=>{const p=c.getContext('2d').getImageData(0,0,c.width,c.height).data;const colors=new Set();for(let i=0;i<p.length;i+=4)colors.add(`${p[i]},${p[i+1]},${p[i+2]}`);return colors.size;});assert.ok(colors>4,`c0${id+1} actual rendered colors`);
    assert.equal(await page.locator('#official-link').getAttribute('href'),`https://kitao.github.io/pyxel/web/showcase/examples/${slugs[id]}.html`);
    await f.locator('#pause').click();const frozen=(await info(f)).frame;await page.waitForTimeout(150);assert.equal((await info(f)).frame,frozen);
    await f.locator('#pause').click();await waitFrames(f,frozen);
    if(id===1){await f.locator('canvas').focus();await page.keyboard.down('Space');await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).wire);const previous=await hash(f);await page.keyboard.up('Space');await f.waitForFunction(()=>!JSON.parse(demo.cube_example_info()).wire);assert.notEqual(await hash(f),previous);}
    if(id===2){const s=await info(f);const box=await f.locator('canvas').boundingBox();const [x,y]=s.centers[0];await page.mouse.move(box.x+x*box.width/256,box.y+y*box.height/192);await page.mouse.down();await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).locked.length>0);await page.mouse.up();await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).shots>0);await f.locator('#audio').click();await f.waitForFunction(()=>document.querySelector('#audio').getAttribute('aria-pressed')==='true');}
    if(id===3){await f.locator('[data-motion="1"]').click();await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).motion===1);await f.locator('canvas').focus();await f.locator('canvas').press('Digit3');await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).motion===2);await f.locator('#shade').click();}
    if(id===4){const before=await info(f);await f.locator('canvas').focus();await page.keyboard.down('ArrowRight');await waitFrames(f,before.frame,5);await page.keyboard.up('ArrowRight');assert.notDeepEqual((await info(f)).player,before.player);await f.locator('canvas').press('Space');await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).jumps===1);await f.locator('canvas').press('Space');await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).jumps===2);await f.locator('#reset').click();await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).jumps===0);}
    if(id===5){const before=await info(f);await f.locator('canvas').focus();await f.locator('canvas').press('Space');await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).shots>=1);assert.ok((await info(f)).bodies>before.bodies);await f.locator('#reset').click();await f.waitForFunction(()=>JSON.parse(demo.cube_example_info()).shots===0);}
    await page.screenshot({path:path.join(output,`rust-${slugs[id]}.png`),fullPage:true});
  }
  // Official pages are loaded on demand and replace the Rust instance.
  for(let id=0;id<6;id++){
    await page.locator('#example').selectOption(String(id));
    await page.locator('#show-official').click();
    const frame=page.frameLocator('#demo-frame');
    await frame.locator('#pyxel-prompt').waitFor({timeout:60000});
    await frame.locator('#canvas').click();
    const reference = page.frames().find(f=>f.url().includes(slugs[id]+'.html'));
    await reference.waitForFunction(()=>{try{return window.pyxelContext?.initialized && !window.pyxelContext.hasFatalError && window.pyxelContext.pyodide.runPython('import pyxel; pyxel.frame_count')>=3;}catch{return false;}},null,{timeout:60000});
    console.log('Official running:',slugs[id]);
    assert.equal(await page.locator('#show-official').getAttribute('aria-pressed'),'true');
    assert.equal(await page.locator('#demo-frame').count(),1);
    await page.screenshot({path:path.join(output,`official-${slugs[id]}.png`),fullPage:true});
    await page.locator('#show-rust').click();
    await page.waitForFunction(()=>document.querySelector('#load-state').textContent.startsWith('Rust版を実行中'));
  }
  const mobile=await b.newPage({viewport:{width:390,height:844},isMobile:true,hasTouch:true});mobile.on('pageerror',e=>errors.push(e.message));await mobile.goto(`${base}/examples/cube/?demo=c05-3d-collision&version=rust&resolution=${resolution}`);await mobile.waitForFunction(()=>document.querySelector('#load-state').textContent.startsWith('Rust版を実行中'));const mf=mobile.frames().find(f=>f.url().includes('/cube/rust.html'));await waitFrames(mf,0,3);const action=mf.locator('[data-key="action"]');await action.tap();await mf.waitForFunction(()=>JSON.parse(demo.cube_example_info()).jumps>0);assert.ok(await mobile.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));assert.ok(await mf.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));await mobile.screenshot({path:path.join(output,'mobile-collision.png'),fullPage:true});
  assert.deepEqual(errors,[]);console.log(JSON.stringify({result:'passed',checks:['six Rust examples boot/render/animate','pause/resume','wireframe','drag lock/release fire','audio activation','motion keys/buttons','collision movement/double jump/reset','physics shoot/reset','six official pages start','version switching','single active frame','mobile touch/layout','no page runtime errors'],screenshots:output}));
}finally{await b.close();}})().catch(e=>{console.error(e);process.exitCode=1;});
