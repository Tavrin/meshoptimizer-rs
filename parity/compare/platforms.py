#!/usr/bin/env python3
"""Compile-only target checks, one compiler invocation at a time, admitted."""
from pathlib import Path
import os,subprocess,json,time,shutil
ROOT=Path(__file__).resolve().parents[2];ART=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp')
assert os.environ.get('MOSS_HEAVY_ACTIVE')
env=dict(os.environ,RUSTC_WRAPPER='',RUSTC_WORKSPACE_WRAPPER='',RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',CARGO_BUILD_JOBS='4')
checks=[
 ('ours-no-std-wasm-no-c', ['cargo','check','--manifest-path',str(ROOT/'Cargo.toml'),'--offline','--locked','--no-default-features','--target','wasm32-unknown-unknown'], {'CC':'/nonexistent-meshopt-cmp-cc','CXX':'/nonexistent-meshopt-cmp-cxx'}),
 ('ours-aarch64',['cargo','check','--manifest-path',str(ROOT/'Cargo.toml'),'--offline','--locked','--target','aarch64-unknown-linux-gnu'],{}),
 ('theirs-wasm-no-c',['cargo','check','--manifest-path',str(ROOT/'parity/compare/Cargo.toml'),'--offline','--locked','--features','theirs','--bin','cmp-decode','--target','wasm32-unknown-unknown'],{'CC_wasm32_unknown_unknown':'/nonexistent-meshopt-cmp-cc','CXX_wasm32_unknown_unknown':'/nonexistent-meshopt-cmp-cxx'}),
 ('theirs-aarch64',['cargo','check','--manifest-path',str(ROOT/'parity/compare/Cargo.toml'),'--offline','--locked','--features','theirs','--bin','cmp-decode','--target','aarch64-unknown-linux-gnu'],{}),
 ('ours-msrv',['cargo','+1.88','check','--manifest-path',str(ROOT/'Cargo.toml'),'--offline','--locked','--all-features'],{}),
 ('theirs-rust-1.88',['cargo','+1.88','check','--manifest-path',str(ROOT/'parity/compare/Cargo.toml'),'--offline','--locked','--features','theirs','--bin','cmp-decode'],{}),
]
record=ART/'platforms.json'
results=json.loads(record.read_text()) if record.exists() else []
started=time.monotonic()
for name,cmd,extra in checks:
 if any(r['name']==name for r in results):continue
 if shutil.disk_usage(ART).free<25*1024**3:raise SystemExit('PAUSED: disk below 25 GiB')
 if time.monotonic()-started>720:raise SystemExit(75)
 start=time.time();log=ART/f'{name}.log'
 with log.open('w') as f:r=subprocess.run(cmd,env=dict(env,**extra),stdout=f,stderr=subprocess.STDOUT)
 results.append({'name':name,'command':cmd,'environment_override':extra,'exit':r.returncode,'seconds':time.time()-start,'log':str(log),'compile_only':True})
 tmp=record.with_suffix('.tmp');tmp.write_text(json.dumps(results,indent=2)+'\n');tmp.replace(record);print(name,r.returncode,flush=True)
# Intentional missing C toolchain must fail; pure Rust without it must succeed.
assert {r['name'] for r in results}=={name for name,_,_ in checks}
by={r['name']:r for r in results}
assert by['ours-no-std-wasm-no-c']['exit']==0 and by['theirs-wasm-no-c']['exit']!=0
