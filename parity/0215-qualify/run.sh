#!/usr/bin/env bash
# Combined qualification of the 0.2.15 train (SPEC 0215-qualify).
# Run each stage inside the CPU-only admission runner, whose adapter exports
# PARITY_RUNNER_ACTIVE and PARITY_RUNNER_GPU=0:
#   bash parity/0215-qualify/run.sh STAGE...
# Environment:
#   QUALIFY_SCRATCH  private evidence directory (base/, base0/, inputs/, logs)
#   CARGO_TARGET_DIR dedicated target, deleted after the qualification
#   MESHOPT_REFERENCE pinned C++ meshoptimizer 1.3 checkout
#   QUALIFY_CORPUS   frozen peer-screen source corpus (*.bin, *-simplify.bin)
#   QUALIFY_SPATIAL_DRIVERS the S4 lane's derived meshlet.rs and quadrics.rs
#                    (sha256 bound in parity/0215-update-spatial/proof-binding.json)
# base0/ is `git archive v0.2.14`; base/ is the same with the package renamed
# meshoptimizer-baseline; inputs/ holds the eight frozen peer inputs and the
# three real overdraw inputs (sha256 in inputs.json). Every step appends
# "name<TAB>exit<TAB>seconds" to $S/exits.tsv; a stage stops at its first failure.
set -uo pipefail
: "${PARITY_RUNNER_ACTIVE:?run through the admission runner}"
[[ ${PARITY_RUNNER_GPU:-} == 0 ]] || { echo "PARITY_RUNNER_GPU=0 required"; exit 2; }
W=$(cd -- "$(dirname -- "$0")/../.." && pwd)
Q=$W/parity/0215-qualify
S=${QUALIFY_SCRATCH:?}
: "${CARGO_TARGET_DIR:?}" "${MESHOPT_REFERENCE:?}"
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2} LC_ALL=C PYTHONDONTWRITEBYTECODE=1
PC=${QUALIFY_CORPUS:?}
FEATURES_ALL='["std", "simd", "parallel", "clusterlod", "experimental", "parity-internals"]'
cd "$W"
mkdir -p "$S/logs"

