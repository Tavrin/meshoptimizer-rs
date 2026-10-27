#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under ${JOB_RUNNER:?}}"
export MESHOPT_ARTIFACTS="${SCRATCH:?}"/meshopt-028-L2/fuzz-artifacts
export MESHOPT_RESULTS="${SCRATCH:?}"/meshopt-028-L2/fuzz-results
export MESHOPT_FUZZ_CPUS=9 CARGO_BUILD_JOBS=1
export RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS=
exec bash parity/fuzz.sh --target simplify_scale --target overdraw --cpu-hours-per-target 0.08333333333333334 --jobs 1 --chunk-seconds 60 --seed 20261002
