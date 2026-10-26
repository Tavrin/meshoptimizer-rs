#!/usr/bin/env bash
set -euo pipefail
export MOSS_HEAVY_GPU=0 MOSS_LANE=meshopt-029-scale MOSS_HEAVY_MAX_PER_LANE=1
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-scale-strided
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 PYTHONDONTWRITEBYTECODE=1
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
exec /mnt/linux-extra/moss-coord/bin/moss-heavy.sh "${MESHOPT_DECLARE_GB:-4}" "$@"
