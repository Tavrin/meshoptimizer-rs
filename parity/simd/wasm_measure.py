#!/usr/bin/env python3
"""Node SIMD comparison, using the owner 5–20 pair stopping rule."""
import base64,json,math,os,statistics,subprocess,time
from pathlib import Path
from qualify import ART,ROOT,BIN,REF,admission,interval,corpus,sources,save
from measure import sha
path=ART/'wasm-performance.json'
if path.exists():raise ValueError('final wasm matrix already exists; do not repeat')
before=sources();ids={p.name:sha(p) for p in BIN.iterdir()};cpu=int(os.environ['MESHOPT_BENCH_CPU'])
proc=subprocess.Popen(['taskset','-c',str(cpu),'node',str(ROOT/'parity/simd/wasm_bench.mjs'),str(BIN/'arithmetic-simd.wasm'),str(BIN/'arithmetic-scalar.wasm'),str(REF/'js/meshopt_decoder.mjs')],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
record={'cpu':cpu,'method':'Node public upstream shipped SIMD decoder; matching source/output copies, validation and allocation; owner 5–20 paired early stopping; borderline-only 30 fresh D146 pairs','sources':before,'binaries':ids,'rows':[],'admissions':[]};burst=time.monotonic()
for into in [0,1]:
    for name,b in corpus():
        op=int.from_bytes(b[4:8],'little')
        if op not in [1,2,3,7]:continue # Upstream JS has no standalone filters or meshlet API.
        q={'input':base64.b64encode(b).decode(),'into':into,'pair':0,'iterations':1}
        def call():
            proc.stdin.write(json.dumps(q)+'\n');proc.stdin.flush();line=proc.stdout.readline();assert line,'Node benchmark stopped';return json.loads(line)
        while not (gate:=admission())['admitted']:record['admissions'].append(gate);save(path,record);time.sleep(15)
        trial=call();q['iterations']=max(1,min(10000,math.ceil(.004/max(trial['times']['cpp'],1e-9))))
        row={'case':name,'api':'caller-buffer' if into else 'allocating','raw_seconds':{'rust':[],'cpp':[],'scalar':[]},'telemetry':[],'iterations':q['iterations']}
        while len(row['raw_seconds']['rust'])<20:
            while not (gate:=admission())['admitted']:save(path,record);time.sleep(15)
            q['pair']=len(row['raw_seconds']['rust']);result=call();after=admission()
            if not after['admitted']:continue
            for k in row['raw_seconds']:row['raw_seconds'][k].append(result['times'][k])
            row['telemetry'].append({'before':gate,'after':after,'order':result['order']})
            if len(row['raw_seconds']['rust'])>=5:
                ci=interval([x/y for x,y in zip(row['raw_seconds']['rust'],row['raw_seconds']['cpp'])])
                if ci[1]<=1.60 or ci[0]>1.60:break
        row['interval']=ci
        if ci[0]<=1.60<ci[1]:
            stage={k:[] for k in row['raw_seconds']}
            while len(stage['rust'])<30:
                while not admission()['admitted']:time.sleep(15)
                q['pair']=len(stage['rust']);result=call()
                if not admission()['admitted']:continue
                for k in stage:stage[k].append(result['times'][k])
            row['stage2']={'raw_seconds':stage,'interval':interval([x/y for x,y in zip(stage['rust'],stage['cpp'])])};row['stage1_interval']=row['interval'];row['interval']=row['stage2']['interval']
        med={k:statistics.median(v) for k,v in row['raw_seconds'].items()};row['time_ratio']=med['rust']/med['cpp'];row['verdict']='pass' if row['interval'][1]<=1.60 else 'fail';row['scalar_time_ratio']=med['rust']/med['scalar']
        record['rows'].append(row);save(path,record);print(into,name,round(row['time_ratio'],3),len(row['raw_seconds']['rust']),row['verdict'],flush=True)
        if time.monotonic()-burst>780:print('release core between bursts',flush=True);time.sleep(10);burst=time.monotonic()
proc.stdin.close();assert proc.wait()==0 and before==sources() and ids=={p.name:sha(p) for p in BIN.iterdir()}
record['complete']=True;save(path,record)
