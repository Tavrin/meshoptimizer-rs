"""Fetch pinned Thingi10K STL meshes and write indexed records; downloads stay outside the repository.

usage: thingi.py OUTPUT_DIR
STL triangle soup is indexed by exact float32 position equality only (no tolerance),
so cracks and non-manifold topology present in the files are preserved.
"""
import csv, hashlib, json, struct, sys, urllib.request
from pathlib import Path
out = Path(sys.argv[1]); out.mkdir(parents=True, exist_ok=True)
rev = '2d5d3b2f3cd3711028ad75b12788c13b25559ec6'  # Hugging Face dataset revision, 2026-08-12
base = f'https://huggingface.co/datasets/Thingi10K/Thingi10K/resolve/{rev}'
# (file id, thing id): selected from metadata/geometry_data.csv for CC BY, vertex- or edge-non-manifold, open boundaries.
chosen = [(170991, 71637), (90248, 30971), (83642, 28240), (66345, 21294), (78660, 26311)]
def get(url): return urllib.request.urlopen(urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})).read()
ctx = {r['Thing ID']: r for r in csv.DictReader(get(f'{base}/metadata/contextual_data.csv').decode().splitlines())}
def triangles(raw):
    if raw[:5] == b'solid' and b'facet' in raw[:400]:
        v = [[float(x) for x in l.split()[1:4]] for l in raw.decode(errors='replace').splitlines() if l.split()[:1] == ['vertex']]
        return [v[i:i + 3] for i in range(0, len(v) - 2, 3)]
    n, = struct.unpack_from('<I', raw, 80)
    return [[struct.unpack_from('<3f', raw, 84 + 50 * t + 12 + 12 * k) for k in range(3)] for t in range(n)]
manifest = []
for fid, tid in chosen:
    url = f'{base}/raw_meshes/{fid}.stl'
    raw = get(url)
    ids = {}; verts = []; idx = []
    for tri in triangles(raw):
        for p in tri:
            key = struct.pack('<3f', *p)
            if key not in ids:
                ids[key] = len(verts); verts.append(key)
            idx.append(ids[key])
    with (out / f'thingi{fid}.bin').open('wb') as f:
        f.write(struct.pack('<II', len(verts), len(idx)))
        for v in verts: f.write(v)
        for i in idx: f.write(struct.pack('<I', i))
    c = ctx[str(tid)]
    manifest.append(dict(name=f'thingi{fid}', title=c['Name'].strip(), author=c['Author'], license=c['License'],
                         thing=f'https://www.thingiverse.com/thing:{tid}', url=url, sha256=hashlib.sha256(raw).hexdigest(),
                         vertices=len(verts), triangles=len(idx) // 3,
                         conversion='STL triangles indexed by exact float32 position equality'))
Path('parity/0217-repair4/receipts/thingi.json').write_text(json.dumps(manifest, indent=2) + '\n')
