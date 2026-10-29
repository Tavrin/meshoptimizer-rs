"""Bind proof sources before execution and reject source drift afterwards."""
import hashlib
import json
import os
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[2]
lane = Path(__file__).resolve().parent
out = Path(os.environ['MESHOPT_LANE_SCRATCH']) / 'source-binding.json'
files = [*root.glob('src/**/*.rs'), *root.glob('tests/*.rs'),
         root / 'Cargo.toml', root / 'Cargo.lock', *lane.glob('*.rs'),
         lane / 'proof.sh', lane / 'bind.py']
binding = {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest()
           for p in sorted(files)}
if len(sys.argv) > 1:
    assert json.loads(out.read_text()) == binding, 'proof source drift'
    print('PASS: proof sources unchanged')
else:
    if out.exists():
        assert json.loads(out.read_text()) == binding, 'use a fresh proof directory for changed sources'
    else:
        out.write_text(json.dumps(binding, indent=2) + '\n')
