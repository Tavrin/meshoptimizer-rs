#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
# report.sh routes 0.2 and 0.4 to codec drivers, and 0.3 to the meshlet driver.
exec parity/report.sh --execute run "$@"
