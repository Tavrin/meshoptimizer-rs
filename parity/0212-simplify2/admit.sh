#!/usr/bin/env bash
set -euo pipefail
: "${MESHOPT_JOB_RUNNER:?set the job runner executable}"
: "${MESHOPT_SCRATCH:?set the scratch directory}"
: "${CARGO_TARGET_DIR:?set the dedicated target directory}"
export MOSS_HEAVY_GPU=0 MOSS_LANE=meshopt-0212-simplify2 MOSS_HEAVY_MAX_PER_LANE=1
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 PYTHONDONTWRITEBYTECODE=1
export CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD -g'
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
exec "$MESHOPT_JOB_RUNNER" 4 "$@"
