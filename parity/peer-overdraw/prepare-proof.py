#!/usr/bin/env python3
"""Prepare a scratch-only two-version checker; never alters the kept harness."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import os, shutil, subprocess
ROOT = Path(__file__).resolve().parents[2]
S = Path(_public_path('<scratch>/meshopt-peer-b'))
base = S / 'proof-baseline'
if not base.exists():
    shutil.copytree(S / 'baseline', base)
    p = base / 'Cargo.toml'
    p.write_text(p.read_text().replace('name = "meshoptimizer-rs"', 'name = "meshoptimizer-baseline"', 1))
p = S / 'proof'
p.mkdir(exist_ok=True)
(p / 'src').mkdir(exist_ok=True)
(p / 'Cargo.toml').write_text(f'''[package]
name = "peer-overdraw-exact"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std"]
[dependencies]
candidate = {{ package = "meshoptimizer-rs", path = "{ROOT}", default-features = false }}
baseline = {{ package = "meshoptimizer-baseline", path = "{base}", default-features = false }}
[profile.release]
codegen-units = 1
lto = "thin"
''')
shutil.copy2(ROOT / 'parity/peer-overdraw/exact.rs', p / 'src/main.rs')
