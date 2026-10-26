#!/usr/bin/env python3
"""CPU-accounted property fuzzing of the production invariant divider."""
import subprocess,resource,json,sys,time,shutil
from control import ART,LANE,TARGET,ROOT,sha,save
assert __import__('os').environ.get('MOSS_HEAVY_ACTIVE')
cmd=['cargo','build','--locked','--offline','--release','--manifest-path',str(LANE/'Cargo.toml'),'--bin','l1-division-fuzz']
with (ART/'division-fuzz-build.log').open('x')as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
assert r.returncode==0
binary=ART/'division-fuzz';shutil.copy2(TARGET/'release/l1-division-fuzz',binary)
def cpu():
    r=resource.getrusage(resource.RUSAGE_CHILDREN);return r.ru_utime+r.ru_stime
budget=float(sys.argv[1])if len(sys.argv)>1 else 300
start=cpu();wall=time.time();rows=[];cases=20_000_000
with (ART/'division-fuzz-shards.jsonl').open('x')as f:
    while cpu()-start<budget:
        seed=20261030+len(rows)*104729;before=cpu()
        p=subprocess.run([str(binary),str(seed),str(cases)],capture_output=True,text=True)
        row={'seed':seed,'cases':cases,'exit':p.returncode,'cpu_seconds':cpu()-before,'stdout':p.stdout,'stderr':p.stderr}
        rows.append(row);f.write(json.dumps(row)+'\n');f.flush()
        if p.returncode:save(ART/'division-fuzz-failure.json',row);raise SystemExit(p.returncode)
        if len(rows)%20==0:print('division fuzz',len(rows),'cpu',round(cpu()-start,2),flush=True)
result={'exit':0,'cpu_seconds':cpu()-start,'wall_seconds':time.time()-wall,'shards':len(rows),'cases':cases*len(rows),'shards_sha256':sha(ART/'division-fuzz-shards.jsonl'),'binary_sha256':sha(binary),'production_source_sha256':sha(ROOT/'src/meshlet_division.rs'),'generator_sha256':sha(LANE/'division_fuzz.rs'),'replay':'division-fuzz SEED 20000000; seeds and commands in division-fuzz-shards.jsonl','method':'full-width usize inputs including 0 and MAX; divisors 1..512; exact comparison to native integer division; stable seeded mutation'}
save(ART/'division-fuzz.json',result)
assert result['cpu_seconds']>=budget
