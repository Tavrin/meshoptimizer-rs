#!/bin/bash
set -euo pipefail
s="${SCRATCH:?}"/meshopt-memory
mkdir -p "$s/bin"
cp "${CARGO_TARGET_DIR:?}"/release/migration-c "$s/bin/"
cargo build --release --manifest-path "$s/harness/Cargo.toml" --no-default-features --features path-b --bin migration-b > "$s/build-cpp.log" 2>&1
cp "${CARGO_TARGET_DIR:?}"/release/migration-b "$s/bin/"
exec "$s/step1.py"
