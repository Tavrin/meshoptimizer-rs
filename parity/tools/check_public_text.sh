#!/usr/bin/env bash
# Include untracked additions so the worktree and CI enforce the same rule.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
pattern='/home/[a-z]+/|/mnt/[a-z]|moss-(scratch|cargo|heavy|coord|lane|capture)|apollo-main|automation_game|etienne|Legion-Pro'
set +e
git grep --untracked -nIE "$pattern" -- . ':!parity/tools/check_public_text.sh'
status=$?
set -e
case "$status" in
  0) echo 'FAIL: private machine paths or project names in public text' >&2; exit 1 ;;
  1) echo 'PASS: public text contains no private machine paths or project names' ;;
  *) exit "$status" ;;
esac
