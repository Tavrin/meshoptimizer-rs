#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?Run through ${JOB_RUNNER:?} 4 with MOSS_HEAVY_GPU=0}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:?}"
export CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
python3 "$(dirname "$0")/decode-proof-filters.py"
