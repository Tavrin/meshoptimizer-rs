#!/usr/bin/env bash
set -euo pipefail
unset CARGO_ENCODED_RUSTFLAGS
cargo fmt
python3 parity/simd/update_audit.py
python3 parity/0216-decode/canonical_build.py
DECODE_PHASE=candidate DECODE_ARMS='base control candidate cpp-scalar cpp-default' python3 parity/0216-decode/measure.py
python3 parity/0216-decode/results.py
python3 parity/0216-decode/verdict.py
export DECODE_ART="$(dirname "$DECODE_ART")/proof-final"
export DECODE_SKIP_BASELINE=1
if [[ -z "${MESHOPT_REFERENCE:-}" ]]; then
  MESHOPT_REFERENCE=$(python3 -c 'import pathlib,subprocess; print(pathlib.Path(subprocess.check_output(["git","rev-parse","--path-format=absolute","--git-common-dir"],text=True).strip()).parents[1]/"refs/meshoptimizer")')
  export MESHOPT_REFERENCE
fi
python3 parity/0216-decode/proof.py
python3 parity/0216-decode/msrv.py
