#!/usr/bin/env python3
"""Encoder lane: immutable epochs, untimed parity/counters, one final paired campaign."""
from pathlib import Path
import os, sys, json, hashlib, subprocess, shutil, struct, random, math, statistics, time
ROOT=Path(__file__).resolve().parents[2]
ART=Path('/mnt/linux-extra/moss-scratch/meshopt-v03')
TARGET=Path('/mnt/linux-extra/moss-cargo-targets/codex-meshopt-v03')
REF=Path('/home/etienne/dev/refs/meshoptimizer')
os.environ.update(MESHOPT_REFERENCE=str(REF),CARGO_TARGET_DIR=str(TARGET),MESHOPT_ARTIFACTS=str(ART),CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='0',MESHOPT_BENCH_CPU='26')
sys.path.insert(0,str(ROOT/'parity/codec'))
from measure import request,Driver,sha,build_cpp,decode,ENV
ENV.update(CARGO_BUILD_JOBS='4',CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='0')
PERF=Path('/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf')
def save(name,obj): (ART/(name+'.json')).write_text(json.dumps(obj,indent=2)+'\n')
def run(args): subprocess.run(list(map(str,args)),cwd=ROOT,env=ENV,check=True)
def build(epoch):
 assert os.environ.get('MOSS_HEAVY_ACTIVE')
 assert shutil.disk_usage(ART).free>=25*1024**3,'build floor 25 GiB'
 TARGET.mkdir(parents=True,exist_ok=True);(TARGET/'tmp').mkdir(exist_ok=True)
 src={str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'src').rglob('*.rs')}
 run(['cargo','build','--offline','--locked','--release','--features','simd','--manifest-path',ROOT/'parity/codec/Cargo.toml'])
 shutil.copy2(TARGET/'release/codec-driver',ART/f'rust-{epoch}')
 if epoch=='before':
  run([ROOT/'parity/check-reference.sh'])
  for scalar,name in [(True,'cpp-scalar'),(False,'cpp')]:
   run(['c++','-std=c++17','-O3','-DNDEBUG','-fno-fast-math','-ffp-contract=off',*(['-DMESHOPTIMIZER_NO_SIMD'] if scalar else []),'-I',REF/'src',ROOT/'parity/encoders/reference.cpp',*[REF/'src'/f for f in ['vertexcodec.cpp','indexcodec.cpp','vertexfilter.cpp','meshletcodec.cpp','allocator.cpp']],'-o',ART/name])
  run(['cargo','generate-lockfile','--offline','--manifest-path',ROOT/'parity/encoders/Cargo.toml'])
  run(['cargo','build','--offline','--locked','--release','--manifest-path',ROOT/'parity/encoders/Cargo.toml'])
  shutil.copy2(TARGET/'release/encoder-crate-oracle',ART/'crate')
 assert src=={str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'src').rglob('*.rs')}
 save('build-'+epoch,{'source':src,'revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'binary':sha(ART/f'rust-{epoch}'),'reference':'4c203430ca565cb59a468a91922c76c208169536','compiler':subprocess.check_output(['rustc','-Vv'],text=True),'admission':os.environ['MOSS_HEAVY_ACTIVE']})
 print('built',epoch,flush=True)
def prepare():
 rng=random.Random(20261006);cases=[];directory=ART/'inputs';directory.mkdir(exist_ok=True)
 def add(name,b):
  p=directory/(name+'.input');p.write_bytes(b);cases.append({'name':name,'path':str(p),'sha256':sha(p)})
 for n,label in [(17,'tiny'),(4097,'resident'),(262145,'streaming')]:
  for stride in [4,12,32]:
   data=bytes((i//stride+(i%stride)*13)&255 for i in range(n*stride))
   for v,l in [(0,2),(1,0),(1,1),(1,2),(1,3)]:add(f'vertex-{label}-s{stride}-v{v}-l{l}',request(11,n,stride,data,version=v,level=l))
  indices=[i%60000 for i in range(n//3*3)];raw=struct.pack('<'+'I'*len(indices),*indices)
  for op,f in [(12,'index'),(13,'sequence')]:
   for v in [0,1]:add(f'{f}-{label}-v{v}',request(op,len(indices),4,raw,version=v))
  # Varied, normalized inputs; avoid the upstream undefined conversion domain.
  normals=[];quats=[];values=[]
  for i in range(n):
   q=[rng.uniform(-1,1) for _ in range(4)];length=math.sqrt(sum(x*x for x in q));quats.extend(x/length for x in q)
   q=q[:3];length=math.sqrt(sum(x*x for x in q));normals.extend([*(x/length for x in q),1. if i%2 else -1.])
   values.extend([rng.uniform(-1e6,1e6),0. if i%7==0 else rng.uniform(-1,1),rng.uniform(-100,100)])
  pack=lambda x:struct.pack('<'+'f'*len(x),*x)
  for stride,bits in [(4,8),(8,12)]:add(f'oct-{label}-s{stride}',request(14,n,stride,pack(normals),level=bits))
  add(f'quat-{label}',request(15,n,8,pack(quats),level=12))
  for m in range(4):add(f'exp-{label}-m{m}',request(16,n,12,pack(values),mode=m,level=15))
 with_driver=Driver(ART/'cpp')
 frozen=Path('/mnt/linux-extra/meshopt-artifacts/p07-fix10')
 metadata=next(r for r in json.loads((frozen/'inputs.json').read_text()) if r['case']=='vertex-v1-streaming-s4')
 p=frozen/metadata['path'];assert sha(p)==metadata['sha256']
 add('vertex-v1-streaming-s4',p.read_bytes());with_driver.close()
 save('inputs',cases);print('prepared',len(cases),flush=True)
def parity(epoch):
 ds={k:Driver(ART/k) for k in ['cpp-scalar','cpp','crate','rust-'+epoch]};records=[]
 def check(name,b):
  results={k:d.call(b)[:2] for k,d in ds.items()};r=results['rust-'+epoch];c=results['cpp-scalar'];assert r==c,(name,{k:(x[0],len(x[1])) for k,x in results.items()})
  assert results['cpp']==c,(name,'SIMD oracle')
  # 0.25 differs at vertex level 3; retain that fact without claiming parity.
  records.append({'name':name,'status':r[0],'sha256':hashlib.sha256(r[1]).hexdigest(),'crate_equal':results['crate']==r})
 for row in json.loads((ART/'inputs.json').read_text()):
  b=Path(row['path']).read_bytes();check(row['name'],b)
  b=bytearray(b);struct.pack_into('<I',b,16,struct.unpack_from('<I',b,16)[0]|128);check(row['name']+'-into',b)
 rng=random.Random(3110606)
 for i in range(240):
  n=rng.choice([0,1,2,15,16,17,255,256,257,513]);s=rng.choice([4,8,12,32,128,252,256]);kind=i%4
  raw=bytes(rng.randrange(256) if kind==0 else (j//s*(kind+1)+j%s*13)&255 if kind<3 else 0 for j in range(n*s))
  for v in [0,1]:
   for l in range(10):
    req=request(11,n,s,raw,version=v,level=l)
    check(f'vertex-seed-{i}-v{v}-l{l}',req)
    req=bytearray(req);struct.pack_into('<I',req,16,128);check(f'vertex-seed-{i}-v{v}-l{l}-into',req)
  if i%40==0:print('parity',epoch,i,flush=True)
 for i in range(120):
  n=rng.choice([0,1,3,4,5,15,16,17,33]);norms=[];quats=[]
  for k in range(n):
   q=[rng.uniform(-1,1) for _ in range(4)];length=math.sqrt(sum(x*x for x in q));quats.extend(x/length for x in q)
   length=math.sqrt(sum(x*x for x in q[:3]));norms.extend([*(x/length for x in q[:3]),1. if k%2 else -1.])
  fp=lambda x:struct.pack('<'+'f'*len(x),*x)
  tests=[request(14,n,4,fp(norms),level=rng.randrange(2,9)),request(14,n,8,fp(norms),level=rng.randrange(2,17)),request(15,n,8,fp(quats),level=rng.randrange(4,17))]
  for stride in [4,8,12,16,32,256]:
   values=[0. if k%5==0 else rng.uniform(-1024,1024) for k in range(n*(stride//4))]
   for mode in range(4):tests.append(request(16,n,stride,fp(values),mode=mode,level=i%24+1))
  ix=[rng.randrange(1<<29) if i%3==0 else k//3 if i%3==1 else 0 for k in range(n//3*3)]
  for v in [0,1]:
   tests.append(request(12,len(ix),4,struct.pack('<'+'I'*len(ix),*ix),version=v));tests.append(request(13,len(ix),4,struct.pack('<'+'I'*len(ix),*ix),version=v))
  for j,req in enumerate(tests):
   check(f'filter-index-seed-{i}-{j}',req);req=bytearray(req);struct.pack_into('<I',req,16,struct.unpack_from('<I',req,16)[0]|128);check(f'filter-index-seed-{i}-{j}-into',req)
 for d in ds.values():d.close()
 save('parity-'+epoch,{'binaries':{k:sha(ART/k) for k in ds},'cases':records,'mismatches':0});print('parity PASS',len(records),flush=True)
def counters(epoch):
 assert os.environ.get('MOSS_HEAVY_ACTIVE')
 rows=json.loads((ART/'inputs.json').read_text());records=[]
 selected=[r for r in rows if ('resident' in r['name'] and ('-l2' in r['name'] or not r['name'].startswith('vertex'))) or r['name']=='vertex-v1-streaming-s4']
 family=os.environ.get('MESHOPT_COUNTER_FAMILY')
 if family:selected=[r for r in selected if r['name'].startswith(family+'-')]
 for row in selected:
  for into in [False,True]:
   for backend in ['cpp','rust-'+epoch]:
    totals=[]
    for it in [50,100] if row['name']=='vertex-v1-streaming-s4' else [300,600]:
     b=bytearray(Path(row['path']).read_bytes());struct.pack_into('<I',b,32,1);struct.pack_into('<I',b,36,it)
     if into:struct.pack_into('<I',b,16,struct.unpack_from('<I',b,16)[0]|128)
     key=f'{epoch}-{row["name"]}-{into}-{backend}-{it}';stat=ART/(key+'.stat');response=ART/(key+'.response')
     p=subprocess.run([str(PERF),'stat','-x',';','-e','instructions:u,cycles:u,branches:u,branch-misses:u','-o',str(stat),'--','taskset','-c','26',str(ART/backend)],input=struct.pack('<I',len(b))+b,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=ENV)
     assert p.returncode==0,p.stderr.decode();response.write_bytes(p.stdout); assert decode(p.stdout[4:])[0]==0
     counts={}
     for line in stat.read_text().splitlines():
      v=line.split(';')
      if len(v)>2 and v[2].endswith(':u'):counts[v[2]]=float(v[0]);assert float(v[4])>=99.0,v
     assert len(counts)==4,stat.read_text();totals.append(counts)
    delta={k:(totals[1][k]-totals[0][k])/(50 if row['name']=='vertex-v1-streaming-s4' else 300) for k in totals[0]}
    records.append({'case':row['name'],'into':into,'backend':backend,'per_call':delta})
  print('counters',epoch,row['name'],flush=True)
 save('counters-'+epoch,{'rows':records,'cpu':26,'timing_assessed':False,'binary':{k:sha(ART/k) for k in ['cpp','rust-'+epoch]}})
if __name__=='__main__':
 action=sys.argv[1];epoch=sys.argv[2] if len(sys.argv)>2 else 'before'
 {'build':lambda:build(epoch),'prepare':prepare,'parity':lambda:parity(epoch),'counters':lambda:counters(epoch)}[action]()
