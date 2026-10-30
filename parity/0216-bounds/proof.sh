#!/usr/bin/env bash
# CPU-only, admitted final-source proof. Logs live in BOUNDS_SCRATCH/proof.
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?}" "${CARGO_TARGET_DIR:?}" "${BOUNDS_SCRATCH:?}"
[[ ${MOSS_HEAVY_GPU:-} == 0 ]]
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 PYTHONDONTWRITEBYTECODE=1
ROOT=$(cd -- "$(dirname -- "$0")/../.." && pwd)
P=$(mktemp -d "$BOUNDS_SCRATCH/proof.XXXXXX")
printf '%s\n' "$P" > "$BOUNDS_SCRATCH/latest-proof"
run() {
 local name=$1 rc=0
 shift
 /usr/bin/time -f 'user=%U sys=%S elapsed=%e' -o "$P/$name.time" "$@" > "$P/$name.log" 2>&1 || rc=$?
 printf '%s\t%s\n' "$name" "$rc" >> "$P/exits.tsv"
 tail -n 3 "$P/$name.log"
 [[ $rc == 0 ]]
}
cd "$ROOT"
run bind-start python3 "$ROOT/parity/0216-bounds/bind.py" "$P/source-binding.json"
run all cargo test --offline --locked --all-features
run no-default cargo test --offline --locked --no-default-features
run clippy cargo clippy --offline --locked --all-features --all-targets -- -D warnings
run fmt cargo fmt -- --check
run boundary bash parity/check-unsafe-boundary.sh
run public-text bash parity/tools/check_public_text.sh
mkdir -p "$P/base" "$P/exact"
cp -r "$BOUNDS_SCRATCH/base/src" "$P/base/"
cp -r "$BOUNDS_SCRATCH/base/docs" "$P/base/"
cp "$BOUNDS_SCRATCH/base/README.md" "$P/base/"
cp "$BOUNDS_SCRATCH/base/Cargo.toml" "$BOUNDS_SCRATCH/base/Cargo.lock" "$P/base/"
sed -i '0,/name = "meshoptimizer-rs"/s//name = "meshoptimizer-baseline"/' "$P/base/Cargo.toml"
cat > "$P/exact/Cargo.toml" <<EOF
[package]
name = "bounds-exact"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std", "candidate/simd", "baseline/simd"]
[dependencies]
candidate = { package = "meshoptimizer-rs", path = "$ROOT", default-features = false, features = ["clusterlod"] }
baseline = { package = "meshoptimizer-baseline", path = "$P/base", default-features = false, features = ["clusterlod"] }
[[bin]]
name = "exact"
path = "$ROOT/parity/0216-bounds/exact.rs"
[[bin]]
name = "geometry"
path = "$ROOT/parity/0215-toward-cpp/geometry-exact.rs"
[profile.release]
lto = "thin"
codegen-units = 1
debug-assertions = true
overflow-checks = true
EOF
for mode in default scalar; do
 flags=()
 [[ $mode == scalar ]] && flags+=(--no-default-features)
 run "build-exact-$mode" cargo build --offline --release --manifest-path "$P/exact/Cargo.toml" "${flags[@]}"
 cp "$CARGO_TARGET_DIR/release/exact" "$P/exact-$mode"
 run "exact-$mode" "$P/exact-$mode"
 cp "$CARGO_TARGET_DIR/release/geometry" "$P/geometry-$mode"
 run "geometry-$mode" "$P/geometry-$mode"
done
run unsafe-free env RUSTFLAGS=-Funsafe_code cargo check --offline --no-default-features

run bind-end python3 "$ROOT/parity/0216-bounds/bind.py" "$P/source-binding.json" --check
