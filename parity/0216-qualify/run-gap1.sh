#!/usr/bin/env bash
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
scratch=${MESHOPT_SCRATCH:?}
name=${GAP_RUN_NAME:?}
seconds=${GAP_RUN_SECONDS:?}
out="$scratch/proof/$name"
mkdir -p "$out"
export CARGO_TARGET_DIR=${MESHOPT_QUALIFY_TARGET:?} CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
floor() { local avail; avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}'); (( avail >= 104857600 )); }
cat > "$out/Cargo.toml" <<MANIFEST
[package]
name = "qualification-gap1"
version = "0.0.0"
edition = "2021"
[features]
all = ["baseline/std", "candidate/std", "baseline/experimental", "candidate/experimental", "baseline/simd", "candidate/simd", "baseline/parallel", "candidate/parallel", "baseline/clusterlod", "candidate/clusterlod", "baseline/parity-internals", "candidate/parity-internals"]
[dependencies]
baseline = { package = "meshoptimizer-baseline", path = "$scratch/proof/base", default-features = false }
candidate = { package = "meshoptimizer-rs", path = "$scratch/candidate", default-features = false }
[[bin]]
name = "gap1"
path = "$repo/parity/0216-qualify/gap1-simplify.rs"
[profile.release]
lto = "thin"
codegen-units = 1
MANIFEST
sha256sum "$repo/parity/0216-qualify/gap1-simplify.rs" "$repo/parity/0216-qualify/run-gap1.sh" > "$out/sources.sha256"
floor || { echo 'disk floor' > "$out/error"; exit 75; }
cargo generate-lockfile --offline --manifest-path "$out/Cargo.toml" > "$out/lock.log" 2>&1
for mode in all none; do
    floor || { echo 'disk floor' > "$out/error"; exit 75; }
    args=(); if [[ "$mode" == all ]]; then args=(--features all); fi
    cargo build --offline --locked --release --manifest-path "$out/Cargo.toml" "${args[@]}" > "$out/build-$mode.log" 2>&1
    cp "$CARGO_TARGET_DIR/release/gap1" "$out/binary-$mode"
    sha256sum "$out/binary-$mode" >> "$out/binaries.sha256"
done
set +e
"$out/binary-all" 511 "$seconds" > "$out/run-all.log" 2>&1 &
pid_all=$!
"$out/binary-none" 127 "$seconds" > "$out/run-none.log" 2>&1 &
pid_none=$!
wait -n -p completed "$pid_all" "$pid_none"
first=$?
if [[ "$first" != 0 && "$first" != 124 ]]; then
    kill "$pid_all" "$pid_none" 2>/dev/null || true
fi
if [[ "$completed" == "$pid_all" ]]; then
    all=$first
    wait "$pid_none"; none=$?
else
    none=$first
    wait "$pid_all"; all=$?
fi
printf 'all %s\nnone %s\nfirst %s\n' "$all" "$none" "$first" > "$out/exits.txt"
if [[ "$first" != 0 && "$first" != 124 ]]; then exit "$first"; fi
if [[ "$all" != 0 || "$none" != 0 ]]; then exit 124; fi
