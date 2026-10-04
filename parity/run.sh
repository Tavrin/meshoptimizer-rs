#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
# report.sh routes --phase 0.2 to the codec harness (parity/codec/runner.py).
exec parity/report.sh --execute run "$@"
