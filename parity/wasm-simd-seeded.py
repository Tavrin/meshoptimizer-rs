#!/usr/bin/env python3
"""Stream the wasm release budget in bounded batches; retain failures and hashes."""
import argparse,hashlib,itertools,json,math,os,struct,subprocess,time
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--cases',type=int,default=1000);p.add_argument('--cpu',type=int,default=0);a=p.parse_args();assert a.cases>0
os.environ['MESHOPT_BENCH_CPU']=str(a.cpu)
import sys
sys.path.insert(0,str(Path(__file__).resolve().parent/'simd'))
import qualify as q
assert a.cpu in os.sched_getaffinity(0)
path=q.ART/f'wasm-seeded-{a.cases}.json';assert not path.exists(),'preserve prior run; use a new artifact directory'
extra=[Path(__file__),q.ROOT/'parity/codec/runner.py',q.ROOT/'parity/codec/runner04.py',q.ROOT/'parity/codec/measure.py',q.ROOT/'parity/codec/reference.cpp']
sources=lambda:{**q.sources(),**{str(f.relative_to(q.ROOT)):q.m.sha(f) for f in extra}}
before=sources();ids={f.name:q.m.sha(f) for f in q.BIN.iterdir()};record={'sources':before,'binaries':ids,'cpu':a.cpu,'node':subprocess.check_output(['node','--version'],text=True).strip(),'goal':a.cases,'cases':0,'executions':{'native-scalar':0,'cpp-scalar':0,'wasm-scalar':0,'wasm-simd':0},'families':{},'batches':[],'complete':False};q.save(path,record)
while record['cases']<a.cases:
    batch=len(record['batches']);seed=20261005+batch;cpp=q.m.Driver(q.BIN/'cpp-scalar');native=q.m.Driver(q.BIN/'rust-scalar');wasm={'wasm-scalar':q.r02.Wasm(q.BIN/'wasm-scalar.wasm'),'wasm-simd':q.r02.Wasm(q.BIN/'wasm-simd.wasm')}
    n=min(1000,math.ceil((a.cases-record['cases'])/10));cases=itertools.chain(q.r02.generated(cpp,seed,n),((name,b) for name,b,_ in q.r04.generated04(cpp,seed,n) if int.from_bytes(b[4:8],'little') in [18,20,21]))
    digest=hashlib.sha256();count=0
    for name,b in cases:
        if record['cases']>=a.cases:break
        expected=native.call(b);reference=cpp.call(b);record['executions']['native-scalar']+=1;record['executions']['cpp-scalar']+=1
        try:
            if reference[0]!=-3:assert (reference[0]==0)==(expected[0]==0) and (reference[0]!=0 or reference[1]==expected[1]),'C++ scalar'
            for label,d in wasm.items():
                for into in [False,True]:
                    data=bytearray(b)
                    if into:struct.pack_into('<I',data,16,int.from_bytes(data[16:20],'little')|128)
                    result=d.call(data);record['executions'][label]+=1
                    assert result[0]==expected[0] and (result[0]!=0 or result[1]==expected[1]),(label,into)
        except BaseException:
            (q.ART/'wasm-seeded-failure.input').write_bytes(b);(q.ART/'wasm-seeded-failure.output').write_bytes(expected[1]);record['failure']={'seed':seed,'case':name,'ordinal':record['cases']};q.save(path,record);raise
        digest.update(struct.pack('<I',len(b)));digest.update(b);digest.update(struct.pack('<iI',expected[0],len(expected[1])));digest.update(expected[1])
        op=str(int.from_bytes(b[4:8],'little'));record['families'][op]=record['families'].get(op,0)+1;record['cases']+=1;count+=1
    for d in [cpp,native,*wasm.values()]:d.close()
    assert before==sources() and ids=={f.name:q.m.sha(f) for f in q.BIN.iterdir()}
    record['batches'].append({'seed':seed,'generation_records_per_family':n,'cases':count,'stream_sha256':digest.hexdigest()});q.save(path,record);print(record['cases'],'exact seeded cases',flush=True)
    time.sleep(.2) # Release the core between bounded, <=10000-case batches.
record['complete']=True;q.save(path,record)
