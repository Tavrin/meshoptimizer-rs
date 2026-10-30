#!/usr/bin/env bash
# Run fuzz.py under the admission wrapper, with its own script identity so the
# queue reserves what the (light) driver needs rather than the shared python class:
#   FUZZ_SCRATCH=... CARGO_TARGET_DIR=... PARITY_RUNNER_ACTIVE=1 \
#     parity/0216-fuzz/run.sh run [TARGET...]
set -euo pipefail
exec python3 "$(dirname "$0")/fuzz.py" "$@"
