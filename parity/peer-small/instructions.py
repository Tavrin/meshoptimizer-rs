#!/usr/bin/env python3
"""Count resident-driver instructions only; elapsed driver replies are discarded."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys

ART = Path(_public_path('<scratch>/meshopt-peer-c'))
PERF = '/usr/lib/linux-tools/6.17.0-22-generic/perf'
OPS = ['adjacency', 'optimize-meshlet', 'fetch', 'remap-indices']
CASES = ['tiny-smooth', 'medium-smooth', 'medium-seams', 'medium-sparse', 'million-smooth']
assert os.environ.get('PARITY_RUNNER_ACTIVE')
epoch = sys.argv[1]
rows = []
for binary in sys.argv[2:]:
    for op in OPS:
        for case in CASES:
            count = 100000 if op == 'optimize-meshlet' or case == 'tiny-smooth' else 2 if case.startswith('million') else 100
            stem = ART / f'instructions-{epoch}-{Path(binary).name}-{op}-{case}'
            assert not stem.with_suffix('.json').exists()
            output = stem.with_suffix('.bin')
            cr, cw = os.pipe()
            ar, aw = os.pipe()
            perf = subprocess.Popen([PERF, 'stat', '-e', 'instructions:u', '-x', ',',
                                     '-o', str(stem.with_suffix('.perf')), '--delay=-1',
                                     f'--control=fd:{cr},{aw}', '--', 'taskset', '-c', '7',
                                     binary, op, str(ART / 'inputs' / f'{case}.bin'), str(output)],
                                    pass_fds=(cr, aw), stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                    stderr=stem.with_suffix('.stderr').open('w'), text=True)
            driver = perf
            assert driver.stdout.readline().strip() == 'READY'
            os.close(cr)
            os.close(aw)
            with os.fdopen(cw, 'w') as control, os.fdopen(ar) as ack:
                control.write('enable\n'); control.flush()
                assert ack.readline().strip('\x00\n\r ') == 'ack'
                driver.stdin.write(f'{count}\n'); driver.stdin.flush()
                assert driver.stdout.readline().strip()
                control.write('disable\n'); control.flush()
                assert ack.readline().strip('\x00\n\r ') == 'ack'
            driver.stdin.write('stop\n'); driver.stdin.flush()
            driver.communicate(timeout=10)
            assert driver.returncode == 0
            raw = stem.with_suffix('.perf').read_text()
            instructions = int(next(line.split(',')[0] for line in raw.splitlines() if ',instructions:u,' in line))
            row = dict(binary=binary, operation=op, case=case, iterations=count,
                       instructions=instructions, per_iteration=instructions / count,
                       sha256=hashlib.sha256(Path(binary).read_bytes()).hexdigest(), cpu=7)
            stem.with_suffix('.json').write_text(json.dumps(row, indent=2) + '\n')
            rows.append(row)
            print(Path(binary).name, op, case, round(instructions / count), flush=True)
(ART / f'instructions-{epoch}.json').write_text(json.dumps(rows, indent=2) + '\n')
