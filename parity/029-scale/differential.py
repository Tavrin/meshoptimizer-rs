#!/usr/bin/env python3
"""Run the existing pinned C++ scale differential, fixtures plus 128 seed cases."""
import array, hashlib, json, os, random, shutil, subprocess, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
ART=Path('/mnt/linux-extra/moss-scratch/meshopt-029-scale')
REF=Path('/home/etienne/dev/refs/meshoptimizer')
assert os.environ.get('MOSS_HEAVY_ACTIVE')
sys.path.insert(0,str(ROOT/'parity'))
import runner as r
os.environ['MESHOPT_REFERENCE']=str(REF)
subprocess.run([str(ROOT/'parity/check-reference.sh')],check=True)
cpp=ART/'cpp-reference'
subprocess.run(['c++',*r.FLAGS,'-I',str(REF/'src'),str(ROOT/'parity/reference.cpp'),*[str(REF/'src'/f) for f in ['vcacheoptimizer.cpp','overdrawoptimizer.cpp','allocator.cpp','simplifier.cpp','indexgenerator.cpp','vfetchoptimizer.cpp','quantization.cpp']],'-o',str(cpp)],check=True)
subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(ROOT/'parity/Cargo.toml'),'--features','meshoptimizer-rs/std'],check=True)
rust=ART/'rust-protocol';shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release/meshopt-driver',rust)
cases=list(r.fixtures());rng=random.Random(20261002)
for i in range(128):
    p,ix,t,kind=r.random_mesh(rng,i);cases.append((f'existing-seed-20261002-{i}-{kind}',p,ix,t))
rows=[]
outputs=ART/'differential';outputs.mkdir(exist_ok=True)
for name,p,ix,t in cases:
    data=r.message(6,p,ix,t)
    a,b=r.execute(cpp,data),r.execute(rust,data)
    for label,raw in [('input',data),('cpp',a),('rust',b)]: (outputs/f'{name}.{label}').write_bytes(raw)
    assert a==b,name
    rows.append(dict(case=name,input_sha256=hashlib.sha256(data).hexdigest(),output_sha256=hashlib.sha256(a).hexdigest()))
(ART/'differential.json').write_text(json.dumps(rows,indent=2)+'\n')
print('PASS',len(rows),'existing C++ scale differential cases')
