#!/usr/bin/env bash
set -uo pipefail
: "${MOSS_HEAVY_ACTIVE:?CPU admission required}" "${CARGO_TARGET_DIR:?}"
export LC_ALL=C CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 PYTHONDONTWRITEBYTECODE=1
S=$CARGO_TARGET_DIR/proof-work
run() {
  local name=$1 rc=0
  shift
  /usr/bin/time -v "$@" > "$S/$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" | tee -a parity/0216-par-clod/exits.tsv
  tail -n 3 "$S/$name.log"
  return "$rc"
}
run test-no-std-clusterlod cargo test --locked --no-default-features --features clusterlod || exit $?
run fuzz-source-check cargo check --offline --manifest-path fuzz/parallel-clod/Cargo.toml --bin parallel_clod || exit $?
run prefixes python3 parity/0216-par-clod/prefixes.py || exit $?
