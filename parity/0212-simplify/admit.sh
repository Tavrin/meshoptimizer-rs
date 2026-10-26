#!/usr/bin/env bash
set -euo pipefail
# This explicit steering acknowledgement is deliberately required even if an
# unrelated train/job clears its own hold. Never invoke while this lane is held.
if [[ ${MESHOPT_SIMPLIFY_TRAIN17_RELEASED:-} != yes ]]; then
  echo 'HOLD: coordinator must steer that train 17 has landed.' >&2
  exit 75
fi
export MOSS_HEAVY_GPU=0 MOSS_LANE=meshopt-0212-simplify MOSS_HEAVY_MAX_PER_LANE=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:?}"
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 PYTHONDONTWRITEBYTECODE=1
export CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD -g'
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
exec "${JOB_RUNNER:?}" 4 "$@"
