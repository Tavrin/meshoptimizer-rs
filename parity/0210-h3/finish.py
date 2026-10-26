"""Apply registered per-family reversions, then qualify the final library state."""
from pathlib import Path
import subprocess,shutil,json,os
R=Path(__file__).resolve().parents[2];S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-h3')
assert os.environ.get('MOSS_HEAVY_ACTIVE')
results=json.loads((S/'aggregates.json').read_text())
D=S/'trial-r2';D.mkdir()
shutil.copytree(R/'src',D/'src')
for p in S.iterdir():
 if p.is_file() and (p.name.startswith(('driver-','exact-','seeds-','sqrt-','build-')) or p.name in ['partition-sqrt.rs','binding.json','cases.json','real-chains.json','raw.json','summary.json','aggregates.json','measurement-exits.json','proof-build.log','measure-r2.log']):shutil.copy2(p,D/p.name)
shutil.move(str(S/'samples'),D/'samples')
reverted=[]
for family,paths in [('partition',['src/partition.rs','src/processing.rs']),('meshlets-spatial',['src/meshlet_spatial.rs'])]:
 if results[family]['verdict'] in ['NO-GAIN','FAIL']:
  for path in paths:(R/path).write_bytes(subprocess.check_output(['git','show',f'v0.2.9:{path}'],cwd=R))
  reverted.append(family)
(S/'disposition.json').write_text(json.dumps(dict(trial=results,reverted=reverted,retained=[f for f in results if f not in reverted]),indent=2)+'\n')
print('REGISTERED DISPOSITION',reverted,flush=True)
if reverted:
 subprocess.run(['bash',str(R/'parity/0210-h3/build.sh')],cwd=R,check=True)
 subprocess.run(['python3',str(R/'parity/0210-h3/proof-build.py')],cwd=R,check=True)
 subprocess.run(['python3',str(R/'parity/0210-h3/measure.py')],cwd=R,check=True,env={**os.environ,'MOSS_HEAVY_MEASUREMENT':'1'})
else:
 # Keep the final sample archive in its conventional location as well.
 shutil.copytree(D/'samples',S/'samples')
subprocess.run(['bash',str(R/'parity/0210-h3/gates.sh')],cwd=R,check=True)
subprocess.run(['python3',str(R/'parity/0210-h3/proof-run.py')],cwd=R,check=True)
subprocess.run(['python3',str(R/'parity/0210-h3/bind.py')],cwd=R,check=True)
print('FINAL QUALIFICATION COMPLETE',flush=True)
