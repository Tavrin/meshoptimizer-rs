"""Pinned C++ valid-input parity: builders, bounds and cluster LOD."""
import hashlib, importlib.util, json, os, shutil, struct, subprocess
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
root=Path(__file__).resolve().parents[2];s=Path(os.environ['BOUNDS_SCRATCH']);d=s/'differential';d.mkdir(exist_ok=True)
ref=Path(os.environ['MESHOPT_REFERENCE']);target=Path(os.environ['CARGO_TARGET_DIR'])
spec=importlib.util.spec_from_file_location('protocol',root/'parity/p03/runner.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
subprocess.run(['bash',str(root/'parity/check-reference.sh')],check=True)
sources=[ref/f'src/{n}.cpp' for n in ['clusterizer','meshletutils','spatialorder','partition','allocator','simplifier','indexgenerator']]
bins={}
for mode in ['scalar','default']:
 b=d/f'cpp-{mode}';flags=r.FLAGS if mode=='scalar' else [f for f in r.FLAGS if f!='-DMESHOPTIMIZER_NO_SIMD']
 subprocess.run(['c++',*flags,'-I'+str(ref/'src'),'-I'+str(ref/'demo'),str(root/'parity/p03/reference.cpp'),*map(str,sources),'-o',str(b)],check=True);bins['cpp' if mode=='scalar' else 'cpp-default']=b
for mode,features in [('scalar',[]),('default',['--features','meshoptimizer-rs/std,meshoptimizer-rs/simd']),('simd-nostd',['--features','meshoptimizer-rs/simd'])]:
 subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(root/'parity/p03/Cargo.toml'),*features,'-j','2'],check=True)
 bins[mode]=d/f'rust-{mode}';shutil.copy2(target/'release/meshopt-p03-driver',bins[mode])
# Scalar and simd128 wasm artifacts are separately built and executed.
for mode,flags,features in [('wasm-scalar','',[]),('wasm-simd','-Ctarget-feature=+simd128',['--features','meshoptimizer-rs/simd'])]:
 env=dict(os.environ,RUSTFLAGS=flags)
 env.pop('CARGO_ENCODED_RUSTFLAGS',None)
 subprocess.run(['cargo','build','--offline','--locked','--release','--lib','--target','wasm32-unknown-unknown','--manifest-path',str(root/'parity/p03/Cargo.toml'),*features,'-j','2'],env=env,check=True)
 bins[mode]=d/f'{mode}.wasm';shutil.copy2(target/'wasm32-unknown-unknown/release/meshopt_p03_parity.wasm',bins[mode])
clients={n:r.Client('wasm' if n.startswith('wasm') else n,b) for n,b in bins.items()}
counts={}
try:
 with (d/'corpus.bin').open('wb') as records:
  for op in [1,3,6,7,8]:
   count=0
   for seed in range(20261029,20261541):
    data=r.case(op,seed);r.compare(clients,data,f'{op}-{seed}',d,records);count+=1
    if op in [1,3]:
     data=bytearray(data);struct.pack_into('<I',data,4,op|0x10000);r.compare(clients,data,f'{op}-{seed}-into',d,records);count+=1
   counts[str(op)]=count
  for seed in range(24):
   p,i=r.grid(2+seed%6,seed%2)
   data=r.message(16,p,i,mv=2+seed%5,min=2+seed%4,mt=[4,8,16,32][seed%4],target=seed*37&511,split=0.1,radii=[j/100 for j in range(len(p))])
   r.compare(clients,data,f'lod-{seed}',d,records)
  counts['16']=24
finally:
 for c in clients.values():c.close()
(d/'result.json').write_text(json.dumps({'exit':0,'counts':counts,'binaries':{n:r.sha(b) for n,b in bins.items()},'sources':{str(p.relative_to(ref)):r.sha(p) for p in sources},'corpus_sha256':r.sha(d/'corpus.bin')},indent=2)+'\n')
print('PASS',counts)
