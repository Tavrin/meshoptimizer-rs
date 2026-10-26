from pathlib import Path
import subprocess, shutil, re, json
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-h3')
R=Path(__file__).resolve().parents[2]
def archive(ref, dest, paths):
 dest.mkdir(exist_ok=True)
 a=subprocess.Popen(['git','archive',ref,*paths],cwd=R,stdout=subprocess.PIPE)
 subprocess.run(['tar','-xf','-','-C',str(dest)],stdin=a.stdout,check=True)
 a.stdout.close();assert a.wait()==0
archive('v0.2.9', S/'base', ['Cargo.toml','Cargo.lock','src'])
archive('bench/peer-screen', S/'peer-screen', ['parity/compare','parity/peer-screen'])
subprocess.run(['tar','-I','zstd','-xf','/mnt/apollo-main/linux-dev-storage-folder/meshoptimizer-rs/scratch-2026-10-26/meshopt-028-L2.tar.zst','-C',str(S),'--wildcards','meshopt-028-L2/harness-candidate/*','meshopt-028-L2/inputs/*','meshopt-028-L2/stages.json','meshopt-028-L2/chain-*.tsv'],check=True)
for arm, lib in [('base',S/'base'),('candidate',R),('M7',S/'base')]:
 h=S/f'harness-{arm}';shutil.copytree(S/'meshopt-028-L2/harness-candidate',h)
 p=h/'Cargo.toml';t=p.read_text();t=re.sub(r'path = "[^"]+", optional = true, default-features',f'path = "{lib}", optional = true, default-features',t);p.write_text(t)
 p=h/'src/main.rs';t=p.read_text();a=t.index('        let iterations: usize = line.parse().unwrap();');b=t.index('\n    }\n}',a);t=t[:a]+'        panic!("count protocol only");'+t[b:];p.write_text(t)
# Retain the profile/driver derivation, not source modifications on the benchmark branch.
(S/'archive-refs.json').write_text(json.dumps({ref:subprocess.check_output(['git','rev-parse',ref],cwd=R,text=True).strip() for ref in ['v0.2.9','bench/peer-screen']},indent=2)+'\n')
