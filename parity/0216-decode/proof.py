"""Sequential proof steps; retain every exit and log, without masking failures."""
import hashlib,json,os,shutil,subprocess,time,resource,types
from pathlib import Path
root=Path.cwd();target=Path(os.environ['CARGO_TARGET_DIR']);art=Path(os.environ['DECODE_ART']);proof=target/'proof-source';proof.mkdir(exist_ok=True)
for name in ['exact.rs','fixtures.rs','wasm_check.rs']:shutil.copy2(root/'parity/0216-decode'/name,proof/name)
# Cargo lockfiles cannot distinguish two same-name/version path packages.
# Rename only the proof baseline package; keep its Rust sources byte-identical.
base=target/'proof-baseline'
shutil.copytree(target/'frozen-base',base,dirs_exist_ok=True)
p=base/'Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
p=base/'Cargo.lock';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))

(proof/'Cargo.toml').write_text('''[package]
name="decode-exact"
version="0.0.0"
edition="2021"
[features]
default=["baseline/std","candidate/std","baseline/simd","candidate/simd"]
[dependencies]
baseline={package="meshoptimizer-baseline",path="../proof-baseline",default-features=false}
candidate={package="meshoptimizer-rs",path='''+repr(str(root))+''',default-features=false}
[[bin]]
name="exact"
path="exact.rs"
[profile.release]
lto="fat"
codegen-units=1
''')
wasm=target/'wasm-proof-source';wasm.mkdir(exist_ok=True)
for name in ['wasm_check.rs','fixtures.rs']:shutil.copy2(proof/name,wasm/name)
(wasm/'Cargo.toml').write_text('[package]\nname="decode-wasm-proof"\nversion="0.0.0"\nedition="2021"\n[lib]\npath="wasm_check.rs"\ncrate-type=["cdylib"]\n[dependencies]\nmeshoptimizer-rs={path='+repr(str(root))+',features=["parity-internals"]}\n[profile.release]\nlto="fat"\ncodegen-units=1\n')
exits=[]
def step(name,args,env=None):
    print(name,flush=True)
    started=time.monotonic(); before=resource.getrusage(resource.RUSAGE_CHILDREN)
    with (art/(name+'.log')).open('w') as f:
        try:
            r=subprocess.run(args,stdout=f,stderr=subprocess.STDOUT,env=dict(os.environ,**(env or {})))
        except OSError as error:
            f.write(str(error)+'\n');r=types.SimpleNamespace(returncode=127)
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    exits.append(dict(step=name,exit=r.returncode,wall_seconds=time.monotonic()-started,cpu_seconds=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime));(art/'proof-exits.json').write_text(json.dumps(exits,indent=2)+'\n')
    print(name,r.returncode,flush=True)
    return r.returncode
if not os.environ.get('DECODE_SKIP_BASELINE'):
    step('wasm-default-build',['python3','parity/0216-decode/build_wasm_default.py'])
    step('wasm-default-baseline',['python3','parity/0216-decode/wasm.py','default'])
