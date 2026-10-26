#!/usr/bin/env python3
"""Compose retained independent proofs; never rerun a passed library gate."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib,json,re,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
ART=Path(_public_path('<scratch>/meshopt-028-L2'))
source={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (ROOT/'src').rglob('*.rs')}
def read(name):return json.loads((ART/name).read_text())
exact,protocol,adapter=read('proof.json'),read('proof-protocol.json'),read('adapter-proof.json')
assert exact['source']==protocol['source']==source
assert all(exact['gates'][key]==0 for key in ['exact-std','exact-no-std'])
assert adapter['passed'] and not adapter['mismatches'] and protocol['passed']
assert len(adapter['frozen'])==23 and len(adapter['stages'])==9
for row in adapter['stages']:assert all(call['exact_capture'] for call in row['calls'])
prior=ART/'proof-stage-replay-failed.json'
if not prior.exists():shutil.copy2(ART/'proof.json',prior)
counts={}
for mode in ['std','no-std']:
    text=(ART/f'proof-exact-{mode}.log').read_text()
    counts[mode]=dict(observations=int(re.search(r'PASS: (\d+) exact observations',text)[1]),scale_seed=int(re.search(r'PASS (\d+) existing scale seed observations',text)[1]))
proof=dict(source=source,passed=True,gates={**{k:v for k,v in exact['gates'].items() if not k.startswith('stage-')},**protocol['gates'],'final-adapters-and-stages':0},exact_counts=counts,frozen=adapter['frozen'],stages=adapter['stages'],cpp=protocol['cpp'],seed_cases=len(read('seed-proof.json')['cases']),prior_failed_stage_proof_sha256=hashlib.sha256(prior.read_bytes()).hexdigest())
(ART/'proof.json').write_text(json.dumps(proof,indent=2)+'\n')
print('PASS retained exact std/no_std, C++ differential, existing seeds, and corrected historical real-stage proof')
