#!/bin/bash
set -uo pipefail
s=/mnt/linux-extra/moss-scratch/meshopt-simplify-perf/release-v2
python3 "$s/campaign.py" "${1:-C}" "${2:-1}"
