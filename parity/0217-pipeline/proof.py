"""Admitted baseline, C++ oracle, corpus equivalence and paired benchmark driver."""
import hashlib, io, json, os, subprocess, tarfile, shutil
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
r=Path.cwd(); lane=r/'parity/0217-pipeline'; out=lane/'receipts'
t=Path(os.environ['CARGO_TARGET_DIR']); s=t/'proof'; s.mkdir(parents=True,exist_ok=True)
def run(name,cmd):
 p=subprocess.run(cmd,capture_output=True,text=True)
 (out/(name+'.log')).write_text(p.stdout+p.stderr)
 with (out/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\n')
 print(name,p.returncode,(p.stdout+p.stderr)[-1800:],flush=True)
 if p.returncode:raise SystemExit(p.returncode)
base=s/'base';base.mkdir(exist_ok=True)
a=subprocess.check_output(['git','archive','v0.2.16','src','docs','Cargo.toml','Cargo.lock','README.md'])
tarfile.open(fileobj=io.BytesIO(a)).extractall(base,filter='data')
p=base/'Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
driver=s/'driver';driver.mkdir(exist_ok=True)
(driver/'Cargo.toml').write_text(f'''[package]
name="pipeline-proof"
version="0.0.0"
edition="2021"
[features]
default=["baseline/std","baseline/simd","candidate/std","candidate/simd"]
[dependencies]
baseline={{package="meshoptimizer-baseline",path="{base}",default-features=false}}
candidate={{package="meshoptimizer-rs",path="{r}",default-features=false}}
meshoptimizer-rs={{path="{r}",features=["clusterlod"]}}
[[bin]]
name="exact"
path="{r/'parity/0217-lodq/exact.rs'}"
[[bin]]
name="differential"
path="{r/'parity/0217-lodq/differential.rs'}"
[[bin]]
name="pipeline"
path="{lane/'driver.rs'}"
''')
# Avoid depending twice on the same crate with different names in one graph.
p=driver/'Cargo.toml';p.write_text(p.read_text().replace('meshoptimizer-rs={path="'+str(r)+'",features=["clusterlod"]}\n',''))
p=lane/'driver.rs'
# Driver uses the candidate alias supplied by the scratch manifest.
run('oracle-build',['c++','-O2','-std=c++17',str(r/'parity/0217-lodq/oracle.cpp'),'-o',str(s/'oracle')])
for mode,flags in [('default',[]),('scalar',['--no-default-features'])]:
 run('proof-build-'+mode,['cargo','build','--release','--manifest-path',str(driver/'Cargo.toml'),*flags])
 if mode=='default':shutil.copy2(t/'release/pipeline',s/'pipeline-default')
 run('exact-'+mode,[str(t/'release/exact')])
 run('cpp-'+mode,[str(t/'release/differential'),str(s/'oracle')])
inputs=Path(os.environ['PIPELINE_INPUTS'])
paths=[inputs/(n+'.bin') for n in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth','real-214e','real-a082','real-b6fd']]
(out/'inputs.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},indent=2)+'\n')
cpu=min(os.sched_getaffinity(0))
(out/'benchmark-environment.json').write_text(json.dumps(dict(cpu=cpu,configuration='default std+simd',binary_sha256=hashlib.sha256((s/'pipeline-default').read_bytes()).hexdigest(),rustc=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2)+'\n')
if not os.environ.get('PIPELINE_PROOFS_ONLY'):
 run('equivalence-bench',['taskset','-c',str(cpu),str(s/'pipeline-default'),*map(str,paths)])
