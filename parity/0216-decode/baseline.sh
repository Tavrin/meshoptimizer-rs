#!/usr/bin/env bash
set -euo pipefail
unset CARGO_ENCODED_RUSTFLAGS
mkdir -p "$DECODE_ART"
python3 parity/0216-decode/build.py base cpp-scalar cpp-default
cp parity/peers/inputs.json "$DECODE_ART/inputs.json"
[[ -e "$DECODE_ART/inputs" ]] || ln -s "$DECODE_INPUTS" "$DECODE_ART/inputs"
python3 parity/0216-decode/measure.py
cargo build --offline --locked --release --manifest-path parity/simd/Cargo.toml --target wasm32-unknown-unknown --lib
# Compile the actual SIMD artifact separately, then retain both for Node execution.
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/meshopt_simd_qualification.wasm" "$DECODE_ART/wasm-scalar.wasm"
RUSTFLAGS='-C target-feature=+simd128' cargo build --offline --locked --release --manifest-path parity/simd/Cargo.toml --target wasm32-unknown-unknown --lib
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/meshopt_simd_qualification.wasm" "$DECODE_ART/wasm-base.wasm"
if [[ -z "${MESHOPT_REFERENCE:-}" ]]; then
  MESHOPT_REFERENCE=$(python3 -c 'import pathlib,subprocess; print(pathlib.Path(subprocess.check_output(["git","rev-parse","--path-format=absolute","--git-common-dir"],text=True).strip()).parents[1]/"refs/meshoptimizer")')
  export MESHOPT_REFERENCE
fi
bash parity/check-reference.sh
python3 parity/0216-decode/wasm.py
