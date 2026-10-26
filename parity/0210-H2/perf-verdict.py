from pathlib import Path
import json,math
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H2')
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
