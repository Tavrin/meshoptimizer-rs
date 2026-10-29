#!/usr/bin/env bash
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
if [[ "$1" == qualify ]]; then
    python3 "$repo/parity/0215-update-spatial/run.py" prepare
    python3 "$repo/parity/0215-update-spatial/run.py" build
    python3 "$repo/parity/0215-update-spatial/run.py" exact
else
    exec python3 "$repo/parity/0215-update-spatial/run.py" "$@"
fi
