#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under admit.sh}"
art=/mnt/linux-extra/moss-scratch/meshopt-029-scale
export TMPDIR="$art/tmp"
mkdir -p "$TMPDIR"
case "$1" in
 qualify)
  python3 parity/029-scale/instructions.py revised > "$art/instructions.log" 2>&1
  code=$?; printf 'instructions\t%s\n' "$code" >> "$art/gates.tsv"
  if [ "$code" -ne 0 ]; then exit "$code"; fi
  bash parity/029-scale/job.sh gates
  code=$?; if [ "$code" -ne 0 ]; then exit "$code"; fi
  bash parity/029-scale/job.sh fuzz > "$art/fuzz.log" 2>&1
  code=$?; printf 'fuzz\t%s\n' "$code" >> "$art/gates.tsv"
  exit "$code" ;;
 build) exec python3 parity/029-scale/build.py ;;
 instructions) shift; exec python3 parity/029-scale/instructions.py "$@" ;;
 gates)
  overall=0
  run() { local name=$1; shift; "$@" > "$art/gate-$name.log" 2>&1; local rc=$?; printf '%s\t%s\n' "$name" "$rc" | tee -a "$art/gates.tsv"; if [ "$rc" -ne 0 ]; then overall=1; fi; }
  run fmt cargo fmt --check
  run all-features cargo test --locked --all-features
  run no-default-features cargo test --locked --no-default-features
  run clippy cargo clippy --locked --all-targets --all-features -- -D warnings
  run boundary python3 parity/simd/check_boundary.py
  for mode in std no-std; do
   flags=(); if [ "$mode" = no-std ]; then flags=(--no-default-features); fi
   run exact-build-$mode cargo build --locked --offline --manifest-path "$art/harness/Cargo.toml" --profile moss --bin scale-exact "${flags[@]}"
   cp "$CARGO_TARGET_DIR/moss/scale-exact" "$art/exact-$mode"
   run exact-$mode "$art/exact-$mode"
  done
  run cpp-differential python3 parity/029-scale/differential.py
  exit "$overall" ;;
 fuzz)
  export MESHOPT_ARTIFACTS="$art/fuzz-artifacts" MESHOPT_RESULTS="$art/fuzz-results" MESHOPT_FUZZ_CPUS=9
  exec bash parity/fuzz.sh --target simplify_scale --cpu-hours-per-target 0.08333333333333334 --jobs 1 --chunk-seconds 60 --seed 20261002 ;;
 *) exit 2 ;;
esac
