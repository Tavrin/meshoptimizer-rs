"""Finalize only a complete admitted campaign, retain receipts, remove its target."""
import hashlib,json,os,shutil,subprocess
from pathlib import Path
R=Path.cwd();S=Path(os.environ['DECODE_ROOT']);T=Path(os.environ['CARGO_TARGET_DIR']);L=R/'parity/0216-vdecode'
assert T.name=='meshopt-0216-vdecode' and T.is_dir()
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
branch=subprocess.check_output(['git','branch','--show-current'],text=True).strip()
assert head=='91969326e363f6b311cffcda6c06a1a36da572a6' and branch=='perf/0216-vdecode'
subprocess.run(['git','diff','--cached','--quiet'],check=True)
subprocess.run(['git','diff','--quiet','--','src/codec/index.rs'],check=True)
subprocess.run(['git','diff','--check'],check=True)
for source in (R/'src').rglob('*.rs'):
 frozen=S/'guard/frozen-candidate'/source.relative_to(R)
 assert source.read_bytes()==frozen.read_bytes(),('source drift',str(source.relative_to(R)))
subprocess.run(['python3','parity/0216-vdecode/report.py'],check=True)
verdict=json.loads((S/'verdict.json').read_text());assert verdict['verdict']=='SHIPS',verdict
subprocess.run(['python3','parity/0216-vdecode/retain.py'],check=True)
files=[p for p in T.rglob('*') if p.is_file()];count=len(files);size=sum(p.stat().st_size for p in files)
shutil.rmtree(T);assert not T.exists()
shutil.rmtree(L/'__pycache__',ignore_errors=True)
cleanup=dict(exit=0,target=T.name,removed=True,verified_absent=True,files_removed=count,apparent_bytes_removed=size,queued_jobs_cancelled=0,head=head,branch=branch,index_unchanged=True,index_source_unchanged=True)
(L/'cleanup.json').write_text(json.dumps(cleanup,indent=2)+'\n')
O=L/'receipts'
manifest={str(p.relative_to(O)):hashlib.sha256(p.read_bytes()).hexdigest() for p in O.rglob('*') if p.is_file() and p.name!='SHA256SUMS.json'}
(O/'SHA256SUMS.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
# This final read-only text check includes all newly retained and untracked text.
p=subprocess.run(['bash','parity/tools/check_public_text.sh'],capture_output=True,text=True)
(L/'final-checks.json').write_text(json.dumps(dict(public_text_exit=p.returncode,public_text_output=p.stdout+p.stderr,diff_check_exit=0,source_binding_exit=0),indent=2)+'\n')
print(p.stdout+p.stderr,end='');assert p.returncode==0
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==head
print(json.dumps(dict(verdict=verdict,cleanup=cleanup),indent=2))
