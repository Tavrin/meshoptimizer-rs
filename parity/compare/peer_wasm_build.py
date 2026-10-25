#!/usr/bin/env python3
import os,pathlib,subprocess,shutil,json,hashlib,time
ROOT=pathlib.Path(__file__).resolve().parents[2];ART=pathlib.Path(os.environ['MESHOPT_COMPARE_ARTIFACTS']);TARGET=pathlib.Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('MOSS_HEAVY_ACTIVE');assert TARGET==pathlib.Path('/mnt/linux-extra/moss-cargo-targets/meshopt-peer-screen')
env=dict(os.environ,RUSTC_WRAPPER='',RUSTC_WORKSPACE_WRAPPER='',CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='4',RUSTFLAGS='-C target-feature=+simd128',CARGO_ENCODED_RUSTFLAGS='')
env.pop('CARGO_ENCODED_RUSTFLAGS',None)
receipts=[]
for profile in ['defaults','moss']:
 for arm,feature in [('O','ours,peer-simd'),('X','optimesh,peer-simd')]:
  cmd=['cargo','build','--manifest-path',str(ROOT/'parity/compare/Cargo.toml'),'--offline','--locked','--target','wasm32-unknown-unknown','--profile',profile,'--features',feature,'--lib']
  with (ART/f'build-wasm-{profile}-{arm}.log').open('w')as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
  if r.returncode:raise SystemExit(r.returncode)
  dst=ART/f'peer-{profile}-{arm}.wasm';shutil.copy2(TARGET/'wasm32-unknown-unknown'/profile/'peer_decode.wasm',dst);receipts.append({'command':cmd,'sha256':hashlib.sha256(dst.read_bytes()).hexdigest(),'admission_pid':os.environ['MOSS_HEAVY_ACTIVE']})
env.update(RUSTFLAGS='',CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD')
cmd=['cargo','build','--manifest-path',str(ROOT/'parity/compare/Cargo.toml'),'--offline','--locked','--profile','defaults','--features','peer7','--bin','peer-stream-ref']
with (ART/'build-stream-ref.log').open('w')as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
if r.returncode:raise SystemExit(r.returncode)
shutil.copy2(TARGET/'defaults'/'peer-stream-ref',ART/'peer-stream-ref');receipts.append({'command':cmd,'sha256':hashlib.sha256((ART/'peer-stream-ref').read_bytes()).hexdigest()})
(ART/'wasm-builds.json').write_text(json.dumps(receipts,indent=2)+'\n')
