#!/usr/bin/env bash
set -uo pipefail
: "${PARITY_RUNNER_ACTIVE:?admission required}"
S="${SCRATCH:?}"/meshopt-0210-H2
overall=0
run() { local name=$1; shift; "$@" > "$S/gate-$name.log" 2>&1; local rc=$?; printf '%s\t%s\n' "$name" "$rc" | tee -a "$S/gates.tsv"; if [ "$rc" -ne 0 ]; then overall=1; fi; }
run final-fmt cargo fmt --check
run final-all-features cargo test --locked --all-features
run final-no-default-features cargo test --locked --no-default-features
run final-clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run final-boundary python3 parity/simd/check_boundary.py
if [ "$overall" -ne 0 ]; then exit "$overall"; fi
run fuzz bash parity/0210-H2/fuzz.sh
exit "$overall"
