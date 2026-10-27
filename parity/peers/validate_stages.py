"""Check the eighteen stage results and compare complete index/error bytes."""
import hashlib,json,math,os,struct
from pathlib import Path
import numpy as np
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
art=Path(os.environ['PEERS_ART']);rows=[json.loads(s) for s in (art/'stage-counts.jsonl').read_text().splitlines()]
assert len(rows)==162 and len({(r['case'],r['op'],r['arm'],r['rep'])for r in rows})==162
identities=[]
for x in json.loads((art/'stage-inputs.json').read_text()):
 case=x['case'];cpp=(art/'outputs'/f'{case}-simplify-cpp-0.bin').read_bytes()
 for arm in ['ours','cpp','rust']:
  b=(art/'outputs'/f'{case}-simplify-{arm}-0.bin').read_bytes();a=np.frombuffer(b[:-4],dtype='<u4');e=struct.unpack('<f',b[-4:])[0]
  assert len(a)%3==0 and len(a)<=x['indices'] and np.all(a<x['vertices']) and math.isfinite(e) and e>=0
  record=dict(case=case,op='simplify',arm=arm,identity='byte-identical' if b==cpp else 'differs',bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
  if b!=cpp:
   n=min(len(b),len(cpp));diff=np.flatnonzero(np.frombuffer(b[:n],dtype='u1')!=np.frombuffer(cpp[:n],dtype='u1'));k=int(diff[0])if len(diff)else n;record.update(first_byte=k,actual_byte=b[k]if k<len(b)else None,cpp_byte=cpp[k]if k<len(cpp)else None,cpp_bytes=len(cpp),cpp_sha256=hashlib.sha256(cpp).hexdigest())
  identities.append(record)
(art/'stage-identities.json').write_text(json.dumps(identities,indent=2)+'\n')
(art/'stage-validation.json').write_text(json.dumps(dict(exit=0,checks=len(identities),samples=len(rows)),indent=2)+'\n')
print('PASS',len(identities),'stage outputs')
