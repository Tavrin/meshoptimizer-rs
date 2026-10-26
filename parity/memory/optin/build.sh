#!/bin/bash
set -euo pipefail
s=/mnt/linux-extra/moss-scratch/meshopt-memory/optin
for arm in A B; do
 cargo build -j 2 --release --manifest-path "$s/harness-$arm/Cargo.toml" --no-default-features --features path-c --bin migration-c > "$s/build-$arm.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/migration-c" "$s/bin/$arm"
 cargo build -j 2 --release --manifest-path "$s/probe-harness-$arm/Cargo.toml" --no-default-features --features path-c --bin migration-c > "$s/build-probe-$arm.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/migration-c" "$s/bin/probe-$arm"
done
sha256sum "$s"/bin/* > "$s/BINARIES.sha256"
