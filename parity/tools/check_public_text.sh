#!/usr/bin/env bash
# Include untracked additions so the worktree and CI enforce the same rule.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
private='mo''ss'
planning='rf''c'
pattern="/home/[a-z]+/|/mnt/[a-z]|${private}-(scratch|cargo|heavy|coord|lane|capture)|apollo-main|automation_game|etienne|Legion-Pro|\b${private}\b|${planning} ?[0-9]|${private}-[a-z0-9]{5}"
set +e
matches=$(git grep --untracked -niIE "$pattern" -- . ':!parity/tools/check_public_text.sh')
status=$?
set -e
case "$status" in
  0|1) ;;
  *) exit "$status" ;;
esac
# Exact licence attribution lines only; other text in these files is checked.
attribution="^src/codec/index\.rs:[0-9]+:// Adapted from ${private^} dc4af42a; MIT, ${private^} contributors and Arseny Kapoulkine\.$|^src/codec/vertex\.rs:[0-9]+:// Byte-group parsing also reuses the ${private^} dc4af42a decoder \(MIT\)\.$"
set +e
remaining=$(printf '%s\n' "$matches" | grep -vE "$attribution")
filtered=$?
set -e
if [ "$filtered" -gt 1 ]; then exit "$filtered"; fi
if [ -n "$remaining" ]; then
  printf '%s\n' "$remaining"
  echo 'FAIL: private machine paths or project names in public text' >&2
  exit 1
fi
echo 'PASS: public text contains no private machine paths or project names'
