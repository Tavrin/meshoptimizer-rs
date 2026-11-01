#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under CPU admission}"
: "${PIPELINE_INPUTS:?provide retained corpus directory}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
out=parity/0217-pipeline/receipts
cargo build --release --manifest-path "$CARGO_TARGET_DIR/proof/driver/Cargo.toml" > "$out/quiet-build.log" 2>&1
rc=$?
printf 'quiet-build\t%s\n' "$rc" >> "$out/exits.tsv"
if [ "$rc" != 0 ]; then exit "$rc"; fi
cpu=$(python3 parity/0217-pipeline/quiet_cpu.py)
export PIPELINE_HZ=$(getconf CLK_TCK)
PIPELINE_QUIET=1 taskset -c "$cpu" "$CARGO_TARGET_DIR/release/pipeline" \
 "$PIPELINE_INPUTS/tiny-smooth.bin" "$PIPELINE_INPUTS/medium-smooth.bin" \
 "$PIPELINE_INPUTS/medium-seams.bin" "$PIPELINE_INPUTS/medium-sparse.bin" \
 "$PIPELINE_INPUTS/million-smooth.bin" "$PIPELINE_INPUTS/real-214e.bin" \
 "$PIPELINE_INPUTS/real-a082.bin" "$PIPELINE_INPUTS/real-b6fd.bin" > "$out/quiet.log" 2>&1
rc=$?
printf 'quiet\t%s\n' "$rc" >> "$out/exits.tsv"
tail -n 15 "$out/quiet.log"
exit "$rc"
