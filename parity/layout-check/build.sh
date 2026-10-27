#!/bin/bash
set -eu
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= CARGO_ENCODED_RUSTFLAGS=
A="${SCRATCH:?}"/meshopt-validate-once/layout-check
export RUSTFLAGS=""
cargo rustc --manifest-path "$A/B-default-harness/Cargo.toml" --offline --profile bench-release --features ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod --bin cmp-driver -- -C link-arg=-Wl,-Map,$A/B-default.map
cp "$CARGO_TARGET_DIR/bench-release/cmp-driver" "$A/B-default"
export RUSTFLAGS="-C llvm-args=-align-all-functions=6"
cargo rustc --manifest-path "$A/B-pinned-harness/Cargo.toml" --offline --profile bench-release --features ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod --bin cmp-driver -- -C link-arg=-Wl,-Map,$A/B-pinned.map
cp "$CARGO_TARGET_DIR/bench-release/cmp-driver" "$A/B-pinned"
export RUSTFLAGS=""
cargo rustc --manifest-path "$A/Bprime-default-harness/Cargo.toml" --offline --profile bench-release --features ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod --bin cmp-driver -- -C link-arg=-Wl,-Map,$A/Bprime-default.map
cp "$CARGO_TARGET_DIR/bench-release/cmp-driver" "$A/Bprime-default"
export RUSTFLAGS=""
cargo rustc --manifest-path "$A/C-default-harness/Cargo.toml" --offline --profile bench-release --features ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod --bin cmp-driver -- -C link-arg=-Wl,-Map,$A/C-default.map
cp "$CARGO_TARGET_DIR/bench-release/cmp-driver" "$A/C-default"
export RUSTFLAGS="-C llvm-args=-align-all-functions=6"
cargo rustc --manifest-path "$A/C-pinned-harness/Cargo.toml" --offline --profile bench-release --features ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod --bin cmp-driver -- -C link-arg=-Wl,-Map,$A/C-pinned.map
cp "$CARGO_TARGET_DIR/bench-release/cmp-driver" "$A/C-pinned"
export RUSTFLAGS=
cargo build --manifest-path "$A/probe-C-src/Cargo.toml" --offline --release
cp "$CARGO_TARGET_DIR/release/probe-after-old" "$A/probe-C"
export RUSTFLAGS=
cargo build --manifest-path "$A/probe-B-src/Cargo.toml" --offline --release
cp "$CARGO_TARGET_DIR/release/probe-v024" "$A/probe-B"
