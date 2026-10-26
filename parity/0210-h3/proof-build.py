"""Freeze the proof producer once; give every arm/feature its own output name."""
from pathlib import Path
import subprocess,os,json,hashlib,shutil
R=Path(__file__).resolve().parents[2]
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-h3')
T=Path(os.environ['CARGO_TARGET_DIR'])
bundle={name:(R/'parity/0210-h3'/name).read_bytes() for name in ['exact.rs','sqrt.rs','seeds.rs']}
bundle['fuzz-generator.rs']=(R/'fuzz/p03/src.rs').read_bytes()
hashes={name:hashlib.sha256(data).hexdigest() for name,data in bundle.items()}
identity=hashlib.sha256(json.dumps(hashes,sort_keys=True).encode()).hexdigest()
t=(R/'src/partition.rs').read_text()
if 'fn sqrt(_ctx:' in t:
 a=t.index('fn sqrt(');b=t.index('\nfn score(',a)
 helper='#[inline(always)]\n'+t[a:b]
else:
 t=(R/'src/math.rs').read_text();a=t.index('fn sqrt(');b=t.index('\n/// Lanes',a)
 helper=t[a:b].replace('fn sqrt(', 'fn backend(')+"\nfn sqrt(_ctx: &mut Context<'_>, x:f32)->f32 { backend(x) }\n"
(S/'partition-sqrt.rs').write_text(helper)
records=[]
for arm,lib in [('base',S/'base'),('candidate',R)]:
 for label in ['std','nostd']:
  key=f'{arm}-{label}'
  h=S/f'proof-frozen-{key}';h.mkdir(exist_ok=True)
  for name,data in bundle.items():
   # Writing bytes gives the new snapshot a current mtime. copy2 can backdate
   # changed sources before an existing Cargo fingerprint, yielding stale code.
   (h/name).write_bytes(data)
  # The include is bound to this frozen bundle, not a live repository file.
  p=h/'seeds.rs';p.write_text(p.read_text().replace('concat!(env!("H3_ROOT"), "/fuzz/p03/src.rs")','"fuzz-generator.rs"'))
  bins={kind:f'h3-{kind}-{key}' for kind in ['exact','sqrt','seeds']}
  manifest=f'''[package]
name="h3-proof-{key}"
version="0.0.0"
edition="2021"
[features]
default=["std"]
std=["meshoptimizer-rs/std"]
[dependencies]
meshoptimizer-rs={{path="{lib}",default-features=false}}
libm={{version="=0.2.16",default-features=false}}
[profile.release]
opt-level=3
lto="thin"
codegen-units=1
'''
  for kind,name in bins.items():manifest+=f'\n[[bin]]\nname="{name}"\npath="{kind}.rs"\n'
  (h/'Cargo.toml').write_text(manifest)
  env={**os.environ,'H3_PROOF_SHA256':identity,'H3_SCRATCH':str(S)}
  subprocess.run(['cargo','generate-lockfile','--offline','--manifest-path',str(h/'Cargo.toml')],check=True,env=env)
  flags=[] if label=='std' else ['--no-default-features']
  subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(h/'Cargo.toml'),*flags],check=True,env=env)
  for kind in ['exact','seeds']:
   source=T/'release'/bins[kind];dst=S/f'{kind}-{key}';shutil.copyfile(source,dst);dst.chmod(0o755)
   records.append(dict(arm=arm,mode=label,kind=kind,binary_sha256=hashlib.sha256(dst.read_bytes()).hexdigest(),source_dir=str(h),harness_sha256=identity))
  if key=='candidate-std':
   shutil.copyfile(T/'release'/bins['sqrt'],S/'sqrt-exhaustive');(S/'sqrt-exhaustive').chmod(0o755)
for name,data in bundle.items():
 original=R/'fuzz/p03/src.rs' if name=='fuzz-generator.rs' else R/'parity/0210-h3'/name
 assert original.read_bytes()==data,('proof source changed during build',name)
(S/'proof-build-binding.json').write_text(json.dumps(dict(harness_sha256=identity,sources=hashes,helper_sha256=hashlib.sha256(helper.encode()).hexdigest(),binaries=records),indent=2)+'\n')
print('FROZEN PROOF BUILD',identity,flush=True)
