#!/usr/bin/env python3
"""Single registered campaign; fail closed on an existing campaign receipt."""

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
import math
import os
from pathlib import Path
import random
import statistics
import subprocess
import sys

ART = Path(_public_path("<scratch>/meshopt-peer-c"))
OPS = ["adjacency", "optimize-meshlet", "fetch", "remap-indices"]
CASES = ["tiny-smooth", "medium-smooth", "medium-seams", "medium-sparse", "million-smooth"]

assert os.environ.get('MOSS_HEAVY_MEASUREMENT') == '1'
OUT = ART / 'campaign.json'
OUT.open('x').close()
candidate, base, peer = sys.argv[1:]
record = dict(registration_sha256=hashlib.sha256(Path(__file__).resolve().with_name('REGISTRATION.md').read_bytes()).hexdigest(),
              cpu=7, pairs=10, seed=20261029, bootstrap=10000, rows=[],
              binaries={p: hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in sys.argv[1:]})


def save():
    OUT.write_text(json.dumps(record, indent=2) + '\n')


class Driver:
    def __init__(self, binary, op, case, tag):
        output = ART / f'wall-{tag}-{op}-{case}.bin'
        self.p = subprocess.Popen(['taskset', '-c', '7', binary, op,
                                   str(ART / 'inputs' / f'{case}.bin'), str(output)],
                                  stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        assert self.p.stdout.readline().strip() == 'READY'

    def sample(self, n):
        self.p.stdin.write(f'{n}\n'); self.p.stdin.flush()
        value = float(self.p.stdout.readline())
        assert value > 0
        return value

    def close(self):
        self.p.stdin.write('stop\n'); self.p.stdin.flush()
        self.p.communicate(timeout=30)
        assert self.p.returncode == 0


def gm(values):
    return math.exp(statistics.mean(math.log(v) for v in values))


def interval(groups):
    rng = random.Random(20261029)
    logs = [[math.log(v) for v in group] for group in groups]
    values = sorted(math.exp(statistics.mean(statistics.mean(rng.choices(g, k=len(g)))
                                            for g in logs)) for _ in range(10000))
    return [values[249], values[9749]]


for comparison, left, right in [('AA', base, base), ('X', candidate, peer), ('base', candidate, base)]:
    for op in OPS:
        for case in CASES:
            a, b = Driver(left, op, case, comparison + '-a'), Driver(right, op, case, comparison + '-b')
            calibration = [a.sample(1), b.sample(1)]
            n = max(1, min(1000000, math.ceil(25000000 / min(calibration))))
            a.sample(n); b.sample(n)
            row = dict(comparison=comparison, operation=op, case=case, iterations=n,
                       calibration_ns=calibration, samples=[])
            record['rows'].append(row); save()
            for pair in range(10):
                order = [0, 1] if pair % 2 == 0 else [1, 0]
                times = [0, 0]
                for arm in order:
                    times[arm] = [a, b][arm].sample(n)
                row['samples'].append(dict(pair=pair, order=order, ns=times, ratio=times[0] / times[1]))
                save()
            a.close(); b.close()
            ratios = [s['ratio'] for s in row['samples']]
            row.update(gm=gm(ratios), ci95=interval([ratios])); save()
            print(comparison, op, case, row['gm'], row['ci95'], flush=True)

record['F'] = max(abs(bound - 1) for row in record['rows'] if row['comparison'] == 'AA' for bound in row['ci95'])
record['families'] = []
for op in OPS:
    groups = {label: [r for r in record['rows'] if r['comparison'] == label and r['operation'] == op]
              for label in ['X', 'base']}
    xgm, bgm = (gm(r['gm'] for r in groups[label]) for label in ['X', 'base'])
    upper = max(r['ci95'][1] for r in groups['base'])
    passes_base = bgm <= 1 and upper <= 1.10 + record['F']
    verdict = ('PASS' if xgm <= 1 else 'PARTIAL') if passes_base else 'FAIL'
    record['families'].append(dict(operation=op, ours_X=xgm, ours_base=bgm, max_upper95=upper,
                                   ci95={k: interval([[s['ratio'] for s in r['samples']] for r in rows])
                                         for k, rows in groups.items()}, verdict=verdict))
record['completed'] = True
save()
print(json.dumps(dict(F=record['F'], families=record['families']), indent=2))
