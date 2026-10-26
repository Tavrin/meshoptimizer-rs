from pathlib import Path
import subprocess, re, shutil, json, hashlib
R=Path(__file__).resolve().parents[2];S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-h3')
t=(R/'src/partition.rs').read_text();a=t.index('fn sqrt(');b=t.index('\nfn score(',a)
(S/'partition-sqrt.rs').write_text('#[inline(always)]\n'+t[a:b])
for arm, lib in [('base',S/'base'),('candidate',R)]:
 h=S/f'proof-{arm}';h.mkdir(exist_ok=True)
 for f in ['exact.rs','sqrt.rs','seeds.rs']:shutil.copy2(R/'parity/0210-h3'/f,h/f)
 (h/'Cargo.toml').write_text(f'''[package]
name="h3-proof"
version="0.0.0"
edition="2021"
[features]
default=["std"]
std=["meshoptimizer-rs/std"]
[dependencies]
meshoptimizer-rs={{path="{lib}",default-features=false}}
libm={{version="=0.2.16",default-features=false}}
[[bin]]
name="h3-exact"
path="exact.rs"
[[bin]]
name="h3-sqrt"
path="sqrt.rs"
[[bin]]
name="h3-seeds"
path="seeds.rs"
[profile.release]
opt-level=3
lto="thin"
codegen-units=1
''')
 subprocess.run(['cargo','generate-lockfile','--offline','--manifest-path',str(h/'Cargo.toml')],check=True)
 for features,label in [([], 'std'),(['--no-default-features'],'nostd')]:
  subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(h/'Cargo.toml'),*features],check=True)
  target=Path('/mnt/linux-extra/moss-cargo-targets/meshopt-h3/release')
  shutil.copy2(target/'h3-exact',S/f'exact-{arm}-{label}')
  shutil.copy2(target/'h3-seeds',S/f'seeds-{arm}-{label}')
  if arm=='candidate' and label=='std':shutil.copy2(target/'h3-sqrt',S/'sqrt-exhaustive')
