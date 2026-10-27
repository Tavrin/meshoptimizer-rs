
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import shutil
R=Path(__file__).resolve().parents[2];S=Path(_public_path('<scratch>/meshopt-0210-H2'))
b=S/'proof-baseline'
if not b.exists():
 shutil.copytree(S/'baseline',b)
 p=b/'Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
h=S/'proof';h.mkdir(exist_ok=True)
(h/'Cargo.toml').write_text(f'''[package]
name = "h2-exact"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std"]
[dependencies]
candidate = {{package="meshoptimizer-rs",path="{R}",default-features=false}}
baseline = {{package="meshoptimizer-baseline",path="{b}",default-features=false}}
[[bin]]
name="overdraw-exact"
path="{R}/parity/0210-H2/overdraw-exact.rs"
[[bin]]
name="scale-exact"
path="{R}/parity/0210-H2/scale-exact.rs"
[profile.moss]
inherits="release"
opt-level=3
lto="thin"
codegen-units=1
''')
