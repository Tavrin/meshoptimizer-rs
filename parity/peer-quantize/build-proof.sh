#!/bin/bash
set -eu
s=/mnt/linux-extra/moss-scratch/meshopt-peer-a
for arm in baseline candidate; do
 cargo build --offline --release --manifest-path "$s/protocol-$arm/Cargo.toml" --bin meshopt-driver > "$s/protocol-build-$arm.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/meshopt-driver" "$s/protocol-$arm-driver"
done
cargo build --offline --release --manifest-path "$s/fuzz-seeds/Cargo.toml" --bin quantize_half --bin quantize_float > "$s/fuzz-seeds-build.log" 2>&1
mkdir -p "$s/seeds"
for f in quantize_half quantize_float; do
 SEED_OUTPUT="$s/seeds" "$CARGO_TARGET_DIR/release/$f" 0 > "$s/seeds-$f.json"
done
r=/home/etienne/dev/refs/meshoptimizer
MESHOPT_REFERENCE="$r" bash parity/check-reference.sh > "$s/reference-pin.log" 2>&1
c++ -std=c++17 -O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD -fno-fast-math -ffp-contract=off -I "$r/src" parity/reference.cpp "$r/src/vcacheoptimizer.cpp" "$r/src/overdrawoptimizer.cpp" "$r/src/allocator.cpp" "$r/src/simplifier.cpp" "$r/src/indexgenerator.cpp" "$r/src/vfetchoptimizer.cpp" "$r/src/quantization.cpp" -o "$s/protocol-cpp-driver" > "$s/protocol-build-cpp.log" 2>&1
