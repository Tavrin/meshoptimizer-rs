from pathlib import Path
import shutil,os
s=Path(os.environ.get('PEER_SCRATCH','/mnt/linux-extra/moss-scratch/meshopt-peer-a'));r=Path.cwd()
assert not (s/'wall.json').exists(), 'completed evidence is immutable; use fresh scratch'
for arm,base in [('baseline',s/'baseline'),('candidate',r)]:
 d=s/f'protocol-{arm}';shutil.copytree(r/'parity/src',d/'src',dirs_exist_ok=True)
 for f in ['Cargo.toml','Cargo.lock']:
  shutil.copy2(r/'parity'/f,d/f)
 p=d/'Cargo.toml';p.write_text(p.read_text().replace('path = ".."',f'path = "{base}"'))
d=s/'fuzz-seeds';shutil.copytree(r/'fuzz/src',d/'src',dirs_exist_ok=True)
for f in ['Cargo.toml','Cargo.lock']:shutil.copy2(r/'fuzz'/f,d/f)
p=d/'Cargo.toml';p.write_text(p.read_text().replace('path = ".."',f'path = "{r}"').replace('path = "../parity"',f'path = "{s}/protocol-candidate"'))
p=d/'src/p01x_main.rs';t=p.read_text().replace('while start.elapsed() < Duration::from_secs(seconds)', 'while executions < 256');t=t.replace('        let result = meshoptimizer_parity::execute(&input);','        std::fs::write(format!("{}/{}-{}.input", std::env::var("SEED_OUTPUT")?, name, executions), &input)?;\n        let result = meshoptimizer_parity::execute(&input);');p.write_text(t)
