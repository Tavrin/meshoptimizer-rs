#!/usr/bin/env python3
"""Bind retained proof, gates, inputs and the single registered campaign."""
import hashlib, json, math
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
ART = Path('/mnt/linux-extra/moss-scratch/meshopt-028-L2')
def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1<<20),b''):h.update(b)
    return h.hexdigest()
def read(name):return json.loads((ART/name).read_text())
wall,proof = read('campaign.json'),read('proof.json')
source = {str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'src').rglob('*.rs')}
assert wall['completed'] and proof['passed']
assert source == wall['source'] == proof['source']
for name,digest in wall['harness_sources'].items(): assert sha(ROOT/name)==digest
assert sha(ROOT/'parity/028-L2/REGISTRATION.md') == wall['registration_sha256']
assert all(sha(item['path']) == item['sha256'] for item in wall['binaries'].values())
assert len(wall['rows'])==92 and all(len(r['samples'])==10 for r in wall['rows'])
assert len(wall['stages'])==6 and all(len(r['samples'])==10 and r['warmup'] for r in wall['stages'])
assert all(code==0 for code in proof['gates'].values())
assert read('seed-proof.json')['passed']
assert len(proof['frozen'])==23
assert read('adapter-proof.json')['passed']
frozen=read('parity/compare/results.json')['outputs']
for case in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']:
    assert read('inputs-sha256.json')[case+'.bin']==next(r['input_sha256'] for r in frozen if r['case']==case and r['operation']=='vertex-decode')
for name,digest in read('inputs-sha256.json').items(): assert sha(ART/'inputs'/name)==digest
floor = max(abs(endpoint-1) for row in wall['rows'] if row['comparison']=='AA' for endpoint in row['ci95'])
assert floor==wall['F']
for family in wall['families']:
    base_pass=family['ratios']['base']<=1 and family['max_baseline_upper95']<=1.10+floor
    expected=('PASS' if family['ratios']['M7']<=1 else 'PARTIAL') if base_pass else 'FAIL'
    assert family['verdict']==expected
codes = {}
for line in (ART/'gates.tsv').read_text().splitlines():
    name,code=line.split('\t');codes[name]=int(code)
assert set(codes)=={'fmt','all-features','no-default-features','clippy'} and not any(codes.values())
fuzz = read('fuzz-artifacts/fuzz/record.json')
assert fuzz['passed'] and fuzz['identities_unchanged']
assert all(r['cpu_seconds']>=300 and r['crashes']==0 for r in fuzz['runs'])
# Keep the complete CPU-accounted fuzz receipt; schema fields are exported as
# written by the existing harness rather than inventing a new acceptance test.
files = [ART/'campaign.json',ART/'proof.json',ART/'seed-proof.json',ART/'stages.json',ART/'gates.tsv',ART/'fuzz-artifacts/fuzz/record.json',ART/'inputs-sha256.json']
files += sorted(ART.glob('instructions-*.json'))
receipt = dict(passed=True,source=source,gates=codes,evidence_sha256={str(p.relative_to(ART)):sha(p) for p in files},registration_sha256=wall['registration_sha256'])
(ART/'audit.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('PASS source, registration, 92 family comparisons, six real-chain comparisons, exact proof, gates and fuzz bindings')
