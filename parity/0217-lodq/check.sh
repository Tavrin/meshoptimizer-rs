#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under admission}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
mkdir -p parity/0217-lodq/receipts
run() {
  local name=$1 rc=0
  shift
  "$@" > "parity/0217-lodq/receipts/$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" >> parity/0217-lodq/receipts/exits.tsv
  tail -n 5 "parity/0217-lodq/receipts/$name.log"
}
run fmt cargo fmt
run focused cargo test --locked --test lod_quality -- --nocapture
run focused-scalar cargo test --locked --no-default-features --test lod_quality -- --nocapture
run all cargo test --locked --all-features
run scalar cargo test --locked --no-default-features
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run wasm cargo build --locked --target wasm32-unknown-unknown --no-default-features
run wasm-simd cargo build --locked --target wasm32-unknown-unknown
