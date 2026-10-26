#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?admission required}"
S="${SCRATCH:?}"/meshopt-0210-H2
export TMPDIR="$S/tmp"
mkdir -p "$TMPDIR"
overall=0
run() { local name=$1; shift; "$@" > "$S/gate-$name.log" 2>&1; local rc=$?; printf '%s\t%s\n' "$name" "$rc" | tee -a "$S/gates.tsv"; if [ "$rc" -ne 0 ]; then overall=1; fi; }
run fmt cargo fmt --check
run proof-fmt rustfmt --edition 2021 --check parity/0210-H2/overdraw-exact.rs parity/0210-H2/scale-exact.rs
run all-features cargo test --locked --all-features
run no-default-features cargo test --locked --no-default-features
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run boundary python3 parity/simd/check_boundary.py
cargo generate-lockfile --offline --manifest-path "$S/proof/Cargo.toml" > "$S/proof-lock.log" 2>&1
for mode in std no-std; do
 flags=(); if [ "$mode" = no-std ]; then flags=(--no-default-features); fi
 run exact-build-$mode cargo build --locked --offline --manifest-path "$S/proof/Cargo.toml" --profile moss "${flags[@]}"
 for family in scale overdraw; do
  cp "$CARGO_TARGET_DIR/moss/$family-exact" "$S/$family-exact-$mode"
  run exact-$family-$mode "$S/$family-exact-$mode" "$S/inputs"
 done
done
run cpp-differential python3 parity/0210-H2/differential.py
exit "$overall"
