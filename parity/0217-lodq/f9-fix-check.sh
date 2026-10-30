#!/usr/bin/env bash
set -euo pipefail
: "${MOSS_HEAVY_ACTIVE:?run under admission}"
export LODQ_RECEIPTS=parity/0217-lodq/receipts/f9-fix
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 LC_ALL=C
mkdir -p "$LODQ_RECEIPTS"
# The existing runner records each exit, including failures, and runs all gates.
bash parity/0217-lodq/final-check.sh
run() {
  local name=$1 rc=0
  shift
  "$@" > "$LODQ_RECEIPTS/$name.log" 2>&1 || rc=$?
  printf '%s\t%s\n' "$name" "$rc" >> "$LODQ_RECEIPTS/exits.tsv"
  tail -n 8 "$LODQ_RECEIPTS/$name.log"
}
run scalar cargo test --locked --no-default-features
run clippy-scalar cargo clippy --locked --all-targets --no-default-features -- -D warnings
run proof python3 parity/0217-lodq/proof.py
# Keep receipts suitable for the public text gate without changing outcomes.
python3 - <<'SCRUB'
import hashlib, json, os
from pathlib import Path
out = Path(os.environ['LODQ_RECEIPTS'])
scrub = {}
for path in out.glob('*.log'):
    before = path.read_text()
    after = before
    for old, new in [(str(Path.cwd()), '<worktree>'), (os.environ['CARGO_TARGET_DIR'], '<target>'), (str(Path.home()/'.cargo'), '<cargo-home>')]:
        after = after.replace(old, new)
    if before != after:
        scrub[path.name] = dict(before=hashlib.sha256(before.encode()).hexdigest(), after=hashlib.sha256(after.encode()).hexdigest())
        path.write_text(after)
(out/'path-scrub.json').write_text(json.dumps(scrub, indent=2)+'\n')
SCRUB
run public-text bash parity/tools/check_public_text.sh
python3 - <<'CHECK'
import hashlib, json, os
from pathlib import Path
out = Path(os.environ['LODQ_RECEIPTS'])
exits = dict(line.split('\t') for line in (out/'exits.tsv').read_text().splitlines())
assert exits and all(rc == '0' for rc in exits.values()), exits
files = ['src/lod_quality.rs', 'tests/lod_quality.rs', 'parity/0217-lodq/proof.py', 'parity/0217-lodq/final-check.sh', 'parity/0217-lodq/f9-fix-check.sh']
(out/'fix-binding.json').write_text(json.dumps({p: hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in files}, indent=2)+'\n')
print(json.dumps(exits, indent=2))
CHECK
