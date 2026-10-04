#!/usr/bin/env python3
"""Run all thirteen cargo-fuzz decoder entry-point smokes; retain corpora."""
import concurrent.futures
import json
import os
from pathlib import Path
import re
import struct
import subprocess
import time
from measure import ROOT,TARGET,ART,sha

NAMES=['vertex','vertex_into','triangles','triangles_into','sequence','sequence_into','oct','quat','exp','vertex_version','index_version','view','view_into']
FUZZ=ART/'tools/bin/cargo-fuzz'
ENV=dict(os.environ,CARGO_HOME=str(ART/'cargo-home'),CARGO_TARGET_DIR=str(TARGET),CARGO_NET_OFFLINE='true',RUSTC_BOOTSTRAP='1',RUSTC_WRAPPER='',TMPDIR=str(TARGET/'tmp'))
def main():
    subprocess.run([FUZZ,'fuzz','build','--fuzz-dir',ROOT/'fuzz/codec','--target-dir',TARGET],env=ENV,check=True)
    before={str(p.relative_to(ROOT)):sha(p) for p in ROOT.glob('src/**/*.rs')}
    cpus=sorted(os.sched_getaffinity(0));benchmark_cpu=int(os.environ.get('MESHOPT_BENCH_CPU',cpus[0]))
    siblings=Path(f'/sys/devices/system/cpu/cpu{benchmark_cpu}/topology/thread_siblings_list').read_text().strip()
    excluded=set()
    for group in siblings.split(','):
        if '-' in group:
            a,b=map(int,group.split('-'));excluded.update(range(a,b+1))
        else:excluded.add(int(group))
    cpus=[c for c in cpus if c not in excluded]
    binary_dir=TARGET/'x86_64-unknown-linux-gnu/release'
    def run(index,name):
        corpus=ART/'fuzz'/name/'corpus';crashes=ART/'fuzz'/name/'crashes';corpus.mkdir(parents=True,exist_ok=True);crashes.mkdir(exist_ok=True)
        operations={'vertex':1,'vertex_into':1,'triangles':2,'triangles_into':2,'sequence':3,'sequence_into':3,'oct':4,'quat':5,'exp':6,'vertex_version':8,'index_version':9,'view':7,'view_into':7}
        for p in (ART/'fixtures').glob('*.input'):
            b=p.read_bytes();op,count,stride,mode,filter=struct.unpack_from('<5I',b,4)
            if op!=operations[name] or count>4096 or stride>256 or len(b)>8192:continue
            header=struct.pack('<IHBBB',count,stride,mode,filter,255)
            (corpus/p.stem).write_bytes(header+b[44:])
        (corpus/'oversized').write_bytes(struct.pack('<IHBBB',0xffffffff,256,0,0,255)+b'\xa1'+bytes(256))
        executable=binary_dir/name;identity=sha(executable)
        # Direct libFuzzer execution after cargo-fuzz's instrumented build
        # avoids repeated rebuilds while retaining the standard cargo-fuzz target.
        command=['taskset','-c',str(cpus[index%len(cpus)]),str(executable),str(corpus),'-max_total_time=300','-seed=20261004','-max_len=8192','-timeout=10','-rss_limit_mb=512',f'-artifact_prefix={crashes}/']
        log=ART/'fuzz'/name/'smoke.log';start=time.monotonic()
        with log.open('w') as f:result=subprocess.run(command,env=ENV,stdout=f,stderr=subprocess.STDOUT)
        elapsed=time.monotonic()-start;text=log.read_text();matches=re.findall(r'Done (\d+) runs',text)
        assert result.returncode==0 and elapsed>=300 and matches and not list(crashes.iterdir()),(name,result.returncode,elapsed)
        assert identity==sha(executable)
        record={'target':name,'elapsed_seconds':elapsed,'executions':int(matches[-1]),'exit_code':result.returncode,'binary_sha256':identity,'log':str(log),'log_sha256':sha(log),'command':command,'cpu':cpus[index%len(cpus)],'sanitizer':'address','compiler':'stable with RUSTC_BOOTSTRAP=1, sanitizer/coverage instrumented by cargo-fuzz 0.13.2','libfuzzer_sys':'0.4.13'}
        print(name,'PASS',record['executions'],flush=True);return record
    with concurrent.futures.ThreadPoolExecutor(max_workers=len(NAMES)) as pool:
        rows=list(pool.map(lambda pair:run(*pair),enumerate(NAMES)))
    assert before=={str(p.relative_to(ROOT)):sha(p) for p in ROOT.glob('src/**/*.rs')}
    (ART/'fuzz.json').write_text(json.dumps({'phase':'0.2','sources':before,'targets':rows,'release_four_cpu_hours_per_target':'not established by this 300-second lane smoke'},indent=2))
if __name__=='__main__':main()
