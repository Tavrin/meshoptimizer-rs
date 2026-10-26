#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?admission required}"
export CXXFLAGS=-DMESHOPTIMIZER_NO_SIMD
S="${SCRATCH:?}"/meshopt-0210-H2
export TMPDIR="$S/tmp"
mkdir -p "$TMPDIR"
python3 parity/0210-H2/build-binding.py before
for arm in base candidate M7; do
 features=ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod
 if [ "$arm" = M7 ]; then features=peer7; fi
 cargo generate-lockfile --offline --manifest-path "$S/harness-$arm/Cargo.toml" > "$S/lock-$arm.log" 2>&1
 cargo build --offline --locked --manifest-path "$S/harness-$arm/Cargo.toml" --profile moss --features "$features" --bin cmp-driver > "$S/build-$arm.log" 2>&1
 cp "$CARGO_TARGET_DIR/moss/cmp-driver" "$S/driver-$arm"
done
python3 parity/0210-H2/build-binding.py after
