#!/bin/bash
set -eu
unset CARGO_ENCODED_RUSTFLAGS
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS='-C llvm-args=-align-all-functions=6'
A="${SCRATCH:?}"/meshopt-validate-once/layout-check
for arm in B C; do
 cp "$A/$arm-pinned.map" "$A/$arm-pinned-preflight.map"
 cp "$A/$arm-pinned" "$A/$arm-pinned-preflight"
 cargo rustc --manifest-path "$A/$arm-pinned-harness/Cargo.toml" --offline --profile bench-release --features ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod --bin cmp-driver -- -C link-arg=-Wl,-Map,$A/$arm-pinned.map
 cp "$CARGO_TARGET_DIR/bench-release/cmp-driver" "$A/$arm-pinned"
done
