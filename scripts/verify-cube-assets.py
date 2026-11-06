"""Verify source asset identity and that the baked effects contain actual audio."""
import argparse, importlib.util, wave, struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent
p=argparse.ArgumentParser(description=__doc__);p.add_argument('--upstream',type=Path);args=p.parse_args()
assets=ROOT/'src/games/cube_examples/assets'
if args.upstream:
    upstream=args.upstream/'python/pyxel/examples/cube/assets'
    for glb in assets.glob('*.glb'):
        assert glb.read_bytes()==(upstream/glb.name).read_bytes(),glb.name
    spec=importlib.util.spec_from_file_location('textures',ROOT/'scripts/prepare-cube-textures.py');textures=importlib.util.module_from_spec(spec);spec.loader.exec_module(textures)
    for name in ['cat_16x16','laser']:
        _,_,pixels=textures.decode((upstream/f'{name}.png').read_bytes())
        assert pixels==(assets/f'{name}.bin').read_bytes(),name
sounds=list((ROOT/'docs/examples/cube/audio').glob('*.wav'))
assert len(sounds)==8
for sound in sounds:
    with wave.open(str(sound)) as audio:
        assert audio.getnchannels()==1 and audio.getsampwidth()==2 and audio.getframerate()==22050,sound.name
        samples=struct.unpack('<'+'h'*audio.getnframes(),audio.readframes(audio.getnframes()))
        assert samples and max(abs(x) for x in samples)>100,sound.name
print('Seven original GLBs / two decoded textures checked' if args.upstream else 'Source comparison skipped (pass --upstream)')
print('Eight finite WAV effects contain non-silent 16-bit audio')
