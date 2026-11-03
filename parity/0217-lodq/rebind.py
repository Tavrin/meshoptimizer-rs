"""Bind the measured binary to final source after a quality-only lint fix."""
import hashlib,json,os,shutil,subprocess
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE')
r=Path.cwd();lane=r/'parity/0217-lodq';out=lane/'receipts';t=Path(os.environ['CARGO_TARGET_DIR']);tree=t/'perf/candidate';peers=tree/'parity/peers'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
old=json.loads((out/'perf-binding.json').read_text())['candidate']['binary']
shutil.copytree(r/'src',tree/'src',dirs_exist_ok=True)
env=dict(os.environ,CARGO_TARGET_DIR=str(t/'perf-candidate'))
with (out/'perf-final-rebuild.log').open('w') as f:
 p=subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(peers/'Cargo.toml'),'--features','ours'],env=env,stdout=f,stderr=subprocess.STDOUT)
assert p.returncode==0
new=sha(t/'perf-candidate/release/meshopt-peers')
record=dict(initial_binary=old,final_binary=new,identical=old==new,sources={str(p.relative_to(tree)):sha(p) for p in [*tree.glob('src/**/*.rs'),tree/'Cargo.toml',peers/'Cargo.toml',peers/'src/main.rs']})
(out/'perf-final-binding.json').write_text(json.dumps(record,indent=2)+'\n')
if old!=new:
 # Retain the first campaign before repeating on the final frozen source.
 prior=out/'perf-initial';prior.mkdir()
 for p in list(out.iterdir()):
  if p.name.startswith('perf-') and p!=prior or p.name in ['counts.jsonl','inputs.json','samples']:
   p.rename(prior/p.name)
 subprocess.run(['python3',str(lane/'perf.py')],check=True)
with (out/'exits.tsv').open('a') as f:f.write('perf-final-binding\t0\n')
print('final-source performance binding: binary identity',old==new,flush=True)
