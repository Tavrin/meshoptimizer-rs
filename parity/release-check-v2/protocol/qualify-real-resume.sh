#!/bin/bash
set -euo pipefail
s=/mnt/linux-extra/moss-scratch/meshopt-simplify-perf/release-v2
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUSTFLAGS='-C llvm-args=-align-all-functions=6' RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER=
unset CARGO_ENCODED_RUSTFLAGS
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-rcv2-Cp
export MESHOPT_REFERENCE=/home/etienne/dev/refs/meshoptimizer MESHOPT_ARTIFACTS="$s/parity-artifacts" MESHOPT_RESULTS="$s/parity-results"
cd "$s/real-harness"
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-rcv2-real
cargo build --offline --locked --release --no-default-features --features path-c --bin migration-c > "$s/build-real-resume.log" 2>&1
cp "$CARGO_TARGET_DIR/release/migration-c" "$s/migration-Cp"
python3 "$s/check-real.py" Cp > "$s/real-qualification.log" 2>&1
