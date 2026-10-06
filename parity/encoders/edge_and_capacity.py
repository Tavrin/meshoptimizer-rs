#!/usr/bin/env python3
"""Differential short-capacity and full-u32 edge cases; never timing."""
import run as lane
import struct,random,hashlib
A=lane.ART;ds={k:lane.Driver(A/k) for k in ['rust-before-fair','rust-after','cpp-scalar','cpp']};rows=[]
def check(name,b):
 results={k:d.call(b)[:2] for k,d in ds.items()};expected=results['cpp-scalar']
 if expected[0]==-3:
  assert results['cpp']==expected
  assert results['rust-before-fair']==results['rust-after']
  rows.append({'case':name,'status':'SKIP upstream signed-negation undefined domain','observed_rust_sha256':hashlib.sha256(results['rust-after'][1]).hexdigest()})
  return len(results['rust-after'][1])
 for k,r in results.items():
  assert (r[0]==0)==(expected[0]==0),(name,k,r[0],expected[0])
  if expected[0]==0:assert r==expected,(name,k)
 rows.append({'case':name,'success':expected[0]==0,'output_sha256':hashlib.sha256(expected[1]).hexdigest()})
 return len(expected[1])
rng=random.Random(306101)
for i in range(48):
 n=rng.choice([0,3,6,15,33]);vals=[rng.choice([0,1,2,29,30,127,128,0x7fffffff,0x80000000,0xfffffffe,0xffffffff]) for _ in range(n)]
 if i%3==0:vals=([0xffffffff,0xffffffff,0xffffffff,0,1,2,2,1,0xffffffff]*((n+8)//9))[:n]
 for op in [12,13]:
  for v in [0,1]:
   b=lane.request(op,n,4,struct.pack('<'+'I'*n,*vals),version=v);used=check(f'full-u32-{i}-{op}-{v}',b)
   for c in sorted(set([0,1,max(0,used-1),used,used+1,used+16,n*5+17])):
    check(f'capacity-{i}-{op}-{v}-{c}',lane.request(op,n,4,b[44:],version=v,filter=c+1,mode=128))
for n in [0,1,15,16,17,255,256,257]:
 for s in [4,12,32,252,256]:
  raw=bytes(rng.randrange(256) for _ in range(n*s))
  for v in [0,1]:
   for l in range(10):
    used=check(f'vertex-{n}-{s}-{v}-{l}',lane.request(11,n,s,raw,version=v,level=l))
    for c in sorted(set([0,1,max(0,used-1),used,used+1,used+16])):
     check(f'vertex-cap-{n}-{s}-{v}-{l}-{c}',lane.request(11,n,s,raw,version=v,level=l,filter=c+1,mode=128))
for d in ds.values():d.close()
lane.save('edge-and-capacity',{'cases':rows,'binaries':{k:lane.sha(A/k) for k in ds},'mismatches':0,'seed':306101,'undefined_oracle_rows':sum(r.get('status','').startswith('SKIP') for r in rows)})
print('full-u32 and capacity parity PASS',len(rows),flush=True)
