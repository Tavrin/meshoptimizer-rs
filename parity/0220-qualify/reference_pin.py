"""Check the cached benchmark reference against its locked commit."""
import hashlib,json,os,subprocess
from pathlib import Path
rev='8cc060f01c3d4d5920781124cc8c32013bb47730'
home=Path(os.environ.get('CARGO_HOME',str(Path.home()/'.cargo')))
paths=list(home.glob('git/checkouts/bevy-*/8cc060f*/crates/bevy_pbr/Cargo.toml'))
assert paths,'pinned reference checkout not found'
for p in paths:
 root=p.parents[2]
 head=subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip()
 assert head==rev,(head,rev)
 subprocess.run(['git','-C',str(root),'diff','--exit-code',rev,'--'],check=True)
 files={str(f.relative_to(root)):hashlib.sha256(f.read_bytes()).hexdigest() for f in (p.parent/'src/meshlet').rglob('*') if f.is_file() and f.suffix in ['.rs','.wgsl','.wesl']}
 assert files,'reference meshlet source not found'
 print(json.dumps(dict(revision=head,meshlet_sources=files),sort_keys=True))
