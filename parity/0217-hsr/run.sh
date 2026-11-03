#!/usr/bin/env bash
# Run this script through a CPU-only admission wrapper; reserve a dedicated target.
set -euo pipefail
: "${CARGO_TARGET_DIR:?}" "${HSR_INPUTS:?}" "${HSR_REFERENCE:?}" "${HSR_PERF:?}"
export LC_ALL=C PYTHONDONTWRITEBYTECODE=1
if [[ ${1:-all} != campaign ]]; then
    python3 parity/0217-hsr/checks.py
fi
if [[ ${1:-all} != checks ]]; then
    python3 parity/0217-hsr/campaign.py
fi
