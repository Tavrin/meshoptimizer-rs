"""Freeze sources and build the registered four arms under CPU admission.

PEERS_ART is a child of the lane scratch directory. MESHOPT_BASELINE is a
clean unpacked v0.2.14 tree. Frozen input fixtures must already be in PEERS_ART.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
root = Path(__file__).resolve().parents[2]
art = Path(os.environ['PEERS_ART'])
scratch = art.parent
base = Path(os.environ['MESHOPT_BASELINE'])
sources = {}
for name, source in [('base', base), ('candidate', root)]:
    dest = scratch / ('frozen-' + name)
    if not dest.exists():
        shutil.copytree(source / 'src', dest / 'src')
        for file in ['Cargo.toml', 'Cargo.lock']:
            shutil.copy2(source / file, dest / file)
        shutil.copytree(root / 'parity/peers', dest / 'parity/peers')
        subprocess.run(['python3', str(root / 'parity/0215-contract-meshlets/prepare_driver.py'),
                        str(dest / 'parity/peers/src/main.rs')], check=True)
    # lib.rs includes the migration/usage guides even in dependency builds.
    if not (dest / 'docs').exists():
        shutil.copytree(source / 'docs', dest / 'docs')
    if not (dest / 'README.md').exists():
        shutil.copy2(source / 'README.md', dest / 'README.md')
    for path in [*dest.glob('src/**/*.rs'), *dest.glob('docs/*.md'), dest / 'README.md', dest / 'Cargo.toml', dest / 'Cargo.lock',
                 *dest.glob('parity/peers/src/*.rs'), dest / 'parity/peers/Cargo.toml',
                 dest / 'parity/peers/Cargo.lock']:
        sources[str(path.relative_to(scratch))] = hashlib.sha256(path.read_bytes()).hexdigest()
    for path in source.glob('src/**/*.rs'):
        assert path.read_bytes() == (dest / path.relative_to(source)).read_bytes(), path.name
binding = art / 'build-binding.json'
if binding.exists():
    assert json.loads(binding.read_text())['sources'] == sources
else:
    # Keep the inherited exploratory executables and their profile bindings.
    archive = art / 'exploratory'
    archive.mkdir(exist_ok=True)
    for path in art.glob('driver-*'):
        saved = archive / path.name
        if not saved.exists():
            shutil.copy2(path, saved)
binaries = {}
for arm, feature, tree in [('base', 'ours', 'base'), ('candidate', 'ours', 'candidate'),
                           ('cpp', 'cpp', 'candidate'), ('rust', 'rust', 'candidate')]:
    target = art / ('driver-' + arm)
    command = ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path',
               str(scratch / ('frozen-' + tree) / 'parity/peers/Cargo.toml'),
               '--no-default-features', '--features', feature, '-j', '1']
    with (art / ('campaign-build-' + arm + '.log')).open('w') as log:
        result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
    assert result.returncode == 0, arm
    shutil.copy2(Path(os.environ['CARGO_TARGET_DIR']) / 'release/meshopt-peers', target)
    binaries[arm] = hashlib.sha256(target.read_bytes()).hexdigest()
    print(arm, 'built', flush=True)
binding.write_text(json.dumps(dict(binaries=binaries, sources=sources), indent=2) + '\n')
