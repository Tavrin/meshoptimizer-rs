"""Freeze sources and build the four scalar campaign arms under CPU admission.

Usage: build.py ARM... with ARM in base, candidate, cpp, rust.
Environment: QUALIFY_SCRATCH (base0/ is `git archive v0.2.14`), CARGO_TARGET_DIR,
QUALIFY_PEERS_SOURCE (the S6 campaign peer consumer: src/main.rs with the
sha256 bound in parity/0215-simplify-cpp/binding.json, plus its Cargo.toml and
Cargo.lock; kept out of the tree because its FFI arms use unsafe code).
base builds from frozen v0.2.14, every other arm from the frozen train tree.
Each frozen tree gets parity/peers = that consumer adapted by prepare.py
(remap ops, SIMD off). The C++ arm is compiled with MESHOPTIMIZER_NO_SIMD.
Binaries and source hashes go to $QUALIFY_SCRATCH/campaign/build-binding.json.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
here = Path(__file__).resolve().parent
root = here.parents[2]
scratch = Path(os.environ['QUALIFY_SCRATCH'])
art = scratch / 'campaign'
art.mkdir(exist_ok=True)
trees = {'base': scratch / 'base0', 'candidate': root}
PEERS_MAIN_SHA256 = '74d821a368c44bbd41db342f6d5ac1a1a07a53459fddd444ecc5d9755d4bc015'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def freeze(name):
    source, dest = trees[name], art / ('frozen-' + name)
    if not dest.exists():
        shutil.copytree(source / 'src', dest / 'src')
        shutil.copytree(source / 'docs', dest / 'docs')
        for file in ['Cargo.toml', 'Cargo.lock', 'README.md']:
            shutil.copy2(source / file, dest / file)
        peers = Path(os.environ['QUALIFY_PEERS_SOURCE'])
        assert sha(peers / 'src/main.rs') == PEERS_MAIN_SHA256, 'unexpected campaign driver'
        shutil.copytree(peers, dest / 'parity/peers')
        subprocess.run(['python3', str(here / 'prepare.py'), str(dest / 'parity/peers')], check=True)
    for path in source.glob('src/**/*.rs'):
        assert path.read_bytes() == (dest / path.relative_to(source)).read_bytes(), ('stale freeze', path)
    return {str(p.relative_to(art)): sha(p) for p in
            [*dest.glob('src/**/*.rs'), dest / 'Cargo.toml', dest / 'Cargo.lock',
             dest / 'parity/peers/src/main.rs', dest / 'parity/peers/Cargo.toml']}


binding_path = art / 'build-binding.json'
binding = json.loads(binding_path.read_text()) if binding_path.exists() else dict(binaries={}, sources={})
for arm in sys.argv[1:]:
    tree = 'base' if arm == 'base' else 'candidate'
    binding['sources'].update(freeze(tree))
    feature = {'base': 'ours', 'candidate': 'ours', 'cpp': 'cpp', 'rust': 'rust'}[arm]
    target = Path(os.environ['CARGO_TARGET_DIR']) / ('campaign-' + arm)
    command = ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path',
               str(art / ('frozen-' + tree) / 'parity/peers/Cargo.toml'),
               '--no-default-features', '--features', feature, '-j', '2', '--target-dir', str(target)]
    env = dict(os.environ, CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD') if arm == 'cpp' else os.environ
    with (art / ('build-' + arm + '.log')).open('w') as log:
        result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, env=env)
    assert result.returncode == 0, arm
    driver = art / ('driver-' + arm)
    shutil.copy2(target / 'release/meshopt-peers', driver)
    binding['binaries'][arm] = sha(driver)
    print(arm, 'built', flush=True)
binding_path.write_text(json.dumps(binding, indent=2, sort_keys=True) + '\n')
