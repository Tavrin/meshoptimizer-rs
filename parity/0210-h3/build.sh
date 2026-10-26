#!/bin/bash
set -eu
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-h3
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS= CXXFLAGS=-DMESHOPTIMIZER_NO_SIMD
S=/mnt/linux-extra/moss-scratch/meshopt-0210-h3
mkdir -p "$S/tmp"
export TMPDIR="$S/tmp"
for arm in base candidate M7; do
 features=ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod
 if [ "$arm" = M7 ]; then features=peer7; fi
 cargo generate-lockfile --offline --manifest-path "$S/harness-$arm/Cargo.toml" > "$S/lock-$arm.log" 2>&1
 cargo build --offline --locked --manifest-path "$S/harness-$arm/Cargo.toml" --profile moss --features "$features" --bin cmp-driver > "$S/build-$arm.log" 2>&1
 cp "$CARGO_TARGET_DIR/moss/cmp-driver" "$S/driver-$arm"
done
