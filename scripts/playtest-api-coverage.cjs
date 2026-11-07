const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const {chromium}=require('playwright');
const base=process.env.PYXEL_TEST_URL||'http://127.0.0.1:8765';
const output=process.env.PYXEL_TEST_OUTPUT||'/private/tmp/pyxel-api-coverage';
const info=p=>p.evaluate(()=>JSON.parse(demo.api_coverage_info()));
(async()=>{await fs.mkdir(output,{recursive:true});const browser=await chromium.launch();try{
 const page=await browser.newPage({viewport:{width:1050,height:1100}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.addInitScript(()=>{
  window.testPads=[];
  Object.defineProperty(navigator,'getGamepads',{value:()=>window.testPads});
 });
 await page.goto(`${base}/examples/api-coverage/`);
 await page.waitForFunction(()=>document.querySelector('#status').textContent==='実行中');
 await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).frame>3);
 const canvas=page.locator('canvas');await canvas.focus();
 await page.keyboard.down('a');await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).repeats>=4);
 const down=await info(page);assert.equal(down.presses,1);assert.equal(down.a,true);
 const repeat=down.repeat_frames;assert.equal(repeat[1]-repeat[0],6);assert.equal(repeat[2]-repeat[1],3);
 await page.keyboard.up('a');await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).releases===1);
 await page.keyboard.down('Shift');await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).shift);await page.keyboard.up('Shift');
 const box=await canvas.boundingBox();await page.mouse.move(box.x+box.width*0.75,box.y+box.height*0.6);
 await page.waitForFunction(()=>{let i=JSON.parse(demo.api_coverage_info());return Math.abs(i.mouse[0]-240)<=1&&Math.abs(i.mouse[1]-120)<=1});
 await page.mouse.down();await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).left);await page.mouse.move(2,2);await page.mouse.up();await page.waitForFunction(()=>!JSON.parse(demo.api_coverage_info()).left);assert.equal((await info(page)).clicks,1);
 await page.mouse.move(box.x+80,box.y+80);await page.mouse.down({button:'right'});await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).right);await page.mouse.up({button:'right'});
 await page.mouse.wheel(0,-100);await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).wheel>0);
 await page.locator('#cursor').click();await page.waitForFunction(()=>document.querySelector('canvas').style.cursor==='none');await page.locator('#cursor').click();await page.waitForFunction(()=>document.querySelector('canvas').style.cursor==='default');
 // Gamepad WebIDL requires actual Gamepad objects: use the Chromium virtual gamepad protocol when available.
 // This fixture overrides the JS prototype so wasm-bindgen's instanceof check sees Web Gamepad objects.
 await page.evaluate(()=>{
  window.testPads=[0,1,2,3].map((_,index)=>{
   const pad=Object.create(Gamepad.prototype);
   const buttons=Array.from({length:17},(_,i)=>Object.defineProperties(Object.create(GamepadButton.prototype),{pressed:{value:i===0},value:{value:i===6?0.5:0}}));
   Object.defineProperties(pad,{connected:{value:true},mapping:{value:'standard'},index:{value:index},buttons:{value:buttons},axes:{value:[-1,0,0,0]}});return pad;
  });
 });
 await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).pads.every(Boolean));assert.deepEqual((await info(page)).axes,[-32768,-32768,-32768,-32768]);
 await page.evaluate(()=>window.testPads=[]);await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).pads.every(v=>!v));assert.deepEqual((await info(page)).axes,[0,0,0,0]);
 await canvas.focus();await page.keyboard.down('a');await page.waitForFunction(()=>JSON.parse(demo.api_coverage_info()).a);await page.evaluate(()=>window.dispatchEvent(new Event('blur')));await page.waitForFunction(()=>!JSON.parse(demo.api_coverage_info()).a);await page.keyboard.up('a');
 assert.equal((await info(page)).ceil,-1);assert.equal((await info(page)).angle,90);
 const colors=await canvas.evaluate(c=>{let d=c.getContext('2d').getImageData(0,0,c.width,c.height).data;let set=new Set();for(let i=0;i<d.length;i+=4)set.add(`${d[i]},${d[i+1]},${d[i+2]}`);return set.size});assert.ok(colors>=7);
 await page.screenshot({path:`${output}/desktop.png`,fullPage:true});
 await page.setViewportSize({width:390,height:844});assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));
 await page.screenshot({path:`${output}/mobile.png`,fullPage:true});assert.deepEqual(errors,[]);
 console.log(JSON.stringify({result:'passed',checks:['public drawing APIs render','degree math','frame-based repeat','modifier groups','CSS scaled mouse coordinates','mouse buttons/release outside canvas','wheel','cursor visibility','four standard gamepads/axes/disconnect','blur releases held input','mobile layout'],screenshots:output}));
}finally{await browser.close()}})().catch(e=>{console.error(e);process.exitCode=1});
