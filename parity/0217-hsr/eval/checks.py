"""CPU-admitted gates and exact unchanged-prerequisite comparison."""
import hashlib,json,os,subprocess,tarfile,io,time
from pathlib import Path
root=Path.cwd();lane=root/'parity/0217-hsr';receipt=lane/'eval' / 'checks';receipt.mkdir(parents=True,exist_ok=True)
target=Path(os.environ['CARGO_TARGET_DIR']);scratch=target/'proof';scratch.mkdir(parents=True,exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU','0')=='0'
os.environ.update(CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',LC_ALL='C',GIT_OPTIONAL_LOCKS='0',RUSTDOCFLAGS='-D warnings')
def run(name,args):
    start=time.monotonic();p=subprocess.run(list(map(str,args)),stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
    log=p.stdout.replace(str(target),'<target>').replace(str(root),'<worktree>').replace(os.environ.get('HSR_SCENES','<unset>'),'<scene-scratch>')
    (receipt/(name+'.log')).write_text(log)
    with (receipt/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\t{time.monotonic()-start:.3f}\n')
    print(name,p.returncode,log[-1500:],flush=True)
    if p.returncode:raise SystemExit(p.returncode)
run('fmt',['cargo','fmt','--all','--','--check'])
run('clippy-all',['cargo','clippy','--locked','--all-features','--all-targets','--','-D','warnings'])
run('clippy-none',['cargo','clippy','--locked','--no-default-features','--all-targets','--','-D','warnings'])
run('tests-all',['cargo','test','--locked','--all-features'])
run('tests-none',['cargo','test','--locked','--no-default-features'])
for mode,features in [('all',['--all-features']),('none',['--no-default-features'])]:
    run('wasm-'+mode,['cargo','build','--locked','--target','wasm32-unknown-unknown',*features] if mode=='none' else ['cargo','build','--locked','--target','wasm32-unknown-unknown','--features','experimental,clusterlod'])
run('boundary',['bash','parity/check-unsafe-boundary.sh'])
base=scratch/'base';base.mkdir(exist_ok=True)
archive=subprocess.check_output(['git','archive','09da597f438471d0ad9c16e460f3598e9356e9ea','Cargo.toml','Cargo.lock','src','docs','README.md'])
with tarfile.open(fileobj=io.BytesIO(archive)) as t:t.extractall(base,filter='data')
p=base/'Cargo.toml';p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
for mode,features in [('all',['std','simd','clusterlod','experimental','parallel','parity-internals']),('none',[])]:
    folder=scratch/('exact-'+mode);folder.mkdir(exist_ok=True)
    (folder/'Cargo.toml').write_text(f'''[package]
name="hsr-exact"
version="0.0.0"
edition="2021"
[dependencies]
baseline={{package="meshoptimizer-baseline",path={json.dumps(str(base))},default-features=false,features={json.dumps(features)}}}
candidate={{package="meshoptimizer-rs",path={json.dumps(str(root))},default-features=false,features={json.dumps(features)}}}
[[bin]]
name="hsr-exact"
path={json.dumps(str(lane/'exact.rs'))}
[profile.release]
overflow-checks=true
debug-assertions=true
''')
    run('exact-build-'+mode,['cargo','build','--offline','--release','--manifest-path',folder/'Cargo.toml'])
    run('exact-'+mode,[target/'release/hsr-exact'])
run('hidden-release',['cargo','test','--locked','--release','--all-features','--test','hidden','--','--nocapture'])
run('docs-all',['cargo','doc','--locked','--no-deps','--all-features'],)
run('public-text',['bash','parity/tools/check_public_text.sh'])
