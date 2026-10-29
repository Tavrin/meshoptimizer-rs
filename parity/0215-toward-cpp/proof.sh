#!/usr/bin/env bash
# Behavioral proof for the 0.2.15 toward-C++ lane (vertex cache, vertex decode,
# meshlets, cluster LOD). Run under the CPU-only admission runner
# (PARITY_RUNNER_ACTIVE set, PARITY_RUNNER_GPU=0) with a dedicated
# CARGO_TARGET_DIR. Environment:
#   MESHOPT_LANE_SCRATCH   private evidence directory (created entries only)
#   MESHOPT_BASELINE       unpacked v0.2.14 source tree
#   MESHOPT_PROOF_INPUTS   frozen peer inputs (<case>.bin)
#   MESHOPT_REFERENCE      pinned C++ meshoptimizer checkout
#   PEERS_ART              artifact directory for the p03 C++ differential
# Each step's exit is appended to proof-exits.tsv; a step already logged as 0
# is skipped on restart.
set -euo pipefail
: "${PARITY_RUNNER_ACTIVE:?run through the admission runner}"
[[ ${PARITY_RUNNER_GPU:-} == 0 ]]
: "${CARGO_TARGET_DIR:?set the dedicated lane target directory}"
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1
ROOT=$(cd -- "$(dirname -- "$0")/../.." && pwd)
LANE=$ROOT/parity/0215-toward-cpp
S=${MESHOPT_LANE_SCRATCH:?}
B=${MESHOPT_BASELINE:?}
I=${MESHOPT_PROOF_INPUTS:?}
: "${MESHOPT_REFERENCE:?}" "${PEERS_ART:?}"
cd "$ROOT"
mkdir -p "$S"
python3 "$LANE/bind.py"
run() {
  local name=$1 rc=0
  shift
  if grep -q "^$name	0$" "$S/proof-exits.tsv" 2>/dev/null; then return 0; fi
  "$@" > "$S/proof-$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" >> "$S/proof-exits.tsv"
  tail -n 3 "$S/proof-$name.log"
  [[ $rc == 0 ]] || exit "$rc"
}
run test-all cargo test --locked --all-features
run test-no-default cargo test --locked --no-default-features
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run fmt cargo fmt -- --check
run boundary bash parity/check-unsafe-boundary.sh
run public-text bash parity/tools/check_public_text.sh

# A separately named v0.2.14 crate for in-process comparison.
if [[ ! -d "$S/proof-base" ]]; then
  cp -r "$B" "$S/proof-base"
  sed -i '0,/name = "meshoptimizer-rs"/s//name = "meshoptimizer-baseline"/' "$S/proof-base/Cargo.toml"
fi
crate() { # crate DIR NAME BIN-SOURCES... : one proof crate with both versions
  local dir=$1 name=$2
  shift 2
  mkdir -p "$dir"
  {
    printf '[package]\nname = "%s"\nversion = "0.0.0"\nedition = "2021"\n' "$name"
    printf '[features]\ndefault = ["std"]\nstd = ["candidate/std", "baseline/std", "candidate/simd", "baseline/simd"]\n'
    printf '[dependencies]\n'
    printf 'candidate = { package = "meshoptimizer-rs", path = "%s", default-features = false, features = ["clusterlod"] }\n' "$ROOT"
    printf 'baseline = { package = "meshoptimizer-baseline", path = "%s", default-features = false, features = ["clusterlod"] }\n' "$S/proof-base"
    for bin in "$@"; do
      printf '[[bin]]\nname = "%s"\npath = "%s"\n' "${bin%%=*}" "${bin#*=}"
    done
    printf '[profile.release]\nlto = "thin"\ncodegen-units = 1\ndebug-assertions = true\n'
  } > "$dir/Cargo.toml"
  [[ -f "$dir/Cargo.lock" ]] || cp "$ROOT/parity/peers/Cargo.lock" "$dir/Cargo.lock"
}
crate "$S/exact" lane-exact "exact=$LANE/exact.rs" "fuzz=$LANE/fuzz.rs" "geometry-exact=$LANE/geometry-exact.rs"
build_exact() {
  cargo build --offline --release --manifest-path "$S/exact/Cargo.toml" "${flags[@]}"
  cp "$CARGO_TARGET_DIR/release/exact" "$S/exact-$mode"
  cp "$CARGO_TARGET_DIR/release/fuzz" "$S/fuzz-$mode"
  cp "$CARGO_TARGET_DIR/release/geometry-exact" "$S/geometry-exact-$mode"
}
for mode in std no-std; do
  flags=()
  [[ $mode == no-std ]] && flags+=(--no-default-features)
  run "exact-build-$mode" build_exact
  run "exact-$mode" "$S/exact-$mode"
  run "geometry-exact-$mode" "$S/geometry-exact-$mode"
