#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
# report.sh routes --phase 0.2 to parity/codec/measure.py and --phase 0.4 to measure04.py.
exec parity/report.sh --execute benchmark "$@"
