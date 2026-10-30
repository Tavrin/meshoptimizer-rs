#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under admission}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
run() {
  local name=$1 rc=0
  shift
  "$@" > "parity/0217-lodq/receipts/$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" >> parity/0217-lodq/receipts/exits.tsv
  tail -n 8 "parity/0217-lodq/receipts/$name.log"
}
run final-fmt cargo fmt
run final-focused cargo test --locked --test lod_quality -- --nocapture
run final-focused-scalar cargo test --locked --no-default-features --test lod_quality -- --nocapture
run final-clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run final-all cargo test --locked --all-features
run final-wasm cargo build --locked --target wasm32-unknown-unknown --no-default-features
run final-wasm-default cargo build --locked --target wasm32-unknown-unknown
run final-fmt-check cargo fmt -- --check
run boundary bash parity/check-unsafe-boundary.sh
