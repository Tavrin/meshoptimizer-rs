#!/usr/bin/env python3
"""Replay the new owned sink at every available native dispatch ceiling."""
import run as lane
import json,struct,subprocess,hashlib
A=lane.ART;binary=A/'rust-after';levels=subprocess.check_output([str(binary),'levels'],env=lane.ENV,text=True).split();cpp=lane.Driver(A/'cpp-scalar');rows=[]
def check(case,b):
 expected=cpp.call(b)[:2]
 for l in levels:
  p=subprocess.run(['taskset','-c','26',str(binary),l],input=struct.pack('<I',len(b))+b,stdout=subprocess.PIPE,env=lane.ENV,check=True)
  got=lane.decode(p.stdout[4:])[:2];assert (got[0]==0)==(expected[0]==0),(case,l,got[0],expected[0])
  if expected[0]==0:assert got==expected,(case,l)
  rows.append({'case':case,'ceiling':l,'success':got[0]==0,'sha256':hashlib.sha256(got[1]).hexdigest()})
for count in [262143,262144,262145]:
 raw=bytes((j*37+j//4)&255 for j in range(count*4))
 for v in [0,1]:
  for l in [0,2,3,9]:
   st,out,_=cpp.call(lane.request(11,count,4,raw,version=v,level=l));assert st==0
   for api in [0,128]:
    for suffix,source in [('valid',out),('short',out[:-1]),('extra',out+b'\0')]:
     check(f'{count}-v{v}-l{l}-{api}-{suffix}',lane.request(1,count,4,source,mode=api))
frozen=next(r for r in json.loads((A/'inputs.json').read_text()) if r['name']=='vertex-v1-streaming-s4');check('historical-frozen',open(frozen['path'],'rb').read());cpp.close()
lane.save('staging-isa',{'levels':levels,'rows':rows,'binary':lane.sha(binary),'oracle':lane.sha(A/'cpp-scalar'),'mismatches':0})
print('staged owned sink ISA parity PASS',len(rows),levels,flush=True)