floor() {
  local avail
  avail=$(df -Pk "$S" | awk 'NR==2 {print $4}')
  (( avail >= 104857600 )) || { echo "disk floor: under 100 GiB free"; exit 75; }
}
rec() {
  local name=$1 rc=0 t0 t1
  shift
  t0=$(date +%s)
  "$@" > "$S/logs/$name.log" 2>&1 || rc=$?
  t1=$(date +%s)
  printf '%s\t%s\t%ss\n' "$name" "$rc" "$((t1 - t0))" | tee -a "$S/exits.tsv"
  tail -n 2 "$S/logs/$name.log"
  (( rc == 0 )) || exit "$rc"
}
# exact_crate DIR NAME MODE BIN=PATH... : a baseline/candidate consumer.
exact_crate() {
  local dir=$1 name=$2 mode=$3 features='[]'
  shift 3
  [[ $mode == all ]] && features=$FEATURES_ALL
  mkdir -p "$dir"
  {
    printf '[package]\nname = "%s"\nversion = "0.0.0"\nedition = "2021"\n' "$name"
    printf '[dependencies]\n'
    printf 'baseline = { package = "meshoptimizer-baseline", path = "%s", default-features = false, features = %s }\n' "$S/base" "$features"
    printf 'candidate = { package = "meshoptimizer-rs", path = "%s", default-features = false, features = %s }\n' "$W" "$features"
    for bin in "$@"; do
      printf '[[bin]]\nname = "%s"\npath = "%s"\n' "${bin%%=*}" "${bin#*=}"
    done
    printf '[profile.release]\ncodegen-units = 1\nlto = "thin"\ndebug-assertions = true\n'
  } > "$dir/Cargo.toml"
}
build_exact() { # DIR MODE BIN...
  local dir=$1 mode=$2
  shift 2
  floor
  cargo generate-lockfile --offline --manifest-path "$dir/Cargo.toml" || return
  cargo build --offline --locked --release --manifest-path "$dir/Cargo.toml" || return
  for bin in "$@"; do cp "$CARGO_TARGET_DIR/release/$bin" "$dir/$bin-$mode" || return; done
  sha256sum "$dir"/*-"$mode" >> "$S/binaries.sha256"
}
# exact_stage NAME RUNARGS... with BINS set: build both modes, run each binary.
lane_exact() {
  local name=$1 mode
  shift
  for mode in all none; do
    exact_crate "$S/$name-$mode" "$name" "$mode" "${BINS[@]}"
    rec "$name-build-$mode" build_exact "$S/$name-$mode" "$mode" "${BINS[@]%%=*}"
    for bin in "${BINS[@]%%=*}"; do
      rec "$name-$bin-$mode" env "${RUN_ENV[@]}" "$S/$name-$mode/$bin-$mode" "${RUN_ARGS[@]}"
    done
  done
}

for stage in "$@"; do case $stage in
gates)
  rec test-all cargo test --locked --all-features
  rec test-none cargo test --locked --no-default-features
  rec clippy cargo clippy --locked --all-features --all-targets -- -D warnings
  rec fmt cargo fmt --all -- --check
  rec boundary bash parity/check-unsafe-boundary.sh
  rec public-text bash parity/tools/check_public_text.sh;;
corpus)
  # LOD chain identity (E5): the lane's runner fixtures, then the frozen
  # simplify corpus including the three real meshes (release profile).
  rm -rf "$S/lod-corpus" "$S/lod-real"; mkdir -p "$S/lod-real"
  rec lod-corpus-export python3 parity/0215-lod-chain/corpus.py "$S/lod-corpus"
  for f in tiny-smooth medium-smooth medium-seams medium-sparse million-smooth real-214e-simplify real-a082-simplify real-b6fd-simplify; do
    ln -s "$PC/$f.bin" "$S/lod-real/$f.bin"
  done
  for mode in all none; do
    flag=--all-features; [[ $mode == none ]] && flag=--no-default-features
    rec "lod-corpus-$mode" env MESHOPT_LOD_CORPUS="$S/lod-corpus" cargo test --locked $flag --test lod_chain corpus_identity -- --ignored --exact --nocapture
    rec "lod-real-$mode" env MESHOPT_LOD_CORPUS="$S/lod-real" cargo test --release --locked $flag --test lod_chain corpus_identity -- --ignored --exact --nocapture
    # Cache tables (E2): default table equals optimize_vertex_cache on the corpus.
    rec "cache-corpus-$mode" env MESHOPT_CACHE_CORPUS="$PC" cargo test --release --locked $flag --test api default_vertex_score_table_identity_on_corpus -- --ignored --exact --nocapture
  done;;
targeted)
  BINS=("merge-exact=$Q/merge-exact.rs" "overdraw-huge=$Q/overdraw-huge.rs"); RUN_ENV=(QUALIFY=1); RUN_ARGS=()
  lane_exact targeted;;
simplify-exact)
  # S6 / combined simplify driver (also reaches S1, S3, S4 and E5 changes).
  BINS=("simplify-exact=$W/parity/0216-qualify/simplify-exact.rs"); RUN_ENV=(QUALIFY=1); RUN_ARGS=()
  lane_exact simplify;;
s1-exact)
  # S1 (#61): families 1 and 2 of its widened driver, and its targeted grid.
  BINS=("s1-simplify=$W/parity/0217-optimesh/simplify-exact.rs"); RUN_ENV=(MESHOPT_FAMILIES=1,2); RUN_ARGS=()
  lane_exact s1
  BINS=("s1-targeted=$W/parity/0217-optimesh/targeted-attributes.rs"); RUN_ENV=(QUALIFY=1); RUN_ARGS=(0 1000000000)
  lane_exact s1t;;
prune-points)
  # S3 (#62): contract-simplify observation machinery + the lane's domain.
  src=$W/parity/0215-contract-simplify/exact.rs
  mkdir -p "$S/prune-points-src"
  python3 - "$src" "$W/parity/0215-prune-points/exact-main.rs" "$S/prune-points-src/exact.rs" <<'PY'
import sys
s = open(sys.argv[1]).read()
open(sys.argv[3], 'w').write(s[:s.index('// Monotone multiplication')] + open(sys.argv[2]).read())
PY
  BINS=("prune-points=$S/prune-points-src/exact.rs"); RUN_ENV=(QUALIFY=1); RUN_ARGS=()
  lane_exact prune-points;;
update-spatial)
  # S4 (#63): the lane's derived drivers (hashes bound in its proof-binding.json).
  U=${QUALIFY_SPATIAL_DRIVERS:?}
  BINS=("spatial=$U/meshlet.rs"); RUN_ENV=(QUALIFY=1); RUN_ARGS=()
  lane_exact spatial
  BINS=("update=$U/quadrics.rs"); RUN_ARGS=(0 12288)
  lane_exact update;;
overdraw)
  # S2 (#56): the lane's proof (exact std/no_std, extra cases, C++, fuzz).
  floor
  rec overdraw-proof env MESHOPT_OVERDRAW_SCRATCH="$S/overdraw" MESHOPT_BASELINE="$S/base0" \
    MESHOPT_PROOF_INPUTS="$S/inputs" bash parity/0215-overdraw/proof.sh;;
toward-cpp)
  # S5 (#64): exact, geometry, inherited meshlet/LOD, C++ cache/decode and
  # meshlet/LOD differentials, differential fuzz.
  floor
  mkdir -p "$S/toward-cpp-art"
  rec toward-cpp-proof env MESHOPT_LANE_SCRATCH="$S/toward-cpp" MESHOPT_BASELINE="$S/base0" \
    MESHOPT_PROOF_INPUTS="$S/inputs" PEERS_ART="$S/toward-cpp-art" bash parity/0215-toward-cpp/proof.sh;;
basics)
  # S7 (#68): remap family exact driver against v0.2.14 (the lane compared
  # against the train head of its time) plus its seeded fuzz mode.
  floor
  rec basics-proof env MESHOPT_LANE_SCRATCH="$S/basics" MESHOPT_BASELINE="$S/base0" bash parity/0215-basics/proof.sh;;
cpp-simplify)
  # Simplify family, overdraw, cache, fetch and quantization protocol vs C++ 1.3.
  floor
  rm -rf "$S/protocol"; mkdir -p "$S/protocol"; ln -s "$W" "$S/protocol/candidate"
  rec cpp-protocol env MESHOPT_SCRATCH="$S/protocol" MESHOPT_QUALIFY_TARGET="$CARGO_TARGET_DIR" bash parity/0216-qualify/run-protocol-modes.sh;;
*) echo "unknown stage $stage"; exit 2;;
esac; done
