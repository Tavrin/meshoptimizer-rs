#!/usr/bin/env bash
# Final release checks. Run under CPU admission with the dedicated target.
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?}" "${CARGO_TARGET_DIR:?}" "${RELEASE_SCRATCH:?}"
[[ ${MOSS_HEAVY_GPU:-} == 0 ]]
R=parity/0215-qualify/receipts/release-0215
S=$RELEASE_SCRATCH/checks
mkdir -p "$S"
run() {
    local name=$1 rc=0
    shift
    "$@" > "$S/$name.log" 2>&1 || rc=$?
    printf '%s\t%s\n' "$name" "$rc" | tee -a "$R/check-exits.tsv"
    tail -n 3 "$S/$name.log"
    (( rc == 0 )) || exit "$rc"
}
run test-all cargo test --locked --all-features
run test-none cargo test --locked --no-default-features
run doctest-all cargo test --locked --doc --all-features
run doctest-none cargo test --locked --doc --no-default-features
run clippy-all cargo clippy --locked --all-features --all-targets -- -D warnings
run clippy-none cargo clippy --locked --no-default-features --all-targets -- -D warnings
run fmt cargo fmt --all -- --check
run boundary bash parity/check-unsafe-boundary.sh
run public-text bash parity/tools/check_public_text.sh
run package-list bash -o pipefail -c 'cargo package --locked --allow-dirty --list | tee "$RELEASE_SCRATCH/checks/package-list.txt" | python3 parity/simd/check_package.py'
# Independently reject development-only files from the complete package list.
run package-inventory python3 -c '
from pathlib import Path
import os
p = Path(os.environ["RELEASE_SCRATCH"]) / "checks/package-list.txt"
paths = p.read_text().splitlines()
assert paths and "docs/PERFORMANCE.md" in paths
assert all(not x.startswith(("parity/", "fuzz/", ".git/", ".github/", "target/")) for x in paths)
assert all(x not in ("AGENTS.md", "RELEASE-0.1-CHECKLIST.md", "llms.txt") for x in paths)
print(f"PASS: {len(paths)} package entries, no internal files")
'
run publish-dry-run cargo publish --locked --allow-dirty --dry-run
run diff-whitespace git diff --check
cp "$S/package-list.txt" "$R/package-list.txt"
printf 'PASS: all final release checks\n'
