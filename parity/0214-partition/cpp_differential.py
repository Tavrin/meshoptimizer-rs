"""Exact partition assignments against the pinned scalar reference."""
import hashlib
import json
import os
from pathlib import Path
import random
import struct
import subprocess

scratch = Path(os.environ['MESHOPT_SCRATCH'])
assert os.environ.get('PARITY_RUNNER_ACTIVE')
rng = random.Random(20261027)
rows = []
for case in range(256):
    nv = rng.randrange(3, 50)
    nc = rng.randrange(1, 50)
    spatial = case % 2
    target = rng.randrange(1, 14)
    positions = [(float(rng.randrange(-8, 9)), float(rng.randrange(-8, 9)), float(rng.randrange(-8, 9))) for _ in range(nv)]
    counts = [rng.randrange(1, 16) for _ in range(nc)]
    indices = [rng.randrange(nv) for _ in range(sum(counts))]
    payload = struct.pack('<5I', nv, len(indices), nc, target, spatial)
    payload += b''.join(struct.pack('<3f', *p) for p in positions)
    payload += struct.pack(f'<{len(indices)}I', *indices) + struct.pack(f'<{nc}I', *counts)
    cpp = subprocess.run([str(scratch / 'cpp-reference')], input=payload, capture_output=True, check=True).stdout
    rust = subprocess.run([str(scratch / 'partition-cpp-compare')], input=payload, capture_output=True, check=True).stdout
    assert cpp == rust, (case, hashlib.sha256(cpp).hexdigest(), hashlib.sha256(rust).hexdigest())
    rows.append({'case': case, 'input_sha256': hashlib.sha256(payload).hexdigest(), 'output_sha256': hashlib.sha256(cpp).hexdigest()})
(scratch / 'cpp-differential.json').write_text(json.dumps(rows, indent=2) + '\n')
print('PASS', len(rows), 'C++ comparisons')
