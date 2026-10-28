#!/usr/bin/env bash
# Run through the required build admission wrapper with an isolated target.
set -euo pipefail
: "${CARGO_TARGET_DIR:?set an isolated target directory}"
: "${MESHOPT_CACHE_CORPUS:?set frozen corpus directory}"
: "${MESHOPT_CACHE_LOGS:?set retained log directory}"
mkdir -p "$MESHOPT_CACHE_LOGS"
cd "$(git rev-parse --show-toplevel)"
receipts=parity/0215-cache-tables/proof-exits.tsv
printf 'check\texit\n' > "$receipts"
failed=0
run() {
    local name=$1 rc=0
    shift
    "$@" > "$MESHOPT_CACHE_LOGS/$name.log" 2>&1 || rc=$?
    printf '%s\t%s\n' "$name" "$rc" | tee -a "$receipts"
    if (( rc != 0 )); then
        cat "$MESHOPT_CACHE_LOGS/$name.log"
        failed=1
    fi
}
run all-features cargo test --locked --all-features
run no-default-features cargo test --locked --no-default-features
run corpus-all cargo test --locked --all-features --test api default_vertex_score_table_identity_on_corpus -- --ignored --exact --nocapture
run corpus-no-default cargo test --locked --no-default-features --test api default_vertex_score_table_identity_on_corpus -- --ignored --exact --nocapture
run doctests-all cargo test --locked --all-features --doc
run doctests-no-default cargo test --locked --no-default-features --doc
run clippy-all cargo clippy --locked --all-features --all-targets -- -D warnings
run clippy-no-default cargo clippy --locked --no-default-features --all-targets -- -D warnings
run fmt cargo fmt --all -- --check
run boundary python3 parity/simd/check_boundary.py
# Run the package half directly without verification-disabling flags.
run package-boundary bash -o pipefail -c 'cargo package --locked --allow-dirty --list | python3 parity/simd/check_package.py'
run public-text bash parity/tools/check_public_text.sh
run diff-check git diff --check
exit "$failed"
