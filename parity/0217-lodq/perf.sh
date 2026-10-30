#!/usr/bin/env bash
set -euo pipefail
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 LC_ALL=C
python3 parity/0217-lodq/perf.py
