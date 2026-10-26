#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?admission required}"
export CXXFLAGS=-DMESHOPTIMIZER_NO_SIMD
if [ "${H2_BUILD:-0}" = 1 ]; then bash parity/0210-H2/build.sh; fi
python3 parity/0210-H2/instructions.py
if [ "${H2_PILOT:-0}" = 1 ]; then
 python3 - <<'PY'
from pathlib import Path
import json,os
s=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H2')/os.environ['H2_SAMPLES']
r=json.loads((s/'summary.json').read_text())
for c in r:
 ratio=c['median']['candidate']/c['median']['base']
 print('pilot candidate/base',c['id'],ratio,flush=True)
 if ratio>1.01:raise SystemExit(3)
PY
 unset H2_PILOT H2_SAMPLES
 python3 parity/0210-H2/instructions.py
fi
printf 'measurement\t0\n' >> /mnt/linux-extra/moss-scratch/meshopt-0210-H2/gates.tsv
python3 parity/0210-H2/perf-verdict.py
bash parity/0210-H2/final-gates.sh
