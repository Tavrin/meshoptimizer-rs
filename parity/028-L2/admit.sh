#!/usr/bin/env bash
set -euo pipefail
export MOSS_HEAVY_GPU=0 MOSS_LANE=meshopt-028-L2
export MOSS_HEAVY_MAX_PER_LANE=1
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-L2
export CARGO_BUILD_JOBS=1
exec /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 4 "$@"
