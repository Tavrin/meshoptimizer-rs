#!/usr/bin/env bash
set -euo pipefail
cd "${REPO:?}"
: "${MOSS_HEAVY_ACTIVE:?CPU admission required}"
: "${MOSS_HEAVY_MEASUREMENT:?measurement admission required}"
exec python3 parity/028-L2/campaign.py "${SCRATCH:?}"/meshopt-028-L2/driver-candidate-4 "${SCRATCH:?}"/meshopt-028-L2/driver-base-3 "${SCRATCH:?}"/meshopt-028-L2/driver-M7-3 "${SCRATCH:?}"/meshopt-028-L2/driver-X-3
