#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
# Complete bounded chunks on SIGTERM; the common driver retains exact replay
# ranges and measured process CPU time. Explicit arguments override defaults.
exec nice -n 19 parity/fuzz.sh --continuous --wall-seconds 28800 \
  --cpu-hours-per-target 1000000000 --cores 4 --chunk-seconds 60 "$@"
