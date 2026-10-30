#!/usr/bin/env bash
# Run inside the CPU-only admission runner with a dedicated target directory.
# MESHOPT_REFERENCE points to the clean, pinned upstream oracle checkout.
set -euo pipefail
: "${CARGO_TARGET_DIR:?set the lane target directory}"
: "${MESHOPT_REFERENCE:?set the reference checkout}"
[[ -n ${MOSS_HEAVY_ACTIVE:-} ]]
[[ ${MOSS_HEAVY_GPU:-} == 0 ]]
export CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 LC_ALL=C
ROOT=$(pwd)
LANE=$ROOT/parity/0216-stream
S=$CARGO_TARGET_DIR/proof-scratch
mkdir -p "$S"
# Log paths are made portable; the commands and unsanitized streams still
# execute normally. Retain exit codes for failures as well as successful reruns.
run() {
    local name=$1 rc=0
    shift
    /usr/bin/time -f "cpu_user=%U cpu_system=%S elapsed=%e exit=%x" "$@" > "$S/raw.log" 2>&1 || rc=$?
    python3 - "$S/raw.log" "$LANE/$name.log" "$ROOT" "$CARGO_TARGET_DIR" "$MESHOPT_REFERENCE" <<'PY'
from pathlib import Path
import sys
s=Path(sys.argv[1]).read_text(errors='replace')
for p, label in zip(sys.argv[3:], ['<worktree>', '<target-dir>', '<reference>']):
    s=s.replace(p, label)
Path(sys.argv[2]).write_text(s)
PY
    printf '%s\t%s\n' "$name" "$rc" >> "$LANE/exits.tsv"
    tail -n 4 "$LANE/$name.log"
    return "$rc"
}
run reference-check bash parity/check-reference.sh
run unchanged-one-shot git --no-optional-locks diff --exit-code 1ecf69abe9f92c17e13dc38fa97be659aee44582 -- src/codec/index.rs src/codec/index_encode.rs src/codec/encode.rs
run fmt cargo fmt --all
run fmt-evidence rustfmt --edition 2021 parity/0216-stream/exact.rs fuzz/codec/fuzz_targets/sequence_stream.rs
run test-stream cargo test --locked --test sequence_stream
run test-all cargo test --locked --all-features
run test-no-default cargo test --locked --no-default-features
run clippy cargo clippy --locked --all-targets --all-features -- -D warnings
run clippy-no-default cargo clippy --locked --all-targets --no-default-features -- -D warnings
run fmt-check cargo fmt --all -- --check
run boundary bash parity/check-unsafe-boundary.sh
run fuzz-build cargo check --locked --manifest-path fuzz/codec/Cargo.toml --bin sequence_stream

# Export the unchanged native and JS codec corpora, including malformed cases.
mkdir -p "$S/fixtures"
run export-native python3 parity/codec/native-fixtures.py "$MESHOPT_REFERENCE" "$S/native.cpp" "$S/fixtures"
# Build fixture capture with assertions enabled, so unchanged upstream tests run.
run build-native c++ -std=c++11 -O2 -DMESHOPTIMIZER_NO_SIMD -I "$MESHOPT_REFERENCE/src" "$S/native.cpp" "$MESHOPT_REFERENCE"/src/*.cpp -o "$S/native"
run native-fixtures "$S/native" "$S/fixtures"
run js-fixtures node parity/codec/js-fixtures.mjs "$MESHOPT_REFERENCE" "$S/fixtures"
python3 - "$S/fixtures" "$LANE/corpus.tsv" <<'PY'
from pathlib import Path
import struct, sys
rows=[]
for p in sorted(Path(sys.argv[1]).glob('*.input')):
    b=p.read_bytes()
    op,n,stride,mode=struct.unpack_from('<4I',b,4)
    if op==3 or (op==7 and mode==2):
        rows.append(f'{p.stem}\t{n}\t{b[44:].hex() or "-"}\n')
assert rows
Path(sys.argv[2]).write_text(''.join(rows))
print('sequence corpus entries:',len(rows))
PY
run build-cpp c++ -std=c++11 -O2 -DMESHOPTIMIZER_NO_SIMD -I "$MESHOPT_REFERENCE/src" "$LANE/reference.cpp" "$MESHOPT_REFERENCE/src/indexcodec.cpp" -o "$S/reference"

# Read-only Git archive: the baseline has a distinct package identity and is
# compiled alongside the candidate. No worktree or index mutation is used.
mkdir -p "$S/base" "$S/exact"
git archive 1ecf69abe9f92c17e13dc38fa97be659aee44582 | tar -x -C "$S/base"
python3 - "$S/base/Cargo.toml" <<'PY'
from pathlib import Path
import sys
p=Path(sys.argv[1]);s=p.read_text().replace('name = "meshoptimizer-rs"','name = "meshoptimizer-baseline"',1);p.write_text(s)
PY
cat > "$S/exact/Cargo.toml" <<TOML
[package]
name = "stream-exact"
version = "0.0.0"
edition = "2021"
[features]
default = ["std"]
std = ["candidate/std", "baseline/std", "candidate/simd", "baseline/simd"]
[dependencies]
candidate = { package = "meshoptimizer-rs", path = "$ROOT", default-features = false }
baseline = { package = "meshoptimizer-baseline", path = "$S/base", default-features = false }
[[bin]]
name = "stream-exact"
path = "$LANE/exact.rs"
[profile.release]
overflow-checks = true
debug-assertions = true
TOML
for mode in default no-default; do
    flags=()
    if [[ $mode == no-default ]]; then flags+=(--no-default-features); fi
    run "exact-build-$mode" cargo build --offline --release --manifest-path "$S/exact/Cargo.toml" "${flags[@]}"
    run "exact-$mode" "$CARGO_TARGET_DIR/release/stream-exact" "$LANE/corpus.tsv" "$S/reference" "$LANE/cases-$mode.tsv"
done
cmp "$LANE/cases-default.tsv" "$LANE/cases-no-default.tsv"
run public-text bash parity/tools/check_public_text.sh
python3 - "$LANE" <<'PY'
from pathlib import Path
import hashlib, sys
root=Path.cwd();lane=Path(sys.argv[1]);files=[*root.glob('src/codec/**/*.rs'),root/'Cargo.toml',root/'Cargo.lock',root/'tests/sequence_stream.rs',root/'docs/USING.md',root/'README.md',root/'fuzz/codec/Cargo.toml',root/'fuzz/codec/fuzz_targets/sequence_stream.rs',*lane.glob('*.rs'),*lane.glob('*.cpp'),*lane.glob('*.sh'),*lane.glob('*.tsv'),*lane.glob('*.log')]
(lane/'sources.sha256').write_text(''.join(f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.relative_to(root)}\n' for p in sorted(set(files))))
PY
