#!/usr/bin/env python3
"""Sequential ASan bursts with CPU accounting and per-level execution checkpoints.

A first-release ledger must total 86400 CPU seconds PER target and native host
with unchanged kernel identities. This script never credits wall time as CPU.
"""
import argparse, hashlib, json, os, resource, shutil, subprocess, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
NAMES=['vertex','vertex_into','triangles','triangles_into','sequence','sequence_into','oct','quat','exp','vertex_version','index_version','view','view_into','color','meshlet_decode','meshlet_decode_into','meshlet_raw','meshlet_raw_into','simd']
p=argparse.ArgumentParser();p.add_argument('--cpu-seconds',type=float,default=60);p.add_argument('--cpu',type=int,default=0);p.add_argument('--select',choices=NAMES);a=p.parse_args()
assert a.cpu_seconds>0 and a.cpu in os.sched_getaffinity(0)
art=Path(os.environ['MESHOPT_ARTIFACTS'])/'simd-fuzz';art.mkdir(parents=True,exist_ok=True)
target=Path(os.environ['CARGO_TARGET_DIR']);host=subprocess.check_output(['rustc','-vV'],text=True).split('host: ')[1].splitlines()[0]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
sources={str(p.relative_to(ROOT)):sha(p) for p in sorted([*ROOT.glob('src/**/*.rs'),*ROOT.glob('fuzz/codec/**/*.rs'),ROOT/'fuzz/codec/Cargo.lock',ROOT/'fuzz/codec/Cargo.toml'])}
subprocess.run(['cargo','fuzz','build','--fuzz-dir',str(ROOT/'fuzz/codec'),'--target-dir',str(target),'--target',host],check=True,env=dict(os.environ,CARGO_BUILD_JOBS='2'))
for name in [a.select] if a.select else NAMES:
    binary=target/host/'release'/name;directory=art/name;directory.mkdir(exist_ok=True)
    corpus=directory/'corpus';corpus.mkdir(exist_ok=True);crashes=directory/'crashes';crashes.mkdir(exist_ok=True)
    shutil.copy2(binary,directory/'binary');identity=sha(binary);used=0;burst=0
    while used<a.cpu_seconds:
        stamp=str(time.time_ns());counts=directory/(stamp+'-counts.json');log=directory/(stamp+'.log')
        command=['taskset','-c',str(a.cpu),str(binary),str(corpus),f'-max_total_time={min(600,max(1,int(a.cpu_seconds-used)))}','-rss_limit_mb=512','-timeout=10','-max_len=65536',f'-artifact_prefix={crashes}/']
        prior=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.monotonic()
        with log.open('w') as f:run=subprocess.run(command,stdout=f,stderr=subprocess.STDOUT,env=dict(os.environ,MESHOPT_FUZZ_COUNTS=str(counts)))
        after=resource.getrusage(resource.RUSAGE_CHILDREN);cpu=after.ru_utime+after.ru_stime-prior.ru_utime-prior.ru_stime
        checkpoint=json.loads(counts.read_text()) if counts.exists() else None
        row={'host':host,'target':name,'sources':sources,'binary_sha256':identity,'command':command,'cpu':a.cpu,'cpu_seconds':cpu,'wall_seconds':time.monotonic()-start,'exit_code':run.returncode,'sanitizer':'address','counts_lower_bounds':checkpoint,'log_sha256':sha(log),'mismatches':0 if run.returncode==0 else None}
        (directory/(stamp+'.json')).write_text(json.dumps(row,indent=2)+'\n')
        assert run.returncode==0 and not list(crashes.iterdir()) and sha(binary)==identity
        assert sources=={str(p.relative_to(ROOT)):sha(p) for p in sorted([*ROOT.glob('src/**/*.rs'),*ROOT.glob('fuzz/codec/**/*.rs'),ROOT/'fuzz/codec/Cargo.lock',ROOT/'fuzz/codec/Cargo.toml'])}
        used+=cpu;burst+=1;print(name,used,'CPU seconds',flush=True)
        time.sleep(10) # Release the core after each burst of at most ten minutes.
