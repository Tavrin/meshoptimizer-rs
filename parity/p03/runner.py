#!/usr/bin/env python3
"""Exact phase 0.3 evidence; unmodified pinned scalar C++ and executed WASM."""
import argparse,base64,hashlib,json,math,os,platform,random,statistics,struct,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
REF=Path(os.environ['MESHOPT_REFERENCE'])
TARGET=Path(os.environ.get('CARGO_TARGET_DIR','/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p03'))
ART=Path(os.environ.get('MESHOPT_ARTIFACTS','/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p03'))
ENV={**os.environ,'MESHOPT_REFERENCE':str(REF),'CARGO_TARGET_DIR':str(TARGET),'RUSTC_WRAPPER':'','RUSTC_WORKSPACE_WRAPPER':'','RUSTFLAGS':'','CARGO_ENCODED_RUSTFLAGS':'','CARGO_NET_OFFLINE':'true','GIT_OPTIONAL_LOCKS':'0'}
NAMES=['build_meshlets','build_meshlets_scan','build_meshlets_flex','build_meshlets_spatial','build_meshlets_bound','compute_cluster_bounds','compute_meshlet_bounds','compute_sphere_bounds','optimize_meshlet','optimize_meshlet_level','extract_meshlet_indices','partition_clusters','spatial_sort_remap','spatial_sort_triangles','spatial_cluster_points']
FIXTURES=['js-clusterizer-1-0.input', 'js-clusterizer-1-7.input', 'js-clusterizer-6-1.input', 'js-clusterizer-6-2.input', 'js-clusterizer-6-3.input', 'js-clusterizer-6-4.input', 'js-clusterizer-6-5.input', 'js-clusterizer-6-6.input', 'js-clusterizer-7-10.input', 'js-clusterizer-7-11.input', 'js-clusterizer-7-12.input', 'js-clusterizer-7-13.input', 'js-clusterizer-7-8.input', 'js-clusterizer-7-9.input', 'js-clusterizer-8-14.input', 'js-clusterizer-8-15.input', 'native-clusterBoundsDegenerate-0.input', 'native-clusterBoundsDegenerate-1.input', 'native-clusterBoundsDegenerate-2.input', 'native-clusterBoundsDegenerate-3.input', 'native-extractMeshlet-17.input', 'native-meshletsDense-7.input', 'native-meshletsEmpty-6.input', 'native-meshletsFlex-10.input', 'native-meshletsFlex-11.input', 'native-meshletsFlex-12.input', 'native-meshletsFlex-13.input', 'native-meshletsFlex-14.input', 'native-meshletsFlex-9.input', 'native-meshletsMax-15.input', 'native-meshletsMax-16.input', 'native-meshletsSparse-8.input', 'native-meshletsSpatial-18.input', 'native-meshletsSpatial-19.input', 'native-meshletsSpatial-20.input', 'native-meshletsSpatial-21.input', 'native-meshletsSpatialDeep-22.input', 'native-meshletsSpatialDeep-23.input', 'native-partitionBasic-24.input', 'native-partitionBasic-25.input', 'native-partitionBasic-26.input', 'native-partitionSpatial-27.input', 'native-partitionSpatial-28.input', 'native-partitionSpatialMerge-29.input', 'native-partitionSpatialMerge-30.input', 'native-sphereBounds-4.input', 'native-sphereBounds-5.input']
# D78: native Rust driver profile: release (fat LTO), consumer (thin LTO,
# codegen-units=1, opt-level=3) or release-defaults (Cargo's release defaults).
PROFILE=os.environ.get('MESHOPT_RUST_PROFILE','release')
if PROFILE not in {'release','consumer','release-defaults'}:raise SystemExit(f'unknown MESHOPT_RUST_PROFILE {PROFILE}')
FLAGS=['-std=c++17','-O3','-DMESHOPTIMIZER_NO_SIMD','-fno-fast-math','-ffp-contract=off']
def command(a,**kw):return subprocess.run([str(x) for x in a],check=True,env=ENV,**kw)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def snapshot():return {str(p):sha(p) for base in [ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'src',ROOT/'tests/meshlets.rs',ROOT/'parity/p03',REF/'src',REF/'demo/clusterlod.h'] for p in ([base] if base.is_file() else sorted(base.rglob('*'))) if p.is_file() and p.suffix in {'.rs','.cpp','.h','.toml','.lock','.py','.mjs'}}
def build(wasm=True):
 command([ROOT/'parity/check-reference.sh']);before=snapshot();TARGET.mkdir(parents=True,exist_ok=True);ART.mkdir(parents=True,exist_ok=True)
 cpp=TARGET/'p03-reference';sources=[REF/f'src/{n}.cpp' for n in ['clusterizer','meshletutils','spatialorder','partition','allocator','simplifier','indexgenerator']]
 command(['c++',*FLAGS,'-I',REF/'src','-I',REF/'demo',ROOT/'parity/p03/reference.cpp',*sources,'-o',cpp]);command(['cargo','build','--offline','--profile',PROFILE,'--manifest-path',ROOT/'parity/p03/Cargo.toml'])
 bins={'cpp':cpp,'rust':TARGET/PROFILE/'meshopt-p03-driver'}
 if not wasm:
  simd=TARGET/'p03-reference-simd';command(['c++',*[f for f in FLAGS if f!='-DMESHOPTIMIZER_NO_SIMD'],'-I',REF/'src','-I',REF/'demo',ROOT/'parity/p03/reference.cpp',*sources,'-o',simd]);bins['cpp-simd']=simd
 if wasm:
  fixtures=TARGET/'p03-fixtures';fixtures.mkdir(exist_ok=True)
  for p in fixtures.glob('*.input'):p.unlink()
  source=TARGET/'p03-native-fixtures.cpp';command(['python3',ROOT/'parity/p03/native-fixtures.py',REF,source]);native=TARGET/'p03-native-fixtures';command(['c++',*FLAGS,'-I',REF/'src',source,*sources,'-o',native]);command([native,fixtures]);command(['node',ROOT/'parity/p03/js-fixtures.mjs',REF,fixtures])

 if wasm:command(['cargo','build','--offline','--release','--target','wasm32-unknown-unknown','--manifest-path',ROOT/'parity/p03/Cargo.toml','--lib']);bins['wasm']=TARGET/'wasm32-unknown-unknown/release/meshopt_p03_parity.wasm'
 if before!=snapshot():raise RuntimeError('source changed during build')
 return bins,{'sources':before,'executables':{n:sha(p) for n,p in bins.items()},'reference':'4c203430ca565cb59a468a91922c76c208169536','flags':FLAGS,'rust_profile':PROFILE,'rustc':command(['rustc','-Vv'],capture_output=True,text=True).stdout,'cxx':command(['c++','--version'],capture_output=True,text=True).stdout,'runtime':command(['node','--version'],capture_output=True,text=True).stdout,'hardware':{k:v for k,v in platform.uname()._asdict().items() if k!='node'},'math':'libm 0.2.16 generic software sqrtf; per-call exact operand memo','fp':'FE_TONEAREST, gradual underflow; scalar strict'}
