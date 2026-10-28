#!/usr/bin/env bash
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
scratch=${MESHOPT_SCRATCH:?}
export CARGO_TARGET_DIR=${MESHOPT_QUALIFY_TARGET:?} CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
out="$scratch/proof/0216b-clod-flags"
mkdir -p "$out"
cat > "$out/Cargo.toml" <<MANIFEST
[package]
name = "qualification-clod-flags"
version = "0.0.0"
edition = "2021"
[features]
all = ["baseline/std", "candidate/std", "baseline/experimental", "candidate/experimental", "baseline/simd", "candidate/simd", "baseline/parallel", "candidate/parallel", "baseline/clusterlod", "candidate/clusterlod", "baseline/parity-internals", "candidate/parity-internals"]
[dependencies]
baseline = { package = "meshoptimizer-baseline", path = "$scratch/proof/base", default-features = false, features = ["clusterlod"] }
candidate = { package = "meshoptimizer-rs", path = "$scratch/candidate", default-features = false, features = ["clusterlod"] }
[[bin]]
name = "clod-flags"
path = "$repo/parity/0216-qualify/clod-flags.rs"
[profile.release]
lto = "thin"
codegen-units = 1
MANIFEST
sha256sum "$repo/parity/0216-qualify/clod-flags.rs" > "$out/source.sha256"
for mode in all none; do
    avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
    (( avail >= 104857600 )) || exit 75
    args=(); if [[ "$mode" == all ]]; then args=(--features all); fi
    cargo generate-lockfile --offline --manifest-path "$out/Cargo.toml" > "$out/lock-$mode.log" 2>&1
    cargo build --offline --locked --release --manifest-path "$out/Cargo.toml" "${args[@]}" > "$out/build-$mode.log" 2>&1
    cp "$CARGO_TARGET_DIR/release/clod-flags" "$out/binary-$mode"
    sha256sum "$out/binary-$mode" >> "$out/binaries.sha256"
    "$out/binary-$mode" > "$out/run-$mode.log" 2>&1
    printf '%s 0\n' "$mode" >> "$out/exits.txt"
done
