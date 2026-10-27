"""Targeted existing C++ differentials and exact legacy FIFO seed replay."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import array, hashlib, json, os, random, shutil, struct, subprocess, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
ART=Path(_public_path('<scratch>/meshopt-0210-H1'))
REF=Path(_public_path('<refs>/meshoptimizer'))
TARGET=Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('MOSS_HEAVY_ACTIVE')
sys.path.insert(0,str(ROOT/'parity'))
import runner as r
import p01x
ENV=dict(os.environ,MESHOPT_REFERENCE=str(REF),RUSTC_WRAPPER='',RUSTC_WORKSPACE_WRAPPER='',RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',CARGO_BUILD_JOBS='1',CARGO_INCREMENTAL='0')
record=dict(gates={},cpp=[],seeds=[],passed=False)
out=ART/'proof-outputs';out.mkdir(exist_ok=True)
def save(): (ART/'cpp-proof.json').write_text(json.dumps(record,indent=2)+'\n')
def run(name,cmd):
 with (ART/f'proof-{name}.log').open('w') as log:
  rc=subprocess.run([str(x) for x in cmd],env=ENV,stdout=log,stderr=subprocess.STDOUT).returncode
 record['gates'][name]=rc;save()
 if rc:raise RuntimeError((name,rc,(ART/f'proof-{name}.log').read_text()[-3000:]))
run('oracle-identity',[ROOT/'parity/check-reference.sh'])
cpp=ART/'cpp-reference'
run('cpp-build',['c++',*r.FLAGS,'-I',REF/'src',ROOT/'parity/reference.cpp',*[REF/'src'/f for f in ['vcacheoptimizer.cpp','overdrawoptimizer.cpp','allocator.cpp','simplifier.cpp','indexgenerator.cpp','vfetchoptimizer.cpp','quantization.cpp']],'-o',cpp])
run('rust-protocol-build',['cargo','build','--offline','--locked','--release','--manifest-path',ROOT/'parity/Cargo.toml','--features','meshoptimizer-rs/std'])
rust=ART/'rust-protocol';shutil.copy2(TARGET/'release/meshopt-driver',rust)
cases=list(r.fixtures());rng=random.Random(20261002)
for i in range(128):
 p,ix,t,kind=r.random_mesh(rng,i);cases.append((f'existing-seed-20261002-{i}-{kind}',p,ix,t))
for name,p,ix,threshold in cases:
 for op in [1]:
  data=r.message(op,p,ix,threshold);a,b=r.execute(cpp,data),r.execute(rust,data)
  assert a==b,(name,op)
  record['cpp'].append(dict(case=name,operation=op,input_sha256=hashlib.sha256(data).hexdigest(),output_sha256=hashlib.sha256(a).hexdigest()));save()
# Existing MO02 differential covers strip and FIFO cache-size variants.
geo=list(p01x.geometries());rng=random.Random(20261003)
for i in range(128):geo.append((f'p01x-seed-{i}',*p01x.random_geometry(rng,i)))
for number,(name,p,ix) in enumerate(geo):
 for op in [7,8]:
  data=p01x.message(op,p,ix,number);a,b=r.execute(cpp,data),r.execute(rust,data)
  assert a==b,(name,op)
  record['cpp'].append(dict(case=name,operation=op,input_sha256=hashlib.sha256(data).hexdigest(),output_sha256=hashlib.sha256(a).hexdigest()));save()
# Existing FIFO generator unchanged, bounded by executions rather than wall time.
source=(ROOT/'fuzz/src/p01x_main.rs').read_text()
source=source.replace('while start.elapsed() < Duration::from_secs(seconds) && executions < count {','while executions < 4096 {')
a=source.index('        let result = meshoptimizer_parity::execute(&input);');b=source.index('        executions += 1;',a)
folder=ART/'fifo-seeds';folder.mkdir(exist_ok=True)
source=source[:a]+f'        std::fs::write(format!("{folder}/{{executions:04}}.input"), &input)?;\n'+source[b:]
source+='\nfn main() { run("vertex_cache_fifo").unwrap(); }\n'
for arm,lib in [('base',ART/'base'),('candidate',ROOT)]:
 h=ART/f'seed-proof-{arm}';h.mkdir(exist_ok=True)
 (h/'generator.rs').write_text(source)
 (h/'Cargo.toml').write_text(f'''[package]
name = "hot-cache-seed-proof"
version = "0.0.0"
edition = "2021"
[dependencies]
meshoptimizer-rs = {{ path = "{lib}", features = ["experimental"] }}
meshoptimizer-parity = {{ path = "{lib}/parity" }}
[[bin]]
name = "hot-cache-seed-protocol"
path = "{ROOT}/parity/028-L2/protocol.rs"
[[bin]]
name = "hot-cache-seed-generator"
path = "generator.rs"
[profile.release]
lto = "thin"
codegen-units = 1
''')
 run('seed-build-'+arm,['cargo','build','--offline','--release','--manifest-path',h/'Cargo.toml'])
 shutil.copy2(TARGET/'release/hot-cache-seed-protocol',ART/f'seed-protocol-{arm}')
run('seed-generator',[TARGET/'release/hot-cache-seed-generator','0'])
for mode in ['std','no-std']:
 run('fifo-exact-'+mode,[ART/f'exact-{mode}','fifo-seeds',folder])
fixtures=ART/'upstream-inputs';fixtures.mkdir(exist_ok=True)
flags=['-std=c++17','-O3','-DMESHOPTIMIZER_NO_SIMD','-fno-fast-math','-ffp-contract=off','-I',str(REF/'src')]
for name,files in [('native',['simplifier','allocator']),('p01x-native',['simplifier','indexgenerator','quantization','allocator','vcacheoptimizer','vfetchoptimizer','overdrawoptimizer','meshletutils'])]:
 cppgen=ART/(name+'-fixtures.cpp');binary=ART/(name+'-fixtures')
 run(name+'-generate',['python3',ROOT/'parity'/(name+'-fixtures.py'),REF,cppgen])
 run(name+'-build',['c++',*flags,cppgen,*[REF/'src'/(f+'.cpp') for f in files],'-o',binary])
 run(name+'-export',[binary,fixtures])
for script in ['js-fixtures.cjs','p01x-js-fixtures.cjs']:
 run(script,['node',ROOT/'parity'/script,REF,fixtures])
class Client:
 def __init__(self,arm):self.p=subprocess.Popen([str(ART/f'seed-protocol-{arm}')],stdin=subprocess.PIPE,stdout=subprocess.PIPE)
 def run(self,data):
  self.p.stdin.write(struct.pack('<I',len(data))+data);self.p.stdin.flush()
  size=self.p.stdout.read(4);assert len(size)==4
  return self.p.stdout.read(struct.unpack('<I',size)[0])
 def close(self):self.p.stdin.close();self.p.wait();assert self.p.returncode==0
base,candidate=Client('base'),Client('candidate')
for p in sorted(folder.glob('*.input'))+sorted(fixtures.glob('*.input')):
 data=p.read_bytes();op=struct.unpack_from('<I',data,4)[0]
 if data[:4]==b'MO01' and op not in [1]:continue
 if data[:4]==b'MO02' and op not in [7,8]:continue
 a,b=base.run(data),candidate.run(data);assert a==b,str(p)
 cpp_hash=None
 if p.parent==fixtures and a[0]==0:
  ref=subprocess.check_output([str(cpp)],input=data);assert a[1:]==ref,str(p)
  cpp_hash=hashlib.sha256(ref).hexdigest()
 record['seeds'].append(dict(input=str(p),input_sha256=hashlib.sha256(data).hexdigest(),result_sha256=hashlib.sha256(a).hexdigest(),error=bool(a[0]),cpp=cpp_hash))
base.close();candidate.close();record['passed']=True;save()
print('PASS C++',len(record['cpp']),'and FIFO/native/JS seeds',len(record['seeds']))
