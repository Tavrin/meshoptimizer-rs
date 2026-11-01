#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under admission}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
out=parity/0217-pipeline/receipts
run() {
 local name=$1 rc=0
 shift
 "$@" > "$out/$name.log" 2>&1 || rc=$?
 printf '%s\t%s\n' "$name" "$rc" >> "$out/exits.tsv"
 tail -n 8 "$out/$name.log"
}
run final-fmt cargo fmt -- --check
run final-clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run final-pipeline cargo test --locked --all-features --test pipeline
run final-pipeline-scalar cargo test --locked --no-default-features --test pipeline
run final-docs env RUSTDOCFLAGS='-D warnings' cargo doc --locked --all-features --no-deps
run final-semver cargo semver-checks check-release --baseline-version 0.2.16
run final-wasm cargo build --locked --target wasm32-unknown-unknown --no-default-features
