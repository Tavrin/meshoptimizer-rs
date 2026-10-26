#!/usr/bin/env python3
"""Prepare a scratch-only two-version checker; never alters the kept harness."""
from pathlib import Path
import os, shutil, subprocess
ROOT = Path(__file__).resolve().parents[2]
S = Path('/mnt/linux-extra/moss-scratch/meshopt-peer-b')
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
