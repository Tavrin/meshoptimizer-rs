#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?admission required}"
S=/mnt/linux-extra/moss-scratch/meshopt-0210-H2
export MESHOPT_ARTIFACTS="$S/fuzz-artifacts" MESHOPT_RESULTS="$S/fuzz-results" MESHOPT_FUZZ_CPUS=9
exec bash parity/fuzz.sh --target overdraw --target simplify_scale --cpu-hours-per-target 0.08333333333333334 --jobs 1 --chunk-seconds 60 --seed 20261002
