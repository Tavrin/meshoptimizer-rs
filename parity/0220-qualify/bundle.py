"""Run remaining 4 GB stages serially inside the existing admission."""
import json, os, subprocess
from pathlib import Path
import run as q
H=q.A;A=q.OUT
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU')=='0'
steps=[('clippy',['gate.py','clippy']),('release',['gate.py','release']),('doc',['gate.py','doc']),('public-text',['gate.py','public-text']),('campaign',['campaign.py']),('supplement',['supplement.py']),('v2-equivalence',['v2_exact.py']),('bench',['bench.py'])]
status=A/'bundle-status.json'
rows=json.loads(status.read_text())['stages'] if status.exists() else []
passed={x['stage'] for x in rows if x['exit']==0}
start=json.loads((A/'start.json').read_text())
assert {str(p.relative_to(q.R)):q.sha(p) for p in (q.R/'src').rglob('*') if p.is_file()}==start['sources']
assert q.sha(q.R/'Cargo.toml')==start['cargo_toml'] and q.sha(q.R/'Cargo.lock')==start['cargo_lock']
for name,args in steps:
 if name in passed:
  print('BUNDLE RETAIN PASS',name,flush=True);continue
 print('BUNDLE START',name,flush=True)
 p=subprocess.run(['python3',str(H/args[0]),*args[1:]])
 rows.append(dict(stage=name,exit=p.returncode))
 (A/'bundle-status.json').write_text(json.dumps(dict(completed=False,stages=rows),indent=2)+'\n')
 print('BUNDLE END',name,p.returncode,flush=True)
(A/'bundle-status.json').write_text(json.dumps(dict(completed=True,stages=rows),indent=2)+'\n')
