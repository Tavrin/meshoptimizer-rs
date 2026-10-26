#!/bin/bash
set -uo pipefail
repo=/home/etienne/dev/meshoptimizer-rs-wt/memory
s=/mnt/linux-extra/moss-scratch/meshopt-memory/optin-fix1
run_gate() {
 local name=$1; shift
 "$@" > "$s/gate-$name.log" 2>&1
 local rc=$?
 printf '%s\t%s\n' "$name" "$rc" >> "$s/GATES.tsv"
 return "$rc"
}
run_gate focused cargo test --manifest-path "$repo/Cargo.toml" --all-features cache_update_cache_releases_scratch_and_preserves_results || exit $?
run_gate test cargo test --manifest-path "$repo/Cargo.toml" --all-features || exit $?
run_gate clippy cargo clippy --manifest-path "$repo/Cargo.toml" --all-targets --all-features -- -D warnings || exit $?
run_gate fmt cargo fmt --manifest-path "$repo/Cargo.toml" --check || exit $?
run_gate build cargo build -j 2 --release --manifest-path "$s/harness-B/Cargo.toml" --no-default-features --features path-c --bin migration-c || exit $?
cp "$CARGO_TARGET_DIR/release/migration-c" "$s/bin/B"
sha256sum "$s"/bin/* > "$s/BINARIES.sha256"
run_gate instructions python3 "$s/measure.py"
