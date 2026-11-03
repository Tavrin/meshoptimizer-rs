"""Download and flatten three attributed OBJ test scenes; data stays in scratch."""
import array
import collections
import hashlib
import io
import json
import math
import os
import struct
import subprocess
import zipfile
from pathlib import Path

SCRATCH = Path(os.environ['HSR_SCENES'])
RECEIPT = Path(__file__).resolve().parent
BASE = 'https://casual-effects.com/g3d/data10/research/model/'
SCENES = [
    ('breakfast_room', 'Breakfast Room', 'Wig42', 'CC BY 3.0'),
    ('living_room', 'Living Room (The White Room Cycles)', 'Jay (2012)', 'CC BY 3.0'),
    ('conference', 'Conference Room', 'Anat Grynberg and Greg Ward; reconstruction by Kenzie Lamar', 'Credit required'),
]

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def triangulate(face, positions):
    if len(face) == 3:
        return [face]
    # Project the polygon onto its dominant Newell-normal plane, then ear clip.
    # OBJ polygons may be concave: a universal triangle fan is not sufficient.
    p = [positions[v] for v in face]
    normal = [sum((a[(k+1)%3]-b[(k+1)%3])*(a[(k+2)%3]+b[(k+2)%3])
                  for a,b in zip(p, p[1:]+p[:1])) for k in range(3)]
    axis = max(range(3), key=lambda k: abs(normal[k]))
    xy = [[v[k] for k in range(3) if k != axis] for v in p]
    def cross(a,b,c):
        return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
    area = sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(xy,xy[1:]+xy[:1]))
    if area == 0:
        return [[face[0],face[j],face[j+1]] for j in range(1,len(face)-1)]
    sign = 1 if area > 0 else -1
    remaining = list(range(len(face))); result = []
    while len(remaining) > 3:
        for at,b in enumerate(remaining):
            a,c = remaining[at-1],remaining[(at+1)%len(remaining)]
            if sign*cross(xy[a],xy[b],xy[c]) < 0:
                continue
            if any(all(sign*cross(x,y,xy[q]) > 0 for x,y in
                       [(xy[a],xy[b]),(xy[b],xy[c]),(xy[c],xy[a])])
                   for q in remaining if q not in [a,b,c]):
                continue
            result.append([face[a],face[b],face[c]])
            remaining.pop(at)
            break
        else:
            raise ValueError('Non-simple OBJ polygon; refusing silent fan fallback')
    return result+[[face[v] for v in remaining]]

def main():
    SCRATCH.mkdir(parents=True, exist_ok=True)
    rows=[]
    for name,title,author,license_name in SCENES:
        url = BASE+name+'/'+name+'.zip'
        archive = SCRATCH/(name+'.zip')
        info = SCRATCH/(name+'-info.js')
        for source,dest in [(url,archive),(BASE+name+'/info.js',info)]:
            if not dest.exists():
                subprocess.run(['curl','-L','--fail','--retry','2',source+'?eval=0217','-o',str(dest)],check=True)
        positions=[]; indices=array.array('I'); groups=set(); counts=collections.Counter(); members=[]
        with zipfile.ZipFile(archive) as z:
            for member in sorted(z.namelist()):
                if member.lower().endswith('.txt'):
                    (SCRATCH/(name+'-'+Path(member).name)).write_bytes(z.read(member))
                if not member.lower().endswith('.obj'):
                    continue
                members.append(member); base=len(positions); local=0
                with io.TextIOWrapper(z.open(member),encoding='utf-8',errors='replace') as obj:
                    for line in obj:
                        fields=line.split()
                        if not fields or fields[0].startswith('#'): continue
                        if fields[0]=='v':
                            v=[float(x) for x in fields[1:4]]
                            assert len(v)==3 and all(math.isfinite(x) for x in v)
                            # Round once to the public f32 input format before triangulation.
                            positions.append(struct.unpack('<3f',struct.pack('<3f',*v))); local+=1
                        elif fields[0] in ['g','o']:
                            groups.add((member,line.strip()))
                        elif fields[0]=='f':
                            face=[]
                            for corner in fields[1:]:
                                v=int(corner.split('/')[0]); v=v-1 if v>0 else local+v
                                assert 0<=v<local
                                face.append(base+v)
                            counts[len(face)]+=1
                            for tri in triangulate(face,positions): indices.extend(tri)
        assert positions and indices and len(indices)%3==0
        source_vertices=len(positions)
        referenced=set(indices)
        remap={old:new for new,old in enumerate(sorted(referenced))}
        positions=[positions[old] for old in sorted(referenced)]
        indices=array.array('I',(remap[v] for v in indices))
        mesh=SCRATCH/(name+'.bin')
        with mesh.open('wb') as f:
            f.write(struct.pack('<II',len(positions),len(indices)))
            for p in positions: f.write(struct.pack('<3f',*p))
            if __import__('sys').byteorder!='little': indices.byteswap()
            indices.tofile(f)
        row=dict(name=name,title=title,author=author,license=license_name,url=url,
                 metadata_url=BASE+name+'/info.js',archive_sha256=sha(archive),metadata_sha256=sha(info),
                 obj_members=members,source_vertices=source_vertices,vertices=len(positions),triangles=len(indices)//3,
                 obj_groups=len(groups),source_face_sizes=dict(counts),mesh_sha256=sha(mesh),
                 transform='OBJ positions already baked; identity world transform; all groups/materials merged, unused vertices compacted, no deduplication or synthetic geometry')
        rows.append(row); print(json.dumps(row),flush=True)
    (RECEIPT/'scenes.json').write_text(json.dumps(rows,indent=2)+'\n')

if __name__=='__main__': main()
