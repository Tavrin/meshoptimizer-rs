#!/usr/bin/env bash
# Build and run an exact v0.2.14-vs-candidate driver in both feature modes.
# Usage: run-targeted.sh DRIVER.rs NAME START END [ARGS...]
# MESHOPT_SCRATCH holds proof/base (v0.2.14 source, package renamed to
# meshoptimizer-baseline) and candidate/ (the candidate source tree).
# MESHOPT_QUALIFY_TARGET is the dedicated cargo target directory.
# Run under the CPU-only admission wrapper.
set -euo pipefail
driver=$(realpath "$1") name=$2 start=$3 end=$4
shift 4
scratch=${MESHOPT_SCRATCH:?}
out="$scratch/proof/$name"
mkdir -p "$out"
export CARGO_TARGET_DIR=${MESHOPT_QUALIFY_TARGET:?} CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
(( avail >= 104857600 )) || exit 75
cat > "$out/Cargo.toml" <<MANIFEST
[package]
name = "exact-$name"
version = "0.0.0"
edition = "2021"
[features]
all = ["baseline/std", "candidate/std", "baseline/simd", "candidate/simd", "baseline/experimental", "candidate/experimental", "baseline/clusterlod", "candidate/clusterlod"]
[dependencies]
baseline = { package = "meshoptimizer-baseline", path = "$scratch/proof/base", default-features = false }
candidate = { package = "meshoptimizer-rs", path = "$scratch/candidate", default-features = false }
[[bin]]
name = "exact"
path = "$driver"
[profile.release]
lto = "thin"
codegen-units = 1
debug-assertions = true
MANIFEST
sha256sum "$driver" "$0" > "$out/sources.sha256"
cargo generate-lockfile --offline --manifest-path "$out/Cargo.toml" > "$out/lock.log" 2>&1
for mode in ${MODES:-all none}; do
    args=(); if [[ "$mode" == all ]]; then args=(--features all); fi
    cargo build --offline --locked --release --manifest-path "$out/Cargo.toml" "${args[@]}" > "$out/build-$mode.log" 2>&1
    cp "$CARGO_TARGET_DIR/release/exact" "$out/binary-$mode"
    sha256sum "$out/binary-$mode" >> "$out/binaries.sha256"
    set +e
    (ulimit -t 7200; /usr/bin/time -p "$out/binary-$mode" "$start" "$end" "$@") > "$out/run-$mode-$start-$end.log" 2>&1
    rc=$?
    set -e
    printf '%s %s %s %s\n' "$mode" "$start" "$end" "$rc" >> "$out/exits.txt"
    (( rc == 0 )) || exit "$rc"
done
