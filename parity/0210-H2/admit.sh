#!/usr/bin/env bash
set -euo pipefail
export MOSS_HEAVY_GPU=0 MOSS_LANE=meshopt-0210-H2 MOSS_HEAVY_MAX_PER_LANE=1
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-H2
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 PYTHONDONTWRITEBYTECODE=1
export CXXFLAGS=-DMESHOPTIMIZER_NO_SIMD
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
exec /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 4 "$@"
