#!/usr/bin/env python3
"""Build existing protocol oracles and exact Result transport, never timing."""

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

ART = Path(_public_path('<scratch>/meshopt-peer-c'))
ROOT = Path(__file__).resolve().parents[2]
REF = Path(_public_path('<refs>/meshoptimizer'))
TARGET = Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('PARITY_RUNNER_ACTIVE')
assert TARGET == Path(_public_path('<target-dir>'))
env = dict(os.environ, RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='', RUSTFLAGS='',
           CARGO_ENCODED_RUSTFLAGS='', CARGO_BUILD_JOBS='4', CARGO_INCREMENTAL='0')
def snapshot():
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in (ROOT / 'src').rglob('*.rs')}

before = snapshot()
subprocess.run([str(ROOT / 'parity/check-reference.sh')], env=dict(env, MESHOPT_REFERENCE=str(REF)), check=True)
for arm, source in [('base', ART / 'baseline'), ('candidate', ROOT)]:
    folder = ART / f'proof-{arm}'
    folder.mkdir(exist_ok=True)
    manifest = folder / 'Cargo.toml'
    manifest.write_text(f'''[package]
name = "peer-small-proof"
version = "0.0.0"
edition = "2021"
[dependencies]
meshoptimizer-rs = {{ path = "{source}", default-features = false }}
meshoptimizer-parity = {{ path = "{source}/parity" }}
meshopt-p03-parity = {{ path = "{source}/parity/p03" }}
[[bin]]
name = "peer-small-proof"
path = "{ROOT}/parity/peer-small/proof.rs"
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
''')
    subprocess.run(['cargo', 'build', '--offline', '--release', '--manifest-path', str(manifest)], env=env, check=True)
    shutil.copy2(TARGET / 'release/peer-small-proof', ART / f'proof-{arm}.bin')

flags = ['-std=c++17', '-O3', '-DMESHOPTIMIZER_NO_SIMD', '-fno-fast-math', '-ffp-contract=off', '-I', str(REF / 'src')]
def cpp(source, names, output, extra=()):
    subprocess.run(['c++', *flags, *extra, str(source),
                    *(str(REF / f'src/{name}.cpp') for name in names), '-o', str(output)], check=True)

cpp(ROOT / 'parity/reference.cpp', ['vcacheoptimizer', 'overdrawoptimizer', 'allocator', 'simplifier', 'indexgenerator', 'vfetchoptimizer', 'quantization'], ART / 'cpp-p01x', ['-DNDEBUG'])
cpp(ROOT / 'parity/p03/reference.cpp', ['clusterizer', 'meshletutils', 'spatialorder', 'partition', 'allocator', 'simplifier', 'indexgenerator'], ART / 'cpp-p03', ['-I', str(REF / 'demo')])
for release, generator, names in [
    ('p01x', ROOT / 'parity/p01x-native-fixtures.py', ['simplifier', 'indexgenerator', 'quantization', 'allocator', 'vcacheoptimizer', 'vfetchoptimizer', 'overdrawoptimizer', 'meshletutils']),
    ('p03', ROOT / 'parity/p03/native-fixtures.py', ['clusterizer', 'meshletutils', 'spatialorder', 'partition', 'allocator', 'simplifier', 'indexgenerator'])]:
    generated = ART / f'{release}-fixtures.cpp'
    subprocess.run(['python3', str(generator), str(REF), str(generated)], check=True)
    binary = ART / f'{release}-fixtures'
    cpp(generated, names, binary)
    dest = ART / f'{release}-inputs'
    dest.mkdir(exist_ok=True)
    subprocess.run([str(binary), str(dest)], check=True)
    js = ROOT / ('parity/p01x-js-fixtures.cjs' if release == 'p01x' else 'parity/p03/js-fixtures.mjs')
    subprocess.run(['node', str(js), str(REF), str(dest)], check=True)

# Export exact inputs from the existing stable fuzz generator, retaining its
# original per-family RNG seeds and mutations; no time-budget fuzz campaign.
source = (ROOT / 'fuzz/src/p01x_main.rs').read_text()
source = source.replace('while start.elapsed() < Duration::from_secs(seconds)', 'while executions < 256')
a = source.index('        let result = meshoptimizer_parity::execute(&input);')
b = source.index('        executions += 1;', a)
source = source[:a] + _public_path('''        std::fs::write(format!("<scratch>/meshopt-peer-c/fuzz-inputs/{name}-{executions}.input"), &input)?;
''') + source[b:]
source += '\nfn main() { run(&std::env::var("PEER_FAMILY").unwrap()).unwrap(); }\n'
generator = ART / 'fuzz-generator.rs'
generator.write_text(source)
subprocess.run(['rustc', '--edition=2021', '-O', str(generator), '-o', str(ART / 'fuzz-generator')], env=env, check=True)
(ART / 'fuzz-inputs').mkdir(exist_ok=True)
for family in ['generate_adjacency_index_buffer', 'remap_index_buffer', 'vertex_fetch', 'generate_vertex_remap']:
    subprocess.run([str(ART / 'fuzz-generator'), '1'], env=dict(env, PEER_FAMILY=family), check=True)
receipt = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in
           [ART / 'proof-base.bin', ART / 'proof-candidate.bin', ART / 'cpp-p01x', ART / 'cpp-p03', generator]}
assert before == snapshot(), 'library changed during proof build'
(ART / 'proof-build.json').write_text(json.dumps(dict(binaries=receipt, source=before), indent=2) + '\n')
