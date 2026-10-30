"""Retain public text receipts; bind raw artifacts and final source separately."""
import hashlib,json,os,shutil,subprocess
from pathlib import Path
R=Path.cwd();S=Path(os.environ['DECODE_ROOT']);T=Path(os.environ['CARGO_TARGET_DIR']);O=R/'parity/0216-vdecode/receipts';O.mkdir(exist_ok=True)
replacements={str(R):'<repo>',str(T):'<target-dir>',str(S):'<artifacts>',str(Path(os.environ['MESHOPT_REFERENCE'])):'<reference>',str(Path.home()):'<home>'}
def clean(s):
 for old,new in sorted(replacements.items(),key=lambda x:-len(x[0])):s=s.replace(old,new)
 return s
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
binding={}
for folder in ['screen','proof','screen2','screen3','proof-final','guard']:
 source=S/folder
 if not source.exists():continue
 for p in source.rglob('*'):
  if not p.is_file() or p.is_symlink():continue
  rel=p.relative_to(source)
  if any(part.startswith('frozen-') for part in rel.parts) or any(part in ['inputs','outputs','binaries'] for part in rel.parts):continue
  if p.suffix not in ['.json','.jsonl','.log','.txt','.md','.stdout','.stderr','.perf','.rss','.asm','.normalized','.instructions','.diff']:continue
  dest=O/folder/rel;dest.parent.mkdir(parents=True,exist_ok=True)
  dest.write_text(clean(p.read_text(errors='replace')))
  binding[str(dest.relative_to(O))]=dict(raw_sha256=sha(p),public_sha256=sha(dest))
for name in ['cpp-binding.json','verdict.json']:
 p=S/name
 if p.exists():(O/name).write_text(clean(p.read_text()))
for name,source in [('guard-seed.txt',Path((S/'guard-source.txt').read_text().strip())),('guard-driver.txt',S/'guard/frozen-candidate/parity/peers/src/main.rs')]:
 if source.exists():(O/name).write_text(clean(source.read_text()))
(O/'receipt-binding.json').write_text(json.dumps(binding,indent=2)+'\n')
source={str(p.relative_to(R)):sha(p) for p in [*R.glob('src/**/*.rs'),R/'src/codec/simd/SAFETY.md',R/'tests/simd.rs',*R.glob('parity/0216-vdecode/*.py'),*R.glob('parity/0216-vdecode/*.rs'),*R.glob('parity/0216-vdecode/*.cpp')]}
(O/'final-source-binding.json').write_text(json.dumps(source,indent=2)+'\n')
versions={c:subprocess.check_output(c.split(),text=True).strip() for c in ['rustc --version','cargo --version','c++ --version']}
(O/'toolchain.json').write_text(clean(json.dumps(versions,indent=2))+'\n')
manifest={str(p.relative_to(O)):sha(p) for p in O.rglob('*') if p.is_file() and p.name!='SHA256SUMS.json'}
(O/'SHA256SUMS.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n');print('retained',len(manifest),'receipts')
