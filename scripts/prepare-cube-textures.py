"""Decode the official PNG textures to Pyxel color indices (standard library only)."""
import argparse, struct, zlib, random, math
from pathlib import Path
PALETTE = [0x000000,0x2b335f,0x7e2072,0x19959c,0x8b4852,0x395c98,0xa9c1ff,0xeeeeee,0xd4186c,0xd38441,0xe9c35b,0x70c6a9,0x7696de,0xa3a3a3,0xff9798,0xedc7b0]
def decode(data):
    p=8; compressed=b''; palette=[]; alpha=b''
    while p<len(data):
        n=struct.unpack_from('>I',data,p)[0]; kind=data[p+4:p+8]; chunk=data[p+8:p+8+n];p+=n+12
        if kind==b'IHDR': w,h,depth,color,_,_,interlace=struct.unpack('>IIBBBBB',chunk)
        elif kind==b'PLTE': palette=[tuple(chunk[i:i+3]) for i in range(0,len(chunk),3)]
        elif kind==b'tRNS': alpha=chunk
        elif kind==b'IDAT': compressed+=chunk
    assert depth==8 and interlace==0 and color in (2,3,6)
    bpp={2:3,3:1,6:4}[color]; raw=zlib.decompress(compressed); previous=[0]*(w*bpp); output=bytearray(); offset=0
    for _ in range(h):
        filt=raw[offset]; offset+=1; row=list(raw[offset:offset+w*bpp]);offset+=w*bpp
        for x in range(len(row)):
            a=row[x-bpp] if x>=bpp else 0;b=previous[x];c=previous[x-bpp] if x>=bpp else 0
            if filt==1: predictor=a
            elif filt==2: predictor=b
            elif filt==3: predictor=(a+b)//2
            elif filt==4:
                q=a+b-c; choices=[abs(q-a),abs(q-b),abs(q-c)];predictor=[a,b,c][choices.index(min(choices))]
            else: predictor=0
            row[x]=(row[x]+predictor)&255
        for x in range(w):
            pixel=row[x*bpp:(x+1)*bpp]
            rgb=palette[pixel[0]] if color==3 else pixel[:3]
            transparent=(color==6 and pixel[3]<128) or (color==3 and pixel[0]<len(alpha) and alpha[pixel[0]]<128)
            output.append(0 if transparent else min(range(16),key=lambda i:sum((rgb[k]-((PALETTE[i]>>(16-8*k))&255))**2 for k in range(3))))
        previous=row
    return w,h,output
if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--upstream',type=Path,required=True);a=p.parse_args()
    out=Path(__file__).resolve().parent.parent/'src/games/cube_examples/assets'
    for name in ['cat_16x16','laser']:
        w,h,data=decode((a.upstream/'python/pyxel/examples/cube/assets'/f'{name}.png').read_bytes());(out/f'{name}.bin').write_bytes(data);print(name,w,h)

    placement=random.Random(8); heading=random.Random(42); tufts=[]
    for _ in range(34):
        x,z=placement.uniform(-133,133),placement.uniform(-133,133)
        height=placement.uniform(6.3,10.5)
        mirror=1.0 if math.cos(heading.uniform(0,math.tau))>=0 else -1.0
        if abs(x)>=63: tufts.append((x,z,height,mirror))
    (out/'tufts.rs').write_text('pub const TUFTS: &[(f32,f32,f32,f32)] = &[\n'+''.join('    ('+','.join(f'{x:.8f}' for x in values)+'),\n' for values in tufts)+'];\n')
