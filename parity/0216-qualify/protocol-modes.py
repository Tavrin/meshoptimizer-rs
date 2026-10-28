"""Run the retained C++ protocol corpus against both feature modes."""
import hashlib
import json
import os
from pathlib import Path
import random
import struct
import sys

assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU') == '0'
repo = Path(__file__).resolve().parents[2]
scratch = Path(os.environ['MESHOPT_SCRATCH'])
out = scratch / 'proof/0216b-protocol'
cpp = out / 'cpp-reference'
sys.path.insert(0, str(repo / 'parity'))
import runner as r
import p01x
rng = random.Random(20261027)
cases = list(r.fixtures())
for i in range(128):
    p, ix, threshold, kind = r.random_mesh(rng, i)
    cases.append((f'seed-{i}-{kind}', p, ix, threshold))
for mode in ['all', 'none']:
    binary = out / f'rust-{mode}'
    rows = []
    for variant, (name, p, ix, threshold) in enumerate(cases):
        for op in [2, 4, 5, 24, 25, 26, 27]:
            data = r.message(op, p, ix, threshold, variant=variant) if op < 24 else p01x.message(op, p, ix, variant)
            if mode == 'none' and op >= 24:
                data = bytearray(data)
                options = struct.unpack_from('<I', data, 36)[0]
                struct.pack_into('<I', data, 36, options & ~384)
                data = bytes(data)
            a, b = r.execute(cpp, data), r.execute(binary, data)
            assert a == b, (mode, op, name)
            rows.append({'operation': op, 'case': name, 'input_sha256': hashlib.sha256(data).hexdigest(), 'output_sha256': hashlib.sha256(a).hexdigest()})
    (out / f'cases-{mode}.jsonl').write_text(''.join(json.dumps(row) + '\n' for row in rows))
    (out / f'exit-{mode}.json').write_text(json.dumps({'exit': 0, 'cases': len(rows), 'cpp_sha256': hashlib.sha256(cpp.read_bytes()).hexdigest(), 'rust_sha256': hashlib.sha256(binary.read_bytes()).hexdigest()}) + '\n')
