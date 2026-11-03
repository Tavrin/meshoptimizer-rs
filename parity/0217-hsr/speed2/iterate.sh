#!/usr/bin/env bash
set -euo pipefail
cargo fmt --all
cargo test --locked --all-features --test hidden
python3 parity/0217-hsr/speed2/iterate.py
if python3 - <<'PYGATE'
import json,sys
from pathlib import Path
rows=json.loads(Path('parity/0217-hsr/speed2/iterate.json').read_text())
limits={'breakfast_room':5.0,'conference':2.0}
sys.exit(0 if all(r['samples'][-1]['seconds'] <= limits.get(r['scene'],float('inf')) for r in rows) else 1)
PYGATE
then
    python3 parity/0217-hsr/speed2/checks.py
    python3 parity/0217-hsr/speed2/oracle.py
    python3 parity/0217-hsr/speed2/campaign.py
fi
