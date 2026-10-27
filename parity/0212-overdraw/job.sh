#!/usr/bin/env bash
set -euo pipefail
: "${MESHOPT_ADMISSION_ID:?run through the admission runner}"
[[ ${MESHOPT_GPU:-} == 0 ]]
ROOT=$(cd -- "$(dirname -- "$0")/../.." && pwd)
S=${MESHOPT_OVERDRAW_SCRATCH:?set lane scratch directory}
cd "$ROOT"
run() {
  local name=$1 rc=0
  shift
  [[ ! -e "$S/job-$name.log" ]]
  "$@" > "$S/job-$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" >> "$S/job-exits.tsv"
  tail -n 8 "$S/job-$name.log"
  [[ $rc == 0 ]] || exit "$rc"
}
case ${1:-} in
  final)
    run build python3 parity/0212-overdraw/build.py final
    cp "$S/final-driver-base" "$S/packed-pilot-driver-base"
    cp "$S/final-driver-candidate" "$S/packed-pilot-driver-candidate"
    # Diagnostic only: preserve rejection, but complete the last registered gate.
    (run packed-precheck python3 parity/0212-overdraw/measure.py packed-pilot) || true
    run proof bash parity/0212-overdraw/gates.sh
    run measure python3 parity/0212-overdraw/measure.py final
    result_rc=0
    (run results python3 parity/0212-overdraw/results.py final) || result_rc=$?
    run audit python3 parity/0212-overdraw/audit.py
    run cleanup python3 parity/0212-overdraw/cleanup.py
    if [[ $result_rc == 0 ]]; then
      run report python3 parity/0212-overdraw/report.py
    fi
    exit "$result_rc"
    ;;
  *) echo 'usage: job.sh final' >&2; exit 2 ;;
esac
