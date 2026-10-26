#!/usr/bin/env bash
set -euo pipefail
cd /home/etienne/dev/meshoptimizer-rs-wt/clod-path
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-L1
export CARGO_BUILD_JOBS=1 RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
export PYTHONDONTWRITEBYTECODE=1
case "$1" in
  diagnose)
    python3 parity/028-L1/control.py build original2 ours M7 X
    python3 parity/028-L1/control.py counts original2 ours M7 X
    python3 parity/028-L1/control.py build equalized ours M7 X
    python3 parity/028-L1/control.py counts equalized ours M7 X
    ;;
  *) shift; exec "$@" ;;
esac
