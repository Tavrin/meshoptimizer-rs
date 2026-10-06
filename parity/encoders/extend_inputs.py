#!/usr/bin/env python3
"""Append frozen cook-like geometry without changing any existing fixture."""
import json,struct,sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent));import run as lane
A=lane.ART;OLD=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp');rows=json.loads((A/'inputs.json').read_text());assert not (A/'timing.json').exists()
assert not any('geometry' in r['name'] for r in rows),'already extended'
for row in rows:assert lane.sha(Path(row['path']))==row['sha256']
metadata=json.loads((OLD/'inputs.json').read_text())
def add(name,b):
 p=A/'inputs'/(name+'.input');assert not p.exists();p.write_bytes(b);rows.append({'name':name,'path':str(p),'sha256':lane.sha(p)})
for name in ['medium-smooth','medium-seams','medium-sparse','million-smooth']:
 entry=next(x for x in metadata if x['name']==name);p=OLD/'inputs'/(name+'.bin');assert lane.sha(p)==entry['sha256'];b=p.read_bytes();n,ni=struct.unpack_from('<II',b);raw=b[8:8+n*12];indices=b[8+n*12:8+n*12+ni*4]
 configs=[(0,2),(1,0),(1,1),(1,2),(1,3)] if name!='million-smooth' else [(0,2),(1,2)]
 for v,l in configs:add(f'vertex-geometry-{name}-v{v}-l{l}',lane.request(11,n,12,raw,version=v,level=l))
 for op,family in [(12,'index'),(13,'sequence')]:
  for v in [0,1]:add(f'{family}-geometry-{name}-v{v}',lane.request(op,ni,4,indices,version=v))
(A/'inputs-before-extension.json').write_bytes((A/'inputs.json').read_bytes());lane.save('inputs',rows);print('extended inputs',len(rows),flush=True)
