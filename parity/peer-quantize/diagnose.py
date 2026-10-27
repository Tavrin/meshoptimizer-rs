
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import subprocess,json,os,hashlib
s=Path(os.environ.get('PEER_SCRATCH',_public_path('<scratch>/meshopt-peer-a')))
assert not (s/'wall.json').exists(), 'completed evidence is immutable; use fresh scratch'
assert os.environ.get('MOSS_HEAVY_ACTIVE')
inputs=s/'inputs';inputs.mkdir(exist_ok=True)
frozen=Path(_public_path('<scratch>/meshopt-peer-screen/campaign-20261025T150615Z/inputs'))
names=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']
import shutil
for n in names:shutil.copy2(frozen/f'{n}.bin',inputs/f'{n}.bin')
cpu=min(os.sched_getaffinity(0));rows=[]
for op in ['quantize-half','quantize-float']:
 for arm in ['B','X','M7']:
  for n in names:
   log=s/f'instructions-baseline-{op}-{arm}-{n}.txt'
   cmd=['/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf','stat','-x,','-e','instructions:u','-o',str(log),'taskset','-c',str(cpu),str(s/f'driver-{arm}'),op,str(inputs/f'{n}.bin'),str(s/f'output-{arm}.bin')]
   r=subprocess.run(cmd,input='100\nstop\n',text=True,capture_output=True)
   rows.append(dict(operation=op,arm=arm,case=n,exit=r.returncode,perf=log.read_text(),stderr=r.stderr))
   print(op,arm,n,r.returncode,flush=True)
(s/'baseline-instructions.json').write_text(json.dumps(rows,indent=2))
