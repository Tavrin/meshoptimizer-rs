"""Freeze and build cache-only scalar arms under the CPU admission runner.

RESTORE_SCRATCH, RESTORE_INPUTS and CARGO_TARGET_DIR are caller-owned paths.
Usage: build.py ARM...; arms base/train/s5/tables/candidate/cpp/rust.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import io

assert os.environ['PARITY_RUNNER_ACTIVE'] and os.environ['PARITY_RUNNER_GPU'] == '0'
root = Path(__file__).resolve().parents[2]
scratch = Path(os.environ['RESTORE_SCRATCH'])
refs = dict(base='v0.2.14', train='68439a3', s5='5854223', tables='cc13453')
binding_path = scratch / 'build-binding.json'
binding = json.loads(binding_path.read_text()) if binding_path.exists() else dict(sources={}, binaries={}, revisions={})

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

for arm in sys.argv[1:]:
    tree = 'train' if arm in ('cpp', 'rust') else arm
    dest = scratch / ('frozen-' + tree)
    if not dest.exists():
        dest.mkdir()
        if tree == 'candidate':
            for name in ['src', 'docs']:
                shutil.copytree(root / name, dest / name)
            for name in ['Cargo.toml', 'Cargo.lock', 'README.md']:
                shutil.copy2(root / name, dest / name)
        else:
            archive = subprocess.check_output(['git', 'archive', refs[tree], 'src', 'docs', 'Cargo.toml', 'Cargo.lock', 'README.md'], cwd=root)
            tarfile.open(fileobj=io.BytesIO(archive)).extractall(dest, filter='data')
        peers = dest / 'parity/peers'
        shutil.copytree(root / 'parity/peers', peers)
        p = peers / 'src/main.rs'
        s = p.read_text().replace('let start=Instant::now();', '').replace('let elapsed=start.elapsed().as_secs_f64();', '').replace('println!("COUNTED {elapsed:.9}")', 'println!("COUNTED")')
        p.write_text(s)
        p = peers / 'Cargo.toml'
        s = p.read_text().replace('optional = true, features = ["clusterlod"]', 'optional = true, default-features = false, features = ["std", "clusterlod"]').replace('optional = true, features = ["simd"]', 'optional = true, default-features = false')
        p.write_text(s)
    if tree == 'candidate':
        for p in (root / 'src').rglob('*.rs'):
            assert p.read_bytes() == (dest / p.relative_to(root)).read_bytes(), ('stale candidate freeze', p.name)
    for p in dest.rglob('*'):
        if p.is_file():
            key = str(p.relative_to(scratch))
            digest = sha(p)
            assert key not in binding['sources'] or binding['sources'][key] == digest
            binding['sources'][key] = digest
    binding['revisions'][tree] = subprocess.check_output(['git', 'rev-parse', refs.get(tree, 'HEAD')], cwd=root, text=True).strip()
    feature = arm if arm in ('cpp', 'rust') else 'ours'
    cmd = ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', str(dest / 'parity/peers/Cargo.toml'), '--no-default-features', '--features', feature, '-j', '2']
    with (scratch / ('build-' + arm + '.log')).open('w') as log:
        r = subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT, env=dict(os.environ, CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD'))
    if r.returncode:
        print((scratch / ('build-' + arm + '.log')).read_text())
        raise SystemExit(r.returncode)
    binary = scratch / ('driver-' + arm)
    shutil.copy2(Path(os.environ['CARGO_TARGET_DIR']) / 'release/meshopt-peers', binary)
    binding['binaries'][arm] = sha(binary)
    binding_path.write_text(json.dumps(binding, indent=2, sort_keys=True) + '\n')
    print(arm, binary.name, binding['binaries'][arm], flush=True)
