"""Wasm execution and timing after the unchanged baseline has been captured."""
import hashlib, json, os, pathlib, re, shutil, subprocess, time
root=pathlib.Path.cwd(); art=root/'parity/0217-decoders/receipts'
target=pathlib.Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
rows=[]
def step(name,args,**env):
    started=time.monotonic(); p=art/('refine-'+name+'.log')
    with p.open('w') as log:
        code=subprocess.run(args,env=dict(os.environ,**env),stdout=log,stderr=subprocess.STDOUT).returncode
    raw=hashlib.sha256(p.read_bytes()).hexdigest()
    text=p.read_text()
    for value,label in [(str(root),'<repo>'),(str(target),'<target-dir>'),(os.environ['MESHOPT_REFERENCE'],'<reference>')]:text=text.replace(value,label)
    p.write_text(re.sub(r'/home/[^/]+/','<home>/',text))
    rows.append(dict(step=name,exit=code,seconds=time.monotonic()-started,raw_log_sha256=raw))
    (art/'refine-exits.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,code,flush=True)
    return code
step('fmt',['cargo','fmt'])
step('fmt-check',['cargo','fmt','--check'])
step('audit',['python3','parity/simd/update_audit.py'])
step('clippy',['cargo','clippy','--locked','--target','wasm32-unknown-unknown','--all-targets','--all-features','--','-D','warnings'],RUSTFLAGS='-C target-feature=+simd128')
step('boundary',['bash','parity/check-unsafe-boundary.sh'])
step('codec-build',['cargo','build','--locked','--offline','--release','--manifest-path','parity/codec/Cargo.toml','--features','simd','--target','wasm32-unknown-unknown','--lib'],RUSTFLAGS='-C target-feature=+simd128')
step('identity',['python3','parity/0217-decoders/identity.py'])
step('after-build',['cargo','build','--locked','--offline','--release','--manifest-path','parity/simd/Cargo.toml','--target','wasm32-unknown-unknown','--lib'],RUSTFLAGS='-C target-feature=+simd128')
shutil.copy2(target/'wasm32-unknown-unknown/release/meshopt_simd_qualification.wasm',art/'after.wasm')
proof=target/'wasm-proof-source'
step('observations-build',['cargo','build','--offline','--release','--manifest-path',str(proof/'Cargo.toml'),'--target','wasm32-unknown-unknown','--lib'],RUSTFLAGS='-C target-feature=+simd128')
step('observations',['node','-e','const fs=require("fs"); WebAssembly.instantiate(fs.readFileSync(process.argv[1]),{}).then(r=>console.log("forced wasm/scalar complete observations="+r.instance.exports.check_vertices()));',str(target/'wasm32-unknown-unknown/release/decode_wasm_proof.wasm')])
step('measure',['python3','parity/0217-decoders/measure.py'])

assert all(row["exit"] == 0 for row in rows), rows
