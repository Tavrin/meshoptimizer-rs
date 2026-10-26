#!/usr/bin/env python3
"""Single registered SPEC-027 A campaign; refuse an existing result stream."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib,json,math,os,random,select,statistics,subprocess,sys,time
S=Path(os.environ.get('PEER_SCRATCH',_public_path('<scratch>/meshopt-peer-a')))
ROOT=Path(__file__).resolve().parents[2]
assert os.environ.get('MOSS_HEAVY_ACTIVE')
assert os.environ.get('MOSS_HEAVY_MEASUREMENT')=='1'
assert not (S/'wall.json').exists(),'exactly one campaign; no reruns'
sys.path.insert(0,str(ROOT/'parity'))
import quiet
core=quiet.physical_core();cpu=core['cpu']
names=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']
ops=['quantize-half','quantize-float']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def gm(xs):return math.exp(statistics.mean(math.log(x)for x in xs))
def interval(xs):
    rng=random.Random(20261029);logs=[math.log(x)for x in xs];n=len(logs)
    v=sorted(math.exp(sum(rng.choices(logs,k=n))/n)for _ in range(10000))
    return [v[249],v[9749]]
def family_interval(cases):
    rng=random.Random(20261029);logs=[[math.log(x)for x in xs]for xs in cases]
    v=sorted(math.exp(statistics.mean(sum(rng.choices(xs,k=len(xs)))/len(xs)for xs in logs))for _ in range(10000))
    return [v[249],v[9749]]
class Driver:
    def __init__(self,arm,op,name,suffix):
        self.out=S/f'wall-output-{op}-{name}-{suffix}.bin'
        self.p=subprocess.Popen(['taskset','-c',str(cpu),str(S/f'driver-{arm}'),op,str(S/'inputs'/f'{name}.bin'),str(self.out)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        assert self.read()=='READY'
    def read(self):
        assert select.select([self.p.stdout],[],[],180)[0],'driver timeout'
        line=self.p.stdout.readline().strip();assert line,self.p.stderr.read();return line
    def sample(self,n):
        self.p.stdin.write(str(n)+'\n');self.p.stdin.flush();return float(self.read())
    def close(self):
        self.p.stdin.write('stop\n');self.p.stdin.flush();self.p.communicate(timeout=60);assert self.p.returncode==0
result=dict(started=time.time(),core=core,registration_sha256=sha(ROOT/'REGISTRATION.md'),source_sha256=sha(ROOT/'src/quantize.rs'),binaries={a:sha(S/f'driver-{a}')for a in ['O','B','X','M7']},rows=[],completed=False)
def save():
    (S/'wall.json').write_text(json.dumps(result,indent=2)+'\n')
save()
for op in ops:
    for name in names:
        iterations=10000 if name.startswith('tiny')else 5 if name.startswith('million')else 500
        ds={a:Driver(a,op,name,a)for a in ['O','B','X']}
        ds['AA']=Driver('O',op,name,'AA')
        try:
            assert len({d.out.read_bytes()for d in ds.values()})==1,(op,name,'bytes differ')
            row=dict(operation=op,case=name,iterations=iterations,input_sha256=sha(S/'inputs'/f'{name}.bin'),output_sha256=sha(ds['O'].out),comparisons={})
            result['rows'].append(row);save()
            for arm in ['AA','X','B']:
                samples=[];rec=dict(warmup={},samples=samples);row['comparisons'][arm]=rec
                for a in ['O',arm]:rec['warmup'][a]=ds[a].sample(iterations)
                save()
                for i in range(10):
                    order=['O',arm]if i%2==0 else[arm,'O'];pending=dict(pair=i,order=order,ns={});rec['pending']=pending;save()
                    for a in order:pending['ns'][a]=ds[a].sample(iterations);save()
                    pending['ratio']=pending['ns']['O']/pending['ns'][arm];samples.append(pending);del rec['pending'];save()
                ratios=[s['ratio']for s in samples];rec.update(gm=gm(ratios),ci95=interval(ratios));save()
            print(op,name,{a:round(r['gm'],4)for a,r in row['comparisons'].items()},flush=True)
        finally:
            for d in ds.values():d.close()
floor=max(abs(v-1)for r in result['rows']for v in r['comparisons']['AA']['ci95']);result['F']=floor
families=[]
for op in ops:
    rows=[r for r in result['rows']if r['operation']==op]
    entry=dict(family=op,F=floor,max_baseline_upper95=max(r['comparisons']['B']['ci95'][1]for r in rows))
    for arm in ['X','B']:
        cases=[[s['ratio']for s in r['comparisons'][arm]['samples']]for r in rows]
        entry[arm]=dict(gm=gm([gm(xs)for xs in cases]),ci95=family_interval(cases))
    baseline_ok=entry['B']['gm']<=1 and entry['max_baseline_upper95']<=1.10+floor
    entry['verdict']='PASS'if baseline_ok and entry['X']['gm']<=1 else'PARTIAL'if baseline_ok else'FAIL'
    families.append(entry)
result.update(families=families,completed=True,finished=time.time());save()
print(json.dumps(families,indent=2))