class Client:
 def __init__(self,name,p):self.wasm=name=='wasm';self.proc=subprocess.Popen(['node',str(ROOT/'parity/wasm.cjs'),str(p)] if self.wasm else [str(p)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,env=ENV)
 def run(self,data):
  if self.wasm:self.proc.stdin.write(base64.b64encode(data)+b'\n');self.proc.stdin.flush();out=base64.b64decode(self.proc.stdout.readline())
  else:
   self.proc.stdin.write(struct.pack('<I',len(data))+data);self.proc.stdin.flush();hdr=self.proc.stdout.read(4)
   if len(hdr)!=4:raise RuntimeError('driver exited')
   out=self.proc.stdout.read(struct.unpack('<I',hdr)[0])
  if out[:4]!=b'MR03':raise RuntimeError('bad response')
  n=struct.unpack_from('<I',out,4)[0]
  return out[8:8+n],struct.unpack_from('<dQ',out,8+n)
 def close(self):self.proc.stdin.close();self.proc.wait();assert self.proc.returncode==0

def message(op,p,idx,mv=64,min=16,mt=64,weight=0.,split=0.,target=4,counts=(),radii=None,repeats=0):
 if radii is None:radii=[0.]*len(p)
 return struct.pack('<4s6I2f3I',b'MO03',op,len(p),len(idx),mv,min,mt,weight,split,target,len(counts),repeats)+b''.join(struct.pack('<3f',*v) for v in p)+struct.pack('<'+'I'*len(idx),*idx)+struct.pack('<'+'I'*len(counts),*counts)+struct.pack('<'+'f'*len(p),*radii)
def grid(n,kind=0):
 p=[(float(x),float(y),math.sin(x*.4)*math.cos(y*.3) if kind else 0.) for y in range(n) for x in range(n)];idx=[]
 for y in range(n-1):
  for x in range(n-1):a=y*n+x;idx.extend([a,a+1,a+n,a+n,a+1,a+n+1])
 return p,idx

def case(op,seed):
 r=random.Random(seed);n=r.randrange(0,100);p=[(r.uniform(-10,10),r.uniform(-10,10),r.uniform(-10,10)) for _ in range(n)];idx=[r.randrange(n) for _ in range(r.randrange(0,100)*3)] if n else []
 if seed%5==0:p,idx=grid(r.randrange(2,12),seed%2);n=len(p)
 if seed%7==0:p=[(float(i%4),0.,0.) for i in range(n)]
 if seed%11==0:p=[(0.,0.,0.)]*n
 if seed%13==0 and op not in [5,9,10,11,12]:
  scale=2.**[-64,-30,0,16][seed%4];p=[tuple(x*scale for x in v) for v in p]
 if op in [6,7,9,10,11]:idx=idx[:1536]
 mv=r.choice([3,4,16,32,64,128,256]);mt=r.choice([1,2,4,16,64,124,512]);min=r.randrange(1,mt+1);weight=r.choice([0.,.5,1.]);split=r.choice([0.,.1,1.,2.,10.]);target=r.randrange(1,50);counts=[]
 if op==8:target=seed%2
 if op==10:target=seed%10
 if op==11 and seed%3==0:idx=[(v*1024+0xffff0000)&0xffffffff for v in idx]
 if op==12:
  target=r.randrange(1,20)|(0x80000000 if seed%2 else 0);at=0
  while at<len(idx):size=min_builtin(r.randrange(1,30),len(idx)-at);counts.append(size);at+=size
 return message(op,p,idx,mv,min,mt,weight,split,target,counts,[r.random()*2 for _ in p])
min_builtin=min

def compare(clients,data,label,archive,records):
 outputs={n:c.run(data)[0] for n,c in clients.items()};expected=outputs['cpp']
 for n,out in outputs.items():
  if out!=expected:
   (archive/f'{label}.input').write_bytes(data)
   for name,value in outputs.items():(archive/f'{label}.{name}').write_bytes(value)
   pos=next((i for i,(a,b) in enumerate(zip(out,expected)) if a!=b),min(len(out),len(expected)))
   raise RuntimeError(f'{label}: {n} differs at byte {pos}; lengths {len(out)}/{len(expected)}')
 # Retain every input and meaningful output; compact length-delimited corpus.
 records.write(struct.pack('<II',len(data),len(expected)));records.write(data);records.write(expected)

def main():
 parser=argparse.ArgumentParser();parser.add_argument('action',choices=['run','sweep','benchmark','clusterlod']);parser.add_argument('--phase',default='0.3');parser.add_argument('--profile',default='scalar-strict');parser.add_argument('--cases-per-family',type=int,default=2000);parser.add_argument('--seed',type=int,default=20261004);parser.add_argument('--enforce',action='store_true');args=parser.parse_args()
 if args.action=='benchmark':
  import sys
  os.execv(sys.executable,[sys.executable,str(ROOT/'parity/p03/benchmark.py'),*sys.argv[2:]])
 bins,identity=build(args.action!='benchmark');clients={n:Client(n,p) for n,p in bins.items()};folder=ART/args.action;folder.mkdir(exist_ok=True);counts={};start=time.time()
 try:
  with (folder/'corpus.bin').open('wb') as records:
   if args.action=='run':
    fixtures=sorted((TARGET/'p03-fixtures').glob('*.input'))
    if [p.name for p in fixtures]!=FIXTURES:raise RuntimeError('required upstream fixture inventory changed')
    for fixture in fixtures:compare(clients,fixture.read_bytes(),fixture.stem,folder,records)
    counts['upstream-fixtures']=len(fixtures)

   for op,name in ([(16,'clusterlod')] if args.action=='clusterlod' else enumerate(NAMES,1)):
    if op==16:
     for seed in range(80):
      p,idx=grid(2+seed%12,seed%2)
      target=seed*37&511;extra=[]
      if seed>=32:
       target|=1024|2048|4096|((seed%10)<<13)
       if seed%2:
        width=[1,2,8,12,32][seed%5];target|=(width-1)<<20
        extra=[0] if width==1 else [struct.unpack('<I',struct.pack('<f',(i%7+k)/100))[0] for i in range(len(p)) for k in range(width)]
        if seed%3==0:target|=512
       if seed%4==0:
        old=len(p);p+=p[:];idx=[v+old*(i//6%2) for i,v in enumerate(idx)]
        if extra:
         width=((target>>20)&31)+1;extra=[0] if width==1 else [struct.unpack('<I',struct.pack('<f',(i%7+k)/100))[0] for i in range(len(p)) for k in range(width)]
      # The C++ demo takes &vector[0] on empty storage. Check the Rust empty
      # boundary in focused tests, rather than invoking that undefined case.
      if seed==79:p,idx=grid(2);extra=[];target=0
      data=message(16,p,idx,mv=2+seed%5,min=2+seed%4,mt=[4,8,16,32][seed%4],target=target,split=0.1,counts=extra,radii=[i/100 for i in range(len(p))]);compare(clients,data,f'clusterlod-{seed}',folder,records)
     counts[name]=80;print(name,80,flush=True);continue
    seeds=range(args.seed,args.seed+(args.cases_per_family if args.action=='sweep' else 25))
    for seed in seeds:
     data=case(op,seed);compare(clients,data,f'{name}-{seed}',folder,records)
     if op not in [5,6,7,8]:
      into=bytearray(data);struct.pack_into('<I',into,4,op|0x10000);compare(clients,into,f'{name}-{seed}-into',folder,records)
    counts[name]=len(seeds);print(name,counts[name],flush=True)
  if identity['sources']!=snapshot() or identity['executables']!={n:sha(p) for n,p in bins.items()}:raise RuntimeError('identity changed')
  record={'schema':'meshopt-p03/1','action':args.action,'seed':args.seed,'cases':counts,'variants':'allocating and into for every buffer-returning function; scalar value APIs otherwise','fixtures':FIXTURES if args.action=='run' else [],'mismatches':0,'elapsed':time.time()-start,'identity':identity,'corpus_sha256':sha(folder/'corpus.bin')};(folder/'record.json').write_text(json.dumps(record,indent=2)+'\n')
 finally:
  for c in clients.values():c.close()
if __name__=='__main__':main()
