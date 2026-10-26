from pathlib import Path
import subprocess,hashlib,json
R=Path(__file__).resolve().parents[2];S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-h3')
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
base={str(p.relative_to(S/'base')):sha(p) for p in (S/'base/src').rglob('*.rs')}
for path,digest in base.items():assert hashlib.sha256(subprocess.check_output(['git','show',f'v0.2.9:{path}'],cwd=R)).hexdigest()==digest,path
inputs={str(p.relative_to(S)):sha(p) for p in (S/'meshopt-028-L2/inputs').glob('*.bin') if p.name in [f'{n}.bin' for n in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']]}
for path,digest in inputs.items():assert sha(Path('/mnt/linux-extra/moss-scratch/meshopt-0210-attr')/path)==digest,path
sources={str(p.relative_to(R)):sha(p) for base in ['src','parity/0210-h3'] for p in (R/base).rglob('*') if p.is_file() and '__pycache__' not in str(p)}
binaries={p.name:sha(p) for p in S.iterdir() if p.is_file() and (p.name.startswith(('driver-','exact-','seeds-','p03-')) or p.name in ['sqrt-exhaustive','cpp-p03'])}
record=dict(base_commit=subprocess.check_output(['git','rev-parse','v0.2.9^{commit}'],cwd=R,text=True).strip(),peer_screen=subprocess.check_output(['git','rev-parse','bench/peer-screen'],cwd=R,text=True).strip(),base=base,inputs=inputs,sources=sources,binaries=binaries)
(S/'final-binding.json').write_text(json.dumps(record,indent=2)+'\n')
print('verified baseline source and frozen inputs;',len(binaries),'retained binaries')
