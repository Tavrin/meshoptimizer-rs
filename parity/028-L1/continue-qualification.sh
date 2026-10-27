#!/usr/bin/env bash
set -euo pipefail
python3 parity/028-L1/proof.py build
python3 parity/028-L1/proof.py final
python3 parity/028-L1/proof.py cpp
python3 parity/028-L1/campaign.py build-rfc
python3 parity/028-L1/campaign.py preflight-rfc
python3 parity/028-L1/fuzz.py
python3 parity/028-L1/division_fuzz.py 300
python3 - <<'PY'

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import json,hashlib
A=Path(_public_path('<scratch>/meshopt-028-L1'));R=Path.cwd()
load=lambda p:json.loads((A/p).read_text())
assert all(x['exit']==0 for x in load('gates.json'))
assert load('exact-proof.json')['exit']==0
assert len(load('cpp-seeded-differential.json'))==6
assert all(x['mismatches']==0 for x in load('cpp-seeded-differential.json'))
assert len(load('rfc-preflight.json'))==50
assert load('fuzz.json')['exit']==0 and load('fuzz.json')['target_cpu_seconds']>=300
assert load('division-fuzz.json')['exit']==0 and load('division-fuzz.json')['cpu_seconds']>=300
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(A/'driver-final-ours')==sha(A/'driver-trial7-ours'),'instruction image identity changed; diagnose before timing'
assert sha(R/'src/meshlet_division.rs')==load('division-fuzz.json')['production_source_sha256']
source={str(p.relative_to(R)):sha(p)for p in [*sorted((R/'src').rglob('*.rs')),*sorted((R/'parity/028-L1').rglob('*.rs')),R/'Cargo.toml',R/'Cargo.lock',R/'parity/028-L1/Cargo.toml',R/'parity/028-L1/Cargo.lock']}
(A/'QUALIFIED.json').write_text(json.dumps({'exit':0,'source':source,'instructions_reused_identical_image':'counts-trial7.json','proof':['gates.json','exact-proof.json','cpp-seeded-differential.json','rfc-preflight.json','fuzz.json','division-fuzz.json']},indent=2)+'\n')
PY
