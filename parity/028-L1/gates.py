#!/usr/bin/env python3
"""Required targeted gates, each exit retained; no full release matrix."""
from pathlib import Path
import subprocess,os,json,sys
from control import ROOT,ART,LANE,save
assert os.environ.get('PARITY_RUNNER_ACTIVE')
commands=[
 ('all-features',['cargo','test','--locked','--all-features']),
 ('no-default-features',['cargo','test','--locked','--no-default-features']),
 ('clippy',['cargo','clippy','--locked','--all-features','--all-targets','--','-D','warnings']),
 ('fmt',['cargo','fmt','--all','--','--check']),
 ('job-fmt',['cargo','fmt','--manifest-path',str(LANE/'Cargo.toml'),'--','--check']),
]
rows=[]
for name,cmd in commands:
    with (ART/f'gate-{name}.log').open('x')as f:p=subprocess.run(cmd,cwd=ROOT,stdout=f,stderr=subprocess.STDOUT)
    rows.append({'gate':name,'command':cmd,'exit':p.returncode});save(ART/'gates.json',rows)
    print(name,p.returncode,flush=True)
raise SystemExit(any(r['exit']for r in rows))