step('format',['cargo','fmt'])
step('audit-inventory',['python3','parity/simd/update_audit.py'])
step('fmt',['cargo','fmt','--check'])
step('all-features',['cargo','test','--locked','--all-features'])
step('no-default',['cargo','test','--locked','--no-default-features'])
step('no-std-simd',['cargo','test','--locked','--no-default-features','--features','simd,parity-internals','--test','simd','--','--test-threads=1'])
step('clippy',['cargo','clippy','--locked','--all-targets','--all-features','--','-D','warnings'])
step('aarch64-default',['cargo','check','--locked','--all-features','--all-targets','--target','aarch64-unknown-linux-gnu'])
step('aarch64-build',['cargo','build','--locked','--all-features','--lib','--target','aarch64-unknown-linux-gnu'])
step('aarch64-build-no-std-simd',['cargo','build','--locked','--no-default-features','--features','simd','--lib','--target','aarch64-unknown-linux-gnu'])
step('aarch64-no-std-simd',['cargo','check','--locked','--no-default-features','--features','simd,parity-internals','--target','aarch64-unknown-linux-gnu'])
step('unsafe-free',['cargo','build','--locked','--no-default-features'],dict(RUSTFLAGS='-Funsafe_code'))
step('boundary',['bash','parity/check-unsafe-boundary.sh'])
step('package',['cargo','package','--locked','--allow-dirty'])
step('public-text',['bash','parity/tools/check_public_text.sh'])
step('exact-default',['cargo','run','--offline','--release','--manifest-path',str(proof/'Cargo.toml'),'--target-dir',str(target/'exact')])
step('exact-no-default',['cargo','run','--offline','--release','--manifest-path',str(proof/'Cargo.toml'),'--target-dir',str(target/'exact'),'--no-default-features'])
step('build-differential',['python3','parity/0216-decode/build_differential.py'])
step('cpp-differential',['python3','parity/0216-decode/differential.py'])
no_std_env=dict(DECODE_NO_STD='1',RUSTFLAGS='-C target-feature=+ssse3,+popcnt,+sse4.1')
step('build-differential-no-std',['python3','parity/0216-decode/build_differential.py'],no_std_env)
step('cpp-differential-no-std',['python3','parity/0216-decode/differential.py'],no_std_env)
step('wasm-build',['cargo','build','--offline','--release','--lib','--manifest-path',str(wasm/'Cargo.toml'),'--target','wasm32-unknown-unknown'],dict(RUSTFLAGS='-C target-feature=+simd128'))
step('wasm-execution',['node','-e','const fs=require("fs"); WebAssembly.instantiate(fs.readFileSync(process.argv[1]),{}).then(r=>console.log("forced wasm/scalar complete observations="+r.instance.exports.check_vertices()));',str(target/'wasm32-unknown-unknown/release/decode_wasm_proof.wasm')])
# Run a bounded supported x86 Miri subset; NEON execution belongs to native CI.
step('miri',['cargo','+nightly','miri','test','--locked','--features','parity-internals','--lib','codec::simd::tests::packed_groups_and_meshlet_masks_match_integer_reference'],dict(MIRIFLAGS='-Zmiri-disable-isolation',RUSTFLAGS='-C target-feature=+ssse3,+popcnt,+sse4.1'))
step('asan-build',['cargo','+nightly','fuzz','build','--fuzz-dir','fuzz/codec','--target-dir',str(target),'--target','x86_64-unknown-linux-gnu','simd'])
for name,file in [('fuzz',target/'x86_64-unknown-linux-gnu/release/simd'),('wasm-proof',target/'wasm32-unknown-unknown/release/decode_wasm_proof.wasm')]:
    if file.exists():(art/(name+'-binding.json')).write_text(json.dumps(dict(binary_sha256=hashlib.sha256(file.read_bytes()).hexdigest()),indent=2)+'\n')
# Explicit fixed iteration budget; completion is not a timeout cancellation.
from differential import stream
corpus=target/'fuzz-seeds';corpus.mkdir(exist_ok=True)
for n in [1,15,16,17,33,257]:
    for ch in [0,1,2,0xf2]:
        source=stream(n,12,1,ch,0,3)
        (corpus/f'{n}-{ch}').write_bytes(bytes([1])+n.to_bytes(4,'little')+(12).to_bytes(2,'little')+bytes([0,0,16])+source)
step('asan-fuzz',[str(target/'x86_64-unknown-linux-gnu/release/simd'),str(corpus),'-runs=20000','-seed=216','-max_len=16384','-print_final_stats=1','-artifact_prefix='+str(art)+'/'],dict(MESHOPT_FUZZ_COUNTS=str(art/'fuzz-level-counts.json')))
assert sum(x['cpu_seconds'] for x in exits)<=7200,exits
assert all(x['exit']==0 for x in exits),exits
