#!/bin/bash
set -u
s=/mnt/linux-extra/moss-scratch/meshopt-peer-a
: > "$s/gate-exits.tsv"
run_gate() {
 local name="$1"
 shift
 "$@" > "$s/gate-$name.log" 2>&1
 local code=$?
 printf '%s\t%s\n' "$name" "$code" >> "$s/gate-exits.tsv"
 return "$code"
}
run_gate test cargo test --all-features
run_gate clippy cargo clippy --all-targets --all-features -- -D warnings
run_gate fmt cargo fmt --all -- --check
run_gate protocol-build bash "$s/build-proof.sh"
run_gate exact python3 parity/peer-quantize/proof.py
