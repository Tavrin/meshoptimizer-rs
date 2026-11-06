#!/usr/bin/env bash
# Run this script only through the caller's admitted CPU wrapper.
set -euo pipefail
evidence="${1:?evidence directory}"
mkdir -p "$evidence"
status=0
run_gate() {
    local name="$1"; shift
    local code=0
    "$@" > "$evidence/$name.log" 2>&1 || code=$?
    printf '%s %s\n' "$name" "$code" | tee -a "$evidence/gates.txt"
    if ((code != 0)); then status=1; fi
}
run_gate fmt cargo fmt --check
run_gate clippy cargo clippy --all-targets --all-features -- -D warnings
run_gate release cargo test --release
run_gate release-all-features cargo test --release --all-features
run_gate public-text bash parity/tools/check_public_text.sh
run_gate fuzz-build cargo build --manifest-path fuzz/r0220/Cargo.toml --bin dag_format_v2
exit "$status"
