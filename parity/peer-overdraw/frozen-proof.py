#!/usr/bin/env python3
"""Existing comparator, untimed exact output check for all frozen lane B cases."""
from pathlib import Path
import hashlib
import json
import os
import subprocess
S = Path('/mnt/linux-extra/moss-scratch/meshopt-peer-b')
assert os.environ.get('MOSS_HEAVY_ACTIVE')
rows = []
for item in json.loads((S / 'frozen-inputs.json').read_text()):
    case = Path(item['name']).stem
    row = dict(case=case)
    for arm in ['O', 'B', 'M7', 'X']:
        out = S / 'proof-outputs' / f'{case}-{arm}.bin'
        out.parent.mkdir(exist_ok=True)
        subprocess.run([str(S / f'driver-{arm}'), 'overdraw', str(S / 'inputs' / item['name']), str(out), '0'], check=True)
        h = hashlib.sha256()
        with out.open('rb') as f:
            for b in iter(lambda: f.read(1 << 20), b''):
                h.update(b)
        row[arm] = dict(path=str(out), sha256=h.hexdigest(), bytes=out.stat().st_size)
    # cmp is byte-for-byte, not a digest-only comparison.
    row['exact_baseline'] = subprocess.run(['cmp', '-s', row['O']['path'], row['B']['path']]).returncode == 0
    row['exact_m7'] = subprocess.run(['cmp', '-s', row['O']['path'], row['M7']['path']]).returncode == 0
    row['exact_x'] = subprocess.run(['cmp', '-s', row['O']['path'], row['X']['path']]).returncode == 0
    rows.append(row)
    (S / 'frozen-exact.json').write_text(json.dumps(rows, indent=2) + '\n')
    assert row['exact_baseline'] and row['exact_m7'], case
    print(case, 'byte-exact baseline/M7', 'X exact:', row['exact_x'], flush=True)
