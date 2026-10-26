#!/bin/bash
set -u
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-h3
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
R=/home/etienne/dev/meshoptimizer-rs-wt/hot-partition
S=/mnt/linux-extra/moss-scratch/meshopt-0210-h3
cd "$R"
run() { name="$1"; shift; "$@" > "$S/gate-$name.log" 2>&1; rc=$?; echo "$name $rc" >> "$S/gate-exits.txt"; return "$rc"; }
failed=0
run test-all cargo test --locked --all-features || failed=1
run test-nostd cargo test --locked --no-default-features || failed=1
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings || failed=1
run fmt cargo fmt --check || failed=1
run boundary python3 parity/simd/check_boundary.py || failed=1
run differential python3 parity/0210-h3/differential.py || failed=1
run sqrt "$S/sqrt-exhaustive" || failed=1
exit "$failed"
