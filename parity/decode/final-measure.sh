#!/bin/bash
set -euo pipefail
s=/mnt/linux-extra/moss-scratch/meshopt-decode
python3 - <<'PY'
import hashlib,json
from pathlib import Path
s=Path('/mnt/linux-extra/moss-scratch/meshopt-decode')
x=json.loads((s/'MEASUREMENT-IDENTITY.json').read_text())
assert all(Path(p).exists() and hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in x['source_and_artifacts'].items())
assert all(line.split('\t')[1]=='0'for line in (s/'gates.tsv').read_text().splitlines())
assert not (s/'browser/data/chromium-timing.json').exists()
print('Frozen source/binary/input identity and all proof gates verified',flush=True)
PY
cat /proc/loadavg /proc/pressure/{cpu,io,memory} > "$s/host-before-timing.txt"
python3 parity/decode/native.py
printf '{"bytes_only":false}\n' > "$s/browser/data/mode.json"
node "$s/browser/drive.mjs"
cat /proc/loadavg /proc/pressure/{cpu,io,memory} > "$s/host-after-timing.txt"
python3 parity/decode/analyze.py
