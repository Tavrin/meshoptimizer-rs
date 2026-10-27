#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
# report.sh routes --release 0.2 to parity/codec/measure.py and --release 0.4 to measure04.py.
exec parity/report.sh --execute benchmark "$@"
