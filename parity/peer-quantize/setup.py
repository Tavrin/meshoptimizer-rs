from pathlib import Path
import subprocess, shutil
s=Path('/mnt/linux-extra/moss-scratch/meshopt-peer-a'); repo=Path.cwd()
assert not (s/'wall.json').exists(), 'completed evidence is immutable; use fresh scratch'
(s/'baseline').mkdir(exist_ok=True)
p=subprocess.Popen(['git','archive','33556c4','src','Cargo.toml','Cargo.lock'],stdout=subprocess.PIPE)
subprocess.run(['tar','-x','-C',str(s/'baseline')],stdin=p.stdout,check=True); assert p.wait()==0
for arm,dep in [('O',repo),('B',s/'baseline'),('X',repo),('M7',repo)]:
 d=s/arm/'parity/compare';d.parent.mkdir(parents=True,exist_ok=True);shutil.copytree(s/'parity/compare',d,dirs_exist_ok=True)
 f=d/'Cargo.toml';f.write_text(f.read_text().replace('path = "../.."',f'path = "{dep}"'))
# Import the unchanged frozen corpus generator from this checkout.
shutil.copytree(repo/'parity',s/'helpers',ignore=shutil.ignore_patterns('compare'),dirs_exist_ok=True)
