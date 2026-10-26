#!/usr/bin/env python3
"""Build the exported, frozen comparator under lane C admission."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

ART = Path('/mnt/linux-extra/moss-scratch/meshopt-peer-c')
TARGET = Path('/mnt/linux-extra/moss-cargo-targets/meshopt-peer-c')
assert os.environ.get('MOSS_HEAVY_ACTIVE')
assert Path(os.environ['CARGO_TARGET_DIR']) == TARGET
for arm in sys.argv[1:]:
    feature = {'base': 'ours', 'candidate': 'ours', 'X': 'optimesh', 'M7': 'peer7'}[arm]
    if feature == 'ours':
        feature += ',meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod'
    manifest = ART / f'harness-{arm}' / 'Cargo.toml'
    env = dict(os.environ, RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='',
               RUSTFLAGS='', CARGO_ENCODED_RUSTFLAGS='', CARGO_INCREMENTAL='0',
               CARGO_BUILD_JOBS='4', CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD')
    subprocess.run(['cargo', 'generate-lockfile', '--offline', '--manifest-path', str(manifest)], env=env, check=True)
    cmd = ['cargo', 'build', '--locked', '--offline', '--manifest-path', str(manifest),
           '--profile', 'moss', '--features', feature, '--bin', 'cmp-driver']
    number = len(list(ART.glob(f'build-{arm}-*.json')))
    stem = ART / f'build-{arm}-{number}'
    with stem.with_suffix('.log').open('w') as log:
        result = subprocess.run(cmd, env=env, stdout=log, stderr=subprocess.STDOUT)
    receipt = {'command': cmd, 'exit': result.returncode, 'admission': os.environ['MOSS_HEAVY_ACTIVE']}
    if result.returncode == 0:
        dst = ART / f'driver-{arm}-{number}'
        shutil.copy2(TARGET / 'moss/cmp-driver', dst)
        receipt['binary'] = str(dst)
        receipt['sha256'] = hashlib.sha256(dst.read_bytes()).hexdigest()
        source = ART / 'baseline' if arm == 'base' else Path(__file__).resolve().parents[2]
        receipt['source'] = {str(p.relative_to(source)): hashlib.sha256(p.read_bytes()).hexdigest()
                             for p in (source / 'src').rglob('*.rs')}
    stem.with_suffix('.json').write_text(json.dumps(receipt, indent=2) + '\n')
    if result.returncode:
        print(stem.with_suffix('.log').read_text())
        sys.exit(result.returncode)
    print(dst, flush=True)
