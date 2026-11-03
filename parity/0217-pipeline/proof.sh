#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under CPU admission}"
exec python3 parity/0217-pipeline/proof.py
