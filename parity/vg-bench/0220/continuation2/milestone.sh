#!/usr/bin/env bash
set -euo pipefail
cargo fmt --check >"$VG_CONTINUATION/fmt.log" 2>&1
cargo test --release --all-features --lib maximum_tests >"$VG_CONTINUATION/maximum-tests.log" 2>&1
cargo build --release --locked --features bevy --manifest-path parity/vg-bench/Cargo.toml >"$VG_CONTINUATION/build.log" 2>&1
cp "$CARGO_TARGET_DIR/release/vg-bench" "$VG_CONTINUATION/candidate-vg-bench"
python3 parity/vg-bench/continue0220.py >"$VG_CONTINUATION/measure.log" 2>&1
python3 "$VG_CONTINUATION/profile.py" >"$VG_CONTINUATION/profile.log" 2>&1
