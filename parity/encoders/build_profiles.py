#!/usr/bin/env python3
"""Build SPEC-03 consumers; invoke inside <job-runner> admission."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ.get('MESHOPT_ENCODER_ARTIFACTS', _public_path('<scratch>/meshopt-enc03')))
TARGET = Path(_public_path('<target-dir>'))
assert Path(os.environ['CARGO_TARGET_DIR']) == TARGET
assert os.environ.get('MOSS_HEAVY_ACTIVE'), _public_path('run through <job-runner>')
ART.mkdir(parents=True, exist_ok=True)
(TARGET / 'tmp').mkdir(parents=True, exist_ok=True)
ENV = dict(os.environ, MESHOPT_REFERENCE=_public_path('<refs>/meshoptimizer'),
           CARGO_INCREMENTAL='0', CARGO_BUILD_JOBS='4', RUSTFLAGS='',
           CARGO_ENCODED_RUSTFLAGS='', RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='',
           TMPDIR=str(TARGET / 'tmp'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sources():
    return {str(p.relative_to(ROOT)): sha(p) for p in (ROOT / 'src').rglob('*.rs')}


def consumers():
    return {str(p.relative_to(ROOT)): sha(p)
            for directory in ('parity/codec', 'parity/encoders', 'parity/compare')
            for p in (ROOT / directory).rglob('*')
            if p.is_file() and p.suffix in ('.rs', '.cpp', '.toml', '.lock')}


source = sources()
consumer = consumers()
state_path = ART / 'build.json'
prior = json.loads(state_path.read_text()) if state_path.exists() else None
if prior:
    assert prior['source'] == source, 'use a fresh named source epoch'
    assert prior.get('consumer', consumer) == consumer, 'consumer changed'
commands = []
binaries = {}


def run(command):
    command = list(map(str, command))
    print('BUILD', command, flush=True)
    started = time.monotonic()
    subprocess.run(command, cwd=ROOT, env=ENV, check=True)
    commands.append({'command': command, 'seconds': time.monotonic() - started,
                     'release_lto': ENV.get('CARGO_PROFILE_RELEASE_LTO'),
                     'release_codegen_units': ENV.get('CARGO_PROFILE_RELEASE_CODEGEN_UNITS')})
    size = sum(p.stat().st_size for p in TARGET.rglob('*') if p.is_file())
    assert size < 10_000_000_000, 'Cargo target exceeds 10 GB'


def existing(path):
    if not path.exists():
        return False
    assert prior and prior['binaries'].get(path.name) == sha(path), 'unbound existing binary; retain it and use a new epoch'
    binaries[path.name] = sha(path)
    return True


for profile in ('defaults', 'moss'):
    ENV.update(CARGO_PROFILE_RELEASE_LTO='false' if profile == 'defaults' else 'thin',
               CARGO_PROFILE_RELEASE_CODEGEN_UNITS='16' if profile == 'defaults' else '1',
               CARGO_PROFILE_RELEASE_STRIP='symbols')
    for manifest, binary, features in (
            ('parity/codec/Cargo.toml', 'codec-driver', ['--features', 'simd']),
            ('parity/encoders/Cargo.toml', 'encoder-crate-oracle', [])):
        destination = ART / f'{binary}-{profile}'
        if existing(destination):
            continue
        run(['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', manifest, *features])
        shutil.copy2(TARGET / 'release' / binary, destination)
        binaries[destination.name] = sha(destination)
    for backend in ('ours', 'theirs'):
        destination = ART / f'cmp-driver-{profile}-{backend}'
        if existing(destination):
            continue
        run(['cargo', 'build', '--offline', '--locked', '--profile', profile,
             '--manifest-path', 'parity/compare/Cargo.toml', '--features', backend, '--bin', 'cmp-driver'])
        shutil.copy2(TARGET / profile / 'cmp-driver', destination)
        binaries[destination.name] = sha(destination)

reference = Path(ENV['MESHOPT_REFERENCE'])
run(['parity/check-reference.sh'])
for scalar in (True, False):
    destination = ART / ('cpp-scalar' if scalar else 'cpp')
    if existing(destination):
        continue
    run(['c++', '-std=c++17', '-O3', '-DNDEBUG', '-fno-fast-math', '-ffp-contract=off',
         *(['-DMESHOPTIMIZER_NO_SIMD'] if scalar else []), '-I', reference / 'src',
         ROOT / 'parity/encoders/reference.cpp',
         *[reference / 'src' / f for f in ('vertexcodec.cpp', 'indexcodec.cpp', 'vertexfilter.cpp', 'meshletcodec.cpp', 'allocator.cpp')],
         '-o', destination])
    binaries[destination.name] = sha(destination)

assert source == sources() and consumer == consumers(), 'source changed during compilation'
manifest = {'revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
            'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
            'cxx': subprocess.check_output(['c++', '--version'], text=True),
            'source': source, 'consumer': consumer, 'binaries': binaries,
            'commands': commands, 'admission': os.environ['MOSS_HEAVY_ACTIVE'],
            'profiles': {'defaults': {'lto': False, 'codegen-units': 16}, 'moss': {'lto': 'thin', 'codegen-units': 1}}}
if prior:
    # Do not erase the original build receipt when checking a completed build.
    (ART / 'build-reuse.json').write_text(json.dumps(manifest, indent=2) + '\n')
else:
    state_path.write_text(json.dumps(manifest, indent=2) + '\n')
