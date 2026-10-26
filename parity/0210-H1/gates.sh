#!/bin/bash
set -u
S=/mnt/linux-extra/moss-scratch/meshopt-0210-H1
cd /home/etienne/dev/meshoptimizer-rs-wt/hot-cache
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-H1
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
run() {
  name=$1; shift
  "$@" > "$S/gate-$name.log" 2>&1
  r=$?
  echo "$*: $r" | tee -a "$S/gate-exits.txt"
  if [ "$r" != 0 ]; then tail -70 "$S/gate-$name.log"; exit "$r"; fi
}
cargo fmt
run test-all cargo test --locked --all-features
run test-no-default cargo test --locked --no-default-features
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run fmt cargo fmt -- --check
run boundary bash parity/check-unsafe-boundary.sh
python3 parity/0210-H1/prepare-proof.py
for mode in std no-std; do
  flags=()
  if [ "$mode" = no-std ]; then flags+=(--no-default-features); fi
  cargo build --offline --release --manifest-path "$S/proof/Cargo.toml" "${flags[@]}" > "$S/exact-build-$mode.log" 2>&1 || exit $?
  cp "$CARGO_TARGET_DIR/release/hot-cache-exact" "$S/exact-$mode"
  "$S/exact-$mode" "$S/meshopt-028-L2/inputs" > "$S/exact-$mode.log" 2>&1 || { tail -30 "$S/exact-$mode.log"; exit 1; }
  echo exact-$mode=0 | tee -a "$S/gate-exits.txt"
done
python3 parity/0210-H1/proof.py > "$S/cpp-proof.log" 2>&1
r=$?;echo cpp-proof=$r | tee -a "$S/gate-exits.txt";exit "$r"
