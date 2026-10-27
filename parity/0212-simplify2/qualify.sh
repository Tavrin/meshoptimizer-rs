#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?invoke through admit.sh}"
[[ ${MOSS_HEAVY_GPU:-} == 0 ]]
cargo fmt
bash parity/0212-simplify2/gates.sh
python3 parity/0212-simplify2/build.py
python3 parity/0212-simplify2/measure.py
python3 parity/0212-simplify2/results.py
python3 parity/0212-simplify2/audit.py
python3 parity/0212-simplify2/report.py
