#!/usr/bin/env bash
set -euo pipefail
: "${PARITY_RUNNER_ACTIVE:?invoke through shared admission}"
[[ ${PARITY_RUNNER_GPU:-} == 0 ]]
cargo fmt
cargo fmt --manifest-path "${MESHOPT_SCRATCH:?}/proof/Cargo.toml"
bash parity/0213-simplify3/gates.sh
python3 parity/0213-simplify3/build.py candidate
python3 parity/0213-simplify3/measure.py
python3 parity/0213-simplify3/results.py
python3 parity/0213-simplify3/audit.py
python3 parity/0213-simplify3/report.py
