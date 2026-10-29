"""Build the broad guard at the same canonical path as decoder screening."""
import hashlib,io,json,os,shutil,subprocess,tarfile
from pathlib import Path
R=Path.cwd();T=Path(os.environ['CARGO_TARGET_DIR']);D=T/'canonical';A=Path(os.environ['DECODE_ART']);A.mkdir(exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
binding=dict(binaries={},sources={})
for arm in ['base','control','candidate','cpp-scalar','cpp-default']:
 archive=subprocess.check_output(['git','archive','v0.2.15','src','docs','Cargo.toml','Cargo.lock','README.md'])
 tarfile.open(fileobj=io.BytesIO(archive)).extractall(D,filter='data')
 if arm in ['control','candidate']:
  shutil.copytree(R/'src',D/'src',dirs_exist_ok=True)
  if arm=='control':
   p=D/'src/codec/simd/mod.rs';s=p.read_text()
   assert s.count('if stride == 12 {')==1
   p.write_text(s.replace('if stride == 12 {','if false && stride == 12 {'))
 peers=D/'parity/peers';shutil.rmtree(peers);shutil.copytree(R/'parity/peers',peers)
 subprocess.run(['python3','parity/0216-vdecode/prepare_guard.py',str(peers)],check=True)
 for p in [*D.glob('src/**/*.rs'),D/'Cargo.toml',D/'Cargo.lock',peers/'Cargo.toml',peers/'src/main.rs']:p.touch()
 frozen=A/('frozen-'+arm);frozen.mkdir()
 for part in ['src','docs','parity/peers']:shutil.copytree(D/part,frozen/part)
 for part in ['Cargo.toml','Cargo.lock','README.md']:shutil.copy2(D/part,frozen/part)
 with (A/('build-'+arm+'.log')).open('x') as f:subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(peers/'Cargo.toml'),'--no-default-features','--features','cpp' if arm.startswith('cpp') else 'ours','-j','2'],env=dict(os.environ,CARGO_INCREMENTAL='0',CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD' if arm=='cpp-scalar' else ''),stdout=f,stderr=subprocess.STDOUT,check=True)
 assert arm.startswith('cpp') or 'Compiling meshoptimizer-rs' in (A/('build-'+arm+'.log')).read_text()
 b=A/('driver-'+arm);shutil.copy2(T/'release/meshopt-peers',b)
 def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
 binding['binaries'][arm]=sha(b);binding['sources'][arm]={str(p.relative_to(D)):sha(p) for p in [*D.glob('src/**/*.rs'),D/'Cargo.toml',peers/'Cargo.toml',peers/'src/main.rs']}
 (A/'build-binding.json').write_text(json.dumps(binding,indent=2)+'\n');print(arm,'guard built',flush=True)
(A/'inputs').symlink_to(A.parent/'screen/inputs');shutil.copy2(A.parent/'screen/inputs.json',A/'inputs.json')
