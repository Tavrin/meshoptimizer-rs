#!/bin/bash
set -euo pipefail
s=/mnt/linux-extra/moss-scratch/meshopt-decode
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS= CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0
cargo fmt
sed -i 's|path = "../baseline"|path = "/home/etienne/dev/meshoptimizer-rs-wt/decode"|' "$s/native/Cargo.toml" "$s/wasm/Cargo.toml"
cargo build --manifest-path "$s/native/Cargo.toml" --locked --offline --profile moss --features ours --bin cmp-driver
cp "$CARGO_TARGET_DIR/moss/cmp-driver" "$s/after"
python3 parity/decode/instructions.py after
python3 parity/decode/instructions.py after --codec
cargo build --manifest-path "$s/wasm/Cargo.toml" --locked --offline --release --target wasm32-unknown-unknown
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/decode_browser.wasm" "$s/browser/out/after-scalar.wasm"
for stream in 0 1 10 11; do
    for calls in 0 100; do
        /usr/lib/linux-tools/6.17.0-22-generic/perf stat -x ';' -e instructions:u -o "$s/wasm-instructions-after-scalar-$stream-$calls.txt" -- taskset -c 2 node "$s/browser/probe.mjs" after-scalar "$stream" "$calls"
    done
done
env -u CARGO_ENCODED_RUSTFLAGS RUSTFLAGS='-C target-feature=+simd128' cargo build --manifest-path "$s/wasm/Cargo.toml" --locked --offline --release --target wasm32-unknown-unknown --features simd
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/decode_browser.wasm" "$s/browser/out/after-simd.wasm"
# REPORT-3 used wasm-opt -Oz. Preserve raw modules alongside the timed artifacts.
for arm in before-scalar after-scalar; do
    wasm-opt -Oz "$s/browser/out/$arm.wasm" -o "$s/browser/out/$arm-oz.wasm"
done
for arm in before-simd after-simd; do
    wasm-opt -Oz --enable-simd "$s/browser/out/$arm.wasm" -o "$s/browser/out/$arm-oz.wasm"
done
wasm-dis "$s/browser/out/after-simd-oz.wasm" -o "$s/after-simd.wat"
