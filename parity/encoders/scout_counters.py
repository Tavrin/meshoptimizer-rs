#!/usr/bin/env python3
"""Read-only counters from the exact archived 0.2.0 comparison binary."""
import sys,os,json,struct,subprocess,hashlib,ctypes
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];ART=Path('/mnt/linux-extra/moss-scratch/meshopt-v03');OLD=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp');REF=Path('/home/etienne/dev/refs/meshoptimizer')
manifest=json.loads((OLD/'measurement-source.json').read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
files=[p for p in manifest['files'] if p.startswith('src/') or p in ['Cargo.toml','Cargo.lock','parity/compare/src/main.rs','parity/compare/Cargo.toml','parity/compare/Cargo.lock']]
assert all(sha(ROOT/f)==manifest['files'][f] for f in files),'archived source mismatch'
os.environ.update(MESHOPT_REFERENCE=str(REF),CARGO_TARGET_DIR=str(ART),MESHOPT_ARTIFACTS=str(ART),MESHOPT_BENCH_CPU='26')
sys.path.insert(0,str(ROOT/'parity/codec'));from measure import Driver,request
cpp=Path('/mnt/linux-extra/meshopt-artifacts/rel020/binaries/2ae0a9f7cf2485919968f388e1ed3913366ab78b1f003458801ad37657ec3e58');assert sha(cpp)==cpp.name
runbin=OLD/'cmp-driver-defaults-ours';assert sha(runbin)==(OLD/'cmp-driver-defaults-ours.sha256').read_text().strip()
input=OLD/'inputs/medium-smooth.bin';b=input.read_bytes();n,ni=struct.unpack_from('<II',b);raw=b[8:8+n*12];ix=b[8+n*12:8+n*12+ni*4]
f32=lambda x:struct.unpack('<f',struct.pack('<f',x))[0]
lib=ctypes.CDLL('libm.so.6');lib.sinf.argtypes=[ctypes.c_float];lib.sinf.restype=ctypes.c_float;lib.cosf.argtypes=[ctypes.c_float];lib.cosf.restype=ctypes.c_float
floats=[]
for j in range(n):
 a=f32(f32(j)*f32(.01));floats += [f32(lib.sinf(a)*.5),f32(lib.cosf(a)*.5),f32(.75**.5),1.]
pack=lambda f:struct.pack('<'+'f'*len(f),*f)
quats=[f32(x*f32(2**-.5)) for x in floats]
reqs={'vertex-encode':request(11,n,12,raw,version=1),'vertex-encode-v0':request(11,n,12,raw,version=0),'index-encode':request(12,ni,4,ix,version=1),'sequence-encode':request(13,ni,4,ix,version=1),'oct-encode':request(14,n,8,pack(floats),level=12),'quat-encode':request(15,n,8,pack(quats),level=12),'exp-encode':request(16,n,16,pack(floats),level=20)}
perf='/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf';records=[]
for op,req in reqs.items():
 p=ART/('scout-'+op+'.bin');subprocess.run(['taskset','-c','26',str(runbin),op,str(input),str(p)],input=b'stop\n',stdout=subprocess.PIPE,check=True)
 d=Driver(cpp);status,out,_=d.call(req);d.close();assert status==0 and out==p.read_bytes(),op
 totals={}
 for backend in ['rust','cpp']:
  vals=[]
  for it in [1000,2000]:
   stat=ART/f'scout-{op}-{backend}-{it}.stat';cmd=[perf,'stat','-x',';','-e','instructions:u,cycles:u,branches:u,branch-misses:u','-o',str(stat),'--','taskset','-c','26']
   if backend=='rust':cmd +=[str(runbin),op,str(input),str(p)];data=(str(it)+'\nstop\n').encode()
   else:
    cmd +=[str(cpp)];r=bytearray(req);struct.pack_into('<II',r,32,1,it);data=struct.pack('<I',len(r))+r
   z=subprocess.run(cmd,input=data,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert z.returncode==0,z.stderr
   counts={}
   for line in stat.read_text().splitlines():
    v=line.split(';')
    if len(v)>2 and v[2].endswith(':u'):counts[v[2]]=float(v[0]);assert float(v[4])>=99.
   vals.append(counts)
  totals[backend]={k:(vals[1][k]-vals[0][k])/1000 for k in vals[0]}
 records.append({'op':op,'n':n,'indices':ni,'output':sha(p),'counters':totals,'instruction_ratio':totals['rust']['instructions:u']/totals['cpp']['instructions:u']});print(op,records[-1]['instruction_ratio'],flush=True)
(ART/'scout-counters.json').write_text(json.dumps({'binary':sha(runbin),'cpp':sha(cpp),'source_manifest':sha(OLD/'measurement-source.json'),'source_files_verified':files,'input':sha(input),'cpu':26,'timing_assessed':False,'rows':records},indent=2)+'\n')
