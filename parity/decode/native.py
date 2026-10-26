#!/usr/bin/env python3
"""One final campaign using bet-0 resident driver/inputs and pairing rules."""
import hashlib, json, math, os, pathlib, select, statistics, subprocess, time
ART = pathlib.Path('/mnt/linux-extra/moss-scratch/meshopt-decode')
INPUT = pathlib.Path('/mnt/linux-extra/moss-scratch/meshopt-peer-screen/campaign-20261025T150615Z/inputs')
CASES = ['tiny-smooth', 'medium-smooth', 'medium-seams', 'medium-sparse', 'million-smooth']
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_MEASUREMENT') == '1'
OUT = ART / 'native-timing.json'
assert not OUT.exists(), 'do not rerun a verdict'

def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def interval(ratios):
    logs = list(map(math.log, ratios))
    mean = statistics.mean(logs)
    t = {5: 2.776445105, 20: 2.093024054}[len(logs)]
    half = t * statistics.stdev(logs) / math.sqrt(len(logs))
    return [math.exp(mean-half), math.exp(mean+half)]
class Driver:
    def __init__(self, arm, op, case, suffix):
        self.out = ART / f'output-{arm}-{op}-{case}-{suffix}.bin'
        self.p = subprocess.Popen(['taskset', '-c', '2', str(ART/arm), op, str(INPUT/(case+'.bin')), str(self.out)],
                                  stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        assert self.read() == 'READY'
    def read(self):
        if not select.select([self.p.stdout], [], [], 180)[0]: raise TimeoutError('driver timeout')
        line = self.p.stdout.readline()
        if not line: raise RuntimeError(self.p.stderr.read())
        return line.strip()
    def sample(self, n):
        self.p.stdin.write(str(n)+'\n'); self.p.stdin.flush()
        return float(self.read())
    def close(self):
        self.p.stdin.write('stop\n'); self.p.stdin.flush(); self.p.communicate(timeout=180)
        assert self.p.returncode == 0
rows = []
identity = {a: sha(ART/a) for a in ['before', 'after', 'm7']}
for phase, ops, arms in [('AA', ['oct-decode','quat-decode','vertex-decode'], ['before','before']),
                         ('M7', ['oct-decode','quat-decode'], ['after','m7']),
                         ('old', ['vertex-decode'], ['after','before'])]:
    for case in CASES:
        for op in ops:
            ds = [Driver(a, op, case, f'{phase}-{i}') for i,a in enumerate(arms)]
            assert ds[0].out.read_bytes() == ds[1].out.read_bytes(), (phase,case,op,'byte mismatch')
            cal = [d.sample(1) for d in ds]
            iterations = max(1, min(1000000, math.ceil(25000000/min(cal))))
            row = dict(phase=phase, case=case, operation=op, arms=arms, cpu=2, samples=[], calibration_ns=cal,
                       iterations=iterations, input_sha256=sha(INPUT/(case+'.bin')), output_sha256=sha(ds[0].out), binaries=identity)
            rows.append(row)
            for pair in range(20):
                if pair >= 5 and not row['five_pair_ci'][0] <= 1 <= row['five_pair_ci'][1]: break
                order = [0,1] if pair%2 == 0 else [1,0]
                ns = {}
                for i in order: ns[i] = ds[i].sample(iterations)
                row['samples'].append(dict(pair=pair, order=order, ns=ns, ratio=ns[0]/ns[1], utc=time.time(), load=os.getloadavg()))
                if pair == 4: row['five_pair_ci'] = interval([s['ratio'] for s in row['samples']])
                OUT.write_text(json.dumps(rows, indent=2)+'\n')
            for d in ds: d.close()
            ratios=[s['ratio'] for s in row['samples']]
            row.update(gm=math.exp(statistics.mean(map(math.log,ratios))), ci95=interval(ratios), completed=True)
            OUT.write_text(json.dumps(rows, indent=2)+'\n')
            print(phase, case, op, row['gm'], row['ci95'], flush=True)
