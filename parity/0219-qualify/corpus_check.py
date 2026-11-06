"""Verify the benchmark's used inputs against the retained corpus manifest."""
import hashlib,json,os
from pathlib import Path
A=Path(__file__).resolve().parent;R=A.parents[1];root=Path(os.environ['VG_CORPUS'])
expected={line.split()[1]:line.split()[0] for line in (R/'parity/vg-bench/corpus.sha256').read_text().splitlines() if line.strip() and not line.startswith('#')}
names=['bunny.ply','dragon.ply','dead_tree_trunk_02.gltf','dead_tree_trunk_02.bin','namaqualand_boulder_02.gltf','namaqualand_boulder_02.bin','coast_rocks_01.gltf','coast_rocks_01.bin','flighthelmet.gltf','FlightHelmet.bin'];rows=[]
for name in names:
 with (root/name).open('rb') as f:actual=hashlib.file_digest(f,'sha256').hexdigest()
 rows.append(dict(file=name,expected=expected.get(name),actual=actual,equal=expected.get(name)==actual))
(A/'corpus-check.json').write_text(json.dumps(rows,indent=2)+'\n');print(json.dumps(rows,indent=2));assert all(r['equal'] for r in rows)
