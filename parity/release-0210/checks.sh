#!/usr/bin/env bash
# Run this script through CPU-only <job-runner> admission, declaring 4 GB.
set -euo pipefail
cd -- "$(dirname -- "$0")/../.."
: "${PARITY_RUNNER_ACTIVE:?CPU-only admission required}"
: "${MESHOPT_ARTIFACTS:?logs must live outside the target}"
test "${PARITY_RUNNER_GPU:?}" = 0
test "${CARGO_TARGET_DIR:?}" = "${CARGO_TARGET_DIR:?}"
mkdir -p "$MESHOPT_ARTIFACTS"
test ! -e "$MESHOPT_ARTIFACTS/exits.tsv"
trap 'rm -rf -- "$CARGO_TARGET_DIR"; test ! -e "$CARGO_TARGET_DIR" && printf "target removed\n" > "$MESHOPT_ARTIFACTS/cleanup.txt"' EXIT
failed=0
run() {
  local name=$1 rc=0
  shift
  "$@" > "$MESHOPT_ARTIFACTS/$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" | tee -a "$MESHOPT_ARTIFACTS/exits.tsv"
  if [ "$rc" -ne 0 ]; then failed=1; fi
}
run no-default-features cargo test --locked --no-default-features
run all-features cargo test --locked --all-features
run doc-all-features cargo test --doc --all-features
run doc-default cargo test --doc
run simd-boundary python3 parity/simd/check_boundary.py
run package cargo package --locked --allow-dirty
run publish-dry-run cargo publish --dry-run --locked --allow-dirty
# Cargo refuses a plain listing of uncommitted release edits. Retain that
# result, then inspect the same package with the required dirty-tree allowance.
list_rc=0
cargo package --list > "$MESHOPT_ARTIFACTS/package-list-plain.log" 2>&1 || list_rc=$?
printf 'package-list-plain\t%s\n' "$list_rc" | tee -a "$MESHOPT_ARTIFACTS/exits.tsv"
if [ "$list_rc" -ne 0 ] && ! grep -q 'allow-dirty' "$MESHOPT_ARTIFACTS/package-list-plain.log"; then failed=1; fi
run package-list cargo package --list --allow-dirty
package_contents() {
  grep -qx 'docs/MIGRATING_FROM_MESHOPT.md' "$MESHOPT_ARTIFACTS/package-list.log" || return 1
  if grep -E '(^|/)[^/]*(REPORT|REGISTRATION)[^/]*$' "$MESHOPT_ARTIFACTS/package-list.log"; then return 1; fi
}
run package-contents package_contents
internal_references() {
  python3 - "$MESHOPT_ARTIFACTS/public-text.txt" <<'PY'
import re
import sys
from pathlib import Path
changelog = Path('CHANGELOG.md').read_text().split('## [0.2.10]', 1)[1].split('\n## [', 1)[0]
texts = [('README.md', Path('README.md').read_text()), ('CHANGELOG.md (0.2.10)', changelog)]
texts += [(str(p), p.read_text()) for p in sorted(Path('docs').rglob('*')) if p.is_file()]
# Historical link URLs are the only exceptions in these public documents.
Path(sys.argv[1]).write_text('\n'.join(name + '\n' + re.sub(r'https?://[^\s)<>]+', '', text) for name, text in texts))
PY
  local rc=0
  local private='mo''ss' planning='rf''c' brief='sp''ec-'
  grep -niE "${planning} ?[0-9]|\b${private}\b|${brief}|\blane\b|${private}-[a-z0-9]{5}" "$MESHOPT_ARTIFACTS/public-text.txt" || rc=$?
  test "$rc" -eq 1
}
run internal-references internal_references
lock_versions() {
  grep -rn 'name = "meshoptimizer-rs"' -A1 --include=Cargo.lock .
  python3 - <<'PY'
import tomllib
from pathlib import Path
assert tomllib.loads(Path('Cargo.toml').read_text())['package']['version'] == '0.2.10'
count = 0
for path in Path('.').rglob('Cargo.lock'):
    for package in tomllib.loads(path.read_text())['package']:
        if package['name'] == 'meshoptimizer-rs':
            assert package['version'] == '0.2.10', path
            count += 1
assert count > 0
print(f'All {count} crate lockfile entries pin 0.2.10.')
PY
}
run lock-versions lock_versions
run diff-check git diff --check
exit "$failed"
