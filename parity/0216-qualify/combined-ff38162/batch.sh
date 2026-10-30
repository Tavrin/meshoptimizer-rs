#!/usr/bin/env bash
# Each stage enters admission separately; queue waits have no timeout.
set -euo pipefail
: "${QUALIFY_ADMISSION:?}" "${CARGO_TARGET_DIR:?}" "${MESHOPT_REFERENCE:?}"
export MOSS_HEAVY_GPU=0 CARGO_INCREMENTAL=0 LC_ALL=C PYTHONDONTWRITEBYTECODE=1
Q=parity/0216-qualify/combined-ff38162
stage() {
    local rc=0
    "$QUALIFY_ADMISSION" 4 python3 "$Q/run.py" "$@" || rc=$?
    printf '%s\t%s\n' "$*" "$rc" >> "$Q/stages.tsv"
    return "$rc"
}
stage proof_group
stage differential_group
for group in corpus real-214e real-a082 real-b6fd; do
    stage measure_inputs "$group"
done
stage disclosure_group
