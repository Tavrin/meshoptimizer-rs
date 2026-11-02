"""Fetch pinned MIT test meshes; downloaded inputs remain outside the repository."""
import urllib.request,json,hashlib,struct,sys
from pathlib import Path
out=Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=True)
repo='nmwsharp/geometry-central'
pin=json.load(urllib.request.urlopen(f'https://api.github.com/repos/{repo}/commits/master'))['sha']
license_url=f'https://raw.githubusercontent.com/{repo}/{pin}/LICENSE'
license_text=urllib.request.urlopen(license_url).read().decode()
assert 'MIT' in license_text
(out/'LICENSE').write_text(license_text)
manifest=[]
for name in ['hourglass_ico','fan3','triple_vierbein','platonic_shelf']:
 url=f'https://raw.githubusercontent.com/{repo}/{pin}/test/assets/{name}.obj'
 raw=urllib.request.urlopen(url).read();(out/(name+'.obj')).write_bytes(raw)
 vertices=[];indices=[]
 for line in raw.decode().splitlines():
  parts=line.split()
  if parts and parts[0]=='v': vertices.append([float(v) for v in parts[1:4]])
  if parts and parts[0]=='f':
   face=[int(v.split('/')[0])-1 for v in parts[1:]]
   for j in range(1,len(face)-1):indices.extend([face[0],face[j],face[j+1]])
 with (out/(name+'.bin')).open('wb') as f:
  f.write(struct.pack('<II',len(vertices),len(indices)))
  for v in vertices:f.write(struct.pack('<3f',*v))
  for i in indices:f.write(struct.pack('<I',i))
 manifest.append(dict(name=name,url=url,license='MIT',license_url=license_url,sha256=hashlib.sha256(raw).hexdigest(),vertices=len(vertices),triangles=len(indices)//3,conversion='OBJ polygons triangulated as a fan, position indices retained'))
Path('parity/0217-repair4/receipts/corpus.json').write_text(json.dumps(manifest,indent=2)+'\n')
