"""Build and retain peer executables one arm at a time under admission."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
assert os.environ.get('CARGO_INCREMENTAL') == '0'
art = Path(os.environ['PEERS_ART'])
scratch = art.parent
target_dir = Path(os.environ['CARGO_TARGET_DIR'])


def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def floor_ok():
    stat = os.statvfs(art)
    return stat.f_bavail * stat.f_frsize >= 100 * 1024**3


source_hashes = {}
for name in ['base', 'candidate']:
    root = scratch/name
    for p in [*root.glob('src/**/*.rs'), root/'parity/peers/src/main.rs', root/'Cargo.toml',
              root/'parity/peers/Cargo.toml', root/'parity/peers/Cargo.lock']:
        source_hashes[f'{name}/{p.relative_to(root)}'] = sha(p)
source_binding = art/'source-binding.json'
if source_binding.exists():
    assert json.loads(source_binding.read_text()) == source_hashes
else:
    source_binding.write_text(json.dumps(source_hashes, indent=2)+'\n')
binaries = {}
for arm, feature, source in [('base','ours','base'), ('candidate','ours','candidate'),
                              ('cpp','cpp','candidate'), ('rust','rust','candidate')]:
    target = art/f'driver-{arm}'
    receipt = art/f'build-{arm}.json'
    if receipt.exists() and target.exists():
        previous = json.loads(receipt.read_text())
        if previous.get('exit') == 0 and previous.get('sha256') == sha(target):
            binaries[arm] = previous['sha256']
            if previous.get('clean_exit') != 0:
                clean = subprocess.run(['cargo','clean','--manifest-path',
                                        str(scratch/source/'Cargo.toml')],
                                       capture_output=True, text=True)
                (art/f'clean-{arm}.log').write_text(clean.stdout+clean.stderr)
                previous['clean_exit'] = clean.returncode
                receipt.write_text(json.dumps(previous)+'\n')
                if clean.returncode:
                    raise SystemExit(clean.returncode)
            continue
    if not floor_ok():
        print('build floor below 100 GiB; resume after space is available', flush=True)
        raise SystemExit(75)
    command = ['cargo','build','--offline','--locked','--release','--manifest-path',
               str(scratch/source/'parity/peers/Cargo.toml'), '--no-default-features',
               '--features',feature,'-j','1']
    with (art/f'build-{arm}.log').open('w') as log:
        result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
    if result.returncode:
        receipt.write_text(json.dumps(dict(exit=result.returncode))+'\n')
        raise SystemExit(result.returncode)
    shutil.copy2(target_dir/'release/meshopt-peers', target)
    binaries[arm] = sha(target)
    receipt.write_text(json.dumps(dict(exit=0,sha256=binaries[arm],clean_exit=None))+'\n')
    clean = subprocess.run(['cargo','clean','--manifest-path',str(scratch/source/'Cargo.toml'),
                            ], capture_output=True, text=True)
    (art/f'clean-{arm}.log').write_text(clean.stdout+clean.stderr)
    receipt.write_text(json.dumps(dict(exit=0,sha256=binaries[arm],clean_exit=clean.returncode))+'\n')
    if clean.returncode:
        raise SystemExit(clean.returncode)
    for incremental in target_dir.glob('*/incremental'):
        shutil.rmtree(incremental)
    for leftover in [target_dir/'release/meshopt-peers', target_dir/'release/meshopt-peers.d']:
        leftover.unlink(missing_ok=True)
    print(arm, 'built, hashed and cleaned', flush=True)
(art/'build-binding.json').write_text(json.dumps(dict(binaries=binaries, sources=source_hashes), indent=2)+'\n')
