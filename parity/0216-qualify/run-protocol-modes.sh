#!/usr/bin/env bash
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
scratch=${MESHOPT_SCRATCH:?}
export CARGO_TARGET_DIR=${MESHOPT_QUALIFY_TARGET:?} CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
out="$scratch/proof/0216b-protocol"
mkdir -p "$out"
bash "$repo/parity/check-reference.sh" > "$out/reference.log" 2>&1
avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
(( avail >= 104857600 )) || exit 75
c++ -std=c++17 -O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD -fno-fast-math -ffp-contract=off -I"${MESHOPT_REFERENCE:?}/src" "$repo/parity/reference.cpp" \
    "$MESHOPT_REFERENCE/src/vcacheoptimizer.cpp" "$MESHOPT_REFERENCE/src/overdrawoptimizer.cpp" \
    "$MESHOPT_REFERENCE/src/allocator.cpp" "$MESHOPT_REFERENCE/src/simplifier.cpp" \
    "$MESHOPT_REFERENCE/src/indexgenerator.cpp" "$MESHOPT_REFERENCE/src/vfetchoptimizer.cpp" \
    "$MESHOPT_REFERENCE/src/quantization.cpp" -o "$out/cpp-reference" > "$out/build-cpp.log" 2>&1
for mode in all none; do
    avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
    (( avail >= 104857600 )) || exit 75
    features='[]'
    if [[ "$mode" == all ]]; then features='["std", "simd", "parallel", "clusterlod", "experimental", "parity-internals"]'; fi
    cat > "$out/Cargo.toml" <<MANIFEST
[package]
name = "meshoptimizer-parity"
version = "0.0.0"
edition = "2021"
[dependencies]
meshoptimizer-rs = { path = "$scratch/candidate", default-features = false, features = $features }
libm = { version = "=0.2.16", default-features = false }
[lib]
path = "$repo/parity/src/lib.rs"
[[bin]]
name = "meshopt-driver"
path = "$repo/parity/src/main.rs"
[profile.release]
codegen-units = 1
lto = "fat"
MANIFEST
    cargo generate-lockfile --offline --manifest-path "$out/Cargo.toml" > "$out/lock-$mode.log" 2>&1
    cargo build --offline --locked --release --manifest-path "$out/Cargo.toml" > "$out/build-$mode.log" 2>&1
    cp "$CARGO_TARGET_DIR/release/meshopt-driver" "$out/rust-$mode"
done
python3 "$repo/parity/0216-qualify/protocol-modes.py"
