from pathlib import Path
import subprocess,os,sys,shutil,json,hashlib
s=Path(os.environ.get('PEER_SCRATCH','/mnt/linux-extra/moss-scratch/meshopt-peer-a'))
assert not (s/'wall.json').exists(), 'completed evidence is immutable; use fresh scratch'
assert os.environ.get('MOSS_HEAVY_ACTIVE')
for arm in sys.argv[1:]:
 feature={'O':'ours','B':'ours','X':'optimesh','M7':'peer7'}[arm]
 cmd=['cargo','build','--offline','--manifest-path',str(s/arm/'parity/compare/Cargo.toml'),'--profile','moss','--features',feature,'--bin','cmp-driver']
 with (s/f'build-{arm}.log').open('a') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==0,(s/f'build-{arm}.log').read_text()
 shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'moss/cmp-driver',s/f'driver-{arm}')
 (s/f'build-{arm}-final.json').write_text(json.dumps({'command':cmd,'binary_sha256':hashlib.sha256((s/f'driver-{arm}').read_bytes()).hexdigest(),'quantize_sha256':hashlib.sha256((Path.cwd()/'src/quantize.rs').read_bytes()).hexdigest(),'admission_pid':os.environ['MOSS_HEAVY_ACTIVE']},indent=2))
 print('built',arm,flush=True)
