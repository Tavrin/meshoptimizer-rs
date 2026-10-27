"""Prepare the proven reciprocal/normalization guard variant on top of F."""
from pathlib import Path
import hashlib,json,os,shutil,subprocess
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH']);d=A/'variant-G';assert not d.exists()
shutil.copytree(A/'variant-F',d)
p=d/'src/overdraw.rs';s=p.read_text()
assert s.count('finite(1. / area_sum)?')==1 and s.count('finite(1. / length)?')==1
s=s.replace('finite(1. / area_sum)?','1. / area_sum').replace('finite(1. / length)?','1. / length').replace('*x = finite(*x * inv_length)?;','*x *= inv_length;')
p.write_text(s)
shutil.copytree(A/'harness-F',A/'harness-G')
p=A/'harness-G/Cargo.toml';p.write_text(p.read_text().replace('variant-F','variant-G'))
(A/'G.patch').write_text(subprocess.run(['diff','-ru',str(A/'base/src'),str(d/'src')],capture_output=True,text=True).stdout)
(A/'finite-sources.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in (d/'src').rglob('*.rs')},indent=2)+'\n')
