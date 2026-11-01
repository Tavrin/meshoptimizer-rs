"""Execute the new API under wasm with default and no_std candidate features."""
import hashlib,json,os,subprocess
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE')
r=Path.cwd();t=Path(os.environ['CARGO_TARGET_DIR']);p=t/'wasm-smoke';p.mkdir(exist_ok=True)
(p/'Cargo.toml').write_text(f'''[package]
name="pipeline-wasm-smoke"
version="0.0.0"
edition="2021"
[lib]
crate-type=["cdylib"]
path="smoke.rs"
[features]
default=["candidate/std","candidate/simd"]
[dependencies]
candidate={{package="meshoptimizer-rs",path="{r}",default-features=false}}
''')
(p/'smoke.rs').write_text('''use candidate::*;
#[no_mangle]
pub extern "C" fn pipeline_smoke() -> u32 {
 let p=[[0f32,0.,0.],[1.,0.,0.],[0.,1.,0.]];
 let v:Vec<u8>=p.iter().flatten().flat_map(|x|x.to_le_bytes()).collect();
 let mut o=ProcessOptions::default();o.lod=None;
 match process_mesh(&[0,1,2],&v,12,0,&o,&mut Workspace::default()) {
  Ok(m) if m.levels.len()==1 && m.levels[0].indices.len()==3 && m.report.stages.iter().all(|s|s.elapsed_ns.is_none()) => 1,
  _ => 0,
 }
}
''')
out=r/'parity/0217-pipeline/receipts'
def run(name,args):
 v=subprocess.run(args,capture_output=True,text=True)
 (out/(name+'.log')).write_text(v.stdout+v.stderr)
 with (out/'exits.tsv').open('a') as f:f.write(f'{name}\t{v.returncode}\n')
 print(name,v.returncode,(v.stdout+v.stderr)[-500:],flush=True)
 if v.returncode:raise SystemExit(v.returncode)
for mode,flags in [('default',[]),('no-std',['--no-default-features'])]:
 run('wasm-smoke-build-'+mode,['cargo','build','--manifest-path',str(p/'Cargo.toml'),'--target','wasm32-unknown-unknown',*flags])
 binary=t/'wasm32-unknown-unknown/debug/pipeline_wasm_smoke.wasm'
 run('wasm-smoke-'+mode,['node','-e',"const fs=require('fs'); WebAssembly.instantiate(fs.readFileSync(process.argv[1]),{}).then(({instance})=>{if(instance.exports.pipeline_smoke()!==1)process.exit(1);console.log('process_mesh wasm runtime and clock fallback PASS')}).catch(e=>{console.error(e);process.exit(1)})",str(binary)])
 (out/('wasm-smoke-'+mode+'.json')).write_text(json.dumps(dict(sha256=hashlib.sha256(binary.read_bytes()).hexdigest()))+'\n')
