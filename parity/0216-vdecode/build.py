"""Build all arms at one canonical source path, invalidating Cargo freshness."""
import hashlib,io,json,os,shutil,subprocess,sys,tarfile
from pathlib import Path
R=Path.cwd();T=Path(os.environ['CARGO_TARGET_DIR']);A=Path(os.environ['DECODE_ART']);D=T/'canonical'
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
A.mkdir(parents=True,exist_ok=True);D.mkdir(parents=True,exist_ok=True)
subprocess.run(['python3','parity/0216-vdecode/audit.py'],check=True)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
binding=json.loads((A/'build-binding.json').read_text()) if (A/'build-binding.json').exists() else dict(binaries={},sources={})
for arm in sys.argv[1:]:
 if (A/('build-'+arm+'.log')).exists():raise RuntimeError('refusing to overwrite build '+arm)
 archive=subprocess.check_output(['git','archive','v0.2.15','src','docs','Cargo.toml','Cargo.lock','README.md'])
 tarfile.open(fileobj=io.BytesIO(archive)).extractall(D,filter='data')
 if arm not in ['base','cpp-scalar','cpp-default']:shutil.copytree(Path(os.environ.get('DECODE_CANDIDATE_SOURCE',str(R/'src'))),D/'src',dirs_exist_ok=True)
 if arm in ['control','unisolated']:
  p=D/'src/codec/vertex.rs';s=p.read_text().replace('// Keep vertex specialization decisions out of the shared codec allocator.\n#[cfg_attr(all(feature = "simd", target_arch = "x86_64"), inline(never))]\n','');p.write_text(s)
 if arm=='control':
  p=D/'src/codec/simd/x86.rs';s=p.read_text().replace('        (true, 12) => vertex_kernel::<128, 12, D>,\n        (false, 12) => vertex_kernel::<1024, 12, D>,\n','');p.write_text(s)
 peers=D/'parity/peers';peers.mkdir(parents=True,exist_ok=True)
 shutil.copytree(R/'parity/peers',peers,dirs_exist_ok=True)
 # Touch after copy: copying timestamps can otherwise falsely reuse the last arm.
 for p in [*D.glob('src/**/*.rs'),D/'Cargo.toml',D/'Cargo.lock',peers/'Cargo.toml',peers/'src/main.rs']:p.touch()
 frozen=A/('frozen-'+arm);frozen.mkdir()
 for part in ['src','docs','parity/peers']:shutil.copytree(D/part,frozen/part)
 for part in ['Cargo.toml','Cargo.lock','README.md']:shutil.copy2(D/part,frozen/part)
 cmd=['cargo','build','--offline','--locked','--release','--manifest-path',str(peers/'Cargo.toml'),'--no-default-features','--features','cpp' if arm.startswith('cpp') else 'ours','-j','2']
 with (A/('build-'+arm+'.log')).open('x') as f:subprocess.run(cmd,env=dict(os.environ,CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD' if arm=='cpp-scalar' else '',CARGO_INCREMENTAL='0'),stdout=f,stderr=subprocess.STDOUT,check=True)
 assert arm.startswith('cpp') or 'Compiling meshoptimizer-rs' in (A/('build-'+arm+'.log')).read_text()
 b=A/('driver-'+arm);shutil.copy2(T/'release/meshopt-peers',b)
 binding['binaries'][arm]=sha(b)
 binding['sources'][arm]={str(p.relative_to(D)):sha(p) for p in [*D.glob('src/**/*.rs'),D/'Cargo.toml',peers/'Cargo.toml',peers/'src/main.rs']}
 (A/'build-binding.json').write_text(json.dumps(binding,indent=2)+'\n');print(arm,'built',flush=True)
if sys.argv[1:]==['base','control','unisolated','candidate','cpp-scalar','cpp-default']:
 # Input discovery is supplied as an existing artifact directory by the operator.
 if not (A/'inputs').exists():
  source=Path((A.parent/'input-source.txt').read_text().strip())
  (A/'inputs').symlink_to(source)
  shutil.copy2(R/'parity/peers/inputs.json',A/'inputs.json')
 env=dict(os.environ,DECODE_PHASE='screen',DECODE_ARMS='base control unisolated candidate cpp-scalar cpp-default')
 subprocess.run(['python3','parity/0216-vdecode/measure.py'],env=env,check=True)
 subprocess.run(['python3','parity/0216-vdecode/results.py'],env=env,check=True)
 subprocess.run(['python3','parity/0216-vdecode/proof.py'],env=dict(os.environ,DECODE_ART=str(A.parent/'proof')),check=True)
