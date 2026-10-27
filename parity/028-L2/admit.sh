#!/usr/bin/env bash
set -euo pipefail
export PARITY_RUNNER_GPU=0 PARITY_JOB=meshopt-028-L2
export DOWNSTREAM_HEAVY_MAX_PER_LANE=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:?}"
export CARGO_BUILD_JOBS=1
exec "${JOB_RUNNER:?}" 4 "$@"
