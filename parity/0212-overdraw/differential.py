"""Existing pinned scalar C++ differential for simplify and attributes."""
from pathlib import Path
import hashlib
import json
import os
import random
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
REF = Path(os.environ['MESHOPT_REFERENCE'])
assert os.environ.get('MESHOPT_ADMISSION_ID')
sys.path.insert(0, str(ROOT / 'parity'))
import runner as r
os.environ['MESHOPT_REFERENCE'] = str(REF)
subprocess.run(['bash', str(ROOT / 'parity/check-reference.sh')], check=True)
cpp = ART / 'cpp-reference'
subprocess.run(['c++', *r.FLAGS, '-I', str(REF / 'src'), str(ROOT / 'parity/reference.cpp'),
                *[str(REF / 'src' / f) for f in ['vcacheoptimizer.cpp', 'overdrawoptimizer.cpp', 'allocator.cpp',
                  'simplifier.cpp', 'indexgenerator.cpp', 'vfetchoptimizer.cpp', 'quantization.cpp']], '-o', str(cpp)], check=True)
subprocess.run(['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path',
                str(ROOT / 'parity/Cargo.toml'), '--features', 'meshoptimizer-rs/std'], check=True)
rust = ART / 'rust-protocol'
shutil.copy2(Path(os.environ['CARGO_TARGET_DIR']) / 'release/meshopt-driver', rust)
cases = list(r.fixtures())
rng = random.Random(20261026)
for i in range(128):
    p, ix, t, kind = r.random_mesh(rng, i)
    cases.append((f'seed-20261026-{i}-{kind}', p, ix, t))
outputs = ART / 'cpp-outputs'
outputs.mkdir()
rows = []
for variant, (name, p, ix, t) in enumerate(cases):
    for op in [2]:
        data = r.message(op, p, ix, t, variant=variant)
        a, b = r.execute(cpp, data), r.execute(rust, data)
        for label, value in [('input', data), ('cpp', a), ('rust', b)]:
            (outputs / f'{op}-{name}.{label}').write_bytes(value)
        assert a == b, (op, name)
        rows.append(dict(operation=op, case=name, input_sha256=hashlib.sha256(data).hexdigest(), output_sha256=hashlib.sha256(a).hexdigest()))
(ART / 'cpp-differential.json').write_text(json.dumps(rows, indent=2) + '\n')
print('PASS', len(rows), 'pinned C++ overdraw comparisons')
