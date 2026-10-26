#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?admission required}"
python3 parity/0210-H2/preserve-short-reader.py
bash parity/0210-H2/gates.sh
export H2_BUILD=1
unset H2_PILOT H2_SAMPLES
bash parity/0210-H2/instructions.sh
