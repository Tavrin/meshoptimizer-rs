#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under CPU admission}"
: "${PIPELINE_INPUTS:?provide retained corpus directory}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
out=parity/0217-pipeline/receipts
cargo build --release --manifest-path "$CARGO_TARGET_DIR/proof/driver/Cargo.toml" > "$out/supplement-build.log" 2>&1
rc=$?
printf 'supplement-build\t%s\n' "$rc" >> "$out/exits.tsv"
if [ "$rc" != 0 ]; then exit "$rc"; fi
cpu=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
PIPELINE_SUPPLEMENT=1 taskset -c "$cpu" "$CARGO_TARGET_DIR/release/pipeline" \
 "$PIPELINE_INPUTS/tiny-smooth.bin" "$PIPELINE_INPUTS/medium-smooth.bin" \
 "$PIPELINE_INPUTS/medium-seams.bin" "$PIPELINE_INPUTS/medium-sparse.bin" \
 "$PIPELINE_INPUTS/million-smooth.bin" > "$out/supplement.log" 2>&1
rc=$?
printf 'supplement\t%s\n' "$rc" >> "$out/exits.tsv"
cat "$out/supplement.log"
exit "$rc"
