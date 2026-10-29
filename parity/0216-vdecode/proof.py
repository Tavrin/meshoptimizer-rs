"""Exact resource-contract and ISA/C++ checks, bounded to two CPU-hours."""
import hashlib,io,json,os,resource,shutil,subprocess,tarfile,time
from pathlib import Path
R=Path.cwd();T=Path(os.environ['CARGO_TARGET_DIR']);A=Path(os.environ['DECODE_ART']);A.mkdir(parents=True,exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
base=T/'proof-baseline';base.mkdir(exist_ok=True,parents=True)
archive=subprocess.check_output(['git','archive','v0.2.15','src','docs','Cargo.toml','Cargo.lock','README.md'])
tarfile.open(fileobj=io.BytesIO(archive)).extractall(base,filter='data')
for name in ['Cargo.toml','Cargo.lock']:
 p=base/name;p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
proof=T/'proof-source';proof.mkdir(exist_ok=True)
for name in ['exact.rs','fixtures.rs']:shutil.copy2(R/'parity/0216-vdecode'/name,proof/name)
(proof/'Cargo.toml').write_text('''[package]
name="decode-exact"
version="0.0.0"
edition="2021"
[features]
default=["baseline/std","candidate/std","baseline/simd","candidate/simd"]
[dependencies]
baseline={package="meshoptimizer-baseline",path="../proof-baseline",default-features=false}
candidate={package="meshoptimizer-rs",path='''+repr(str(R))+''',default-features=false}
[[bin]]
name="exact"
path="exact.rs"
[profile.release]
lto="fat"
codegen-units=1
''')
exits=[]
def step(name,args,env=None):
 before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.monotonic()
 with (A/(name+'.log')).open('x') as f:
  p=subprocess.run(args,stdout=f,stderr=subprocess.STDOUT,env=dict(os.environ,**(env or {})))
 if p.returncode==0 and name.startswith(('exact-','build-differential')):
  binaries=A/'binaries';binaries.mkdir(exist_ok=True)
  paths=[T/'exact/release/exact'] if name.startswith('exact-') else [T/'proof/release/differential',T/'cpp-scalar-exact',T/'cpp-default-exact']
  binding={}
  for binary in paths:
   dest=binaries/(name+'-'+binary.name);shutil.copy2(binary,dest)
   binding[dest.name]=hashlib.sha256(dest.read_bytes()).hexdigest()
  (A/(name+'-binaries.json')).write_text(json.dumps(binding,indent=2)+'\n')
  manifest=T/('proof-source' if name.startswith('exact-') else 'differential-source')/'Cargo.toml'
  (A/(name+'-manifest.txt')).write_text(manifest.read_text())
 after=resource.getrusage(resource.RUSAGE_CHILDREN)
 exits.append(dict(step=name,exit=p.returncode,wall_seconds=time.monotonic()-start,cpu_seconds=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime))
 (A/'exits.json').write_text(json.dumps(exits,indent=2)+'\n');print(name,p.returncode,flush=True)
 assert sum(x['cpu_seconds'] for x in exits)<=7200
 if p.returncode:print((A/(name+'.log')).read_text()[-4000:],flush=True)
 return p.returncode
steps=[('format',['cargo','fmt']),('audit',['python3','parity/0216-vdecode/audit.py']),
 ('all-features',['cargo','test','--offline','--locked','--all-features']),
 ('no-default',['cargo','test','--offline','--locked','--no-default-features']),
 ('no-std-simd',['cargo','test','--offline','--locked','--no-default-features','--features','simd,parity-internals','--test','simd','--','--test-threads=1']),
 ('clippy',['cargo','clippy','--offline','--locked','--all-targets','--all-features','--','-D','warnings']),
 ('fmt',['cargo','fmt','--check']),('boundary',['bash','parity/check-unsafe-boundary.sh']),
 ('reference',['bash','parity/check-reference.sh']),
 ('exact-default',['cargo','run','--offline','--release','--manifest-path',str(proof/'Cargo.toml'),'--target-dir',str(T/'exact')]),
 ('exact-no-default',['cargo','run','--offline','--release','--manifest-path',str(proof/'Cargo.toml'),'--target-dir',str(T/'exact'),'--no-default-features']),
 ('build-differential',['python3','parity/0216-vdecode/build_differential.py']),
 ('cpp-differential',['python3','parity/0216-vdecode/differential.py'])]
for name,cmd in steps:step(name,cmd)
e=dict(DECODE_NO_STD='1',RUSTFLAGS='-C target-feature=+ssse3,+popcnt,+sse4.1')
step('build-differential-no-std',['python3','parity/0216-vdecode/build_differential.py'],e)
step('cpp-differential-no-std',['python3','parity/0216-vdecode/differential.py'],e)
step('unsafe-free',['cargo','check','--offline','--locked','--no-default-features'],dict(RUSTFLAGS='-Funsafe_code'))
step('package',['cargo','package','--offline','--locked','--allow-dirty'])
step('miri',['cargo','+nightly','miri','test','--offline','--locked','--features','parity-internals','--lib','codec::simd::tests::packed_groups_and_meshlet_masks_match_integer_reference'],dict(MIRIFLAGS='-Zmiri-disable-isolation',RUSTFLAGS='-C target-feature=+ssse3,+popcnt,+sse4.1'))
step('asan-build',['cargo','+nightly','fuzz','build','--fuzz-dir','fuzz/codec','--target-dir',str(T),'--target','x86_64-unknown-linux-gnu','simd'])
from differential import stream
corpus=T/'fuzz-seeds';corpus.mkdir(exist_ok=True)
for n in [1,15,16,17,31,32,33,257]:
 for ch in [0,1,2,0xf2]:
  source=stream(n,12,1,ch,0,3)
  (corpus/f'{n}-{ch}').write_bytes(bytes([1])+n.to_bytes(4,'little')+(12).to_bytes(2,'little')+bytes([0,0,16])+source)
step('asan-fuzz',[str(T/'x86_64-unknown-linux-gnu/release/simd'),str(corpus),'-runs=20000','-seed=216','-max_len=16384','-print_final_stats=1','-artifact_prefix='+str(A)+'/'],dict(MESHOPT_FUZZ_COUNTS=str(A/'fuzz-level-counts.json')))
step('public-text',['bash','parity/tools/check_public_text.sh'])
# Retain the exact manifests and hashes before the dedicated target is deleted.
bind={str(p.relative_to(T)):hashlib.sha256(p.read_bytes()).hexdigest() for d in ['proof-source','proof-baseline','differential-source'] for p in (T/d).rglob('*') if p.is_file()}
(A/'proof-binding.json').write_text(json.dumps(bind,indent=2)+'\n')
assert all(x['exit']==0 for x in exits),exits

# Only a complete successful decoder screen admits the broader guard.
screen_dir=A.parent/os.environ.get('DECODE_SCREEN_DIR','screen')
screen=json.loads((screen_dir/(os.environ.get('DECODE_SCREEN_DIR','screen')+'-results.json')).read_text())['summary']
vertex=next(x for x in screen if x['op']=='vertex-decode' and x['group']=='real')
index=next(x for x in screen if x['op']=='index-decode' and x['group']=='real')
if vertex['base']<=.95 and index['base']<=1.005 and max(x['worst'] for x in screen if x['op']=='index-decode')<=1.01:
 env=dict(os.environ,DECODE_ART=str(A.parent/'guard'),GUARD_DRIVER=(A.parent/'guard-source.txt').read_text().strip(),DECODE_PHASE='guard',DECODE_ARMS='base control candidate cpp-scalar cpp-default',DECODE_OPS='simplify simplify-attributes simplify-sloppy simplify-update simplify-prune simplify-points cache overdraw fetch meshlets meshlets-flex meshlets-spatial cluster-lod vertex-encode vertex-decode index-encode index-decode remap-vertex remap-index')
 subprocess.run(['python3','parity/0216-vdecode/guard_build.py'],env=env,check=True)
 subprocess.run(['python3','parity/0216-vdecode/measure.py'],env=env,check=True)
 subprocess.run(['python3','parity/0216-vdecode/results.py'],env=env,check=True)
