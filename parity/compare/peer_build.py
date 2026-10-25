#!/usr/bin/env python3
"""CPU admitted builds, retain every executable; no measurements here."""
import os,sys,pathlib,subprocess,json,hashlib,shutil,time
ROOT=pathlib.Path(os.environ.get('MESHOPT_PEER_SOURCE',str(pathlib.Path(__file__).resolve().parents[2])))
ART=pathlib.Path(os.environ['MESHOPT_COMPARE_ARTIFACTS'])
TARGET=pathlib.Path('/mnt/linux-extra/moss-cargo-targets/meshopt-peer-screen')
assert os.environ.get('MOSS_HEAVY_ACTIVE') and pathlib.Path(os.environ['CARGO_TARGET_DIR'])==TARGET
assert shutil.disk_usage(ART).free>=25*1024**3
assert (ART/'SAFETY.md').exists()
safety=json.loads((ART/'safety-reuse.json').read_text())
assert safety['tree_exact_match'] and not safety['prior_receipt_mismatches'] and not safety['live_source_mismatches']
profile,arm,mode=sys.argv[1:]
features={'AA':'ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod','O':'ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod','M7':'peer7','REF':'peer7','X':'optimesh'}[arm]
if mode=='simd' and arm in ['O','AA','X']:features+=',peer-simd'
env=dict(os.environ,RUSTC_WRAPPER='',RUSTC_WORKSPACE_WRAPPER='',RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='4',CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD' if mode=='scalar' else '')
cmd=['cargo','build','--manifest-path',str(ROOT/'parity/compare/Cargo.toml'),'--locked','--offline','--profile',profile,'--features',features,'--bin','cmp-driver']
log=ART/f'build-{profile}-{arm}-{mode}.log'
start=time.time()
receipt_sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in list((ROOT/'src').rglob('*.rs'))+list((ROOT/'parity/compare/src').glob('*.rs'))+[ROOT/'parity/compare/Cargo.toml',ROOT/'parity/compare/Cargo.lock']}
assert not log.exists(), 'retain prior build attempt'
with log.open('w') as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
receipt={'source_sha256':receipt_sources,'command':cmd,'env':{k:env[k] for k in ['RUSTFLAGS','CXXFLAGS','CARGO_BUILD_JOBS']},'exit':r.returncode,'started':start,'finished':time.time(),'admission_pid':os.environ['MOSS_HEAVY_ACTIVE']}
if not r.returncode:
 dst=ART/f'cmp-driver-{profile}-{arm}-{mode}';shutil.copy2(TARGET/profile/'cmp-driver',dst);receipt['sha256']=hashlib.sha256(dst.read_bytes()).hexdigest()
(ART/f'build-{profile}-{arm}-{mode}.json').write_text(json.dumps(receipt,indent=2)+'\n')
if r.returncode:print(log.read_text())
if r.returncode:sys.exit(r.returncode)
# Complete missing matrix arms sequentially under this same admitted reservation.
# Children inherit admission and never overlap; no measurement takes place here.
if not os.environ.get('MESHOPT_PEER_BUILD_CHILD'):
 childenv=dict(os.environ,MESHOPT_PEER_BUILD_CHILD='1')
 for pp in ['defaults','moss']:
  for aa in ['O','M7','X']:
   for mm in ['scalar','simd']:
    receipt=ART/f'build-{pp}-{aa}-{mm}.json'
    if receipt.exists():
     assert json.loads(receipt.read_text())['exit']==0
     continue
    subprocess.run([sys.executable,__file__,pp,aa,mm],env=childenv,check=True)
 aa_root=ART/'aa-source'
 if not aa_root.exists():
  aa_root.mkdir()
  for d in ['src','parity/compare/src']:shutil.copytree(ROOT/d,aa_root/d)
  for f in ['Cargo.toml','Cargo.lock','parity/compare/Cargo.toml','parity/compare/Cargo.lock']:
   (aa_root/f).parent.mkdir(parents=True,exist_ok=True);shutil.copy2(ROOT/f,aa_root/f)
 for pp in ['defaults','moss']:
  for mm in ['scalar','simd']:
   if not (ART/f'build-{pp}-AA-{mm}.json').exists():
    subprocess.run([sys.executable,__file__,pp,'AA',mm],env=dict(childenv,MESHOPT_PEER_SOURCE=str(aa_root)),check=True)
 if not (ART/'wasm-builds.json').exists():
  subprocess.run([sys.executable,str(ROOT/'parity/compare/peer_wasm_build.py')],env=childenv,check=True)

