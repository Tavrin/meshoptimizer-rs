#!/usr/bin/env bash
# Behavioral proof for the 0.2.16 remap lane. Run under the
# CPU-only admission runner (MOSS_HEAVY_ACTIVE set, MOSS_HEAVY_GPU=0)
# with a dedicated CARGO_TARGET_DIR. Environment:
#   MESHOPT_LANE_SCRATCH   private evidence directory (created entries only)
#   MESHOPT_BASELINE       unpacked train/0.2.15 head tree (the base arm)
# Each step's exit is appended to proof-exits.tsv; a step already logged as 0
# is skipped on restart. The C++ differential uses safe oracle wrappers,
# in addition to the campaign's four-arm per-cell output identity.
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?run through admission}"
[[ ${MOSS_HEAVY_GPU:-} == 0 ]]
: "${CARGO_TARGET_DIR:?set the dedicated lane target directory}"
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1
ROOT=$(cd -- "$(dirname -- "$0")/../.." && pwd)
LANE=$ROOT/parity/0216-remap
S=${MESHOPT_LANE_SCRATCH:?}
B=${MESHOPT_BASELINE:?}
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

# A separately named base crate for in-process comparison.
if [[ ! -d "$S/proof-base" ]]; then
  cp -r "$B" "$S/proof-base"
  sed -i '0,/name = "meshoptimizer-rs"/s//name = "meshoptimizer-baseline"/' "$S/proof-base/Cargo.toml"
fi
mkdir -p "$S/exact"
{
  printf '[package]\nname = "lane-exact"\nversion = "0.0.0"\nedition = "2021"\n'
  printf '[features]\ndefault = ["std"]\nstd = ["candidate/std", "baseline/std", "candidate/simd", "baseline/simd"]\n'
  printf '[dependencies]\n'
  printf 'candidate = { package = "meshoptimizer-rs", path = "%s", default-features = false }\n' "$ROOT"
  printf 'baseline = { package = "meshoptimizer-baseline", path = "%s", default-features = false }\n' "$S/proof-base"
  printf 'meshopt = "=0.7.0"\n'
  printf '[[bin]]\nname = "cpp-differential"\npath = "%s"\n' "$LANE/cpp-differential.rs"
  printf '[[bin]]\nname = "exact"\npath = "%s"\n' "$LANE/exact.rs"
  printf '[profile.release]\nlto = "thin"\ncodegen-units = 1\ndebug-assertions = true\noverflow-checks = true\n'
} > "$S/exact/Cargo.toml"
[[ -f "$S/exact/Cargo.lock" ]] || cp "$ROOT/parity/peers/Cargo.lock" "$S/exact/Cargo.lock"
build_exact() {
  cargo build --offline --release --manifest-path "$S/exact/Cargo.toml" "${flags[@]}"
  cp "$CARGO_TARGET_DIR/release/exact" "$S/exact-$mode"
  cp "$CARGO_TARGET_DIR/release/cpp-differential" "$S/cpp-$mode"
}
for mode in std no-std; do
  flags=()
  [[ $mode == no-std ]] && flags+=(--no-default-features)
  run "exact-build-$mode" build_exact
  run "cpp-$mode" "$S/cpp-$mode"
  run "exact-$mode" "$S/exact-$mode"
  run "fuzz-$mode" "$S/exact-$mode" fuzz 3000 "$([[ $mode == std ]] && echo 21601 || echo 21602)"
done
sha256sum "$S"/exact-std "$S"/exact-no-std > "$S/proof-binaries.sha256"

python3 "$LANE/bind.py" verify
