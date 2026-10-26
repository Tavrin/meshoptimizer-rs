#!/bin/bash
set -uo pipefail
s=/mnt/linux-extra/moss-scratch/meshopt-decode
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS= CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0
export MESHOPT_REFERENCE=/home/etienne/dev/refs/meshoptimizer MESHOPT_ARTIFACTS="$s" TMPDIR="$CARGO_TARGET_DIR/tmp"
mkdir -p "$TMPDIR"
record() {
    local label="$1"
    shift
    "$@" > "$s/gate-$label.log" 2>&1
    local code=$?
    printf '%s\t%s\n' "$label" "$code" | tee -a "$s/gates.tsv"
    return "$code"
}
record fmt cargo fmt --check
record test cargo test --all-features --locked --offline
record clippy cargo clippy --all-targets --all-features --locked --offline -- -D warnings
record scalar-nostd cargo build --manifest-path parity/codec/Cargo.toml --release --offline --locked --bin codec-driver
cp "$CARGO_TARGET_DIR/release/codec-driver" "$s/codec-scalar-nostd"
record scalar-std cargo build --manifest-path parity/codec/Cargo.toml --release --offline --locked --features std --bin codec-driver
cp "$CARGO_TARGET_DIR/release/codec-driver" "$s/codec-scalar-std"
record scalar-tests cargo test --no-default-features --features std --locked --offline --test codec
record simd-protocol cargo build --manifest-path parity/codec/Cargo.toml --release --offline --locked --features simd --bin codec-driver
cp "$CARGO_TARGET_DIR/release/codec-driver" "$s/codec-simd"
record simd-baseline cargo build --manifest-path "$s/baseline/parity/codec/Cargo.toml" --release --offline --locked --features simd --bin codec-driver
cp "$CARGO_TARGET_DIR/release/codec-driver" "$s/codec-simd-before"
record oracle-check bash parity/check-reference.sh
record oracle c++ -std=c++17 -O3 -DNDEBUG -fno-fast-math -ffp-contract=off -DMESHOPTIMIZER_NO_SIMD -I "$MESHOPT_REFERENCE/src" parity/codec/reference.cpp "$MESHOPT_REFERENCE/src/vertexcodec.cpp" "$MESHOPT_REFERENCE/src/indexcodec.cpp" "$MESHOPT_REFERENCE/src/vertexfilter.cpp" "$MESHOPT_REFERENCE/src/meshletcodec.cpp" "$MESHOPT_REFERENCE/src/allocator.cpp" -o "$s/cpp-scalar"
record differential python3 parity/decode/differential.py
