#!/usr/bin/env bash
set -euo pipefail
: "${PARITY_RUNNER_ACTIVE:?invoke through admit.sh after steering}"
[[ ${PARITY_RUNNER_GPU:-} == 0 ]]
[[ ${MESHOPT_SIMPLIFY_TRAIN17_RELEASED:-} == yes ]]
ROOT=$(cd -- "$(dirname -- "$0")/../.." && pwd)
release=${1:?prepare, build, proof, measure, report or audit}
# Child commands retain release-specific logs. The wrapper retains admission exit.
case $release in
  prepare) exec python3 "$ROOT/parity/0212-simplify/prepare.py" ;;
  build) exec python3 "$ROOT/parity/0212-simplify/build.py" ;;
  proof) exec bash "$ROOT/parity/0212-simplify/gates.sh" ;;
  measure) exec python3 "$ROOT/parity/0212-simplify/measure.py" ;;
  report) exec python3 "$ROOT/parity/0212-simplify/results.py" ;;
  audit) exec python3 "$ROOT/parity/0212-simplify/audit.py" ;;
  *) echo 'Unknown release' >&2; exit 64 ;;
esac
