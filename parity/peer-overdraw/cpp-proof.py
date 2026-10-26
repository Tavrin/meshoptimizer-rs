#!/usr/bin/env python3
"""Target only overdraw using the existing pinned C++ differential and fixtures."""
from pathlib import Path
import hashlib
import json
import os
import random
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[2]
S = Path('/mnt/linux-extra/moss-scratch/meshopt-peer-b')
sys.path.insert(0, str(ROOT / 'parity'))
import runner as r
assert os.environ.get('MOSS_HEAVY_ACTIVE')
reference = Path('/home/etienne/dev/refs/meshoptimizer')
os.environ['MESHOPT_REFERENCE'] = str(reference)
r.ENV['MESHOPT_REFERENCE'] = str(reference)
r.reference_check()
cpp = S / 'cpp-reference'
cmd = ['c++', *r.FLAGS, '-I', str(reference / 'src'), str(ROOT / 'parity/reference.cpp'), *[str(reference / 'src' / p) for p in ['vcacheoptimizer.cpp', 'overdrawoptimizer.cpp', 'allocator.cpp', 'simplifier.cpp', 'indexgenerator.cpp', 'vfetchoptimizer.cpp', 'quantization.cpp']], '-o', str(cpp)]
subprocess.run(cmd, check=True)
subprocess.run(['cargo', 'build', '--manifest-path', str(ROOT / 'parity/Cargo.toml'), '--offline', '--locked', '--release', '--features', 'meshoptimizer-rs/std'], check=True)
rust = Path(os.environ['CARGO_TARGET_DIR']) / 'release/meshopt-driver'
import shutil
shutil.copy2(rust, S / 'rust-parity-std')
rust = S / 'rust-parity-std'
cases = list(r.fixtures())
rng = random.Random(20261002)
for i in range(128):
    p, ix, threshold, kind = r.random_mesh(rng, i)
    cases.append((f'existing-seed-20261002-{i}-{kind}', p, ix, threshold))
rows = []
for name, p, ix, threshold in cases:
    if len(ix):
        import array
        cached, _ = r.response(r.execute(rust, r.message(1, p, ix)), len(ix))
        ix = array.array('I'); ix.frombytes(cached)
    message = r.message(2, p, ix, threshold)
    a, b = r.execute(cpp, message), r.execute(rust, message)
    r.response(a, len(ix)); r.response(b, len(ix))
    output_dir = S / 'cpp-outputs'; output_dir.mkdir(exist_ok=True)
    # Retain the exact existing protocol input and both complete responses.
    for ext, data in [('input', message), ('cpp', a), ('rust', b)]:
        (output_dir / f'{name}.{ext}').write_bytes(data)
    rows.append(dict(case=name, exact=a == b, input_sha256=hashlib.sha256(message).hexdigest(), output_sha256=hashlib.sha256(a).hexdigest()))
    (S / 'cpp-exact.json').write_text(json.dumps(rows, indent=2) + '\n')
    assert a == b, name
print(f'PASS {len(rows)} existing overdraw differential cases', flush=True)
