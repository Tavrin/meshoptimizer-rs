"""Compare frozen and seeded inputs against the pinned C++ reference."""
from pathlib import Path
import hashlib
import json
import os
import random
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_SCRATCH'])
REF = Path(os.environ['MESHOPT_REFERENCE'])
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
sys.path.insert(0, str(ROOT / 'parity'))
import runner as r
import p01x
subprocess.run(['bash', str(ROOT / 'parity/check-reference.sh')], check=True)
cpp = ART / 'cpp-reference'
subprocess.run(['c++', *r.FLAGS, '-I', str(REF / 'src'), str(ROOT / 'parity/reference.cpp'),
                *[str(REF / 'src' / f) for f in ['vcacheoptimizer.cpp', 'overdrawoptimizer.cpp', 'allocator.cpp',
                  'simplifier.cpp', 'indexgenerator.cpp', 'vfetchoptimizer.cpp', 'quantization.cpp']], '-o', str(cpp)], check=True)
rust = ART / 'rust-protocol'
assert rust.is_file(), 'build and retain the protocol executable before running this differential'
rng = random.Random(20261027)
cases = list(r.fixtures())
for i in range(128):
    p, ix, threshold, kind = r.random_mesh(rng, i)
    cases.append((f'seed-{i}-{kind}', p, ix, threshold))
rows = []
for variant, (name, p, ix, threshold) in enumerate(cases):
    for op in [2, 4, 5, 24, 25, 26, 27]:
        data = r.message(op, p, ix, threshold, variant=variant) if op < 24 else p01x.message(op, p, ix, variant)
        a, b = r.execute(cpp, data), r.execute(rust, data)
        assert a == b, (op, name)
        rows.append(dict(operation=op, case=name, input_sha256=hashlib.sha256(data).hexdigest(), output_sha256=hashlib.sha256(a).hexdigest()))
        with (ART / 'cpp-differential.jsonl').open('a') as f:
            f.write(json.dumps(rows[-1]) + '\n')
print('PASS', len(rows), 'pinned C++ comparisons')
