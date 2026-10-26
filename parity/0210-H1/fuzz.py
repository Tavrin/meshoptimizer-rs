"""CPU-budgeted widened seeded exact differential; no performance wall clocks."""
from pathlib import Path
import json,os,resource,subprocess
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H1')
assert os.environ.get('MOSS_HEAVY_ACTIVE')
def cpu():
 u=resource.getrusage(resource.RUSAGE_CHILDREN);return u.ru_utime+u.ru_stime
start=cpu();rows=[];seed=20261026
while cpu()-start < 300:
 before=cpu()
 with (S/f'fuzz-{seed}.log').open('w') as log:
  rc=subprocess.run([str(S/'exact-std'),'fuzz',str(seed)],stdout=log,stderr=subprocess.STDOUT).returncode
 row=dict(seed=seed,exit=rc,cpu_seconds=cpu()-before);rows.append(row)
 (S/'fuzz.json').write_text(json.dumps(dict(cpu_seconds=cpu()-start,shards=rows,passed=rc==0 and cpu()-start>=300),indent=2)+'\n')
 assert rc==0,row
 print(row,flush=True);seed+=1
print('PASS',cpu()-start,'CPU seconds',len(rows),'shards',flush=True)
