import os
from pathlib import Path
import shutil
ROOT=Path(__file__).resolve().parents[2]
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
shutil.copytree(A/'base',A/'proof-base')
p=A/'proof-base/Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
(A/'proof').mkdir();(A/'empty').mkdir();(A/'proof-inputs').mkdir()
R=Path(os.environ['MESHOPT_RESCREEN_SCRATCH'])/'inputs'
for name in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth','real-214e-overdraw','real-b6fd-overdraw','real-a082-overdraw']:
    (A/'proof-inputs'/f'{name}.bin').symlink_to(R/f'{name}.bin')
(A/'proof/Cargo.toml').write_text(f'''[package]
name = "overdraw-exact"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std"]
[dependencies]
candidate = {{ package = "meshoptimizer-rs", path = "{ROOT}", default-features = false }}
baseline = {{ package = "meshoptimizer-baseline", path = "{A}/proof-base", default-features = false }}
[[bin]]
name = "overdraw-exact"
path = "{ROOT}/parity/0212-overdraw/exact.rs"
[profile.release]
lto = "thin"
codegen-units = 1
''')
