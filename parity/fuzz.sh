#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
: "${CARGO_TARGET_DIR:?set an isolated CARGO_TARGET_DIR}"
exec python3 fuzz/smoke.py "$@"
