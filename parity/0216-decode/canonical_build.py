"""Control crate identity/build-path effects; frozen source for each arm is retained."""
import hashlib,io,json,os,shutil,subprocess,tarfile
from pathlib import Path
root=Path.cwd();target=Path(os.environ['CARGO_TARGET_DIR']);art=Path(os.environ['DECODE_ART']);d=target/'canonical';d.mkdir(exist_ok=True)
binding=json.loads((art/'build-binding.json').read_text())
for arm in ['base','control','candidate']:
    archive=subprocess.check_output(['git','archive','v0.2.15','src','docs','Cargo.toml','Cargo.lock','README.md'])
    tarfile.open(fileobj=io.BytesIO(archive)).extractall(d,filter='data')
    if arm!='base':shutil.copytree(root/'src',d/'src',dirs_exist_ok=True)
    if arm=='control':
        p=d/'src/codec/simd/x86.rs';s=p.read_text().replace('        (true, 12) => vertex_kernel::<128, 12, D>,\n        (false, 12) => vertex_kernel::<1024, 12, D>,\n','');p.write_text(s)
    peers=d/'parity/peers';shutil.copytree(root/'parity/peers',peers,dirs_exist_ok=True)
    # copy2 preserves old mtimes: explicitly invalidate Cargo's mtime-based
    # freshness checks before each same-path build, including the control.
    for p in [*d.glob('src/**/*.rs'),d/'Cargo.toml',d/'Cargo.lock',peers/'Cargo.toml',peers/'src/main.rs']:
        p.touch()
    frozen=art/('frozen-'+arm);shutil.copytree(d,frozen)
    # The absolute manifest/source path and package IDs are identical across arms.
    with (art/('build-'+arm+'.log')).open('w') as log:
        subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(peers/'Cargo.toml'),'--no-default-features','--features','ours','-j','2'],env=dict(os.environ,CARGO_INCREMENTAL='0'),stdout=log,stderr=subprocess.STDOUT,check=True)
    assert 'Compiling meshoptimizer-rs' in (art/('build-'+arm+'.log')).read_text(),arm
    binary=art/('driver-'+arm);shutil.copy2(target/'release/meshopt-peers',binary)
    binding['binaries'][arm]=hashlib.sha256(binary.read_bytes()).hexdigest()
    binding['sources'][arm]={str(f.relative_to(d)):hashlib.sha256(f.read_bytes()).hexdigest() for f in [*d.glob('src/**/*.rs'),d/'Cargo.toml',peers/'Cargo.toml',peers/'src/main.rs']}
    (art/'build-binding.json').write_text(json.dumps(binding,indent=2)+'\n');print(arm,'built at canonical path',flush=True)
assert binding['binaries']['candidate'] != binding['binaries']['control'], 'tuning was not compiled'
