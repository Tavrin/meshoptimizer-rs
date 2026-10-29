#!/usr/bin/env bash
# Build one "ours" peers driver from a source tree and retain it under a name.
# Usage: build.sh SOURCE_TREE ART_DIR NAME [line-tables]
# Run under the CPU-only admission wrapper with CARGO_TARGET_DIR and
# CARGO_INCREMENTAL=0 set. "line-tables" adds line tables for perf annotate
# only; measured drivers are built without it.
set -euo pipefail
src=$1 art=$2 name=$3 debug=${4:-}
: "${CARGO_TARGET_DIR:?}"
[ "${CARGO_INCREMENTAL:-}" = 0 ]
free=$(df --output=avail -B1G "$CARGO_TARGET_DIR" 2>/dev/null | tail -1 || df --output=avail -B1G "$(dirname "$CARGO_TARGET_DIR")" | tail -1)
if [ "$free" -lt 100 ]; then echo "build floor below 100 GiB" >&2; exit 75; fi
if [ "$debug" = line-tables ]; then export CARGO_PROFILE_RELEASE_DEBUG=line-tables-only; fi
cargo build --offline --locked --release --manifest-path "$src/parity/peers/Cargo.toml" \
    --no-default-features --features ours -j 1
cp "$CARGO_TARGET_DIR/release/meshopt-peers" "$art/driver-$name"
sha256sum "$art/driver-$name"
