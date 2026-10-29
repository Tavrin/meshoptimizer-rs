"""Focused minimum-version validation under the already admitted CPU job."""
import json,os,resource,subprocess,time
from pathlib import Path
art=Path(os.environ['DECODE_ART']);rows=[]
for name,args in [('msrv-simd',['cargo','+1.88','test','--offline','--locked','--all-features','--test','simd']),('msrv-no-default',['cargo','+1.88','check','--offline','--locked','--no-default-features'])]:
    started=time.monotonic();before=resource.getrusage(resource.RUSAGE_CHILDREN)
    with (art/(name+'.log')).open('w') as log:r=subprocess.run(args,stdout=log,stderr=subprocess.STDOUT)
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    rows.append(dict(step=name,exit=r.returncode,wall_seconds=time.monotonic()-started,cpu_seconds=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime))
    (art/'msrv-exits.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,r.returncode,flush=True)
assert all(x['exit']==0 for x in rows),rows
