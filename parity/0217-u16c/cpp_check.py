"""Bind the locked C++ oracle to the retained pinned-source hashes."""
import hashlib,json,os
from pathlib import Path
cargo_home=Path(os.environ.get('CARGO_HOME',str(Path.home()/'.cargo')))
roots=list(cargo_home.glob('registry/src/*/meshopt-0.7.0/vendor/src'))
assert len(roots)==1, 'Expected exactly one locked oracle source tree'
rows=json.loads(Path('parity/0217-u16/cpp-source-binding.json').read_text())
for row in rows:
 assert row['identical']
 assert hashlib.sha256((roots[0]/row['file']).read_bytes()).hexdigest()==row['sha256'],row['file']
Path('parity/0217-u16c/cpp-source-binding.json').write_text(json.dumps({'pin':'4c203430ca565cb59a468a91922c76c208169536','reference':'../0217-u16/cpp-source-binding.json','current_vendor_files':rows},indent=2)+'\n')
print(len(rows),'locked oracle sources match the retained pin')
