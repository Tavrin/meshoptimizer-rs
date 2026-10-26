
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import os,json,time,datetime,subprocess,hashlib,math,statistics
A=Path(__file__).parent;R=Path(_public_path('<repo>'))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def event(kind,**kw):
 with (A/'CAMPAIGN.jsonl').open('a') as f:f.write(json.dumps(dict(event=kind,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),**kw))+'\n')
def wait_train():
 while True:
  waiting=[]
  for p in Path(_public_path('<local>/downstream-coord')).glob('heavy.queued.*.json'):
   try:
    x=json.loads(p.read_text());pid=int(x['pid']);os.kill(pid,0)
    cmd=Path(f'/proc/{pid}/cmdline').read_bytes().decode(errors='replace');cwd=str(Path(f'/proc/{pid}/cwd').resolve())
    if _public_path('<worktree>') in x.get('worktree_id','')+cmd+cwd:waiting.append(x)
   except (OSError,ValueError,KeyError):pass
  if not waiting:return
  event('PAUSE',reason='train first',waiting=waiting);time.sleep(10)
def run(script,args):
 wait_train();stem='-'.join([script]+args)
 with (A/f'admission-{stem}.log').open('x') as f:r=subprocess.run([_public_path('<job-runner>'),'2','timeout','840','bash',str(A/(script+'.sh'))]+args,env=env,stdout=f,stderr=subprocess.STDOUT)
 event('CASE_EXIT',script=script,args=args,exit=r.returncode)
 if r.returncode:raise SystemExit(r.returncode)
files=[A/'REGISTRATION.md',R/'parity/DECISIONS.md',A/'measure.py',A/'instructions.py',Path(__file__)]+[A/x for x in ['C-default','B-default','Bprime-default','C-pinned','B-pinned','probe-C','probe-B']]+list((A.parent/'inputs').glob('*.bin'))
for name in ['B','Bprime','C']:files+=list((A/f'lib-{name}/src').rglob('*.rs'))
files+=[A/'measure.sh',A/'instructions.sh']
files+=list(A.glob('*harness/Cargo.toml'))+list(A.glob('*harness/Cargo.lock'))+list(A.glob('*harness/src/*.rs'))+list(A.glob('lib-*/Cargo.toml'))+list(A.glob('lib-*/Cargo.lock'))+[A/'build.sh',A/'build-pinned.sh',A/'ALIGNMENT.json']
identity=dict(cpu=11,files={str(p):sha(p) for p in files},started=datetime.datetime.now(datetime.timezone.utc).isoformat())
(A/'MEASUREMENT-IDENTITY.json').write_text(json.dumps(identity,indent=2))
env=dict(os.environ,MOSS_HEAVY_GPU='0',MOSS_HEAVY_MEASUREMENT='1',MOSS_LANE='meshopt-bounds-layout',CARGO_TARGET_DIR=_public_path('<target-dir>'))
cases=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth'];ops=['meshlet-bounds','cluster-bounds']
for comparison in ['AA','default','pinned']:
 for case in cases:
  for op in ops:run('measure',[case,op,comparison])
for case in cases:
 for op in ['meshlet','cluster']:run('instructions',[case,op])
event('COMPLETE');(A/'COMPLETE.json').write_text(json.dumps(dict(complete=True,utc=datetime.datetime.now(datetime.timezone.utc).isoformat())))
