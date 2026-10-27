# 0.2.7 release validation

Run from the release worktree, with CPU-only `<job-runner>` admission declaring
4 GB, and `CARGO_TARGET_DIR=<target-dir>`.
`MESHOPT_ARTIFACTS=<scratch>/meshopt-rel027` retains receipts
outside the target directory. Delete that target after the final checks.

Build `retention`, `vertex_fetch` and `overdraw` from `fuzz/Cargo.toml` in release
mode with `--locked`. Build `optimize_meshlet` with `cargo fuzz build --fuzz-dir
fuzz/p03 --target-dir "$CARGO_TARGET_DIR" --sanitizer none optimize_meshlet`
(`RUSTC_BOOTSTRAP=1` on the installed stable compiler).

`python3 parity/release-027/fuzz.py` uses pinned physical cores, nice 19,
30-second chunks and GNU time user + system CPU accounting. It schedules
until retention has at least 900 CPU-seconds and each widened target has at
least 300 CPU-seconds. Seeds, execution counts, source archives, binaries,
libFuzzer corpora, crash artifacts and stdout/stderr remain in scratch.
Stable targets have no coverage instrumentation; the meshlet target uses
libFuzzer coverage without a sanitizer. A finding stops new chunks and fails
the run. Library code must not be repaired in this release job.

The retention target keeps one Reuse workspace and one Lean workspace across
random sequences of simplify, attribute simplify, update, sloppy, prune,
points, cache and overdraw calls. Successful triangle results feed subsequent
calls. It compares full Results, float bits, mutable topology, positions and
attributes, and counted work. Byte limits leave space for both policies;
work limits, invalid indices, nonfinite positions and invalid error parameters
exercise errors. Retained-capacity byte usage is policy dependent and is not
asserted equal. Replay a stable shard with `BINARY SECONDS SEED EXECUTIONS`;
use a sufficiently long seconds budget to reach the saved execution count.

The meshlet corpus includes `[0,1,2]`, `[2,0,2]` and `[2,2,2]` as exactly one
triangle. Fetch reaches 1,023, 1,024, 1,025 and 2,048 vertices with packed and
padded strides, including nonzero offsets; it compares those views to packed
copies. Overdraw passes packed positions, padded floats and byte views in both
byte orders to all three API forms, including unaligned byte offsets.

After fuzz passes, run `bash parity/release-027/checks.sh` as one admitted
CPU-only job. It retains every exit code and log, checks both doc-test modes,
both CI feature modes, package verification, publication dry-run and package
contents. It never publishes.

Cargo requires `--allow-dirty` for the package listing as well as package
verification while the authorized release edits remain uncommitted. The plain
`cargo package --list` refusal is retained in `checks-initial/6.log`; the final
script lists with `cargo package --list --allow-dirty`. No Git staging or commit
is needed.
