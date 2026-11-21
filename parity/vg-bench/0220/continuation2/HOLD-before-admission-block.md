# Human HOLD: continuation stopped

Effective immediately: no benches, fuzzing or big builds; small local jobs only,
one at a time. No active or queued lane jobs remain. The final attribution/
accounting and gate jobs were cancelled before admission. The dedicated target
is retained to avoid substantial cleanup IO.

The single interleaved experiment completed with exit 0 and all MDAG 2 byte
comparisons passed. Four-triangle leaves were rejected and restored. The helper
prototype and latest documentation have not passed final retained-source gates.
Original-policy sizing now covers all five meshes; dragon's low-work case stops
at exactly 2^28 visits. Storage-only isolation is still pending.

Fully resolved commands are in the caller-owned `HOLD-REMAINING.md`. After the
human lifts HOLD, execute these one at a time with the brief's `HEAVY`,
`CARGO_TARGET_DIR`, `VG_EVIDENCE` and fresh continuation evidence directory:

```bash
env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" VG_EVIDENCE="$VG_EVIDENCE" VG_CONTINUATION="$VG_EVIDENCE/continuation" "$HEAVY" 9 python3 "$VG_EVIDENCE/continuation/post.py"

env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" VG_EVIDENCE="$VG_EVIDENCE" VG_CONTINUATION="$VG_EVIDENCE/continuation" "$HEAVY" 9 bash "$VG_EVIDENCE/continuation/gates.sh"
```

Then publish actual disk/cycle/helper/gate receipts and final source hashes;
update the report, results and sizing documentation. Run the final public-text
check, then clean the dedicated target only after HOLD is lifted:

```bash
env MOSS_HEAVY_GPU=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" "$HEAVY" 9 bash parity/tools/check_public_text.sh

test "$(basename "$CARGO_TARGET_DIR")" = meshopt-vg-0220 && rm -rf -- "$CARGO_TARGET_DIR"
```

No second timing campaign is scheduled. File-byte recovery and the failed cut,
1-thread and resident bars remain unresolved. Earlier hold details are retained
in [the previous hold](HOLD-PREVIOUS.md).
