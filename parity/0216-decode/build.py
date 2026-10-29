"""Freeze and build the default-feature decoder consumers inside CPU admission."""
import hashlib, io, json, os, shutil, subprocess, sys, tarfile
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
root=Path.cwd(); art=Path(os.environ['DECODE_ART']); art.mkdir(parents=True,exist_ok=True)
p=art/'build-binding.json'; binding=json.loads(p.read_text()) if p.exists() else {'binaries':{},'sources':{}}
for arm in sys.argv[1:]:
    if arm in binding['binaries']:
        assert hashlib.sha256((art/('driver-'+arm)).read_bytes()).hexdigest()==binding['binaries'][arm]
        continue
    dest=Path(os.environ['CARGO_TARGET_DIR'])/('frozen-'+arm); dest.mkdir(parents=True,exist_ok=True)
    archive=subprocess.check_output(['git','archive','v0.2.15','src','docs','Cargo.toml','Cargo.lock','README.md'])
    tarfile.open(fileobj=io.BytesIO(archive)).extractall(dest,filter='data')
    if arm=='candidate': shutil.copytree(root/'src',dest/'src',dirs_exist_ok=True)
    peers=dest/'parity/peers'; shutil.copytree(root/'parity/peers',peers,dirs_exist_ok=True)
    cmd=['cargo','build','--offline','--locked','--release','--manifest-path',str(peers/'Cargo.toml'),'--no-default-features','--features','cpp' if arm.startswith('cpp') else 'ours','-j','2']
    env=dict(os.environ,CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD' if arm=='cpp-scalar' else '')
    with (art/('build-'+arm+'.log')).open('w') as log: subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
    binary=art/('driver-'+arm); shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release/meshopt-peers',binary)
    binding['binaries'][arm]=hashlib.sha256(binary.read_bytes()).hexdigest()
    binding['sources'][arm]={str(f.relative_to(dest)):hashlib.sha256(f.read_bytes()).hexdigest() for f in [*dest.glob('src/**/*.rs'),peers/'src/main.rs',peers/'Cargo.toml']}
    p.write_text(json.dumps(binding,indent=2)+'\n'); print(arm,'built',flush=True)
