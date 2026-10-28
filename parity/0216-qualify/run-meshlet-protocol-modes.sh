#!/usr/bin/env bash
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
scratch=${MESHOPT_SCRATCH:?}
export CARGO_TARGET_DIR=${MESHOPT_QUALIFY_TARGET:?} CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
export PARITY_RUNNER_ACTIVE="$MOSS_HEAVY_ACTIVE" PARITY_RUNNER_GPU="$MOSS_HEAVY_GPU"
export PEERS_ART="$scratch/proof/0216b-meshlet-protocol"
mkdir -p "$PEERS_ART"
avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
(( avail >= 104857600 )) || exit 75
python3 "$repo/parity/0216-qualify/meshlet-protocol-modes.py" > "$PEERS_ART/run.log" 2>&1
