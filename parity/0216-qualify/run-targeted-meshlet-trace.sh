#!/usr/bin/env bash
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
scratch=${MESHOPT_SCRATCH:?}
out="$scratch/proof/0216d-meshlet-trace-retry"
mkdir -p "$out"
avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
(( avail >= 104857600 )) || exit 75
cp -a "$scratch/candidate" "$out/candidate-trace"
python3 - "$out/candidate-trace" <<'PY'
from pathlib import Path
import sys
root=Path(sys.argv[1]); p=root/'src/lib.rs'; s=p.read_text(); anchor='extern crate alloc;'; assert s.count(anchor)==1; s=s.replace(anchor,anchor+'\n#[cfg(not(feature = "std"))]\nextern crate std;'); p.write_text(s)
p=root/'src/workspace.rs'; s=p.read_text(); anchor='impl Work {'; assert s.count(anchor)==2; s=s.replace(anchor,anchor+'\n    pub(crate) fn diagnostic_spent(&self) -> u64 { self.limit - self.remaining }',1); p.write_text(s)
p=root/'src/meshlet.rs'; s=p.read_text(); anchor='    let covered = nodes\n        .len()\n        .checked_mul(2)'; assert s.count(anchor)==1; s=s.replace(anchor,'    std::eprintln!("BOUNDARY spent={} nodes={} boundary={}", ctx.work.diagnostic_spent(), nodes.len(), ctx.work.diagnostic_spent() + 2 * nodes.len() as u64);\n'+anchor); p.write_text(s)
PY
cat > "$out/Cargo.toml" <<MANIFEST
[package]
name = "targeted-meshlet-trace"
version = "0.0.0"
edition = "2021"
[features]
all = ["baseline/std", "candidate/std", "baseline/experimental", "candidate/experimental", "baseline/simd", "candidate/simd", "baseline/parallel", "candidate/parallel", "baseline/clusterlod", "candidate/clusterlod", "baseline/parity-internals", "candidate/parity-internals"]
[dependencies]
baseline = { package = "meshoptimizer-baseline", path = "$scratch/proof/base", default-features = false }
candidate = { package = "meshoptimizer-rs", path = "$out/candidate-trace", default-features = false }
[[bin]]
name = "targeted-meshlet-trace"
path = "$repo/parity/0216-qualify/targeted-meshlet-trace.rs"
[profile.release]
lto = "thin"
codegen-units = 1
MANIFEST
export CARGO_TARGET_DIR=${MESHOPT_QUALIFY_TARGET:?} CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1
sha256sum "$repo/parity/0216-qualify/targeted-meshlet-trace.rs" "$repo/parity/0216-qualify/run-targeted-meshlet-trace.sh" "$out/candidate-trace/src/lib.rs" "$out/candidate-trace/src/workspace.rs" "$out/candidate-trace/src/meshlet.rs" > "$out/sources.sha256"
cargo generate-lockfile --offline --manifest-path "$out/Cargo.toml" > "$out/lock.log" 2>&1
for mode in all none; do
    avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}'); (( avail >= 104857600 )) || exit 75
    args=(); if [[ "$mode" == all ]]; then args=(--features all); fi
    cargo build --offline --locked --release --manifest-path "$out/Cargo.toml" "${args[@]}" > "$out/build-$mode.log" 2>&1
    cp "$CARGO_TARGET_DIR/release/targeted-meshlet-trace" "$out/binary-$mode"
    sha256sum "$out/binary-$mode" >> "$out/binaries.sha256"
    set +e
    (ulimit -t 300; /usr/bin/time -p "$out/binary-$mode") > "$out/run-$mode.log" 2>&1
    rc=$?
    set -e
    printf '%s %s\n' "$mode" "$rc" >> "$out/exits.txt"
    (( rc == 0 )) || exit "$rc"
done
