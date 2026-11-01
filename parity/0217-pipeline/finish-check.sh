#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under CPU admission}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
out=parity/0217-pipeline/receipts
cargo test --locked --all-features --lib pipeline::tests > "$out/final-retained.log" 2>&1
rc=$?
printf 'final-retained\t%s\n' "$rc" >> "$out/exits.tsv"
tail -n 8 "$out/final-retained.log"
bash parity/0217-pipeline/post.sh
