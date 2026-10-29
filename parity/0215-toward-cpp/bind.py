"""Bind proof steps to source contents, rejecting stale resume receipts."""
import hashlib
import json
import os
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[2]
base = Path(os.environ['MESHOPT_BASELINE'])
scratch = Path(os.environ['MESHOPT_LANE_SCRATCH'])
sources = {}
for arm, tree in [('base', base), ('candidate', root)]:
    for p in [*tree.glob('src/**/*.rs'), tree / 'Cargo.toml', tree / 'Cargo.lock']:
        data = p.read_bytes()
        relative = str(p.relative_to(tree))
        if arm == 'base':
            assert data == subprocess.check_output(['git', '-C', str(root), 'show', 'v0.2.14:' + relative])
        sources[arm + '/' + relative] = hashlib.sha256(data).hexdigest()
for p in (root / 'parity/0215-toward-cpp').glob('*'):
    if p.suffix in ['.rs', '.sh', '.py']:
        sources[str(p.relative_to(root))] = hashlib.sha256(p.read_bytes()).hexdigest()
path = scratch / 'proof-source-binding.json'
if path.exists():
    assert json.loads(path.read_text()) == sources, 'source changed; use a new proof directory'
else:
    path.write_text(json.dumps(sources, indent=2) + '\n')
