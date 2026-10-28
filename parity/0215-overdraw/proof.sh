#!/usr/bin/env bash
# Behavioral proof for the 0.2.15 overdraw lane. Run under the CPU-only
# admission runner (PARITY_RUNNER_ACTIVE set, PARITY_RUNNER_GPU=0) with a
# dedicated CARGO_TARGET_DIR. Environment:
#   MESHOPT_OVERDRAW_SCRATCH  private evidence directory (created entries only)
#   MESHOPT_BASELINE          unpacked v0.2.14 source tree
#   MESHOPT_PROOF_INPUTS      peers-format corpus and real overdraw inputs
#   MESHOPT_REFERENCE         pinned C++ meshoptimizer checkout
# Each step's exit is appended to proof-exits.tsv; a step already logged is
# skipped on restart.
set -euo pipefail
: "${PARITY_RUNNER_ACTIVE:?run through the admission runner}"
[[ ${PARITY_RUNNER_GPU:-} == 0 ]]
: "${CARGO_TARGET_DIR:?set the dedicated lane target directory}"
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 PYTHONDONTWRITEBYTECODE=1
ROOT=$(cd -- "$(dirname -- "$0")/../.." && pwd)
S=${MESHOPT_OVERDRAW_SCRATCH:?}
B=${MESHOPT_BASELINE:?}
I=${MESHOPT_PROOF_INPUTS:?}
: "${MESHOPT_REFERENCE:?}"
cd "$ROOT"
mkdir -p "$S"
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

# Exact comparison against a separately named v0.2.14 crate.
if [[ ! -d "$S/proof-base" ]]; then
  cp -r "$B" "$S/proof-base"
  sed -i '0,/name = "meshoptimizer-rs"/s//name = "meshoptimizer-baseline"/' "$S/proof-base/Cargo.toml"
fi
mkdir -p "$S/proof" "$S/proof-inputs" "$S/empty"
for name in tiny-smooth medium-smooth medium-seams medium-sparse million-smooth \
            real-214e-overdraw real-a082-overdraw real-b6fd-overdraw; do
  ln -sf "$I/$name.bin" "$S/proof-inputs/$name.bin"
done
cat > "$S/proof/Cargo.toml" <<MANIFEST
[package]
name = "overdraw-exact"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std"]
[dependencies]
candidate = { package = "meshoptimizer-rs", path = "$ROOT", default-features = false }
baseline = { package = "meshoptimizer-baseline", path = "$S/proof-base", default-features = false }
[[bin]]
name = "overdraw-exact"
path = "$ROOT/parity/0212-overdraw/exact.rs"
[[bin]]
name = "overdraw-extra"
path = "$ROOT/parity/0215-overdraw/exact-extra.rs"
[profile.release]
lto = "thin"
codegen-units = 1
MANIFEST
[[ -f "$S/proof/Cargo.lock" ]] || cargo generate-lockfile --offline --manifest-path "$S/proof/Cargo.toml"
for mode in std no-std; do
  flags=()
  [[ $mode == no-std ]] && flags+=(--no-default-features)
  run "exact-build-$mode" cargo build --offline --locked --release --manifest-path "$S/proof/Cargo.toml" "${flags[@]}"
  cp "$CARGO_TARGET_DIR/release/overdraw-exact" "$S/exact-$mode"
  cp "$CARGO_TARGET_DIR/release/overdraw-extra" "$S/extra-$mode"
  run "exact-$mode" "$S/exact-$mode" "$S/proof-inputs"
  run "extra-$mode" "$S/extra-$mode"
done
sha256sum "$S"/exact-* "$S"/extra-* > "$S/proof-binaries.sha256"

# Pinned C++ differential and five CPU-minutes of exact differential fuzz.
export MESHOPT_ADMISSION_ID=$PARITY_RUNNER_ACTIVE
run cpp python3 parity/0212-overdraw/differential.py
run fuzz python3 parity/0212-overdraw/fuzz.py
