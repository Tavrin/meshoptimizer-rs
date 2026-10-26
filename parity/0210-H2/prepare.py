"""Archive immutable inputs and prepare scratch-only count drivers."""
from pathlib import Path
import subprocess, shutil, re, json, hashlib
R=Path(__file__).resolve().parents[2]
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H2')
A=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-attr')
def archive(ref,d):
 if not d.exists():
  d.mkdir();subprocess.run(['tar','-x','-C',str(d)],input=subprocess.check_output(['git','archive',ref]),check=True)
archive('v0.2.9',S/'baseline')
archive('bench/peer-screen',S/'bet0')
# Selectively extract the replay harness/metadata, never modify the NAS archive.
if not (S/'meshopt-028-L2/harness-candidate/src/main.rs').exists():
 subprocess.run(['tar','--zstd','-xf','/mnt/apollo-main/linux-dev-storage-folder/meshoptimizer-rs/scratch-2026-10-26/meshopt-028-L2.tar.zst','-C',str(S),'--wildcards','meshopt-028-L2/harness-candidate/*','meshopt-028-L2/stages.json','meshopt-028-L2/chain-*.tsv'],check=True)
# The attribution stage adapter removes the timer path and adds single-call replay
# to the archived replay harness. Retain its complete source and binding.
for arm in ['base','candidate','M7']:
 h=S/f'harness-{arm}'
 if not h.exists():shutil.copytree(S/'meshopt-028-L2/harness-candidate',h)
 shutil.copy2(A/'harness-base/src/main.rs',h/'src/main.rs')
 p=h/'Cargo.toml';s=p.read_text();s=re.sub(r'path = "[^"]+", optional = true, default-features',f'path = "{S / "baseline" if arm=="base" else R}", optional = true, default-features',s);p.write_text(s)
# Inputs are frozen artifacts, copied without regeneration.
(S/'inputs').mkdir(exist_ok=True)
for p in (A/'meshopt-028-L2/inputs').glob('*.bin'):shutil.copy2(p,S/'inputs'/p.name)
cases=json.loads((A/'cases.json').read_text());cases=[c for c in cases if c['family'] in ['simplify-scale','overdraw']]
for c in cases:
 c['source']=c['source'].replace(str(A/'meshopt-028-L2/inputs'),str(S/'inputs')).replace(str(A),str(S))
 c['args'][1]=c['source']
 if c['kind']=='chain':
  original=A/Path(c['source']).name
  (S/original.name).write_text(original.read_text()) # references retained frozen input files
 elif c['family']=='simplify-scale':shutil.copy2(A/Path(c['source']).name,c['source'])
 c['source_sha256']=hashlib.sha256(Path(c['source']).read_bytes()).hexdigest()
(S/'cases.json').write_text(json.dumps(cases,indent=2)+'\n')
(S/'preparation.json').write_text(json.dumps({'baseline':subprocess.check_output(['git','rev-parse','v0.2.9'],text=True).strip(),'bet0':subprocess.check_output(['git','rev-parse','bench/peer-screen'],text=True).strip(),'adapter_sha256':hashlib.sha256((A/'harness-base/src/main.rs').read_bytes()).hexdigest()},indent=2)+'\n')
