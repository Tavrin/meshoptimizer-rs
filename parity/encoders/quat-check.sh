#!/usr/bin/env bash
set -euo pipefail
bash parity/encoders/fix.sh quat-swizzle
cargo test --offline --locked --test codec04 quaternion_encoder_preserves_first_maximum_nan_and_signed_zero
