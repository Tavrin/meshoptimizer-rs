# Human HOLD: sizing addendum

No further jobs are queued or authorized until the human lifts HOLD. The sizing
campaign stopped before a completed rocks result was recorded. Only the three completed successful
rows in [limits-partial.json](limits-partial.json) are admitted; rocks and dragon
have no completed sizing measurement. The reported consumer dragon result
(512 MiB / 2^28 fails, 1 GiB / 2^34 passes) is not a measurement from this lane
and does not isolate which budget was exhausted.

Rustdoc on `v2::build` and `docs/DAG_FORMAT.md` now explain successful-run usage,
per-source-triangle calibration, separate storage/work ceilings, and finite serial
accounting. No suggested-limits helper was added: its larger-mesh calibration is
missing. The performance policy and original benchmark measurements are unchanged.
The pre-addendum gates, including legacy identity, exited 0. The latest sizing
documentation has not had the heavy gates rerun.

The dedicated target is retained: removing it would create substantial IO during
the quiet hold. Canonical logs and binaries remain in the caller's evidence folder.

## Commands remaining, only after HOLD is lifted

Use the exact values of `HEAVY`, `CARGO_TARGET_DIR`, and `VG_EVIDENCE` from this
lane's brief. Fully resolved commands are also retained in the caller's
`HOLD-REMAINING.md`. Run commands one at a time. The `limits` command's JSON
`pass`/`result` fields determine build success; process exit 0 only means the
diagnostic completed.

```bash
env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 taskset -c 0-7 "$VG_EVIDENCE/limits-vg-bench" limits ours "$VG_EVIDENCE/coast_rocks_01.vgm" --bytes 1073741824 --work 17179869184 --jobs 8

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 taskset -c 0-7 "$VG_EVIDENCE/limits-vg-bench" limits ours "$VG_EVIDENCE/dragon.vgm" --bytes 1073741824 --work 17179869184 --jobs 8

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 taskset -c 0-7 "$VG_EVIDENCE/limits-vg-bench" limits ours "$VG_EVIDENCE/dragon.vgm" --bytes 536870912 --work 268435456 --jobs 8

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 taskset -c 0-7 "$VG_EVIDENCE/limits-vg-bench" limits ours "$VG_EVIDENCE/dragon.vgm" --bytes 536870912 --work 17179869184 --jobs 8

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 cargo fmt --check

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 cargo clippy --all-targets --all-features -- -D warnings

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 cargo test --release

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 cargo test --release --all-features

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 env RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 bash parity/tools/check_public_text.sh

test "$(basename "$CARGO_TARGET_DIR")" = meshopt-vg-0220 && rm -rf -- "$CARGO_TARGET_DIR"
```

After the missing calibration, decide whether a triangle-count helper has adequate
headroom for the measured range, implement and validate it if supported, and
update the sizing table before the final gates. No second timing/cut campaign is
needed for documentation alone; any build-policy change needs fresh qualification.
