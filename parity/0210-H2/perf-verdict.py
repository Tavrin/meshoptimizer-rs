
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import json,math
S=Path(_public_path('<scratch>/meshopt-0210-H2'))
rows=json.loads((S/'samples/summary.json').read_text())
failed=False
for f in ['simplify-scale','overdraw']:
 for population in ['frozen','all']:
  rr=[r for r in rows if r['family']==f and (population=='all' or r['kind']!='chain')]
  cb=math.exp(sum(math.log(r['median']['candidate']/r['median']['base']) for r in rr)/len(rr))
  cm=math.exp(sum(math.log(r['median']['candidate']/r['median']['M7']) for r in rr)/len(rr))
  worst=max(r['median']['candidate']/r['median']['base'] for r in rr)
  v='FAIL' if worst>1.01 else 'PASS' if cm<=1 else 'PARTIAL' if cb<=.95 else 'NO-GAIN'
  print(f,population,v,'candidate/base',cb,'candidate/M7',cm,'worst',worst,flush=True)
  failed|=v in ['FAIL','NO-GAIN']
raise SystemExit(int(failed))
