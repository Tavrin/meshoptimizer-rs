#!/usr/bin/env python3
from pathlib import Path
import os,subprocess,sys,shutil,json,time,hashlib
ROOT=Path(__file__).resolve().parents[2];ART=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp');TARGET=Path('/mnt/linux-extra/moss-cargo-targets/codex-meshopt-cmp')
assert os.environ.get('MOSS_HEAVY_ACTIVE') and Path(os.environ['CARGO_TARGET_DIR'])==TARGET
profile,backend=sys.argv[1:]
if shutil.disk_usage(ART).free<25*1024**3:raise SystemExit('PAUSED: disk below 25 GiB')
env=dict(os.environ,RUSTC_WRAPPER='',RUSTC_WORKSPACE_WRAPPER='',RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',CARGO_BUILD_JOBS='4',CARGO_INCREMENTAL='0')
cmd=['cargo','build','--manifest-path',str(ROOT/'parity/compare/Cargo.toml'),'--offline','--locked','--profile',profile,'--features',backend+',full','--bin','cmp-driver']
log=ART/f'driver-final-{profile}-{backend}.log';start=time.time()
with log.open('w') as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
if r.returncode:print(log.read_text());raise SystemExit(r.returncode)
src=TARGET/profile/'cmp-driver';dst=ART/f'cmp-driver-{profile}-{backend}';shutil.copy2(src,dst);digest=hashlib.sha256(dst.read_bytes()).hexdigest();Path(str(dst)+'.sha256').write_text(digest+'\n')
(ART/f'driver-final-{profile}-{backend}.json').write_text(json.dumps({'command':cmd,'seconds':time.time()-start,'exit':r.returncode,'binary_sha256':digest,'main_sha256':hashlib.sha256((ROOT/'parity/compare/src/main.rs').read_bytes()).hexdigest(),'admission_pid':os.environ['MOSS_HEAVY_ACTIVE']},indent=2)+'\n')
print(profile,backend,'final driver',time.time()-start,flush=True)
