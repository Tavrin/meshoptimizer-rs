#!/usr/bin/env python3
"""Finish only S4 scalar-baseline borderline intervals; keep the matrix fixed."""
import hashlib,json,struct,sys,time
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent/'simd'))
import qualify as q
path=q.ART/'s4-assessment.json';assert not path.exists()
native=json.loads((q.ART/'performance.json').read_text())
assert native['complete'] and native['sources']==q.sources()
assert json.loads((q.ART/'wasm-performance.json').read_text())['complete']
before=q.sources();ids={p.name:q.m.sha(p) for p in q.BIN.iterdir()}
assert ids==native['binaries']
inputs=dict(q.corpus());record={'sources':before,'binaries':ids,'controller_sha256':q.m.sha(__file__),'native_sha256':q.m.sha(q.ART/'performance.json'),'method':'S4 only: preserve stage-1 means; top up scalar-C++ borderline cases to 5–20 total pairs; only remaining borderline cases get 30 fresh D146 pairs','rows':[]}
ds={'rust':q.Rust(),'cpp-scalar':q.m.Driver(q.BIN/'cpp-scalar')};burst=time.monotonic()
try:
 for row in native['rows']:
  if row['family'] not in ['index','sequence']:continue
  raw={k:list(row['raw_seconds'][k]) for k in ds}
  ci=q.interval([x/y for x,y in zip(raw['rust'],raw['cpp-scalar'])])
  out={'api':row['api'],'case':row['case'],'original_interval':ci,'original_pairs':len(raw['rust']),'input_sha256':hashlib.sha256(inputs[row['case']]).hexdigest(),'additional_raw_seconds':{k:[] for k in ds},'telemetry':[],'discarded':[]}
  if ci[0]<=1.5<ci[1]:
   probe=bytearray(inputs[row['case']])
   if row['api']=='caller-buffer':struct.pack_into('<I',probe,16,struct.unpack_from('<I',probe,16)[0]|128)
   struct.pack_into('<I',probe,32,1);struct.pack_into('<I',probe,36,row['iterations'])
   def pair(n,stage):
    while not (gate:=q.admission())['admitted']:time.sleep(2)
    order=list(ds);order=order[n%2:]+order[:n%2];times={};outputs={}
    for k in order:
     status,data,t=ds[k].call(probe);assert status==0;times[k]=t[0];outputs[k]=data
    assert outputs['rust']==outputs['cpp-scalar']
    after=q.admission();telemetry={'before':gate,'after':after,'order':order,'stage':stage}
    if not after['admitted']:out['discarded'].append({'raw_seconds':times,**telemetry});return None
    out['telemetry'].append(telemetry);return times
   # Untimed setup uses the same input and fixed iteration count.
   while not q.admission()['admitted']:time.sleep(2)
   for d in ds.values():assert d.call(probe)[0]==0
   while len(raw['rust'])<20 and ci[0]<=1.5<ci[1]:
    result=pair(len(raw['rust']),1)
    if result is None:continue
    for k in ds:raw[k].append(result[k]);out['additional_raw_seconds'][k].append(result[k])
    ci=q.interval([x/y for x,y in zip(raw['rust'],raw['cpp-scalar'])])
   out['stage1_interval']=ci
   if ci[0]<=1.5<ci[1]:
    stage={k:[] for k in ds}
    while len(stage['rust'])<30:
     result=pair(len(stage['rust']),2)
     if result is None:continue
     for k in ds:stage[k].append(result[k])
    out['stage2']={'raw_seconds':stage,'interval':q.interval([x/y for x,y in zip(stage['rust'],stage['cpp-scalar'])])};ci=out['stage2']['interval']
  out['interval']=ci;out['verdict']='pass' if ci[1]<=1.5 else 'fail' if ci[0]>1.5 else 'borderline'
  record['rows'].append(out);q.save(path,record)
  print(row['api'],row['case'],out['verdict'],'additional',len(out['additional_raw_seconds']['rust']),flush=True)
  if time.monotonic()-burst>780:
   for d in ds.values():d.close()
   time.sleep(10);ds={'rust':q.Rust(),'cpp-scalar':q.m.Driver(q.BIN/'cpp-scalar')};burst=time.monotonic()
finally:
 for d in ds.values():d.close()
assert before==q.sources() and ids=={p.name:q.m.sha(p) for p in q.BIN.iterdir()}
assert q.m.sha(__file__)==record['controller_sha256'] and q.m.sha(q.ART/'performance.json')==record['native_sha256']
record['complete']=True;q.save(path,record)
