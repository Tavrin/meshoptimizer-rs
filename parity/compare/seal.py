#!/usr/bin/env python3
"""Seal a fresh comparison source epoch after all driver builds finish."""
from pathlib import Path
import json,hashlib,subprocess
ROOT=Path(__file__).resolve().parents[2]
ART=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp')
p=ART/'measurement-source.json'
assert not p.exists(),'retain the existing campaign; do not replace its source epoch'
files=[ROOT/'Cargo.toml',ROOT/'Cargo.lock']
files += [q for q in sorted((ROOT/'src').rglob('*')) if q.is_file()]
files += [ROOT/'parity'/q for q in ['runner.py','performance.py','quiet.py']]
files += [ROOT/'parity/compare'/q for q in ['Cargo.toml','Cargo.lock','build.py','build_drivers.py','measure.py']]
files += sorted((ROOT/'parity/compare/src').glob('*.rs'))
def sha(q):return hashlib.sha256(q.read_bytes()).hexdigest()
for profile in ['defaults','moss']:
 for backend in ['ours','theirs']:
  q=ART/f'cmp-driver-{profile}-{backend}'
  assert sha(q)==Path(str(q)+'.sha256').read_text().strip()
p.write_text(json.dumps({'schema':1,'revision':subprocess.check_output(['git','-C',str(ROOT),'rev-parse','HEAD'],text=True).strip(),'files':{str(q.relative_to(ROOT)):sha(q) for q in files}},indent=2)+'\n')
print('sealed',len(files),'files')
