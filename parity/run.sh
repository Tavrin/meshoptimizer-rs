#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
# report.sh routes --phase 0.2 to parity/codec/runner.py and --phase 0.4 to runner04.py.
exec parity/report.sh --execute run "$@"
