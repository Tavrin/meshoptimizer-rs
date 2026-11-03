#!/usr/bin/env bash
set -euo pipefail
s=$CARGO_TARGET_DIR/stream-proof
mkdir -p "$s/fixtures" "$s/wasm" "$s/seeds"
python3 parity/codec/native-fixtures.py "$MESHOPT_REFERENCE" "$s/native.cpp" "$s/fixtures"
c++ -std=c++11 -O2 -DMESHOPTIMIZER_NO_SIMD -I "$MESHOPT_REFERENCE/src" "$s/native.cpp" "$MESHOPT_REFERENCE"/src/*.cpp -o "$s/native"
"$s/native" "$s/fixtures"
node parity/codec/js-fixtures.mjs "$MESHOPT_REFERENCE" "$s/fixtures"
STREAM_FIXTURES="$s/fixtures" STREAM_SEEDS="$s/seeds" cargo test --locked --all-features --test buffer_stream -- --nocapture
python3 - "$s/wasm" "$PWD" <<'PY'
from pathlib import Path
import sys
p=Path(sys.argv[1]);root=Path(sys.argv[2])
s=(root/'tests/buffer_stream.rs').read_text().replace('#[test]','').replace('#[path = "support/buffer_stream.rs"]', '#[path = '+repr(str(root/'tests/support/buffer_stream.rs')).replace("'",'"')+']')
# Also accept the pre-format spelling when preparing before rustfmt.
s=s.replace('#[path="support/buffer_stream.rs"]','#[path="'+str(root/'tests/support/buffer_stream.rs')+'"]')
s+='\n#[no_mangle]\npub extern "C" fn run() {random_roundtrips_and_malformed(); arbitrary_errors(); empty_output_sticky_and_narrowing(); every_available_simd_path();}\n'
(p/'lib.rs').write_text(s)
(p/'Cargo.toml').write_text('[package]\nname="stream-wasm-proof"\nversion="0.0.0"\nedition="2021"\n[lib]\npath="lib.rs"\ncrate-type=["cdylib"]\n[features]\ndefault=["simd","parity-internals"]\nsimd=[]\nparity-internals=[]\n[dependencies]\nmeshoptimizer-rs={path="'+str(root)+'",default-features=false,features=["simd","parity-internals"]}\n')
PY
cargo build --release --target wasm32-unknown-unknown --manifest-path "$s/wasm/Cargo.toml"
node - "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/stream_wasm_proof.wasm" <<'JS'
const fs=require('fs');WebAssembly.instantiate(fs.readFileSync(process.argv[2]),{}).then(({instance})=>{instance.exports.run();console.log('PASS: wasm streaming identity, malformed inputs, SIMD, and narrowing');}).catch(e=>{console.error(e);process.exitCode=1;});
JS
