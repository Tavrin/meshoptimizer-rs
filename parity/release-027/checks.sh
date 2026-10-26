#!/usr/bin/env bash
# Run as one CPU-only moss-heavy job, declaring 4 GB.
set -euo pipefail
cd -- "$(dirname -- "$0")/../.."
: "${MESHOPT_ARTIFACTS:?retain logs outside the target}"
: "${CARGO_TARGET_DIR:?isolated target required}"
mkdir -p "$MESHOPT_ARTIFACTS/checks"
commands=(
  'cargo test --doc --all-features'
  'cargo test --doc'
  'cargo test --locked --no-default-features'
  'cargo test --locked --all-features'
  'cargo package --locked --allow-dirty'
  'cargo publish --dry-run --locked --allow-dirty'
  'cargo package --list --allow-dirty'
)
failed=0
: > "$MESHOPT_ARTIFACTS/checks/exits.tsv"
for i in "${!commands[@]}"; do
  rc=0
  bash -c "${commands[$i]}" > "$MESHOPT_ARTIFACTS/checks/$i.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "${commands[$i]}" "$rc" | tee -a "$MESHOPT_ARTIFACTS/checks/exits.tsv"
  if [ "$rc" -ne 0 ]; then failed=1; fi
done
content_rc=0
if rg '(^|/)(REPORT|REGISTRATION)[^/]*$' "$MESHOPT_ARTIFACTS/checks/6.log"; then
  echo 'package contains REPORT/REGISTRATION files' >&2
  failed=1
  content_rc=1
fi
printf 'package excludes REPORT/REGISTRATION\t%s\n' "$content_rc" | tee -a "$MESHOPT_ARTIFACTS/checks/exits.tsv"
exit "$failed"
