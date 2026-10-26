"""Build two separately named crate versions for exact public-API observations."""
from pathlib import Path
import shutil
R=Path(__file__).resolve().parents[2]
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H1')
b=S/'proof-baseline'
if not b.exists():
 shutil.copytree(S/'base',b)
 p=b/'Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
p=S/'proof';p.mkdir(exist_ok=True)
(p/'Cargo.toml').write_text(f'''[package]
name = "hot-cache-exact"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std"]
[dependencies]
candidate = {{ package = "meshoptimizer-rs", path = "{R}", default-features = false }}
baseline = {{ package = "meshoptimizer-baseline", path = "{b}", default-features = false }}
[[bin]]
name = "hot-cache-exact"
path = "{R}/parity/0210-H1/exact.rs"
[profile.release]
lto = "thin"
codegen-units = 1
''')
