"""Extract the complete API inventory from a checkout of the pinned upstream release."""
import argparse
import json,re
import subprocess
from pathlib import Path
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--upstream', type=Path, required=True, help='Pyxel 3.0.0 checkout')
parser.add_argument('--check', action='store_true')
args=parser.parse_args()
rows=[]
rev='e0b528f9a6f98bfc92bb47ebe1c7623cb3f609fd'
head=subprocess.check_output(['git', '-C', str(args.upstream), 'rev-parse', 'HEAD'], text=True).strip()
if head != rev:
    raise SystemExit('Expected the pinned Pyxel 3.0.0 release commit')
for filename,group in [('api-reference.md','2D / 共通'),('api-reference-cube.md','Cube / 3D')]:
    lines=(args.upstream / 'docs' / filename).read_text().splitlines()
    category=''
    for i,line in enumerate(lines):
        if line.startswith('## '): category=line[3:].replace(' Class','')
        if not line.startswith('### '): continue
        match=re.search(r'`([^`]+)`',line)
        if not match:
            # Expand documented constant lists individually, not one blanket badge.
            end=next((j for j in range(i+1,len(lines)) if lines[j].startswith('##')),len(lines))
            names=re.findall(r'`([A-Z][A-Z0-9_]+)`','\n'.join(lines[i+1:end]))
            signatures=list(dict.fromkeys(names))
        else:
            name=match[1]
            brace=re.fullmatch(r'(\w+)\.\{([^}]+)\}',name)
            signatures=[f'{brace[1]}.{x.strip()}' for x in brace[2].split(',')] if brace else [name]
        for signature in signatures:
            name=signature.split('(')[0]
            if group=='Cube / 3D' and not name.startswith(category+'.') and name!=category:
                name=category+'.'+name
            rows.append({'id':f'{filename}:{i+1}:{name}','name':name,'signature':signature,'category':category,'group':group,'reference':f'https://github.com/kitao/pyxel/blob/{rev}/docs/{filename}#L{i+1}'})
output=Path(__file__).resolve().parent / 'pyxel-3.0-api.json'
encoded=json.dumps({'upstream':rev,'version':'3.0.0','entries':rows},ensure_ascii=False,indent=2)+'\n'
if args.check:
    if output.read_text()!=encoded:
        raise SystemExit('API baseline does not match the upstream documentation')
else:
    output.write_text(encoded)
print(len(rows),'documented API entries')
