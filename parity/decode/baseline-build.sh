#!/bin/bash
# Invoke through <job-runner> (4 GiB), with the registered protocol target environment.
set -euo pipefail
s="${SCRATCH:?}"/meshopt-decode
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS= CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0
export CXXFLAGS=-DMESHOPTIMIZER_NO_SIMD
cargo generate-lockfile --offline --manifest-path "$s/native/Cargo.toml"
cargo build --manifest-path "$s/native/Cargo.toml" --locked --offline --profile bench-release --features ours --bin cmp-driver
cp "$CARGO_TARGET_DIR/bench-release/cmp-driver" "$s/before"
cargo build --manifest-path "$s/native/Cargo.toml" --locked --offline --profile bench-release --features peer7 --bin cmp-driver
cp "$CARGO_TARGET_DIR/bench-release/cmp-driver" "$s/m7"
python3 parity/decode/instructions.py before
python3 parity/decode/instructions.py before --codec
cargo generate-lockfile --offline --manifest-path "$s/wasm/Cargo.toml"
cargo build --manifest-path "$s/wasm/Cargo.toml" --locked --offline --release --target wasm32-unknown-unknown
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/decode_browser.wasm" "$s/browser/out/before-scalar.wasm"
env -u CARGO_ENCODED_RUSTFLAGS RUSTFLAGS='-C target-feature=+simd128' cargo build --manifest-path "$s/wasm/Cargo.toml" --locked --offline --release --target wasm32-unknown-unknown --features simd
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/decode_browser.wasm" "$s/browser/out/before-simd.wasm"
