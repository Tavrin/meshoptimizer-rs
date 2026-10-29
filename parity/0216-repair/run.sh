#!/usr/bin/env bash
# Run under a CPU-only admission wrapper with a dedicated CARGO_TARGET_DIR.
set -euo pipefail
: "${CARGO_TARGET_DIR:?}"
root=$(cd "$(dirname "$0")/../.." && pwd)
lane=$root/parity/0216-repair
scratch=$CARGO_TARGET_DIR/proof
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 LC_ALL=C GIT_OPTIONAL_LOCKS=0
mkdir -p "$scratch/base" "$scratch/exact" "$lane/receipts"
rec() {
  local name=$1 rc=0
  shift
  "$@" > "$scratch/$name.log" 2>&1 || rc=$?
  # Retain complete logs with local paths redacted for public-text hygiene.
  python3 - "$scratch/$name.log" "$lane/receipts/$name.log" "$root" "$CARGO_TARGET_DIR" <<'PY'
import sys
s=open(sys.argv[1]).read()
s=s.replace(sys.argv[3],'<worktree>').replace(sys.argv[4],'<target>')
open(sys.argv[2],'w').write(s)
PY
  printf '%s\t%s\n' "$name" "$rc" | tee -a "$lane/receipts/exits.tsv"
  tail -n 3 "$lane/receipts/$name.log"
  return "$rc"
}
cd "$root"
rec fmt cargo fmt --all -- --check
rec test-all cargo test --locked --all-features
rec test-none cargo test --locked --no-default-features
rec test-repair-no-std cargo test --locked --no-default-features --features repair
rec doctest-all cargo test --locked --all-features --doc
rec doctest-repair-no-std cargo test --locked --no-default-features --features repair --doc
rec clippy-all cargo clippy --locked --all-features --all-targets -- -D warnings
rec clippy-repair-no-std cargo clippy --locked --no-default-features --features repair --all-targets -- -D warnings
rec boundary bash parity/check-unsafe-boundary.sh
# Read-only archive of the exact base; no checkout, staging or ref changes.
git archive 1ecf69abe9f92c17e13dc38fa97be659aee44582 Cargo.toml Cargo.lock src docs README.md | tar -x -C "$scratch/base"
python3 - "$scratch/base/Cargo.toml" <<'PY'
from pathlib import Path
import sys
p=Path(sys.argv[1]);p.write_text(p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1))
PY
for mode in all none; do
  base_features='["std", "simd", "clusterlod", "experimental", "parallel", "parity-internals"]'
  candidate_features='["std", "simd", "clusterlod", "experimental", "parallel", "parity-internals", "repair"]'
  if [[ $mode == none ]]; then base_features='[]'; candidate_features='["repair"]'; fi
  cat > "$scratch/exact/Cargo.toml" <<MANIFEST
[package]
name = "repair-proof"
version = "0.0.0"
edition = "2021"
[dependencies]
baseline = { package = "meshoptimizer-baseline", path = "$scratch/base", default-features = false, features = $base_features }
candidate = { package = "meshoptimizer-rs", path = "$root", default-features = false, features = $candidate_features }
[[bin]]
name = "repair-exact"
path = "$lane/exact.rs"
[[bin]]
name = "repair-corpus"
path = "$lane/corpus.rs"
[profile.release]
overflow-checks = true
debug-assertions = true
MANIFEST
  rec "exact-build-$mode" cargo build --offline --release --manifest-path "$scratch/exact/Cargo.toml"
  rec "exact-$mode" "$CARGO_TARGET_DIR/release/repair-exact"
  rec "corpus-export-$mode" python3 parity/0215-lod-chain/corpus.py "$scratch/corpus"
  rec "corpus-$mode" "$CARGO_TARGET_DIR/release/repair-corpus" "$scratch/corpus"
  sha256sum "$CARGO_TARGET_DIR/release/repair-exact" "$CARGO_TARGET_DIR/release/repair-corpus" | sed "s|$CARGO_TARGET_DIR/release/||" > "$lane/receipts/binaries-$mode.sha256"
done
python3 - "$scratch/corpus" "$lane/receipts/corpus.json" <<'PY'
from pathlib import Path
import hashlib,json,sys
p=Path(sys.argv[1])
json.dump({f.name:hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(p.glob('*.bin'))},open(sys.argv[2],'w'),indent=2)
PY
rec public-text bash parity/tools/check_public_text.sh
