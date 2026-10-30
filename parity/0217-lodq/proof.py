"""Reproducible admitted proof; scratch and tool paths are supplied by environment."""
import hashlib,io,json,os,shutil,subprocess,tarfile,resource,time
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
os.environ.update(CARGO_BUILD_JOBS='2', CARGO_INCREMENTAL='0', LC_ALL='C')
r=Path.cwd();lane=r/'parity/0217-lodq';out=lane/'receipts';t=Path(os.environ['CARGO_TARGET_DIR']);s=t/'proof';s.mkdir(exist_ok=True,parents=True)
def run(name,cmd,env=None):
 before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.monotonic()
 p=subprocess.run(cmd,capture_output=True,text=True,env=env)
 after=resource.getrusage(resource.RUSAGE_CHILDREN)
 timing=dict(step=name,exit=p.returncode,wall_seconds=time.monotonic()-start,cpu_seconds=(after.ru_utime-before.ru_utime)+(after.ru_stime-before.ru_stime))
 with (out/'proof-timing.jsonl').open('a') as f:f.write(json.dumps(timing)+'\n')
 (out/(name+'.log')).write_text(p.stdout+p.stderr)
 with (out/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\n')
 print(name,p.returncode,(p.stdout+p.stderr)[-1500:],flush=True)
 assert p.returncode==0,name
base=s/'base';base.mkdir(exist_ok=True)
archive=subprocess.check_output(['git','archive','v0.2.16','src','docs','Cargo.toml','Cargo.lock','README.md'])
tarfile.open(fileobj=io.BytesIO(archive)).extractall(base,filter='data')
p=base/'Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
driver=s/'driver';driver.mkdir(exist_ok=True)
(driver/'Cargo.toml').write_text(f'''[package]
name = "lodq-proof"
version = "0.0.0"
edition = "2021"
[features]
default = ["baseline/std", "baseline/simd", "candidate/std", "candidate/simd"]
[dependencies]
baseline = {{package="meshoptimizer-baseline", path="{base}", default-features=false}}
candidate = {{package="meshoptimizer-rs", path="{r}", default-features=false, features=["lod-quality"]}}
[[bin]]
name="exact-lodq"
path="{lane/'exact.rs'}"
[[bin]]
name="differential-lodq"
path="{lane/'differential.rs'}"
[profile.release]
codegen-units=1
overflow-checks=true
''')
run('oracle-build',['c++','-O2','-std=c++17',str(lane/'oracle.cpp'),'-o',str(s/'oracle')])
for mode,flags in [('default',[]),('scalar',['--no-default-features'])]:
 run('proof-build-'+mode,['cargo','build','--offline','--release','--manifest-path',str(driver/'Cargo.toml'),*flags])
 run('exact-'+mode,[str(t/'release/exact-lodq')])
 run('cpp-oracle-'+mode,[str(t/'release/differential-lodq'),str(s/'oracle')])
# Shared implementation files are exactly the base; only feature wiring changes.
hashes={}
for p in sorted((r/'src').rglob('*.rs')):
 rel=p.relative_to(r);sha=hashlib.sha256(p.read_bytes()).hexdigest();hashes[str(rel)]=sha
 if rel.as_posix() not in ['src/lib.rs','src/lod_quality.rs']:assert p.read_bytes()==(base/rel).read_bytes(),rel
(out/'source-binding.json').write_text(json.dumps(hashes,indent=2)+'\n')

assert sum(json.loads(line)["cpu_seconds"] for line in (out/"proof-timing.jsonl").read_text().splitlines()) < 7200
