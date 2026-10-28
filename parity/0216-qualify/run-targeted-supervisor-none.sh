#!/usr/bin/env bash
set -euo pipefail
scratch=${MESHOPT_SCRATCH:?}
wrapper=${MESHOPT_HEAVY_WRAPPER:?}
repo=$(git rev-parse --show-toplevel)
out="$scratch/proof/0216d-supervisor-none-independent"
mkdir -p "$out"
run_group() {
    local start=$1 end=$2 rc
    set +e
    MOSS_HEAVY_GPU=0 QUAL_MODE=none QUAL_START="$start" QUAL_END="$end" "$wrapper" 4 bash "$repo/parity/0216-qualify/run-targeted-supervisor-group.sh" > "$out/$start-$end.log" 2>&1
    rc=$?
    set -e
    printf '%s %s %s\n' "$start" "$end" "$rc" >> "$out/exits.txt"
    (( rc == 0 )) || exit "$rc"
}
run_group 1 9
run_group 9 17
run_group 17 25
run_group 25 32
