#!/usr/bin/env bash
set -euo pipefail
: "${MESHOPT_ADMISSION_ID:?all builds and validation require <job-runner>}"
[[ ${MESHOPT_GPU:-} == 0 ]]
ROOT=$(cd -- "$(dirname -- "$0")/../.." && pwd)
S=${MESHOPT_OVERDRAW_SCRATCH:?set the lane scratch directory}
cd "$ROOT"
python3 - <<'PY_GATE'
from pathlib import Path
import json,os
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
(A/'gate-entry.json').write_text(json.dumps({'admission':os.environ['MESHOPT_ADMISSION_ID'],'gpu':os.environ['MESHOPT_GPU'],'target':os.environ['CARGO_TARGET_DIR']},indent=2)+'\n')
PY_GATE
run() {
  local name=$1 rc=0
  shift
  [[ ! -e "$S/gate-$name.log" ]]
  "$@" > "$S/gate-$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" >> "$S/gate-exits.tsv"
  [[ $rc == 0 ]] || exit "$rc"
}
run test-all cargo test --locked --all-features
run test-no-default cargo test --locked --no-default-features
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run fmt cargo fmt -- --check
run boundary bash parity/check-unsafe-boundary.sh
for mode in std no-std; do
  flags=()
  if [[ $mode == no-std ]]; then flags+=(--no-default-features); fi
  # The isolated consumer has its own generated lockfile; create it once, then
  # demand --locked on all builds (library dependency locks remain unchanged).
  if [[ ! -f "$S/proof/Cargo.lock" ]]; then
    run exact-lock cargo generate-lockfile --offline --manifest-path "$S/proof/Cargo.toml"
  fi
  run "exact-build-$mode" cargo build --offline --locked --release --manifest-path "$S/proof/Cargo.toml" "${flags[@]}"
  cp "$CARGO_TARGET_DIR/release/overdraw-exact" "$S/exact-$mode"
  run "exact-$mode" "$S/exact-$mode" "$S/proof-inputs"
done
run cpp python3 "$ROOT/parity/0212-overdraw/differential.py"
run fuzz python3 "$ROOT/parity/0212-overdraw/fuzz.py"
