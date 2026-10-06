#!/usr/bin/env python3
"""Sequential consumer builds. Invoke inside CPU moss-heavy admission."""
import os,sys,time,json,subprocess,shutil,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
ART=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp')
TARGET=Path('/mnt/linux-extra/moss-cargo-targets/codex-meshopt-cmp')
assert Path(os.environ['CARGO_TARGET_DIR'])==TARGET
assert os.environ.get('MOSS_HEAVY_ACTIVE'), 'shared admission required'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def disk():
 free=shutil.disk_usage(TARGET.parent).free
 if free<25*1024**3:raise SystemExit('PAUSED: disk below 25 GiB')
 return free
profile,backend=sys.argv[1:3]
refresh="--refresh" in sys.argv[3:]
env=dict(os.environ,RUSTC_WRAPPER='',RUSTC_WORKSPACE_WRAPPER='',RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',CARGO_BUILD_JOBS='4',CARGO_INCREMENTAL='0')
# The named target is exclusively this lane's disposable compilation cache.
disk()
prior=ART/f'build-{profile}-{backend}.json'
records=json.loads(prior.read_text()) if prior.exists() else []
if not records:
 if TARGET.exists():shutil.rmtree(TARGET)
 TARGET.mkdir(parents=True)
base=['cargo','build','--manifest-path',str(ROOT/'parity/compare/Cargo.toml'),'--offline','--locked','--profile',profile,'--features',backend]
for kind,bins in [('clean',['cmp-decode']),('incremental',['cmp-decode']),('decode-footprint',['cmp-decode']),('footprint-and-driver',['cmp-full','cmp-driver','cmp-empty'])]:
 if any(r['kind']==kind and r['exit']==0 for r in records) and not (refresh and kind in ['decode-footprint','footprint-and-driver']):continue
 free=disk()
 if kind=='incremental':os.utime(ROOT/'parity/compare/src/decode.rs',None)
 cmd=base+(['--features','full'] if kind=='footprint-and-driver' else [])+sum((['--bin',b] for b in bins),[])
 log=ART/f'build-{profile}-{backend}-{kind}.log'
 started=time.time()
 with log.open('w') as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
 records.append({'kind':kind,'seconds':time.time()-started,'exit':r.returncode,'command':cmd,'log':str(log),'free_bytes':free})
 (ART/f'build-{profile}-{backend}.json').write_text(json.dumps(records,indent=2)+'\n')
 print(profile,backend,kind,records[-1]['seconds'],'exit',r.returncode,flush=True)
 if r.returncode:print(log.read_text());sys.exit(r.returncode)
 for name in bins:
  src=TARGET/profile/name;dst=ART/f'{name}-{profile}-{backend}';shutil.copy2(src,dst)
  (ART/f'{name}-{profile}-{backend}.sha256').write_text(sha(dst)+'\n')
