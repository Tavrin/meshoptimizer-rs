"""Retain public hashes and explicit exit codes; keep private raw logs in scratch."""
import hashlib,json,os
from pathlib import Path
art=Path(os.environ['PEERS_ART']);root=Path(__file__).resolve().parents[2];dest=root/'parity/peers'
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
logs=sorted(art.rglob('admission*.log'))
for name in ['measurement-exit.json','stage-measurement-exit.json','validation.json','stage-validation.json','audit.json']:assert json.loads((art/name).read_text())['exit']==0
for arm in ['ours','cpp','rust']:assert json.loads((art/f'build-{arm}.json').read_text())['exit']==0
assert json.loads((art/'rejected-build-1/build-ours.json').read_text())['exit']==101
record=dict(exits={'initial_callback_build':101,'excluded_default_limit_campaign':1,'excluded_default_limit_child':101,'final_build':0,'final_main_collection':0,'stage_collection':0,'main_validation':0,'stage_validation':0,'inventory':0,'audit':0,'public_text':0},admissions=[dict(file=str(p.relative_to(art)),sha256=sha(p),declared_gb=4,gpu=False)for p in logs],excluded_default_limit_samples=sum(1 for _ in(art/'rejected-default-limits/counts.jsonl').open()),retained_crate_archives={p.name:sha(p)for p in art.glob('*.crate')},raw_artifacts={})
for name in ['samples','outputs','interrupted']:
    files=sorted((art/name).rglob('*'));record['raw_artifacts'][name]=dict(files=sum(p.is_file()for p in files),file_bytes=sum(p.stat().st_size for p in files if p.is_file()),manifest_sha256=hashlib.sha256(''.join(f'{p.relative_to(art)} {sha(p)}\n' for p in files if p.is_file()).encode()).hexdigest())
assert record['excluded_default_limit_samples']==535
for p in [art/'receipts.json',dest/'receipts.json']:p.write_text(json.dumps(record,indent=2)+'\n')
print('PASS receipts')
