
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import sys,os,json,subprocess,hashlib,time,math,statistics,select
from pathlib import Path
A=Path(_public_path('<scratch>/meshopt-validate-once/layout-check'));R=Path(_public_path('<repo>'))
sys.path.insert(0,str(R/'parity'));import quiet
case,op,comparison=sys.argv[1:];stem=f'{case}-{op}-{comparison}';D=A/'timing';D.mkdir(exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_MEASUREMENT')=='1'
assert (A/'REGISTRATION.md').exists()
identity=json.loads((A/'MEASUREMENT-IDENTITY.json').read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert all(sha(p)==v for p,v in identity['files'].items()),'frozen epoch changed'
assert not (D/(stem+'.json')).exists(),'completed case may not be repeated'
core=identity['cpu'];path=A.parent/'inputs'/(case+'.bin')
# Same paired log Student-t interval and once-to-twenty stopping as bet 0.
def interval(r):
 logs=[math.log(x)for x in r];mu=statistics.mean(logs);n=len(logs);t={5:2.776445,20:2.093024}[n];radius=t*statistics.stdev(logs)/math.sqrt(n)
 return [math.exp(mu-radius),math.exp(mu+radius)]
class Driver:
 def __init__(self,label,binary,cached):
  self.out=D/f'{stem}-attempt{attempt}-{label}.bin'
  self.p=subprocess.Popen(['taskset','-c',str(core),str(binary),op,str(path),str(self.out)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,env=dict(os.environ,VALIDATE_ONCE=cached))
  try:
   if self.read()!='READY':raise RuntimeError('not ready')
  except BaseException:
   self.kill();raise
 def read(self):
  if not select.select([self.p.stdout],[],[],180)[0]:raise TimeoutError('driver timeout')
  line=self.p.stdout.readline().strip()
  if not line:raise RuntimeError(self.p.stderr.read())
  return line
 def sample(self,n):self.p.stdin.write(str(n)+'\n');self.p.stdin.flush();return float(self.read())
 def close(self):
  if self.p.poll() is None:self.p.stdin.write('stop\n');self.p.stdin.flush();self.p.communicate(timeout=180)
  if self.p.returncode!=0:raise RuntimeError(f'driver exit {self.p.returncode}')
 def kill(self):
  if self.p.poll() is None:self.p.kill();self.p.wait()
rows=[]
initial=int(os.environ.get('VALIDATE_ONCE_ATTEMPT','0'))
for attempt in range(initial,2):
 drivers={}
 try:
  drivers['A']=Driver('A',A/('Bprime-default' if comparison=='AA' else 'C-'+comparison),'0')
  drivers['B']=Driver('B',A/('B-default' if comparison=='AA' else 'B-'+comparison),'0')
  assert drivers['A'].out.read_bytes()==drivers['B'].out.read_bytes(),'resident bytes differ'
  row=dict(case=case,operation=op,comparison=comparison,cpu=core,attempt=attempt,samples=[],binary_sha256={k:sha(A/(('Bprime-default' if comparison=='AA' else 'C-'+comparison) if k=='A' else ('B-default' if comparison=='AA' else 'B-'+comparison)))for k in drivers})
  cal={k:d.sample(1)for k,d in drivers.items()};n=max(1,min(1000000,math.ceil(25000000/min(cal.values()))));row.update(iterations=n,calibration_ns=cal)
  (D/f'{stem}-calibration-{attempt}.json').write_text(json.dumps(row,indent=2))
  for pair in range(20):
   if pair>=5 and not row['five_pair_ci'][0]<=1<=row['five_pair_ci'][1]:break
   order=['A','B']if pair%2==0 else['B','A'];v={}
   for arm in order:
    v[arm]=drivers[arm].sample(n)
    (D/f'{stem}-attempt{attempt}-pair{pair}-{arm}.json').write_text(json.dumps(dict(ns=dict(v),order=order,iterations=n,utc=time.time(),load=os.getloadavg())))
   row['samples'].append(dict(pair=pair,order=order,ns=v,ratio=v['A']/v['B']))
   if pair==4:row['five_pair_ci']=interval([x['ratio']for x in row['samples']])
  ratios=[x['ratio']for x in row['samples']];row.update(gm=math.exp(statistics.mean(map(math.log,ratios))),ci95=interval(ratios),completed=True)
  for d in drivers.values():d.close()
  (D/(stem+'.json')).write_text(json.dumps(row,indent=2));print(stem,row['gm'],flush=True);break
 except (RuntimeError,TimeoutError,subprocess.TimeoutExpired) as e:
  for d in drivers.values():d.kill()
  (D/f'{stem}-failure-{attempt}.json').write_text(json.dumps(dict(error=repr(e),samples=locals().get('row'))))
  retry=A/'INFRA_RETRY.json'
  if retry.exists():raise
  retry.write_text(json.dumps(dict(case=case,op=op,comparison=comparison,error=repr(e))))
