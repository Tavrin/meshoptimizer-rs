"""Build exact drivers and five peer arms inside CPU admission.

CARGO_TARGET_DIR is dedicated disposable storage. MESHOPT_INPUTS points at the
frozen eight peer fixtures; MESHOPT_REFERENCE pins the upstream 1.3 checkout.
No Git state is changed: the baseline is read through git archive.
"""
import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile

assert os.environ.get('MOSS_HEAVY_ACTIVE')
root = Path(__file__).resolve().parents[2]
lane = Path(__file__).resolve().parent
target = Path(os.environ['CARGO_TARGET_DIR'])
scratch = target / 'proof-work'
scratch.mkdir(parents=True, exist_ok=True)
base = scratch / 'base'
if not base.exists():
    base.mkdir()
    data = subprocess.check_output(['git', 'archive', '1ecf69abe9f92c17e13dc38fa97be659aee44582'], cwd=root)
    with tarfile.open(fileobj=io.BytesIO(data)) as tar:
        tar.extractall(base, filter='data')
    p = base / 'Cargo.toml'
    p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))

def run(cmd, name, env=None):
    with (scratch / (name+'.log')).open('w') as f:
        result = subprocess.run(cmd, stdout=f, stderr=subprocess.STDOUT, env=env)
    print(name,result.returncode,flush=True)
    if result.returncode:
        print((scratch/(name+'.log')).read_text()[-5000:],flush=True)
        raise SystemExit(result.returncode)

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

binding = dict(base_revision='1ecf69abe9f92c17e13dc38fa97be659aee44582',sources={},binaries={})
for mode in ['simd','scalar']:
    exact=scratch/('exact-'+mode);exact.mkdir(exist_ok=True)
    features=['parallel','clusterlod','parity-internals']+(['simd'] if mode=='simd' else [])
    deps=''
    for alias,package,path in [('baseline','meshoptimizer-baseline',base),('candidate','meshoptimizer-rs',root)]:
        deps+=f'{alias} = {{ package = "{package}", path = "{path}", default-features = false, features = {json.dumps(features)} }}\n'
    (exact/'Cargo.toml').write_text(f'[package]\nname = "clod-exact"\nversion = "0.0.0"\nedition = "2021"\n[dependencies]\n{deps}rayon = "=1.11.0"\n[[bin]]\nname = "clod-exact"\npath = "{lane / "exact.rs"}"\n[profile.release]\nlto = "thin"\ncodegen-units = 1\ndebug-assertions = true\noverflow-checks = true\n')
    run(['cargo','build','--offline','--release','--manifest-path',str(exact/'Cargo.toml')], 'build-exact-'+mode)
    binary=scratch/('exact-'+mode+'.bin');shutil.copy2(target/'release/clod-exact',binary)
    binding['binaries']['exact-'+mode]=sha(binary)

peer=scratch/'peer';peer.mkdir(exist_ok=True)
original=(root/'parity/peers/src/main.rs').read_text()
# Keep the established counting region and serialization. Disable incidental
# elapsed collection for instruction samples; retain it for wall samples.
original=original.replace('let mut w=m::Workspace::new(m::Limits{max_bytes:3usize<<30,..m::Limits::default()});','let mut w=m::Workspace::new(m::Limits{max_bytes:usize::MAX,max_work:u64::MAX});')
for arm in ['base','serial','candidate','cpp-scalar','cpp-default']:
    path=base if arm=='base' else root
    package='meshoptimizer-baseline' if arm=='base' else 'meshoptimizer-rs'
    text=original
    if arm=='candidate':
        text=text.replace('m::clusterlod::build_with_output(', 'm::clusterlod::build_parallel_with_output(')
    # Avoid timing syscall instructions in the instructions region.
    text=text.replace('let start=Instant::now();','let start=if std::env::var_os("PROBE_CTL").is_none(){Some(Instant::now())}else{None};')
    text=text.replace('let elapsed=start.elapsed().as_secs_f64();','let elapsed=start.map_or(0.0,|s|s.elapsed().as_secs_f64());')
    (peer/'main.rs').write_text(text)
    (peer/'Cargo.toml').write_text(f'[package]\nname = "clod-peer"\nversion = "0.0.0"\nedition = "2021"\n[features]\nours = ["dep:meshoptimizer-rs"]\ncpp = ["dep:meshopt"]\nrust = []\n[dependencies]\nmeshoptimizer-rs = {{ package = "{package}", path = "{path}", optional = true, features = ["parallel", "clusterlod"] }}\nmeshopt = {{ version = "=0.7.0", optional = true }}\n[[bin]]\nname = "clod-peer"\npath = "main.rs"\n[profile.release]\nlto = "thin"\ncodegen-units = 1\n')
    env=dict(os.environ)
    if arm=='cpp-scalar':env['CXXFLAGS']='-DMESHOPTIMIZER_NO_SIMD'
    else:env.pop('CXXFLAGS',None)
    run(['cargo','build','--offline','--release','--manifest-path',str(peer/'Cargo.toml'),'--features','cpp' if arm.startswith('cpp') else 'ours'], 'build-peer-'+arm,env)
    binary=scratch/('driver-'+arm);shutil.copy2(target/'release/clod-peer',binary)
    binding['binaries'][arm]=sha(binary)
    binding['sources']['consumer-'+arm]=hashlib.sha256(text.encode()).hexdigest()
for path in sorted(root.glob('src/**/*.rs')):
    binding['sources'][str(path.relative_to(root))]=sha(path)
for name in ['Cargo.toml','Cargo.lock','parity/0216-par-clod/exact.rs','parity/0216-par-clod/build.py','parity/0216-par-clod/measure.py','parity/0216-par-clod/checks.sh']:
    binding['sources'][name]=sha(root/name)
reference=Path(os.environ['MESHOPT_REFERENCE'])
binding['reference_revision']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=reference,text=True).strip()
binding['reference_header_sha256']=sha(reference/'src/meshoptimizer.h')
registry=next((Path.home()/'.cargo/registry/src').glob('*/meshopt-0.7.0'))
header=next(registry.glob('**/meshoptimizer.h'))
assert '#define MESHOPTIMIZER_VERSION 1030' in header.read_text()
binding['cpp_dependency_header_sha256']=sha(header)
for path in sorted((registry/'vendor/src').iterdir()):
    if path.suffix in ['.cpp','.h']:
        assert path.read_bytes()==(reference/'src'/path.name).read_bytes(), ('C++ source drift',path.name)
assert (registry/'vendor/demo/clusterlod.h').read_bytes()==(reference/'demo/clusterlod.h').read_bytes()
binding['cpp_reference_source_identity']=True
# The crate contains the 1.3 demo as well as core sources; retain hashes.
binding['cpp_sources']={str(p.relative_to(registry)):sha(p) for p in sorted(registry.rglob('*')) if p.suffix in ['.cpp','.h']}
(lane/'build-binding.json').write_text(json.dumps(binding,indent=2,sort_keys=True)+'\n')
