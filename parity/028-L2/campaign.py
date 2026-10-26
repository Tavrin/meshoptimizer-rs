#!/usr/bin/env python3
"""The sole registered wall campaign; exclusive receipt forbids reruns."""
import hashlib, json, math, os, random, statistics, subprocess, sys, time
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
ART = Path('/mnt/linux-extra/moss-scratch/meshopt-028-L2')
OPS = ['cache','cache-fifo','simplify-scale','overdraw']
CASES = ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']
CPU = 9
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_MEASUREMENT') == '1'
assert json.loads((ART/'proof.json').read_text())['passed']
frozen = json.loads((ART/'parity/compare/results.json').read_text())['outputs']
inputs = json.loads((ART/'inputs-sha256.json').read_text())
for case in CASES:
    expected = next(r['input_sha256'] for r in frozen if r['case']==case and r['operation']=='vertex-decode')
    assert inputs[case+'.bin']==expected
path = ART/'campaign.json'
path.open('x').close()
binaries = dict(zip(['candidate','base','M7','X'],sys.argv[1:]))
def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for chunk in iter(lambda:f.read(1<<20),b''): h.update(chunk)
    return h.hexdigest()
def source(): return {str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'src').rglob('*.rs')}
def harness(): return {str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'parity/028-L2').iterdir() if p.suffix in ['.py','.rs','.sh','.cpp']}
record = dict(started=time.time(),completed=False,source=source(),harness_sources=harness(),registration_sha256=sha(ROOT/'parity/028-L2/REGISTRATION.md'),binaries={a:dict(path=p,sha256=sha(p)) for a,p in binaries.items()},cpu=CPU,thread_siblings=Path(f'/sys/devices/system/cpu/cpu{CPU}/topology/thread_siblings_list').read_text().strip(),rows=[],stages=[],seed=20261030,bootstrap=10000)
def save():
    temporary = path.with_suffix('.tmp'); temporary.write_text(json.dumps(record,indent=2)+'\n'); temporary.replace(path)
def gm(values): return math.exp(statistics.mean(math.log(v) for v in values))
def interval(groups):
    rng = random.Random(20261030)
    logs = [[math.log(v) for v in g] for g in groups]
    values = sorted(math.exp(statistics.mean(statistics.mean(rng.choices(g,k=len(g))) for g in logs)) for _ in range(10000))
    return [values[249],values[9749]]
