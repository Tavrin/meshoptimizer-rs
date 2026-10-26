#!/usr/bin/env python3
"""Prepare a scratch-only two-version checker; never alters the kept harness."""
from pathlib import Path
import os, shutil, subprocess
ROOT = Path(__file__).resolve().parents[2]
S = Path('/mnt/linux-extra/moss-scratch/meshopt-028-L2')
base = S / 'proof-baseline'
if not base.exists():
    shutil.copytree(S / 'baseline', base)
    p = base / 'Cargo.toml'
    p.write_text(p.read_text().replace('name = "meshoptimizer-rs"', 'name = "meshoptimizer-baseline"', 1))
p = S / 'proof'
p.mkdir(exist_ok=True)
(p / 'src').mkdir(exist_ok=True)
(p / 'Cargo.toml').write_text(f'''[package]
name = "cook-chain-exact"
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
shutil.copy2(ROOT / 'parity/028-L2/exact.rs', p / 'src/main.rs')
# Reuse the scale fuzz generator verbatim up to and after the calls under test.
# Replace only its observation body with the two-version checker. This retains
# the exact existing seed/RNG stream, including unused constructor mutations.
text = (ROOT / 'fuzz/src/lib.rs').read_text()
text = text[text.index('pub fn run_simplifier('):]
text = text.replace('pub fn run_simplifier(', 'fn scale_seed_replay(')
text = text.replace('    let args: Vec<String> = std::env::args().collect();', '')
text = text.replace('    let args: Vec<_> = std::env::args().collect();', '')
a = text.index('    let seconds: u64 =')
b = text.index('    if seed == 0', a)
text = text[:a] + '    let seed = 20261002u64;\n    let mut checks = 0usize;\n' + text[b:]
text = text.replace('while start.elapsed() < Duration::from_secs(seconds) && executions < limit {', 'while executions < 4096 {')
a = text.index('        let mut ws = Workspace::new(limits);')
b = text.index('        let _ = SimplifyOptions::from_bits(rng.next());', a)
text = text[:a] + '''        checks += check(&Case { p, indices: ib, threshold: 1.05 },
            &format!("existing-scale-seed={seed} execution={executions}"),
            &[limits.max_work], &[limits.max_bytes], 8);
''' + text[b:]
a = text.index('    let name = match kind {')
text = text[:a] + '    println!("PASS {checks} existing scale seed observations");\n    Ok(())\n}\n'
text = text.replace('    let mut successes = 0u64;', '')
text = text.replace('.collect::<Result<_, _>>()?;', '.collect::<Result<_, _>>().unwrap();')
text = text.replace('SimplifyOptions::from_bits([0, 1, 4, 16, 32, 64, 33, 36][rng.next() as usize % 8])?;', 'SimplifyOptions::from_bits([0, 1, 4, 16, 32, 64, 33, 36][rng.next() as usize % 8]).unwrap();')
text = text.replace('    let start = Instant::now();', '')
text = text.replace('    let current = Arc::new(AtomicU64::new(0));', '    use candidate::*;\n    use std::sync::{Arc, atomic::{AtomicU64, Ordering}};\n    let current = Arc::new(AtomicU64::new(0));')
# use candidate must precede generator types/constructors; scoped import is visible
# throughout its function, independent of the textual position of the statement.
checker = (p / 'src/main.rs').read_text()
checker = checker.replace('fn main() {', 'fn main() {\n    scale_seed_replay(6).unwrap();', 1)
(p / 'src/main.rs').write_text(checker + '\n' + text)
