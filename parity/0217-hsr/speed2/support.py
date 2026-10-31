"""Shared admitted harness helpers. Scene/binary artifacts stay outside the repo."""
import hashlib,io,json,os,shutil,subprocess,tarfile,time
from pathlib import Path
ROOT=Path.cwd(); RECEIPT=ROOT/'parity/0217-hsr/speed2'
TARGET=Path(os.environ['CARGO_TARGET_DIR']); SCRATCH=TARGET/'speed2';SCRATCH.mkdir(parents=True,exist_ok=True)
SCENES=Path(os.environ['HSR_SCENES'])
assert os.environ.get('MOSS_HEAVY_ACTIVE')
os.environ.update(CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',LC_ALL='C',GIT_OPTIONAL_LOCKS='0',RUSTDOCFLAGS='-D warnings')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(name,args,**kwargs):
    assert not (RECEIPT/(name+'.log')).exists(), 'Refusing to overwrite a retained receipt'
    start=time.monotonic();p=subprocess.run(list(map(str,args)),text=True,capture_output=True,**kwargs)
    (RECEIPT/(name+'.log')).write_text((p.stdout+p.stderr).replace(str(ROOT),'<worktree>').replace(str(TARGET),'<target>').replace(str(SCENES),'<scenes>'))
    with (RECEIPT/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\t{time.monotonic()-start:.3f}\n')
    print(name,p.returncode,(p.stdout+p.stderr)[-1500:],flush=True)
    if p.returncode:raise SystemExit(p.returncode)
    return p.stdout

def build(arm,source,features):
    folder=SCRATCH/('build-'+arm);folder.mkdir(exist_ok=True)
    (folder/'Cargo.toml').write_text(f'''[package]
name="hsr-speed2"
version="0.0.0"
edition="2021"
[features]
parallel=["dep:rayon"]
[dependencies]
rayon={{version="=1.11.0",optional=true}}
meshoptimizer-rs={{path={json.dumps(str(source))},default-features=false,features={json.dumps(features)}}}
[[bin]]
name="hsr-speed2"
path={json.dumps(str(RECEIPT/'bench.rs'))}
[profile.release]
lto="fat"
codegen-units=1
overflow-checks=true
''')
    run('build-'+arm,['cargo','build','--release','--offline','--manifest-path',folder/'Cargo.toml']+(['--features','parallel'] if 'parallel' in features else []))
    dest=SCRATCH/('driver-'+arm);shutil.copy2(TARGET/'release/hsr-speed2',dest);return dest

def reference():
    base=SCRATCH/'reference'
    if not base.exists():
        base.mkdir()
        raw=subprocess.check_output(['git','archive','f9859cc','Cargo.toml','Cargo.lock','src','docs','README.md'])
        with tarfile.open(fileobj=io.BytesIO(raw)) as archive:archive.extractall(base,filter='data')
    return base
