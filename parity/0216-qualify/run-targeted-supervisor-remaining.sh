#!/usr/bin/env bash
set -euo pipefail
scratch=${MESHOPT_SCRATCH:?}
wrapper=${MESHOPT_HEAVY_WRAPPER:?}
repo=$(git rev-parse --show-toplevel)
out="$scratch/proof/0216d-supervisor-remaining"
mkdir -p "$out"
run_group() {
    local mode=$1 start=$2 end=$3 rc
    set +e
    MOSS_HEAVY_GPU=0 QUAL_MODE="$mode" QUAL_START="$start" QUAL_END="$end" "$wrapper" 4 bash "$repo/parity/0216-qualify/run-targeted-supervisor-group.sh" > "$out/$mode-$start-$end.log" 2>&1
    rc=$?
    set -e
    printf '%s %s %s %s\n' "$mode" "$start" "$end" "$rc" >> "$out/exits.txt"
    (( rc == 0 )) || exit "$rc"
}
run_group all 9 17
run_group all 17 25
run_group all 25 32
run_group none 1 9
run_group none 9 17
run_group none 17 25
run_group none 25 32
