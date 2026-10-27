#!/usr/bin/env python3
"""Read-only audit of the single retained campaign; acceptance exits 1 on FAIL."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib,json,os,sys
S=Path(os.environ.get('PEER_SCRATCH',_public_path('<scratch>/meshopt-peer-a')));R=Path(__file__).resolve().parents[2]
x=json.loads((S/'wall.json').read_text());assert x['completed']and len(x['rows'])==10
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert x['source_sha256']==sha(R/'src/quantize.rs')
assert x['registration_sha256']==sha(R/'REGISTRATION.md')
assert all(sha(S/f'driver-{a}')==h for a,h in x['binaries'].items())
for row in x['rows']:
 assert row['input_sha256']==sha(S/'inputs'/f"{row['case']}.bin")
 for a,c in row['comparisons'].items():
  assert len(c['samples'])==10 and 'pending'not in c and set(c['warmup'])=={'O',a}
  for i,p in enumerate(c['samples']):
   assert p['order']==(['O',a]if i%2==0 else[a,'O'])
   assert p['ratio']==p['ns']['O']/p['ns'][a]
 assert len({sha(S/f"wall-output-{row['operation']}-{row['case']}-{a}.bin")for a in ['O','B','X','AA']})==1
assert json.loads((S/'proof.json').read_text())['passed']
assert json.loads((S/'direct.json').read_text())['passed']
accept=all(f['verdict']=='PASS'for f in x['families'])
result=dict(integrity='PASS',acceptance='PASS'if accept else'FAIL',families=x['families'],source_sha256=x['source_sha256'],registration_sha256=x['registration_sha256'])
(S/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
sys.exit(0 if accept else 1)
