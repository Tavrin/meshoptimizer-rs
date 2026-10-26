#!/usr/bin/env python3
"""Fixed-call instruction probes. No wall-clock iteration/calibration."""
import hashlib, json, os, pathlib, subprocess, sys
ART = pathlib.Path('/mnt/linux-extra/moss-scratch/meshopt-decode')
INPUT = pathlib.Path('/mnt/linux-extra/moss-scratch/meshopt-peer-screen/campaign-20261025T150615Z/inputs')
assert os.environ.get('MOSS_HEAVY_ACTIVE')
phase = sys.argv[1]
codec = '--codec' in sys.argv
rows = []
for arm in (['before', 'm7'] if phase == 'before' else [phase]):
    binary = ART / arm
    for case in ['medium-smooth', 'medium-seams', 'medium-sparse']:
        for op in (['vertex-decode', 'index-decode'] if codec else ['oct-decode', 'quat-decode']):
            row = dict(arm=arm, case=case, operation=op, calls=100,
                       binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
            values = {}
            for n in [0, 100]:
                receipt = ART / f'instructions-{arm}-{case}-{op}-{n}.txt'
                assert not receipt.exists()
                cmd = [os.environ.get('PERF', '/usr/lib/linux-tools/6.17.0-22-generic/perf'), 'stat', '-x', ';', '-e', 'instructions:u', '-o', str(receipt), '--',
                       'taskset', '-c', '2', str(binary), op, str(INPUT / (case + '.bin')), '/dev/null', str(n)]
                r = subprocess.run(cmd, capture_output=True, text=True)
                row[f'exit_{n}'] = r.returncode
                if r.returncode:
                    row['error'] = r.stderr
                    continue
                values[n] = int(next(l.split(';')[0] for l in receipt.read_text().splitlines() if ';instructions:u;' in l))
            if len(values) == 2:
                row['raw'] = values
                row['instructions_per_call'] = (values[100] - values[0]) / 100
            rows.append(row)
            (ART / f'instructions-{phase}{"-codec" if codec else ""}.json').write_text(json.dumps(rows, indent=2) + '\n')
            print(row, flush=True)
