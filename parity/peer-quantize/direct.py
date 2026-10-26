#!/usr/bin/env python3
"""Compare actual Rust Result values to git-archived v0.2.6, including NaN bits."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import os,subprocess
S=Path(os.environ.get('PEER_SCRATCH',_public_path('<scratch>/meshopt-peer-a')));R=Path(__file__).resolve().parents[2]
assert os.environ.get('MOSS_HEAVY_ACTIVE')
d=S/'direct';(d/'src').mkdir(parents=True,exist_ok=True)
(d/'Cargo.toml').write_text(f'''[package]
name="peer-direct"
version="0.0.0"
edition="2021"
[dependencies]
meshoptimizer-rs={{path="{R}",default-features=false}}
libm="=0.2.16"
[profile.release]
opt-level=3
''')
baseline=subprocess.check_output(['git','show','33556c4:src/quantize.rs'],cwd=R,text=True)
(d/'src/baseline.rs').write_text('\n'.join(baseline.splitlines()[1:]))
(d/'src/main.rs').write_text('''pub use meshoptimizer_rs::Error;
#[allow(dead_code)]
mod baseline;
fn check(raw:u32) {
 let f=f32::from_bits(raw);
 assert_eq!(meshoptimizer_rs::quantize_half(f),baseline::quantize_half(f),"half {raw:x}");
 for bits in (0..=32).chain([u32::MAX]) {
  assert_eq!(meshoptimizer_rs::quantize_float(f,bits).map(f32::to_bits),baseline::quantize_float(f,bits).map(f32::to_bits),"float {raw:x}/{bits}");
 }
}
fn main() {
 let mut count=0u64;
 for p in std::env::args_os().skip(1) {
  let b=std::fs::read(p).unwrap();let n=u32::from_le_bytes(b[..4].try_into().unwrap()) as usize;
  for raw in b[8..8+n*12].chunks_exact(4) {check(u32::from_le_bytes(raw.try_into().unwrap()));count+=1;}
 }
 for seed in [20261033u64,20261034u64] {
  let mut state=seed;
  for _ in 0..4096 {state^=state<<13;state^=state>>7;state^=state<<17;check(state as u32);count+=1;}
 }
 for exp in 0..256u32 {
  for mantissa in [0,1,0xfff,0x1000,0x1fff,0x3fffff,0x7fefff,0x7ff000,0x7ffffe,0x7fffff] {
   for sign in [0,0x80000000] {check(sign|(exp<<23)|mantissa);count+=1;}
  }
 }
 println!("{{\\"passed\\":true,\\"values\\":{count},\\"float_widths\\":34}}");
}
''')
subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(d/'Cargo.toml')],check=True)
p=subprocess.run([str(Path(os.environ['CARGO_TARGET_DIR'])/'release/peer-direct'),*map(str,sorted((S/'inputs').glob('*.bin')))],check=True,capture_output=True,text=True)
(S/'direct.json').write_text(p.stdout);print(p.stdout)
