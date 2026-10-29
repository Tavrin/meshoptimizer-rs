"""Build frozen scalar Rust and scalar/default C++ arms under admission."""
import hashlib, json, os, shutil, subprocess, sys
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE')
root = Path(__file__).resolve().parents[2]
lane = Path(__file__).resolve().parent
s = Path(os.environ['REMAP_SCRATCH'])
art = s / sys.argv[1]
art.mkdir(exist_ok=True)
binding = {'sources': {}, 'binaries': {}}
def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()
for arm in ['base', 'candidate', 'cpp', 'cpp-default']:
    tree = art / ('frozen-' + arm)
    source = s / 'base' if arm == 'base' else root
    if not tree.exists():
        for d in ['src', 'docs']:
            shutil.copytree(source/d, tree/d)
        for f in ['Cargo.toml', 'Cargo.lock', 'README.md']:
            shutil.copy2(source/f, tree/f)
        peers = tree/'parity/peers'
        shutil.copytree(root/'parity/peers', peers)
        subprocess.run(['python3', str(root/'parity/0215-contract-meshlets/prepare_driver.py'), str(peers/'src/main.rs')], check=True)
        subprocess.run(['python3', str(root/'parity/0215-basics/prepare_driver.py'), str(peers)], check=True)
        # Disclose wall time from the same resident counted loop.
        p = peers/'src/main.rs'
        t = p.read_text().replace('control("enable");for _', 'control("enable");let start=Instant::now();for _')
        t = t.replace('control("disable");println!("COUNTED")', 'let elapsed=start.elapsed().as_secs_f64();control("disable");println!("COUNTED {elapsed:.9}")')
        p.write_text(t)
    for p in tree.rglob('*'):
        if p.is_file():
            binding['sources'][str(p.relative_to(s))] = sha(p)
    target = Path(os.environ['CARGO_TARGET_DIR'])/arm
    cmd = ['cargo','build','--offline','--locked','--release','--manifest-path',str(tree/'parity/peers/Cargo.toml'),
           '--no-default-features','--features','ours' if arm in ['base','candidate'] else 'cpp','-j','2','--target-dir',str(target)]
    env = dict(os.environ, CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD' if arm == 'cpp' else '')
    with (art/('build-'+arm+'.log')).open('w') as f:
        r = subprocess.run(cmd, stdout=f, stderr=subprocess.STDOUT, env=env)
    if r.returncode:
        print((art/('build-'+arm+'.log')).read_text()[-4000:])
    assert r.returncode == 0, arm
    shutil.copy2(target/'release/meshopt-peers', art/('driver-'+arm))
    binding['binaries'][arm] = sha(art/('driver-'+arm))
    print(arm, 'built', flush=True)
(art/'build-binding.json').write_text(json.dumps(binding,indent=2)+'\n')
