#!/usr/bin/env python3
"""Bind final source, proof, registration and all first-campaign samples."""
import hashlib
import json
import math
from pathlib import Path
import statistics

ROOT = Path(__file__).resolve().parents[2]
ART = Path('/mnt/linux-extra/moss-scratch/meshopt-peer-c')
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

campaign = json.loads((ART / 'campaign.json').read_text())
proof = json.loads((ART / 'proof.json').read_text())
build = json.loads((ART / 'build-candidate-4.json').read_text())
assert campaign['completed'] and proof['passed'] and proof['mismatches'] == 0
assert campaign['registration_sha256'] == sha(ROOT / 'parity/peer-small/REGISTRATION.md')
source = {str(p.relative_to(ROOT)): sha(p) for p in (ROOT / 'src').rglob('*.rs')}
assert source == proof['build']['source'] == build['source']
for binary, expected in campaign['binaries'].items():
    assert sha(Path(binary)) == expected
assert campaign['binaries'][build['binary']] == build['sha256']
assert len(campaign['rows']) == 60
assert len({(r['comparison'], r['operation'], r['case']) for r in campaign['rows']}) == 60
for row in campaign['rows']:
    assert len(row['samples']) == 10
    for pair, sample in enumerate(row['samples']):
        assert sample['pair'] == pair
        assert sample['order'] == ([0, 1] if pair % 2 == 0 else [1, 0])
        assert sample['ratio'] == sample['ns'][0] / sample['ns'][1]
    assert row['gm'] == math.exp(statistics.mean(math.log(s['ratio']) for s in row['samples']))
floor = max(abs(x - 1) for r in campaign['rows'] if r['comparison'] == 'AA' for x in r['ci95'])
assert floor == campaign['F']
for family in campaign['families']:
    rows = [r for r in campaign['rows'] if r['operation'] == family['operation']]
    for label, key in [('X', 'ours_X'), ('base', 'ours_base')]:
        value = math.exp(statistics.mean(math.log(r['gm']) for r in rows if r['comparison'] == label))
        assert value == family[key]
    passes = family['ours_base'] <= 1 and family['max_upper95'] <= 1.10 + floor
    assert family['verdict'] == (('PASS' if family['ours_X'] <= 1 else 'PARTIAL') if passes else 'FAIL')
assert (ART / 'gates.tsv').read_text().splitlines()[-3:] == ['fmt\t0', 'test\t0', 'clippy\t0']
assert proof['limits_sha256'][0] == proof['limits_sha256'][1]
out = dict(passed=True, source=source, floor=floor, families=campaign['families'],
           gate_exits=dict(fmt=0, test=0, clippy=0, differential=0, exact=0, campaign=0),
           proof_cases=len(proof['cases']),
           receipt_sha256={p.name: sha(p) for p in [ART / 'campaign.json', ART / 'proof.json',
                                                   ART / 'proof-build.json', ROOT / 'parity/peer-small/REGISTRATION.md']})
(ART / 'audit.json').write_text(json.dumps(out, indent=2) + '\n')
print('PASS: final source, proof, registration, 600 samples and first verdicts match')
