#!/bin/bash
set -euo pipefail
s=/mnt/linux-extra/moss-scratch/meshopt-memory
mkdir -p "$s/bin"
cp /mnt/linux-extra/moss-cargo-targets/meshopt-memory/release/migration-c "$s/bin/"
cargo build --release --manifest-path "$s/harness/Cargo.toml" --no-default-features --features path-b --bin migration-b > "$s/build-cpp.log" 2>&1
cp /mnt/linux-extra/moss-cargo-targets/meshopt-memory/release/migration-b "$s/bin/"
exec "$s/step1.py"
