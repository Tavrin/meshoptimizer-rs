"""Freeze sources and build the requested peer arms under CPU admission.

Usage: build.py ARM... with ARM in base, candidate, cpp, rust. PEERS_ART is a
child of the lane scratch directory; MESHOPT_BASELINE is the clean train head
tree. base builds from the frozen baseline, every other arm from the frozen
candidate (the worktree at the time of the first candidate build). Each tree
gets the scratch-adapted consumer (contract-meshlets counting-only patch, then
prepare_driver.py: remap ops, SIMD off). The C++ arm is compiled with
MESHOPTIMIZER_NO_SIMD. Rebuilding an arm overwrites its driver and binding.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
root = Path(__file__).resolve().parents[2]
lane = Path(__file__).resolve().parent
art = Path(os.environ['PEERS_ART'])
scratch = art.parent
trees = {'base': Path(os.environ['MESHOPT_BASELINE']), 'candidate': root}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def freeze(name):
    source, dest = trees[name], scratch / ('frozen-' + name)
    if not dest.exists():
        shutil.copytree(source / 'src', dest / 'src')
        shutil.copytree(source / 'docs', dest / 'docs')
        for file in ['Cargo.toml', 'Cargo.lock', 'README.md']:
            shutil.copy2(source / file, dest / file)
        shutil.copytree(root / 'parity/peers', dest / 'parity/peers')
        for script in [root / 'parity/0215-contract-meshlets/prepare_driver.py', lane / 'prepare_driver.py']:
            arg = dest / 'parity/peers' / ('src/main.rs' if 'contract' in str(script) else '')
            subprocess.run(['python3', str(script), str(arg)], check=True)
    for path in source.glob('src/**/*.rs'):
        assert path.read_bytes() == (dest / path.relative_to(source)).read_bytes(), ('stale freeze', path)
    return {str(p.relative_to(scratch)): sha(p) for p in
            [*dest.glob('src/**/*.rs'), dest / 'Cargo.toml', dest / 'Cargo.lock',
             dest / 'parity/peers/src/main.rs', dest / 'parity/peers/Cargo.toml']}


binding_path = art / 'build-binding.json'
binding = json.loads(binding_path.read_text()) if binding_path.exists() else dict(binaries={}, sources={})
for arm in sys.argv[1:]:
    tree = 'base' if arm == 'base' else 'candidate'
    binding['sources'].update(freeze(tree))
    feature = {'base': 'ours', 'candidate': 'ours', 'cpp': 'cpp', 'rust': 'rust'}[arm]
    command = ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path',
               str(scratch / ('frozen-' + tree) / 'parity/peers/Cargo.toml'),
               '--no-default-features', '--features', feature, '-j', '2']
    env = dict(os.environ, CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD') if arm == 'cpp' else os.environ
    with (art / ('build-' + arm + '.log')).open('w') as log:
        result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, env=env)
    assert result.returncode == 0, arm
    target = art / ('driver-' + arm)
    shutil.copy2(Path(os.environ['CARGO_TARGET_DIR']) / 'release/meshopt-peers', target)
    binding['binaries'][arm] = sha(target)
    print(arm, 'built', flush=True)
binding_path.write_text(json.dumps(binding, indent=2, sort_keys=True) + '\n')
