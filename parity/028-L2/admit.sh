#!/usr/bin/env bash
set -euo pipefail
export MOSS_HEAVY_GPU=0 MOSS_LANE=meshopt-028-L2
export MOSS_HEAVY_MAX_PER_LANE=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:?}"
export CARGO_BUILD_JOBS=1
exec "${JOB_RUNNER:?}" 4 "$@"
