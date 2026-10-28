#!/usr/bin/env bash
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
scratch=${MESHOPT_SCRATCH:?}
out="$scratch/proof/0216b-affected"
mkdir -p "$out"
export CARGO_TARGET_DIR=${MESHOPT_QUALIFY_TARGET:?} CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
manifest="$scratch/proof/driver/Cargo.toml"
floor() { local avail; avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}'); (( avail >= 104857600 )) || exit 75; }
sha256sum "$repo/parity/0216-qualify/simplify-exact.rs" "$repo/parity/0216-qualify/overdraw-exact.rs" > "$out/sources.sha256"
for mode in all none; do
    floor
    args=(); if [[ "$mode" == all ]]; then args=(--features all); fi
    cargo build --offline --locked --release --manifest-path "$manifest" "${args[@]}" > "$out/build-$mode.log" 2>&1
    cp "$CARGO_TARGET_DIR/release/simplify-exact" "$out/simplify-$mode"
    cp "$CARGO_TARGET_DIR/release/overdraw-exact" "$out/overdraw-$mode"
    sha256sum "$out/simplify-$mode" "$out/overdraw-$mode" >> "$out/binaries.sha256"
    "$out/simplify-$mode" > "$out/simplify-$mode.log" 2>&1
    printf 'simplify-%s 0\n' "$mode" >> "$out/exits.txt"
    "$out/overdraw-$mode" "$scratch/proof/small-inputs" > "$out/overdraw-$mode.log" 2>&1
    printf 'overdraw-%s 0\n' "$mode" >> "$out/exits.txt"
done
