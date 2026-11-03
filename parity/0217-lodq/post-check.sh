#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under admission}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
run() {
  local name=$1 rc=0
  shift
  "$@" > "parity/0217-lodq/receipts/$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" >> parity/0217-lodq/receipts/exits.tsv
  tail -n 6 "parity/0217-lodq/receipts/$name.log"
}
run post-fmt cargo fmt -- --check
run post-clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run post-rebind python3 parity/0217-lodq/rebind.py
