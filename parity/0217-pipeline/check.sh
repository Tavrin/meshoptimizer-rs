#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under admission}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
out=parity/0217-pipeline/receipts
mkdir -p "$out"
run() {
 local name=$1 rc=0
 shift
 "$@" > "$out/$name.log" 2>&1 || rc=$?
 printf '%s\t%s\n' "$name" "$rc" >> "$out/exits.tsv"
 tail -n 6 "$out/$name.log"
}
run fmt cargo fmt -- --check
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run tests cargo test --locked --all-features
run default cargo test --locked
run no-std cargo test --locked --no-default-features
run doctests cargo test --locked --doc --all-features
run docs env RUSTDOCFLAGS='-D warnings' cargo doc --locked --all-features --no-deps
run semver cargo semver-checks check-release --baseline-version 0.2.16
run wasm cargo build --locked --target wasm32-unknown-unknown --no-default-features
run wasm-default cargo build --locked --target wasm32-unknown-unknown
run public bash parity/tools/check_public_text.sh
