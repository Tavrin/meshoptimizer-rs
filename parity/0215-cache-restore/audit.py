"""Verify retained cache evidence and export portable receipts. No benchmarks run."""
import hashlib
import json
import os
from pathlib import Path
import shutil

root = Path(__file__).resolve().parents[2]
scratch = Path(os.environ['RESTORE_SCRATCH'])
public = Path(__file__).resolve().parent

def sha(p):
    with p.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

binding = json.loads((scratch / 'build-binding.json').read_text())
for key, digest in binding['sources'].items():
    assert sha(scratch / key) == digest, key
for arm, digest in binding['binaries'].items():
    assert sha(scratch / ('driver-' + arm)) == digest, arm
for p in (root / 'src').rglob('*.rs'):
    assert p.read_bytes() == (scratch / 'frozen-candidate' / p.relative_to(root)).read_bytes(), p.name
for x in json.loads((scratch / 'inputs.json').read_text()):
    assert sha(Path(os.environ['RESTORE_INPUTS']) / (x['case'] + '.bin')) == x['sha256']
files = {}
for campaign in ['attribution', 'candidate-1']:
    d = scratch / campaign
    reg = json.loads((d / 'campaign.json').read_text())
    assert sha(public / 'measure.py') == reg['runner_sha256']
    for key, digest in reg['binding']['sources'].items():
        assert sha(scratch / key) == digest
    rows = [json.loads(s) for s in (d / 'counts.jsonl').read_text().splitlines()]
    assert len(rows) == 120
    for row in rows:
        name = '-'.join(str(row[k]) for k in ['case', 'op', 'arm', 'rep'])
        prefix = d / 'samples' / name
        assert json.loads(prefix.with_suffix('.json').read_text()) == row
        assert prefix.with_suffix('.stdout').read_text().splitlines() == ['READY', 'COUNTED']
        perf = prefix.with_suffix('.perf')
        fields = next(s.split(';') for s in perf.read_text().splitlines() if ';instructions:u;' in s)
        assert int(fields[0]) == row['instructions'] and float(fields[4]) == 100
        out = d / 'outputs' / (name + '.bin')
        assert sha(out) == row['output_sha256'] and out.stat().st_size == row['output_bytes']
        for p in [perf, prefix.with_suffix('.stdout'), prefix.with_suffix('.stderr'), prefix.with_suffix('.json')]:
            files[str(p.relative_to(scratch))] = sha(p)
    for name in ['campaign.json', 'counts.jsonl', 'results.json', 'measurement-exit.json']:
        dest = public / campaign
        dest.mkdir(exist_ok=True)
        shutil.copy2(d / name, dest / name)
proof = scratch / 'proof'
exits = [line.split('\t') for line in (proof / 'exits.tsv').read_text().splitlines()]
assert len(exits) == 22 and all(code == '0' for _, code in exits), exits
for p in proof.glob('*.log'):
    files[str(p.relative_to(scratch))] = sha(p)
for p in proof.glob('*-*/driver'):
    files[str(p.relative_to(scratch))] = sha(p)
for name in ['build-binding.json', 'environment.json', 'inputs.json']:
    shutil.copy2(scratch / name, public / name)
shutil.copy2(proof / 'exits.tsv', public / 'proof-exits.tsv')
sources = {str(p.relative_to(root)): sha(p) for p in (root / 'src').rglob('*.rs')}
for p in public.glob('*'):
    if p.suffix in ['.py', '.sh', '.rs']:
        sources[str(p.relative_to(root))] = sha(p)
receipt = dict(exit=0, samples=240, all_output_hashes_verified=True, retained_files=files,
               proof_steps=len(exits), sources=sources,
               exact={p.stem: p.read_text().strip() for p in proof.glob('exact-*.log')})
(public / 'audit.json').write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n')
print('PASS: 240 sample counters, outputs, source/binary bindings and 22 proof steps verified')
