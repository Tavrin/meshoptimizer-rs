#!/usr/bin/env bash
set -uo pipefail
: "${PARITY_RUNNER_ACTIVE:?run under ${JOB_RUNNER:?}}"
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
export CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0
art="${SCRATCH:?}"/meshopt-peer-c
overall=0
run_gate() {
    local name=$1
    shift
    "$@" > "$art/gate-$name.log" 2>&1
    local code=$?
    printf '%s\t%s\n' "$name" "$code" | tee -a "$art/gates.tsv"
    if [ "$code" -ne 0 ]; then overall=1; fi
}
run_gate fmt cargo fmt --check
run_gate test cargo test --locked --all-features
run_gate clippy cargo clippy --locked --all-targets --all-features -- -D warnings
exit "$overall"
