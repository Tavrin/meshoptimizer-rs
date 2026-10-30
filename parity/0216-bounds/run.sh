#!/usr/bin/env bash
# One admitted stage at a time; external evidence survives target cleanup.
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?}" "${BOUNDS_SCRATCH:?}" "${CARGO_TARGET_DIR:?}"
[[ ${MOSS_HEAVY_GPU:-} == 0 ]]
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 LC_ALL=C GIT_OPTIONAL_LOCKS=0
export PEERS_PERF=${PEERS_PERF:-/usr/lib/linux-tools/6.17.0-22-generic/perf}
stage=$1
shift
stage_log=$(mktemp "$BOUNDS_SCRATCH/stage-$stage.XXXXXX.log")
exec > >(tee "$stage_log") 2>&1
trap 'rc=$?; printf "%s\t%s\t%s\n" "$stage" "$rc" "$stage_log" >> "$BOUNDS_SCRATCH/stage-exits.tsv"' EXIT
case $stage in
  build)
    cargo fmt --
    python3 parity/0216-bounds/inventory.py
    cargo test --offline --locked --all-features --lib codec::simd::bounds::tests
    cargo clippy --offline --locked --all-features --all-targets -- -D warnings
    python3 parity/0216-bounds/build.py "$@"
    if [[ "$#" == 2 && "$1" == candidate*-default && "$2" == candidate*-scalar ]]; then
      python3 parity/0216-bounds/measure.py "screen-${1%-default}" base-default "$1"
      python3 parity/0216-bounds/gate.py "screen-${1%-default}"
    fi
    ;;
  screen) python3 parity/0216-bounds/measure.py "$@" ;;
  measure)
    python3 parity/0216-bounds/measure.py "$@"
    python3 parity/0216-bounds/results.py "$1"
    ;;
  proof)
    bash parity/0216-bounds/proof.sh
    python3 parity/0216-bounds/differential.py
    cargo build --offline --release --lib --no-default-features --features simd,clusterlod --target aarch64-unknown-linux-gnu
    ;;
  *) printf 'unknown stage: %s\n' "$stage" >&2; exit 2 ;;
esac
