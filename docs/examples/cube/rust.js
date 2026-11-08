import { examples } from './examples.js';
const id = Number(new URLSearchParams(location.search).get('id') || 0);
const status = document.getElementById('status');
const canvas = document.getElementById('pyxel_canvas');
const keys = new Set();
let pointerHeld = false;
let touch = new Set();
let mouse = { mouse_x: 0, mouse_y: 0, pointer: false };
let lastAction = false;
let paused = false;
let wasm;
let audio;
let gain;
let enabled = false;
let audioError = false;
const buffers = new Map();
const sources = new Map();
function report(type, message) { parent.postMessage({type,message},location.origin); }
async function toggleAudio() {
  try {
    audio ||= new AudioContext();
    if (!gain) { gain=audio.createGain(); gain.gain.value=.35; gain.connect(audio.destination); }
    if (audioError) throw new Error('効果音を読み込めませんでした');
    enabled=!enabled;
    await audio.resume();
    document.getElementById('audio').textContent = enabled ? '音声をオフ' : '音声をオン';
    document.getElementById('audio').setAttribute('aria-pressed',String(enabled));
    document.getElementById('audio-status').textContent = enabled ? '音声：オン（公式MMLから生成した効果音）' : '音声：オフ';
    if (!enabled) { for(const source of sources.values()) source.stop(); sources.clear(); }
  } catch(error) { document.getElementById('audio-status').textContent=`音声を開始できません：${error.message}`; }
}
function emitSound(effect) {
  if (!enabled || !audio || !buffers.has(effect)) return;
  // IDs match the original examples' independent audio channels.
  const old=sources.get(effect); if(old)old.stop();
  const source=audio.createBufferSource(); source.buffer=buffers.get(effect); source.connect(gain); source.start(); sources.set(effect,source);
  source.onended=()=>{if(sources.get(effect)===source)sources.delete(effect);};
}
async function preloadAudio() {
  const count=id===2?4:id===4?3:id===5?1:0;
  if(!count) { document.getElementById('audio').hidden=true; document.getElementById('audio-status').textContent='この公式サンプルには効果音がありません。'; return; }
  try {
    // Decode on the first user gesture so browsers do not start audio by themselves.
    const pending = await Promise.all(Array.from({length:count},async(_,effect)=>{
      const response=await fetch(`./audio/c${id+1}-${effect}.wav`);if(!response.ok)throw new Error(`HTTP ${response.status}`);return [effect,await response.arrayBuffer()];
    }));
    const button=document.getElementById('audio');
    button.onclick=async()=>{
      try {
        audio ||= new AudioContext();
        if(!buffers.size)for(const [effect,data]of pending)buffers.set(effect,await audio.decodeAudioData(data.slice(0)));
        await toggleAudio();
      }catch(error){audioError=true;document.getElementById('audio-status').textContent=`効果音を読み込めません：${error.message}`;}
    };
    button.disabled=false;
  } catch(error) {audioError=true;document.getElementById('audio-status').textContent=`効果音を読み込めません：${error.message}`;}
}
function updateInput() {
  if(!wasm)return;
  const pad=Array.from(navigator.getGamepads?.()||[]).find(Boolean);
  const down=(key,index)=>keys.has(key)||touch.has(key)||Boolean(pad?.buttons[index]?.pressed);
  const action=down('Space',0)||pointerHeld||touch.has('action');
  const data={left:keys.has('ArrowLeft')||touch.has('left')||Boolean(pad?.buttons[14]?.pressed),right:keys.has('ArrowRight')||touch.has('right')||Boolean(pad?.buttons[15]?.pressed),up:keys.has('ArrowUp')||touch.has('up')||Boolean(pad?.buttons[12]?.pressed),down:keys.has('ArrowDown')||touch.has('down')||Boolean(pad?.buttons[13]?.pressed),action,pressed:action&&!lastAction,released:!action&&lastAction,...mouse};
  if(pad?.buttons[1]?.pressed&&!updateInput.padB) { if(id===5)data.reset=true; else if(id===4)data.shade=true; else if(id===3)data.motion=1; }
  if(pad?.buttons[2]?.pressed&&!updateInput.padX&&id===3)data.motion=2;
  if(pad?.buttons[3]?.pressed&&!updateInput.padY&&id===3)data.shade=true;
  if(pad?.buttons[0]?.pressed&&!updateInput.padA&&id===3)data.motion=0;
  updateInput.padA=Boolean(pad?.buttons[0]?.pressed);updateInput.padB=Boolean(pad?.buttons[1]?.pressed);updateInput.padX=Boolean(pad?.buttons[2]?.pressed);updateInput.padY=Boolean(pad?.buttons[3]?.pressed);
  lastAction=action;wasm.cube_example_input(JSON.stringify(data));
}
function pointer(event){const box=canvas.getBoundingClientRect();const [w,h]=examples[id].size;mouse={mouse_x:(event.clientX-box.left)*w/box.width,mouse_y:(event.clientY-box.top)*h/box.height,pointer:true};updateInput();}
try {
  if(!Number.isInteger(id)||!examples[id])throw new Error('存在しないデモです');
  document.getElementById('help').textContent=examples[id].help;
  const compare = new URLSearchParams(location.search).get('compare') === '1';
  canvas.style.width = compare ? '100%' : `min(100%, ${55*examples[id].size[0]/examples[id].size[1]}vh)`;
  if (compare) new ResizeObserver(() => {
    parent.postMessage({type:'cube-resize',height:Math.ceil(document.body.getBoundingClientRect().height)},location.origin);
  }).observe(document.body);
  for(const button of document.querySelectorAll('[data-key]:not(#action)'))button.hidden=[0,2].includes(id);
  document.getElementById('motion-controls').hidden=id!==3;
  document.getElementById('shade').hidden=![3,4].includes(id);
  document.getElementById('reset').hidden=![4,5].includes(id);
  document.getElementById('audio').disabled=true;
  document.getElementById('action').textContent= id===1?'ワイヤー表示':id===2?'ロック／発射':id===4?'ジャンプ':id===5?'発射':'アクション';
  document.getElementById('action').hidden=[0,3].includes(id);
  // A versioned URL avoids mixing a cached pre-port loader/WASM with this page.
  const build='a4cbdeaa366b';
  wasm=await import(`../retro3d/pkg/pyxel_rust.js?build=${build}`);
  await wasm.default({module_or_path:new URL(`../retro3d/pkg/pyxel_rust_bg.wasm?build=${build}`,import.meta.url)});
  const requestedResolution = Number(new URLSearchParams(location.search).get("resolution")) || 1;
  wasm.start_cube_example_resolution(id, [1,2,4].includes(requestedResolution) ? requestedResolution : 1);
  window.demo=wasm;
  const post=data=>wasm.cube_example_input(JSON.stringify(data));
  for(const event of ['keydown','keyup'])document.addEventListener(event,e=>{
    if(e.target.closest('button,input,select,a'))return;
    if(event==='keydown'){if(e.repeat)return;keys.add(e.code);if(e.code==='KeyS')post({shade:true});if(e.code==='KeyR')post({reset:true});if(['Digit1','Digit2','Digit3'].includes(e.code))post({motion:Number(e.code.at(-1))-1});if(e.code==='KeyQ'){paused=true;wasm.pause_cube_example(true);document.getElementById('pause').textContent='再開';document.getElementById('pause').setAttribute('aria-pressed','true');status.textContent='停止中（再開できます）';}}else keys.delete(e.code);
    if(['ArrowLeft','ArrowRight','ArrowUp','ArrowDown','Space'].includes(e.code))e.preventDefault();updateInput();
  });
  canvas.addEventListener('pointermove',pointer);
  canvas.addEventListener('pointerdown',e=>{canvas.focus();canvas.setPointerCapture(e.pointerId);pointerHeld=true;pointer(e);});
  const release=e=>{pointerHeld=false;pointer(e);};canvas.addEventListener('pointerup',release);canvas.addEventListener('pointercancel',release);
  for(const button of document.querySelectorAll('[data-key]')) {
    button.addEventListener('pointerdown',e=>{e.preventDefault();button.setPointerCapture(e.pointerId);touch.add(button.dataset.key);updateInput();});
    for(const event of ['pointerup','pointercancel','lostpointercapture'])button.addEventListener(event,()=>{touch.delete(button.dataset.key);updateInput();});
    button.addEventListener('keydown',event=>{if(['Space','Enter'].includes(event.code)){event.preventDefault();touch.add(button.dataset.key);updateInput();}});
    button.addEventListener('keyup',event=>{if(['Space','Enter'].includes(event.code)){event.preventDefault();touch.delete(button.dataset.key);updateInput();}});
    button.addEventListener('blur',()=>{touch.delete(button.dataset.key);updateInput();});
  }
  for(const button of document.querySelectorAll('[data-motion]'))button.onclick=()=>post({motion:Number(button.dataset.motion)});
  document.getElementById('shade').onclick=()=>post({shade:true});document.getElementById('reset').onclick=()=>post({reset:true});
  document.getElementById('pause').onclick=()=>{paused=!paused;wasm.pause_cube_example(paused);document.getElementById('pause').textContent=paused?'再開':'一時停止';document.getElementById('pause').setAttribute('aria-pressed',String(paused));status.textContent=paused?'一時停止中':'Rust版 実行中';};
  window.addEventListener('blur',()=>{keys.clear();touch.clear();pointerHeld=false;updateInput();});
  function poll(){updateInput();for(const sound of JSON.parse(wasm.cube_example_sounds()))emitSound(sound);requestAnimationFrame(poll);}poll();
  status.textContent='Rust版 実行中';report('cube-ready');preloadAudio();
}catch(error){status.textContent=`起動できません：${error.message||error}`;status.setAttribute('role','alert');report('cube-error',String(error.message||error));}
