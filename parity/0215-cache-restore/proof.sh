#!/usr/bin/env bash
# Bounded cache proof. Run under the CPU admission runner.
set -euo pipefail
: "${PARITY_RUNNER_ACTIVE:?}" "${CARGO_TARGET_DIR:?}" "${RESTORE_SCRATCH:?}" "${RESTORE_CORPUS:?}"
[[ $PARITY_RUNNER_GPU == 0 ]]
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
S=$RESTORE_SCRATCH/proof
mkdir -p "$S"
cd "$ROOT"
run() {
    local name=$1 rc=0
    shift
    "$@" > "$S/$name.log" 2>&1 || rc=$?
    printf '%s\t%s\n' "$name" "$rc" | tee -a "$S/exits.tsv"
    tail -n 3 "$S/$name.log"
    (( rc == 0 )) || exit "$rc"
}
run test-all cargo test --locked --all-features
run test-none cargo test --locked --no-default-features
run clippy-all cargo clippy --locked --all-targets --all-features -- -D warnings
run clippy-none cargo clippy --locked --all-targets --no-default-features -- -D warnings
run fmt cargo fmt --all -- --check
run boundary python3 parity/simd/check_boundary.py
run package-boundary bash -o pipefail -c 'cargo package --locked --allow-dirty --list | python3 parity/simd/check_package.py'
run public-text bash parity/tools/check_public_text.sh
for pair in base:v0.2.14 tables:cc13453; do
    side=${pair%%:*}; ref=${pair#*:}
    mkdir -p "$S/$side"
    git archive "$ref" src docs Cargo.toml Cargo.lock README.md | tar -x -C "$S/$side"
    sed -i "0,/name = \"meshoptimizer-rs\"/s//name = \"meshoptimizer-$side-baseline\"/" "$S/$side/Cargo.toml"
done
for mode in all none; do
    features='["std", "simd", "parallel", "clusterlod", "experimental", "parity-internals"]'
    flags=(--all-features)
    if [[ $mode == none ]]; then features='[]'; flags=(--no-default-features); fi
    run "table-corpus-$mode" env MESHOPT_CACHE_CORPUS="$RESTORE_CORPUS" cargo test --release --locked "${flags[@]}" --test api default_vertex_score_table_identity_on_corpus -- --ignored --exact --nocapture
    for kind in base tables; do
        dir=$S/$kind-$mode
        mkdir -p "$dir"
        driver=$ROOT/parity/0215-toward-cpp/exact.rs
        [[ $kind == tables ]] && driver=$ROOT/parity/0215-cache-restore/table-exact.rs
        cat > "$dir/Cargo.toml" <<EOF
[package]
name = "cache-$kind-$mode"
version = "0.0.0"
edition = "2021"
[dependencies]
baseline = { package = "meshoptimizer-$kind-baseline", path = "$S/$kind", default-features = false, features = $features }
candidate = { package = "meshoptimizer-rs", path = "$ROOT", default-features = false, features = $features }
[[bin]]
name = "cache-$kind-$mode"
path = "$driver"
[profile.release]
lto = "thin"
codegen-units = 1
debug-assertions = true
EOF
        run "lock-$kind-$mode" cargo generate-lockfile --offline --manifest-path "$dir/Cargo.toml"
        run "build-$kind-$mode" cargo build --offline --locked --release --manifest-path "$dir/Cargo.toml"
        cp "$CARGO_TARGET_DIR/release/cache-$kind-$mode" "$dir/driver"
        run "exact-$kind-$mode" "$dir/driver"
    done
done
