"""Per-function mutation smokes; elapsed time, replay seeds and identities retained."""
import argparse
import concurrent.futures
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
import runner as r
from p01x import FAMILIES

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--seconds',type=int,default=300);parser.add_argument('--workers',type=int,default=32);args=parser.parse_args()
    if args.seconds<1 or args.workers<1:parser.error('positive budgets required')
    reference,target,results=r.paths();artifacts=r.artifacts_path();before=r.snapshot(reference)
    r.command(['cargo','build','--offline','--locked','--release','--manifest-path',r.ROOT/'fuzz/Cargo.toml','--bins'])
    if before!=r.snapshot(reference):raise ValueError('source changed during fuzz build')
    binaries={name:target/'release'/name for name in FAMILIES.values()};identities={name:r.sha(p)for name,p in binaries.items()}
    allowed=sorted(os.sched_getaffinity(0));excluded=set(map(int,os.environ.get('MESHOPT_FUZZ_EXCLUDE_CPUS','').replace(',',' ').split()));allowed=[cpu for cpu in allowed if cpu not in excluded]
    if not allowed:raise ValueError('no fuzz cores remain')
    directory=artifacts/'p01x-fuzz';directory.mkdir(exist_ok=True)
    for name,binary in binaries.items():shutil.copy2(binary,directory/name)
    record={'schema':1,'phase':'0.1.x','budget_seconds':args.seconds,'instrumentation':'seeded mutation/determinism; no coverage or sanitizer instrumentation; elapsed smoke only, not the RFC 4 CPU-hour release gate','source_sha256':before,'executable_sha256':identities,'runs':[],'passed':False}
    def run(item):
        op,name=item;seed=20261003+op;cpu=allowed[op%len(allowed)];started=time.time();cmd=['taskset','-c',str(cpu),str(binaries[name]),str(args.seconds),str(seed)]
        with (directory/(name+'.stdout')).open('wb')as out,(directory/(name+'.stderr')).open('wb')as err:
            status=subprocess.run(cmd,stdout=out,stderr=err,env=r.ENV)
        stats=None
        if status.returncode==0:stats=json.loads((directory/(name+'.stdout')).read_text())
        result={'target':name,'seed':seed,'cpu':cpu,'command':cmd,'started_unix':started,'finished_unix':time.time(),'exit':status.returncode,'statistics':stats,'stdout_sha256':r.sha(directory/(name+'.stdout')),'stderr_sha256':r.sha(directory/(name+'.stderr'))}
        print(f'{name}: exit {status.returncode}, executions {stats["executions"]if stats else None}',flush=True);return result
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers)as pool:
        for result in pool.map(run,FAMILIES.items()):
            record['runs'].append(result);(results/'p01x-fuzz.partial.json').write_text(json.dumps(record,indent=2)+'\n')
    record['identities_unchanged']=before==r.snapshot(reference)and identities=={name:r.sha(p)for name,p in binaries.items()}
    record['passed']=record['identities_unchanged']and len(record['runs'])==len(FAMILIES)and all(run['exit']==0 and run['statistics']['seconds']>=args.seconds and run['statistics']['successes']>0 for run in record['runs'])
    (results/'p01x-fuzz.json').write_text(json.dumps(record,indent=2)+'\n')
    if not record['passed']:raise SystemExit(1)

if __name__=='__main__':main()
