from pathlib import Path
import json,os,shutil
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
ROOT=Path(__file__).resolve().parents[2]
entry=json.loads((A/'ENTRY.json').read_text())
target=Path(os.environ['CARGO_TARGET_DIR'])
assert target==Path(entry['target_dir'])
assert target.is_absolute() and not target.is_symlink() and len(target.parts)>3
assert target!=ROOT and target not in ROOT.parents and ROOT not in target.parents
size=sum(p.stat().st_size for p in target.rglob('*') if p.is_file()) if target.exists() else 0
shutil.rmtree(target)
assert not target.exists()
(A/'cleanup.json').write_text(json.dumps({'exit':0,'target':str(target),'absent':True,'apparent_file_bytes':size,'admission':os.environ['MESHOPT_ADMISSION_ID']},indent=2)+'\n')
print('PASS designated target cleanup')
