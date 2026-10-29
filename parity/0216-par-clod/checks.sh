#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?CPU admission required}" "${CARGO_TARGET_DIR:?}"
export LC_ALL=C CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 PYTHONDONTWRITEBYTECODE=1
LANE=parity/0216-par-clod
S=$CARGO_TARGET_DIR/proof-work
mkdir -p "$S"
run() {
  local name=$1 rc=0
  shift
  "$@" > "$S/$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" | tee -a "$LANE/exits.tsv"
  tail -n 3 "$S/$name.log"
  return "$rc"
}
run fmt cargo fmt --all -- --check || exit $?
run test-all cargo test --locked --all-features || exit $?
run test-no-default cargo test --locked --no-default-features || exit $?
run test-scalar-parallel cargo test --locked --no-default-features --features parallel,clusterlod --test clusterlod_parallel || exit $?
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings || exit $?
run unsafe-boundary bash parity/check-unsafe-boundary.sh || exit $?
run build-drivers python3 "$LANE/build.py" || exit $?
run corpus-export python3 parity/0215-lod-chain/corpus.py "$S/lod-corpus" || exit $?
mkdir -p "$S/real-corpus"
for name in tiny-smooth medium-smooth medium-seams medium-sparse million-smooth real-214e real-a082 real-b6fd; do
  ln -sf "$MESHOPT_INPUTS/$name.bin" "$S/real-corpus/$name.bin"
done
for mode in simd scalar; do
  run "targeted-$mode" "$S/exact-$mode.bin" || exit $?
  run "lod-corpus-$mode" "$S/exact-$mode.bin" "$S/lod-corpus" || exit $?
  run "real-corpus-$mode" "$S/exact-$mode.bin" "$S/real-corpus" || exit $?
done
run measure python3 "$LANE/measure.py" || exit $?
