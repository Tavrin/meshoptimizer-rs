#!/usr/bin/env python3
"""Isolated baseline export and identical-profile scale drivers; admission required."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib, json, os, shutil, subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
ART = Path(_public_path('<scratch>/meshopt-029-scale'))
TARGET = Path(os.environ['CARGO_TARGET_DIR'])
REF = Path(_public_path('<refs>/meshoptimizer'))
assert os.environ.get('PARITY_RUNNER_ACTIVE')
assert TARGET == Path(_public_path('<target-dir>'))
assert subprocess.check_output(['git', '-C', str(REF), 'rev-parse', 'HEAD'], text=True).strip() == '4c203430ca565cb59a468a91922c76c208169536'
base = ART/'baseline'
if not base.exists():
    base.mkdir()
    subprocess.run(['tar','-x','-C',str(base)], input=subprocess.check_output(['git','archive','v0.2.8']),check=True)
    p=base/'Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
h = ART/'harness';h.mkdir(exist_ok=True)
(h/'Cargo.toml').write_text(f'''[package]
name = "scale-proof"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std"]
base-arm = []
[dependencies]
candidate = {{ package = "meshoptimizer-rs", path = "{ROOT}", default-features = false }}
baseline = {{ package = "meshoptimizer-baseline", path = "{base}", default-features = false }}
[[bin]]
name = "scale-driver"
path = "{ROOT}/parity/029-scale/driver.rs"
[[bin]]
name = "scale-exact"
path = "{ROOT}/parity/029-scale/exact.rs"
[profile.bench-release]
inherits = "release"
opt-level = 3
lto = "thin"
codegen-units = 1
strip = "symbols"
''')
subprocess.run(['cargo','generate-lockfile','--offline','--manifest-path',str(h/'Cargo.toml')],check=True)
for arm in ['base','candidate']:
    cmd=['cargo','build','--offline','--locked','--manifest-path',str(h/'Cargo.toml'),'--profile','bench-release','--bin','scale-driver']
    if arm=='base':cmd += ['--features','base-arm']
    subprocess.run(cmd,check=True)
    shutil.copy2(TARGET/'bench-release/scale-driver',ART/f'driver-{arm}')
subprocess.run(['c++','-std=c++17','-O3','-DNDEBUG','-DMESHOPTIMIZER_NO_SIMD','-fno-fast-math','-ffp-contract=off','-I',str(REF/'src'),str(ROOT/'parity/029-scale/driver.cpp'),str(REF/'src/simplifier.cpp'),str(REF/'src/allocator.cpp'),'-o',str(ART/'driver-M7')],check=True)
(ART/'build.json').write_text(json.dumps(dict(baseline=subprocess.check_output(['git','rev-parse','v0.2.8'],text=True).strip(), oracle='4c203430ca565cb59a468a91922c76c208169536', admission=os.environ['PARITY_RUNNER_ACTIVE'],sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (ROOT/'src').rglob('*.rs')}, binaries={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in ART.glob('driver-*')}),indent=2)+'\n')
