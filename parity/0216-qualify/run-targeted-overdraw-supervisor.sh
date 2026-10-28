#!/usr/bin/env bash
set -euo pipefail
scratch=${MESHOPT_SCRATCH:?}
out="$scratch/proof/0216d-overdraw-supervisor"
mkdir -p "$out"
for mode in all none; do
    avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}'); (( avail >= 104857600 )) || exit 75
    bin="$scratch/proof/0216b-affected/overdraw-$mode"
    sha256sum "$bin" >> "$out/binaries.sha256"
    set +e
    (ulimit -t 600; /usr/bin/time -p "$bin" "$scratch/proof/small-inputs") > "$out/run-$mode.log" 2>&1
    rc=$?
    set -e
    printf '%s %s\n' "$mode" "$rc" >> "$out/exits.txt"
    (( rc == 0 )) || exit "$rc"
done
