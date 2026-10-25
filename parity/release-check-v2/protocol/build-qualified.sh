#!/bin/bash
set -euo pipefail
s=/mnt/linux-extra/moss-scratch/meshopt-simplify-perf/release-v2
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUSTFLAGS= RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER=
unset CARGO_ENCODED_RUSTFLAGS
for v in Bp Cp; do
 export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/meshopt-rcv2-$v
 export RUSTFLAGS=
 if [[ "$v" == Bp || "$v" == Cp ]]; then export RUSTFLAGS='-C llvm-args=-align-all-functions=6'; fi
 cd "$s/src-$v"
 cargo build --manifest-path parity/Cargo.toml --offline --locked --release --bin meshopt-driver > "$s/build-$v-qualified.log" 2>&1
 cp "$CARGO_TARGET_DIR/release/meshopt-driver" "$s/driver-$v"
 sha256sum "$s/driver-$v" >> "$s/binaries.sha256"
done
