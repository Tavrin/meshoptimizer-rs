"""Build frozen baseline, candidate, C++ and optimesh peer consumers."""
import hashlib, json, os, shutil, subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
root=Path(__file__).resolve().parents[2]; art=Path(os.environ['ATTR_ART']); base=Path(os.environ['ATTR_BASE'])
h=art/'base-harness'
if not h.exists():
 shutil.copytree(root/'parity/peers',h,ignore=shutil.ignore_patterns('__pycache__'))
 p=h/'Cargo.toml'; s=p.read_text(); s=s.replace('path = "../.."',f'path = "{base}"'); p.write_text(s)
for arm,feature,manifest in [('base','ours',h/'Cargo.toml'),('candidate','ours',root/'parity/peers/Cargo.toml'),('cpp','cpp',root/'parity/peers/Cargo.toml'),('rust','rust',root/'parity/peers/Cargo.toml')]:
 cmd=['cargo','build','--offline','--locked','--release','--manifest-path',str(manifest),'--no-default-features','--features',feature,'-j','1']
 env=dict(os.environ,CARGO_PROFILE_RELEASE_DEBUG='0');env.pop('CXXFLAGS',None)
 with (art/f'full-build-{arm}.log').open('w') as log:r=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT)
 if r.returncode:raise SystemExit(r.returncode)
 shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release/meshopt-peers',art/f'full-driver-{arm}')
 print(arm,'built',flush=True)
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
binding=dict(binaries={x:sha(art/f'full-driver-{x}') for x in ('base','candidate','cpp','rust')}, source={str(p.relative_to(root)):sha(p) for p in [*root.glob('src/**/*.rs'),*root.glob('parity/peers/src/*.rs'),root/'parity/peers/Cargo.toml',root/'parity/peers/Cargo.lock']})
(art/'full-build-binding.json').write_text(json.dumps(binding,indent=2)+'\n')
