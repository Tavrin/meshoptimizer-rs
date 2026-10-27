#!/bin/bash
set -euo pipefail
bash "${SCRATCH:?}"/meshopt-memory/optin/build-probes.sh
exec python3 "${SCRATCH:?}"/meshopt-memory/optin/measure.py
