#!/usr/bin/env bash
set -euo pipefail
: "${PARITY_RUNNER_ACTIVE:?all builds and validation require <job-runner>}"
[[ ${PARITY_RUNNER_GPU:-} == 0 ]]
ROOT=$(cd -- "$(dirname -- "$0")/../.." && pwd)
S=${MESHOPT_SCRATCH:?set the scratch directory}
cd "$ROOT"
python3 - <<'BIND'
from pathlib import Path
import hashlib
import json
import os
root = Path.cwd()
files = [root/'Cargo.toml', root/'Cargo.lock', *sorted((root/'src').rglob('*.rs'))]
files += [p for p in (root/'parity/0212-simplify2').iterdir() if p.suffix in ['.py', '.rs', '.sh']]
source = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
with (Path(os.environ['MESHOPT_SCRATCH'])/'proof-source.json').open('x') as stream:
    stream.write(json.dumps(source, indent=2)+'\n')
BIND
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
  cp "$CARGO_TARGET_DIR/release/simplify-exact" "$S/exact-$mode"
  run "exact-$mode" "$S/exact-$mode"
done
run cpp python3 "$ROOT/parity/0212-simplify2/differential.py"
run fuzz python3 "$ROOT/parity/0212-simplify2/fuzz.py"
