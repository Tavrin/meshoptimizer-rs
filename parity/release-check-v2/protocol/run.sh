#!/bin/bash
set -uo pipefail
s="${SCRATCH:?}"/meshopt-simplify-perf/release-v2
python3 "$s/campaign.py" "${1:-C}" "${2:-1}"
