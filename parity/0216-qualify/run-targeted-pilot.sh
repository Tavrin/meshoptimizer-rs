#!/usr/bin/env bash
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
scratch=${MESHOPT_SCRATCH:?}
out="$scratch/proof/0216d-pilot"
mkdir -p "$out"
export CARGO_TARGET_DIR=${MESHOPT_QUALIFY_TARGET:?} CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
(( avail >= 104857600 )) || exit 75
cat > "$out/Cargo.toml" <<MANIFEST
[package]
name = "targeted-pilot"
version = "0.0.0"
edition = "2021"
[features]
all = ["baseline/std", "candidate/std", "baseline/experimental", "candidate/experimental", "baseline/simd", "candidate/simd", "baseline/parallel", "candidate/parallel", "baseline/clusterlod", "candidate/clusterlod", "baseline/parity-internals", "candidate/parity-internals"]
[dependencies]
baseline = { package = "meshoptimizer-baseline", path = "$scratch/proof/base", default-features = false }
candidate = { package = "meshoptimizer-rs", path = "$scratch/candidate", default-features = false }
[[bin]]
name = "targeted-pilot"
path = "$repo/parity/0216-qualify/targeted-quadrics-pilot.rs"
[profile.release]
lto = "thin"
codegen-units = 1
MANIFEST
sha256sum "$repo/parity/0216-qualify/targeted-quadrics-pilot.rs" "$repo/parity/0216-qualify/run-targeted-pilot.sh" > "$out/sources.sha256"
cargo generate-lockfile --offline --manifest-path "$out/Cargo.toml" > "$out/lock.log" 2>&1
for mode in all none; do
    avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
    (( avail >= 104857600 )) || exit 75
    args=(); if [[ "$mode" == all ]]; then args=(--features all); fi
    cargo build --offline --locked --release --manifest-path "$out/Cargo.toml" "${args[@]}" > "$out/build-$mode.log" 2>&1
    cp "$CARGO_TARGET_DIR/release/targeted-pilot" "$out/binary-$mode"
    sha256sum "$out/binary-$mode" >> "$out/binaries.sha256"
    set +e
    (ulimit -t 300; /usr/bin/time -p "$out/binary-$mode") > "$out/run-$mode.log" 2>&1
    rc=$?
    set -e
    printf '%s %s\n' "$mode" "$rc" >> "$out/exits.txt"
    (( rc == 0 )) || exit "$rc"
done
