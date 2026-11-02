#!/usr/bin/env bash
set -u
export CARGO_BUILD_JOBS=4
root=$PWD
out=$root/parity/0217-stream
run() {
 local name=$1 base=$1 rc=0 attempt=1
 shift
 while [[ -e "$out/$name.log" ]]; do
  name="$base-attempt-$attempt"
  attempt=$((attempt+1))
 done
 "$@" > "$CARGO_TARGET_DIR/raw-$name.log" 2>&1 || rc=$?
 python3 - "$CARGO_TARGET_DIR/raw-$name.log" "$out/$name.log" "$root" "$CARGO_TARGET_DIR" "$MESHOPT_REFERENCE" <<'PY'
from pathlib import Path
import sys
s=Path(sys.argv[1]).read_text(errors='replace')
for p,label in zip(sys.argv[3:],['<worktree>','<target>','<reference>']):s=s.replace(p,label)
Path(sys.argv[2]).write_text(s)
PY
 printf '%s\t%s\n' "$name" "$rc" | tee -a "$out/exits.tsv"
 tail -n 3 "$out/$name.log"
 return 0
}
run fmt cargo fmt --all
run clippy cargo clippy --locked --all-features --all-targets -- -D warnings
run tests cargo test --locked --all-features
run no-std cargo test --locked --no-default-features
run doctests cargo test --locked --all-features --doc
run docs env RUSTDOCFLAGS='-D warnings' cargo doc --locked --all-features --no-deps
run wasm cargo build --locked --target wasm32-unknown-unknown --no-default-features
if ! command -v cargo-semver-checks >/dev/null; then
 mkdir -p "$CARGO_TARGET_DIR/tools"
 run semver-tool cargo install cargo-semver-checks --locked --root "$CARGO_TARGET_DIR/tools"
 export PATH="$CARGO_TARGET_DIR/tools/bin:$PATH"
fi
run semver cargo semver-checks check-release --baseline-version 0.2.16
run reference bash parity/check-reference.sh
run unsafe bash parity/check-unsafe-boundary.sh
run corpus-wasm bash parity/0217-stream/extra.sh
run benchmark cargo run --locked --release --example buffer_stream_bench
run fuzz-build cargo +nightly fuzz build --fuzz-dir fuzz/codec buffer_stream
run fuzz python3 parity/0217-stream/fuzz.py
run fmt-check cargo fmt --all -- --check
run public-text bash parity/tools/check_public_text.sh
