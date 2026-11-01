"""Wasm execution and timing after the unchanged baseline has been captured."""
import hashlib, json, os, pathlib, re, shutil, subprocess, time
root=pathlib.Path.cwd(); art=root/'parity/0217-decoders/receipts'
target=pathlib.Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
rows=[]
def step(name,args,**env):
    started=time.monotonic(); p=art/(name+'.log')
    with p.open('w') as log:
        code=subprocess.run(args,env=dict(os.environ,**env),stdout=log,stderr=subprocess.STDOUT).returncode
    raw=hashlib.sha256(p.read_bytes()).hexdigest()
    text=p.read_text()
    for value,label in [(str(root),'<repo>'),(str(target),'<target-dir>'),(os.environ['MESHOPT_REFERENCE'],'<reference>')]:text=text.replace(value,label)
    p.write_text(re.sub(r'/home/[^/]+/','<home>/',text))
    rows.append(dict(step=name,exit=code,seconds=time.monotonic()-started,raw_log_sha256=raw))
    (art/'wasm-exits.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,code,flush=True)
    return code
step('wasm-fmt',['cargo','fmt'])
step('wasm-audit',['python3','parity/simd/update_audit.py'])
step('wasm-final-clippy',['cargo','clippy','--locked','--target','wasm32-unknown-unknown','--all-targets','--all-features','--','-D','warnings'],RUSTFLAGS='-C target-feature=+simd128')
step('native-final-clippy',['cargo','clippy','--locked','--all-targets','--all-features','--','-D','warnings'])
step('final-msrv',['cargo','+1.88','check','--locked','--all-features','--lib'])
step('final-fmt',['cargo','fmt','--check'])
step('final-boundary',['bash','parity/check-unsafe-boundary.sh'])
if (art/'differential.json').exists():shutil.copy2(art/'differential.json',art/'differential-initial.json')
step('final-differential-build',['cargo','build','--offline','--release','--manifest-path',str(target/'differential-source/Cargo.toml'),'--target-dir',str(target/'proof')])
step('final-differential',['python3',str(target/'differential.py')])
step('codec-native-build',['cargo','build','--locked','--offline','--release','--manifest-path','parity/codec/Cargo.toml','--features','simd'])
step('codec-wasm-build',['cargo','build','--locked','--offline','--release','--manifest-path','parity/codec/Cargo.toml','--features','simd','--target','wasm32-unknown-unknown','--lib'],RUSTFLAGS='-C target-feature=+simd128')
step('corpus-identity',['python3','parity/0217-decoders/identity.py'])
step('wasm-after-build',['cargo','build','--locked','--offline','--release','--manifest-path','parity/simd/Cargo.toml','--target','wasm32-unknown-unknown','--lib'],RUSTFLAGS='-C target-feature=+simd128')
shutil.copy2(target/'wasm32-unknown-unknown/release/meshopt_simd_qualification.wasm',art/'after.wasm')
proof=target/'wasm-proof-source';proof.mkdir(exist_ok=True)
for name in ['wasm_check.rs','fixtures.rs']:shutil.copy2(root/'parity/0216-decode'/name,proof/name)
(proof/'Cargo.toml').write_text('[package]\nname="decode-wasm-proof"\nversion="0.0.0"\nedition="2021"\n[lib]\npath="wasm_check.rs"\ncrate-type=["cdylib"]\n[dependencies]\nmeshoptimizer-rs={path='+repr(str(root))+',features=["parity-internals"]}\n[profile.release]\nlto="fat"\ncodegen-units=1\n')
step('wasm-observations-build',['cargo','build','--offline','--release','--manifest-path',str(proof/'Cargo.toml'),'--target','wasm32-unknown-unknown','--lib'],RUSTFLAGS='-C target-feature=+simd128')
step('wasm-observations',['node','-e','const fs=require("fs"); WebAssembly.instantiate(fs.readFileSync(process.argv[1]),{}).then(r=>console.log("forced wasm/scalar complete observations="+r.instance.exports.check_vertices()));',str(target/'wasm32-unknown-unknown/release/decode_wasm_proof.wasm')])
step('group-profile',['python3','parity/0217-decoders/profile.py'])
step('wasm-measure',['python3','parity/0217-decoders/measure.py'])

assert all(row["exit"] == 0 for row in rows), rows
