"""Select an allowed core by a fixed one-second idle observation."""
import json,os,time
from pathlib import Path
def snapshot():
 rows={}
 for line in Path('/proc/stat').read_text().splitlines():
  k,*v=line.split()
  if k.startswith('cpu') and k[3:].isdigit():
   values=list(map(int,v[:8]));rows[int(k[3:])]=(sum(values),values[3]+values[4])
 return rows
before=snapshot();time.sleep(1);after=snapshot()
rows=[]
for cpu in sorted(os.sched_getaffinity(0)):
 total=after[cpu][0]-before[cpu][0]; idle=after[cpu][1]-before[cpu][1]
 rows.append(dict(cpu=cpu,busy_fraction=(total-idle)/max(total,1)))
busy={row['cpu']:row['busy_fraction'] for row in rows}
for row in rows:
 path=Path('/sys/devices/system/cpu')/('cpu'+str(row['cpu']))/'topology/thread_siblings_list'
 siblings=[]
 for part in path.read_text().strip().split(','):
  ends=part.split('-'); siblings.extend(range(int(ends[0]),int(ends[-1])+1))
 row['siblings']=siblings
 row['physical_busy_fraction']=max(busy.get(cpu,1.0) for cpu in siblings)
selected=min(rows,key=lambda r:(r['physical_busy_fraction'],r['busy_fraction'],-r['cpu']))['cpu']
Path('parity/0217-pipeline/receipts/quiet-cpu.json').write_text(json.dumps(dict(selected=selected,observation_seconds=1,cores=rows),indent=2)+'\n')
print(selected)
