#!/usr/bin/env python3
"""Prepare SPEC-026 scratch from retained bet-0/REPORT-3 inputs; no timings."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib, json, shutil, subprocess
ROOT = Path(__file__).resolve().parents[2]
ART = Path(_public_path('<scratch>/meshopt-decode'))
PEER = Path(_public_path('<repo>/parity/compare'))
SPEED = Path(_public_path('<scratch>/meshopt-web-decode/speed'))
BASE = 'bea3e04bf22063988a80db4764e80c3dd422a634'
assert not ART.exists(), 'retain completed attempts; never overwrite scratch'
ART.mkdir(parents=True)
baseline=ART/'baseline';baseline.mkdir()
p=subprocess.Popen(['git','-C',str(ROOT),'archive',BASE],stdout=subprocess.PIPE)
subprocess.run(['tar','-x','-C',str(baseline)],stdin=p.stdout,check=True)
assert p.wait()==0
native=ART/'native';(native/'src').mkdir(parents=True)
t=(PEER/'src/main.rs').read_text()
needle='    let out = run(op, &x, &mut w);'
assert t.count(needle)==1
t=t.replace(needle,'''    if let Some(calls) = a.get(4) {
        for _ in 0..calls.parse::<usize>().unwrap() {
            black_box(run(black_box(op), black_box(&x), &mut w));
        }
        return;
    }
'''+needle)
(native/'src/main.rs').write_text(t)
t=(PEER/'Cargo.toml').read_text().split('[[bin]]')[0]
t=t.replace('path = "../.."','path = "../baseline"')
t=t.replace('ours = ["dep:meshoptimizer-rs"]','ours = ["dep:meshoptimizer-rs", "meshoptimizer-rs/experimental", "meshoptimizer-rs/clusterlod"]')
t+='''
[[bin]]
name = "cmp-driver"
path = "src/main.rs"
[profile.moss]
inherits = "release"
opt-level = 3
lto = "thin"
codegen-units = 1
strip = false
'''
(native/'Cargo.toml').write_text(t)
browser=ART/'browser';(browser/'data').mkdir(parents=True);(browser/'out').mkdir()
for name in ['harness.mjs','drive.mjs','probe.mjs','page.html']:
    shutil.copy2(ROOT/'parity/decode'/name,browser/name)
for name in ['streams.json','streams.bin']:
    shutil.copy2(SPEED/'data'/name,browser/'data'/name)
for name in ['arm-m-oz.wasm','upstream-scalar.wasm','upstream-simd.wasm']:
    shutil.copy2(SPEED/'out'/name,browser/'out'/name)
wasm=ART/'wasm';(wasm/'src').mkdir(parents=True)
shutil.copy2(SPEED/'ws/arm-c/src/lib.rs',wasm/'src/lib.rs')
(wasm/'Cargo.toml').write_text('''[package]
name = "decode-browser"
version = "0.0.0"
edition = "2021"
[lib]
name = "decode_browser"
crate-type = ["cdylib"]
[features]
simd = ["meshoptimizer-rs/simd"]
[dependencies]
meshoptimizer-rs = { path = "../baseline", default-features = false }
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
''')
files=[*PEER.glob('src/*.rs'),PEER/'Cargo.toml',*browser.glob('data/*'),*browser.glob('out/*')]
(ART/'setup-identity.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest()for p in files},indent=2)+'\n')
shutil.copy2(ROOT/'parity/decode/REGISTRATION.md',ART/'REGISTRATION.md')
