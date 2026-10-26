#!/bin/bash
set -euo pipefail
s="${SCRATCH:?}"/meshopt-decode
node "$s/browser/drive.mjs" --bytes-only
python3 - <<'PY'

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import json
from pathlib import Path
s=Path(_public_path('<scratch>/meshopt-decode'))
x=json.loads((s/'browser/data/chromium-bytes.json').read_text())
assert len(x['rows'])==42 and not x['stop_timing']
assert all(not r['excluded'] and len(r['bytes'])==7 and all(v=='match' for v in r['bytes'].values()) for r in x['rows'])
assert 'v128' in (s/'after-simd.wat').read_text()
print('42/42 Chromium byte checks, seven arms; SIMD instructions verified')
PY
