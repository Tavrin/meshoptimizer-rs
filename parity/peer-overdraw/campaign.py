#!/usr/bin/env python3
"""One registered registered protocol job B campaign. Refuses replacement or retry."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib
import json
import math
import os
import random
import statistics
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
S = Path(_public_path('<scratch>/meshopt-peer-b'))
sys.path.insert(0, str(ROOT / 'parity'))
sys.path.insert(0, str(S / 'parity/compare'))
import measure as m


def sha(p):
    h = hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda: f.read(1 << 20), b''):
            h.update(b)
    return h.hexdigest()


def save(path, value):
    tmp = path.with_suffix('.tmp')
    tmp.write_text(json.dumps(value, indent=2) + '\n')
    tmp.replace(path)


class Driver(m.Driver):
    def __init__(self, arm, case, suffix):
        self.binary = S / f'driver-{arm}'
        self.output = S / 'wall-outputs' / f'{case}-{arm}-{suffix}.bin'
        self.output.parent.mkdir(exist_ok=True)
        self.p = subprocess.Popen(['taskset', '-c', '20', str(self.binary),
                                   'overdraw', str(S / 'inputs' / f'{case}.bin'),
                                   str(self.output)], stdin=subprocess.PIPE,
                                  stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                  text=True)
        try:
            assert self.read() == 'READY'
        except BaseException:
            self.p.kill()
            self.p.wait()
            raise


def interval(logs):
    rng = random.Random(20261029)
    means = sorted(statistics.mean(rng.choices(logs, k=len(logs))) for _ in range(10000))
    return [math.exp(means[250]), math.exp(means[9750])]


def family_interval(groups):
    rng = random.Random(20261029)
    means = sorted(statistics.mean(statistics.mean(rng.choices(g, k=len(g)))
                                   for g in groups) for _ in range(10000))
    return [math.exp(means[250]), math.exp(means[9750])]


def analyze(record):
    rows = record['rows']
    floor = max(max(0, r['ci95'][1] - 1, 1 / r['ci95'][0] - 1)
                for r in rows if r['arm'] == 'AA')
    families = {}
    for family in ['overdraw', 'real-overdraw']:
        subset = [r for r in rows if r['family'] == family]
        ratios = {}
        for arm in ['X', 'B'] + (['M7'] if family == 'real-overdraw' else []):
            group = [r for r in subset if r['arm'] == arm]
            logs = [[math.log(s['ratio']) for s in r['pairs']] for r in group]
            ratios[arm] = dict(gm=math.exp(statistics.mean(statistics.mean(g) for g in logs)),
                               ci95=family_interval(logs))
        upper = max(r['ci95'][1] for r in subset if r['arm'] == 'B')
        baseline_pass = ratios['B']['gm'] <= 1 and upper <= 1.10 + floor
        verdict = ('PASS' if ratios['X']['gm'] <= 1 else 'PARTIAL') if baseline_pass else 'FAIL'
        families[family] = dict(ratios=ratios, worst_baseline_upper95=upper,
                                no_regression_bar=1.10+floor, verdict=verdict)
    return dict(floor=floor, families=families)


def main():
    assert os.environ.get('PARITY_RUNNER_ACTIVE')
    assert os.environ.get('PARITY_RUNNER_MEASUREMENT') == '1'
    assert os.environ['CARGO_TARGET_DIR'] == _public_path('<target-dir>')
    path = S / 'wall-campaign.json'
    # Exclusive creation makes partial campaigns terminal too.
    with path.open('x') as f:
        f.write('{}\n')
    sources = sorted((ROOT / 'src').rglob('*.rs')) + [Path(__file__).resolve(), ROOT / 'parity/peer-overdraw/REGISTRATION.md', S / 'parity/compare/src/main.rs', S / 'parity/compare/src/peer_x.rs', S / 'parity/compare/Cargo.toml', S / 'parity/compare/Cargo.lock']
    hashes = {str(p): sha(p) for p in sources}
    bins = {a: sha(S / f'driver-{a}') for a in ['O', 'B', 'X', 'M7']}
    inputs = json.loads((S / 'frozen-inputs.json').read_text())
    for p in inputs:
        assert sha(S / 'inputs' / p['name']) == p['sha256']
    correctness = json.loads((S / 'frozen-exact.json').read_text())
    assert all(r['exact_baseline'] and r['exact_m7'] for r in correctness)
    record = dict(started=time.time(), source_sha256=hashes, binaries=bins,
                  inputs=inputs, cpu=20, physical_core=10, sibling=21,
                  admission_pid=os.environ['PARITY_RUNNER_ACTIVE'], rows=[], completed=False,
                  host=subprocess.check_output(['uname', '-a'], text=True).strip())
    save(path, record)
    for item in inputs:
        case = Path(item['name']).stem
        real = case.startswith('real-')
        iterations = 4 if real else 2 if case.startswith('million') else 10000 if case.startswith('tiny') else 100
        for arm in ['AA', 'X', 'B'] + (['M7'] if real else []):
            row = dict(case=case, family='real-overdraw' if real else 'overdraw',
                       arm=arm, iterations=iterations, pairs=[])
            record['rows'].append(row)
            save(path, record)
            with Driver('O', case, arm+'-a') as ours, Driver('O' if arm == 'AA' else arm, case, arm+'-b') as other:
                expected = next(r for r in correctness if r['case'] == case)
                assert sha(ours.output) == expected['O']['sha256']
                assert sha(other.output) == expected['O' if arm == 'AA' else arm]['sha256']
                # Warm-up values are not used for iteration choice or analysis.
                ours.sample(iterations)
                other.sample(iterations)
                for pair in range(10):
                    order = ['O', arm] if pair % 2 == 0 else [arm, 'O']
                    row['pending_pair'] = dict(pair=pair, order=order, ns={})
                    save(path, record)
                    for a in order:
                        row['pending_pair']['ns'][a] = (ours if a == 'O' else other).sample(iterations)
                        save(path, record)
                    ns = row.pop('pending_pair')['ns']
                    assert all(v > 0 and math.isfinite(v) for v in ns.values())
                    row['pairs'].append(dict(pair=pair, order=order, ns=ns, ratio=ns['O']/ns[arm],
                                             utc=time.time(), load=os.getloadavg()))
                    save(path, record)
            logs = [math.log(p['ratio']) for p in row['pairs']]
            row.update(gm=math.exp(statistics.mean(logs)), ci95=interval(logs), completed=True)
            save(path, record)
            print(case, arm, row['gm'], row['ci95'], flush=True)
    assert hashes == {str(p): sha(p) for p in sources}
    assert bins == {a: sha(S / f'driver-{a}') for a in bins}
    for p in inputs:
        assert sha(S / 'inputs' / p['name']) == p['sha256']
    record.update(completed=True, finished=time.time(), analysis=analyze(record))
    save(path, record)
    save(S / 'registered-results.json', record['analysis'])
    print(json.dumps(record['analysis'], indent=2), flush=True)


if __name__ == '__main__':
    main()
