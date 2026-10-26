#!/bin/bash
set -uo pipefail
repo="${REPO:?}"
s="${SCRATCH:?}"/meshopt-memory/optin
cargo fmt --manifest-path "$repo/Cargo.toml" > "$s/format.log" 2>&1
: > "$s/GATES.tsv"
run_gate() {
 local name=$1; shift
 "$@" > "$s/gate-$name.log" 2>&1
 local rc=$?
 printf '%s\t%s\n' "$name" "$rc" >> "$s/GATES.tsv"
}
run_gate test cargo test --manifest-path "$repo/Cargo.toml" --all-features
run_gate clippy cargo clippy --manifest-path "$repo/Cargo.toml" --all-targets --all-features -- -D warnings
run_gate fmt cargo fmt --manifest-path "$repo/Cargo.toml" --check
run_gate focused cargo test --manifest-path "$repo/Cargo.toml" --all-features switching_chain_family_releases_scratch_and_preserves_results
cat "$s/GATES.tsv"
