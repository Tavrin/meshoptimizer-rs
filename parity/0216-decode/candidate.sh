#!/usr/bin/env bash
set -euo pipefail
unset CARGO_ENCODED_RUSTFLAGS
cargo fmt
python3 parity/simd/update_audit.py
python3 parity/0216-decode/build.py candidate
DECODE_PHASE=candidate DECODE_ARMS='base candidate cpp-scalar cpp-default' python3 parity/0216-decode/measure.py
python3 parity/0216-decode/results.py
cargo check --locked --all-features --all-targets --target aarch64-unknown-linux-gnu
