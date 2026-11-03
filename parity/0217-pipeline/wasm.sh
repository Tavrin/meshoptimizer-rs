#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under CPU admission}"
python3 parity/0217-pipeline/wasm.py
bash parity/0217-pipeline/finish-check.sh
