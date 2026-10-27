#!/usr/bin/env python3
"""Exact existing FIFO seed replay and native/JS fixtures, no timed calls."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib, json, os, shutil, struct, subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
ART = Path(_public_path('<scratch>/meshopt-028-L2'))
TARGET = Path(os.environ['CARGO_TARGET_DIR'])
REF = Path(_public_path('<refs>/meshoptimizer'))
assert os.environ.get('PARITY_RUNNER_ACTIVE')
env = dict(os.environ,RUSTC_WRAPPER='',RUSTC_WORKSPACE_WRAPPER='',RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',CARGO_BUILD_JOBS='1')
source = (ROOT/'fuzz/src/p01x_main.rs').read_text()
source = source.replace('while start.elapsed() < Duration::from_secs(seconds) && executions < count {','while executions < 4096 {')
a = source.index('        let result = meshoptimizer_parity::execute(&input);')
b = source.index('        executions += 1;',a)
folder = ART/'fifo-seeds'; folder.mkdir(exist_ok=True)
source = source[:a] + f'        std::fs::write(format!("{folder}/{{executions:04}}.input"), &input)?;\n' + source[b:]
source += '\nfn main() { run("vertex_cache_fifo").unwrap(); }\n'
for arm,lib in [('base',ART/'baseline'),('candidate',ROOT)]:
    folder2 = ART/f'seed-proof-{arm}';folder2.mkdir(exist_ok=True)
    (folder2/'generator.rs').write_text(source)
    (folder2/'Cargo.toml').write_text(f'''[package]
name = "cook-seed-proof"
version = "0.0.0"
edition = "2021"
[dependencies]
meshoptimizer-rs = {{ path = "{lib}", features = ["experimental"] }}
meshoptimizer-parity = {{ path = "{lib}/parity" }}
[[bin]]
name = "cook-seed-protocol"
path = "{ROOT}/parity/028-L2/protocol.rs"
[[bin]]
name = "cook-seed-generator"
path = "generator.rs"
[profile.release]
lto = "thin"
codegen-units = 1
''')
    subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(folder2/'Cargo.toml')],env=env,check=True)
    shutil.copy2(TARGET/'release/cook-seed-protocol',ART/f'seed-protocol-{arm}')
subprocess.run([str(TARGET/'release/cook-seed-generator'),'1'],env=env,check=True)
# Existing upstream fixtures, kept exactly as exported by their generators.
fixtures = ART/'upstream-inputs'; fixtures.mkdir(exist_ok=True)
flags = ['-std=c++17','-O3','-DMESHOPTIMIZER_NO_SIMD','-fno-fast-math','-ffp-contract=off','-I',str(REF/'src')]
for name,files in [('native',['simplifier','allocator']),('p01x-native',['simplifier','indexgenerator','quantization','allocator','vcacheoptimizer','vfetchoptimizer','overdrawoptimizer','meshletutils'])]:
    cpp = ART/(name+'-fixtures.cpp'); binary = ART/(name+'-fixtures')
    subprocess.run(['python3',str(ROOT/'parity'/(name+'-fixtures.py')),str(REF),str(cpp)],env=env,check=True)
    subprocess.run(['c++',*flags,str(cpp),*[str(REF/'src'/(f+'.cpp')) for f in files],'-o',str(binary)],env=env,check=True)
    subprocess.run([str(binary),str(fixtures)],env=env,check=True)
for script in ['js-fixtures.cjs','p01x-js-fixtures.cjs']:
    subprocess.run(['node',str(ROOT/'parity'/script),str(REF),str(fixtures)],env=env,check=True)
class Client:
    def __init__(self,arm): self.p = subprocess.Popen([str(ART/f'seed-protocol-{arm}')],stdin=subprocess.PIPE,stdout=subprocess.PIPE)
    def run(self,data):
        self.p.stdin.write(struct.pack('<I',len(data))+data);self.p.stdin.flush()
        size = self.p.stdout.read(4); assert len(size)==4
        return self.p.stdout.read(struct.unpack('<I',size)[0])
    def close(self): self.p.stdin.close();self.p.wait();assert self.p.returncode==0
base,candidate = Client('base'),Client('candidate')
rows = []
for p in sorted(folder.glob('*.input'))+sorted(fixtures.glob('*.input')):
    data = p.read_bytes();op = struct.unpack_from('<I',data,4)[0]
    if data[:4] == b'MO01' and op not in [1,2,6]: continue
    if data[:4] == b'MO02' and op not in [7,8]: continue
    a,b = base.run(data),candidate.run(data)
    assert a==b, str(p)
    # Only upstream checked valid fixtures are sent through unsafe C++ FFI.
    cpp_hash = None
    if p.parent==fixtures and a[0]==0:
        cpp = subprocess.check_output([str(ART/'cpp-reference')],input=data)
        assert a[1:] == cpp,str(p)
        cpp_hash = hashlib.sha256(cpp).hexdigest()
    rows.append(dict(input=str(p),input_sha256=hashlib.sha256(data).hexdigest(),result_sha256=hashlib.sha256(a).hexdigest(),error=bool(a[0]),cpp=cpp_hash))
base.close();candidate.close()
(ART/'seed-proof.json').write_text(json.dumps(dict(passed=True,cases=rows),indent=2)+'\n')
print('PASS',len(rows),'existing FIFO seeds and native/JS fixtures')