done

# The 0.2.14 meshlet and cluster-LOD exact driver, compiled once against each
# version (work limits 0..success+1 on small meshes, every option).
build_meshlets() {
  cargo build --offline --release --manifest-path "$dir/Cargo.toml" "${flags[@]}"
  cp "$CARGO_TARGET_DIR/release/meshlets-exact" "$S/meshlets-exact-$side-$mode"
}
for side in baseline candidate; do
  for mode in std no-std; do
    dir=$S/meshlets-$side
    mkdir -p "$dir"
    path=$ROOT
    package=meshoptimizer-rs
    if [[ $side == baseline ]]; then path=$S/proof-base; package=meshoptimizer-baseline; fi
    printf '[package]\nname = "meshlets-exact"\nversion = "0.0.0"\nedition = "2021"\n[features]\ndefault = ["std"]\nstd = ["meshoptimizer_rs/std", "meshoptimizer_rs/simd"]\n[dependencies]\nmeshoptimizer_rs = { package = "%s", path = "%s", default-features = false, features = ["clusterlod"] }\n[[bin]]\nname = "meshlets-exact"\npath = "%s"\n[profile.release]\nlto = "thin"\ncodegen-units = 1\n' \
      "$package" "$path" "$ROOT/parity/0215-contract-meshlets/exact.rs" > "$dir/Cargo.toml"
    [[ -f "$dir/Cargo.lock" ]] || cp "$ROOT/parity/peers/Cargo.lock" "$dir/Cargo.lock"
    flags=()
    [[ $mode == no-std ]] && flags+=(--no-default-features)
    run "meshlets-build-$side-$mode" build_meshlets
    run "meshlets-run-$side-$mode" sh -c "'$S/meshlets-exact-$side-$mode' > '$S/meshlets-$side-$mode.txt'"
  done
done
for mode in std no-std; do
  run "meshlets-exact-$mode" cmp "$S/meshlets-baseline-$mode.txt" "$S/meshlets-candidate-$mode.txt"
done

# C++ 1.3 differentials: vertex cache and decode (meshopt 0.7.0 vendored
# 1.3), then the pinned-reference meshlet and cluster-LOD driver.
mkdir -p "$S/cpp"
printf '[package]\nname = "lane-cpp"\nversion = "0.0.0"\nedition = "2021"\n[dependencies]\nmeshoptimizer_rs = { package = "meshoptimizer-rs", path = "%s" }\nmeshopt = "=0.7.0"\n[[bin]]\nname = "cpp-differential"\npath = "%s"\n[profile.release]\nlto = "thin"\ncodegen-units = 1\n' \
  "$ROOT" "$LANE/cpp-differential.rs" > "$S/cpp/Cargo.toml"
[[ -f "$S/cpp/Cargo.lock" ]] || cp "$ROOT/parity/peers/Cargo.lock" "$S/cpp/Cargo.lock"
build_cpp() {
  cargo build --offline --release --manifest-path "$S/cpp/Cargo.toml"
  cp "$CARGO_TARGET_DIR/release/cpp-differential" "$S/cpp-differential"
}
run cpp-build build_cpp
run cpp-cache-decode "$S/cpp-differential" "$I"
run reference bash parity/check-reference.sh
run cpp-meshlets-clod python3 parity/0215-contract-meshlets/differential.py

# Five CPU-minutes of differential fuzz (std build), resumable batches.
export FUZZ_BINARY=$S/fuzz-std
run fuzz env PEERS_ART="$S" python3 parity/0215-contract-meshlets/fuzz.py
sha256sum "$S"/exact-* "$S"/fuzz-* "$S"/meshlets-exact-* "$S/cpp-differential" > "$S/proof-binaries.sha256"
