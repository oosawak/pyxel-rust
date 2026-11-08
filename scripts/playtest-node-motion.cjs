const assert=require('node:assert/strict');const fs=require('node:fs/promises');const path=require('node:path');const {chromium}=require('playwright');
const base=process.env.PYXEL_TEST_URL||'http://127.0.0.1:8765';const output=process.env.PYXEL_TEST_OUTPUT||'/private/tmp/pyxel-node-motion';
const info=p=>p.evaluate(()=>JSON.parse(demo.node_motion_demo_info()));
const close=(a,b)=>assert.ok(Math.abs(a-b)<1e-5,`${a} != ${b}`);
const frames=async(p,count=3)=>{const start=(await info(p)).updates;await p.waitForFunction(({start,count})=>JSON.parse(demo.node_motion_demo_info()).updates>=start+count,{start,count});};
const hash=p=>p.locator('canvas').evaluate(c=>{const a=c.getContext('2d').getImageData(0,0,c.width,c.height).data;let h=0;for(const v of a)h=(Math.imul(h,31)+v)|0;return h;});
(async()=>{await fs.mkdir(output,{recursive:true});const browser=await chromium.launch();try{
 const page=await browser.newPage({viewport:{width:1100,height:1100}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.goto(`${base}/examples/node-motion/`);await page.waitForFunction(()=>window.demo&&JSON.parse(demo.node_motion_demo_info()).updates>3);
 let state=await info(page);assert.equal(state.error,null);assert.equal(state.actors[0].clip,'Bounce');assert.equal(state.actors[1].clip,'Turn');
 const colors=await page.locator('canvas').evaluate(c=>{const p=c.getContext('2d').getImageData(0,0,c.width,c.height).data;const set=new Set();for(let i=0;i<p.length;i+=4)set.add(`${p[i]},${p[i+1]},${p[i+2]}`);return set.size;});assert.ok(colors>3);
 const before=await hash(page);await frames(page,12);assert.notEqual(await hash(page),before);
 await page.locator('#stop-0').click();state=await info(page);const pose=state.actors[0].part_y;const bframe=state.actors[1].frame;await frames(page);state=await info(page);close(state.actors[0].part_y,pose);assert.equal(state.actors[0].playing,false);assert.notEqual(state.actors[1].frame,bframe);
 await page.locator('#stop-1').click();const bStopped=(await info(page)).actors[1];await page.locator('#frame-0').fill('30');await page.locator('#sample-0').click();close((await info(page)).actors[0].part_y,3.8);
 await page.locator('#clip-0').selectOption('1');await page.locator('#speed-0').selectOption('0');await page.locator('#frame-0').fill('15');await page.locator('#play-0').click();await frames(page);state=await info(page);close(state.actors[0].frame,15);close(state.actors[0].part_y,3);close(state.actors[0].root_xx,Math.SQRT1_2);assert.deepEqual(state.actors[1],bStopped);close(state.source_y,3);
 await page.locator('#clip-0').selectOption('0');await page.locator('#speed-0').selectOption('-1');await page.locator('#frame-0').fill('2');await page.locator('#loop-0').uncheck();await page.locator('#play-0').click();await page.waitForFunction(()=>!JSON.parse(demo.node_motion_demo_info()).actors[0].playing);close((await info(page)).actors[0].part_y,3);
 const wrapped=await page.evaluate(()=>{demo.node_motion_demo_play(0,0,true,1,120);return JSON.parse(demo.node_motion_demo_info()).actors[0].frame});assert.equal(wrapped,0);await frames(page);assert.ok((await info(page)).actors[0].frame>0);
 await page.locator('#active-0').uncheck();const frozen=(await info(page)).actors[0].frame;await page.locator('#play-1').click();await frames(page);state=await info(page);assert.equal(state.actors[0].frame,frozen);assert.ok(state.actors[1].frame>0);await page.locator('#active-0').check();await frames(page);assert.notEqual((await info(page)).actors[0].frame,frozen);
 await page.locator('#stop-0').click();await page.locator('#stop-1').click();const valid=(await info(page)).actors[0];assert.equal(await page.evaluate(()=>{try{demo.node_motion_demo_play(0,0,true,NaN,0);return false}catch{return true}}),true);assert.equal(await page.evaluate(()=>{try{demo.node_motion_demo_play(0,99,true,1,0);return false}catch{return true}}),true);assert.deepEqual((await info(page)).actors[0],valid);
 const rustComparison=await page.evaluate(()=>{demo.node_motion_demo_sample(0,0,15);demo.node_motion_demo_sample(1,0,30);const i=JSON.parse(demo.node_motion_demo_info());return [i.actors[0].part_y,i.actors[1].part_y,i.source_y]});
 const loaded=page.waitForEvent('load');await page.locator('#reset').click();await loaded;await page.waitForFunction(()=>window.demo&&JSON.parse(demo.node_motion_demo_info()).updates>3);await page.locator('#state-0').filter({hasText:'Bounce'}).waitFor();
 await page.screenshot({path:`${output}/desktop.png`,fullPage:true});await page.setViewportSize({width:390,height:844});assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));await page.screenshot({path:`${output}/mobile.png`,fullPage:true});
 const official=await browser.newPage();await official.goto('https://kitao.github.io/pyxel/web/showcase/examples/c01-hello-cube.html');await official.locator('#pyxel-prompt').waitFor({timeout:60000});await official.locator('#canvas').click();await official.waitForFunction(()=>{try{return pyxelContext?.initialized&&!pyxelContext.hasFatalError&&pyxelContext.pyodide.runPython('import pyxel; pyxel.frame_count')>3}catch{return false}},null,{timeout:60000});
 const fixture=await fs.readFile(path.join(__dirname,'../docs/examples/retro3d/assets/animated-pyramid.glb'));
 const comparison=await official.evaluate(bytes=>{const p=pyxelContext.pyodide;p.FS.writeFile('/tmp/node-motion.glb',new Uint8Array(bytes));return JSON.parse(p.runPython(`
import json as _compat_json
from pyxel import cube as _compat_cube
_compat_mesh = _compat_cube.Mesh.from_glb('/tmp/node-motion.glb')
_compat_a = _compat_cube.Node.from_mesh(_compat_mesh)
_compat_b = _compat_cube.Node.from_mesh(_compat_mesh)
_compat_a.play_motion(_compat_mesh.motions[0], loop=True, speed=15, start_frame=0)
_compat_b.play_motion(_compat_mesh.motions[0], loop=True, speed=0, start_frame=30)
_compat_a.update()
_compat_b.update()
_compat_values = [_compat_a.find_by_name('Textured pyramid')[0].transform.pos.y, _compat_b.find_by_name('Textured pyramid')[0].transform.pos.y, _compat_mesh.transforms[1].pos.y]
_compat_readonly = []
for _compat_attr in ['pos', 'rot', 'scale']:
    try:
        setattr(_compat_cube.Mat4(), _compat_attr, None)
        _compat_readonly.append(False)
    except (AttributeError, TypeError):
        _compat_readonly.append(True)
_compat_json.dumps({'values':_compat_values, 'readonly':_compat_readonly, 'version':pyxel.VERSION})
`))},Array.from(fixture));
 comparison.values.forEach((value,index)=>close(value,rustComparison[index]));assert.deepEqual(comparison.readonly,[true,true,true]);assert.deepEqual(errors,[]);
 console.log(JSON.stringify({result:'passed',checks:['two animated models render and change pixels','independent stop and playback','frame sampling','clip switch resets authored channels','zero/negative speed','nonloop endpoint','loop wrapping','active pause/resume','invalid requests preserve pose','source mesh remains unchanged','reset','desktop/mobile layout','official Pyxel Node playback comparison','official read-only Mat4 properties'],official:comparison,screenshots:output}));
}finally{await browser.close()}})().catch(e=>{console.error(e);process.exitCode=1});