class Driver:
    def __init__(self,binary,op,case,tag):
        output = ART/f'wall-{tag}-{op}-{case}.bin'
        self.p = subprocess.Popen(['taskset','-c',str(CPU),binary,op,str(ART/'inputs'/f'{case}.bin'),str(output)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        assert self.p.stdout.readline().strip() == 'READY'
        self.hash = sha(output)
    def sample(self,n):
        self.p.stdin.write(f'{n}\n'); self.p.stdin.flush()
        value = float(self.p.stdout.readline()); assert value > 0; return value
    def close(self):
        self.p.stdin.write('stop\n'); self.p.stdin.flush(); self.p.communicate(timeout=60); assert self.p.returncode == 0
save()
for label,left,right in [('AA','base','base'),('M7','candidate','M7'),('base','candidate','base'),('X','candidate','X')]:
    for op in OPS:
        for case in CASES + (['real-214e-overdraw','real-b6fd-overdraw','real-a082-overdraw'] if op == 'overdraw' else []):
            a,b = Driver(binaries[left],op,case,label+'-a'),Driver(binaries[right],op,case,label+'-b')
            # X is disclosed and may differ; the baseline and M7 must be exact.
            if label != 'X': assert a.hash == b.hash, (label,op,case)
            calibration = [a.sample(1), b.sample(1)]
            n = max(1,min(1000000,math.ceil(25000000/min(calibration))))
            warmup = calibration  # The sole warm-up pair also calibrates N.
            row = dict(comparison=label,operation=op,case=case,iterations=n,calibration_ns=calibration,warmup_ns=warmup,warmup_iterations=1,hashes=[a.hash,b.hash],samples=[])
            record['rows'].append(row); save()
            for pair in range(10):
                order = [0,1] if pair % 2 == 0 else [1,0]
                times = [0,0]
                for arm in order: times[arm] = [a,b][arm].sample(n)
                row['samples'].append(dict(pair=pair,order=order,ns=times,ratio=times[0]/times[1],load=os.getloadavg(),utc=time.time())); save()
            a.close(); b.close()
            values = [s['ratio'] for s in row['samples']]
            row.update(gm=gm(values),ci95=interval([values])); save()
            print(label,op,case,row['gm'],row['ci95'],flush=True)
record['F'] = max(abs(bound-1) for row in record['rows'] if row['comparison']=='AA' for bound in row['ci95'])
record['families'] = []
for op in OPS:
    groups = {label:[r for r in record['rows'] if r['comparison']==label and r['operation']==op] for label in ['M7','base','X']}
    ratios = {label:gm(r['gm'] for r in rows) for label,rows in groups.items()}
    upper = max(r['ci95'][1] for r in groups['base'])
    no_regression = ratios['base'] <= 1 and upper <= 1.10+record['F']
    verdict = ('PASS' if ratios['M7'] <= 1 else 'PARTIAL') if no_regression else 'FAIL'
    record['families'].append(dict(operation=op,ratios=ratios,max_baseline_upper95=upper,ci95={label:interval([[s['ratio'] for s in r['samples']] for r in rows]) for label,rows in groups.items()},verdict=verdict))
save()
# Real disclosure: all immutable r2 stage calls, ordinary and legacy, sequential
# processes keep the entire admitted group within 4 GiB. No launch/I/O timing.
for trace in json.loads((ART/'stages.json').read_text()):
    assert sha(trace['manifest']) == trace['manifest_sha256']
    for call in trace['calls']:
        if call.get('input_sha256'): assert sha(call['input_path']) == call['input_sha256']
        assert sha(call['vertex_path']) == call['vertex_sha256']
    for label,left,right in [('M7','candidate','M7'),('base-M7','base','M7')]:
        row = dict(case=trace['case'],comparison=label,samples=[],capture_sha256=trace['capture_sha256'],manifest_sha256=trace['manifest_sha256'])
        record['stages'].append(row); save()
        for pair in range(-1,10):
            order = [left,right] if pair % 2 == 0 else [right,left]
            arms = {}
            for arm in order:
                folder = ART/'stage-campaign'/trace['case']/f'{label}-{pair+1:02d}-{arm}'
                folder.mkdir(parents=True)
                output = folder/'stages.tsv'
                command = ['taskset','-c',str(CPU),binaries[arm],'chain',trace['manifest'],str(output)]
                with (folder/'stderr.log').open('w') as log:
                    p = subprocess.run(command,stdout=log,stderr=subprocess.STDOUT)
                assert p.returncode == 0, str(folder)
                values = {op:0 for op in ['simplify-scale','simplify','cache','overdraw']}
                calls = []
                for line,expected in zip(output.read_text().splitlines(),trace['calls'],strict=True):
                    index,op,ns,file = line.split('\t')
                    digest = sha(file)
                    assert op == expected['operation'] and digest == expected['output_sha256'],(folder,index,op,digest,expected)
                    values[op] += int(ns)
                    calls.append(dict(index=int(index),operation=op,ns=int(ns),sha256=digest))
                    # Exact output bytes already exist in the immutable r2
                    # sidecars; retain their hash instead of duplicate gigabytes.
                    Path(file).unlink()
                arms[arm] = dict(ns=values,calls=calls)
                print('stage',trace['case'],label,pair,arm,values,flush=True)
            sample = dict(pair=pair,order=order,arms=arms,ratios={op:arms[left]['ns'][op]/arms[right]['ns'][op] for op in arms[left]['ns']})
            if pair == -1: row['warmup'] = sample
            else: row['samples'].append(sample)
            save()
        row['ratios'] = {op:dict(gm=gm(s['ratios'][op] for s in row['samples']),ci95=interval([[s['ratios'][op] for s in row['samples']]])) for op in ['simplify-scale','simplify','cache','overdraw']}; save()
assert source() == record['source']
assert harness() == record['harness_sources']
assert all(sha(p['path'])==p['sha256'] for p in record['binaries'].values())
record.update(completed=True,finished=time.time()); save()
print(json.dumps(dict(F=record['F'],families=record['families'],stages=[{k:r[k] for k in ['case','comparison','ratios']} for r in record['stages']]),indent=2))
