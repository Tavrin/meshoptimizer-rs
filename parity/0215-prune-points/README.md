# Prune and point simplification qualification

Run every build and measurement within the configured CPU admission runner,
declaring 4 GB and GPU disabled. Export `PARITY_RUNNER_ACTIVE`,
`PARITY_RUNNER_GPU=0`, `LC_ALL=C`, `CARGO_BUILD_JOBS=1`,
`CARGO_INCREMENTAL=0`, a dedicated `CARGO_TARGET_DIR` and `MESHOPT_SCRATCH`.
The scratch baseline must be a source extraction of v0.2.14 with a `src/`
directory; prepare.py checks each file against Git before building. Retained
peer projections and their input manifest go under `art/inputs` and
`art/inputs.json`, respectively. No private input geometry is committed.

1. `python3 parity/0215-prune-points/prepare.py` freezes candidate sources,
   adapts the existing peer consumer and builds all four arms plus both exact
   feature configurations. It binds sources and binaries by SHA-256.
2. Run `proof/exact-all` and `proof/exact-none` from scratch, retaining logs.
   Their shared observation machinery is derived from the existing
   `parity/0215-contract-simplify/exact.rs`; `exact-main.rs` supplies this
   lane's finite domain and widened fuzz generator.
3. Set `MESHOPT_REFERENCE` to the pinned clean C++ 1.3 checkout and
   `MESHOPT_QUALIFY_TARGET` to the dedicated target. Run
   `bash parity/0216-qualify/run-protocol-modes.sh`. Its full simplify-family
   differential includes operations 25 (prune) and 26 (points).
4. Alias `proof/exact-std` to `proof/exact-all`, set `MESHOPT_SCRATCH` to
   the proof directory, and run `python3 parity/0215-contract-simplify/fuzz.py`.
   This resumes durable seed records until at least 300 child CPU seconds.
5. Restore the original scratch setting, set `PEERS_PERF` to perf 6.17 and
   optionally `PEERS_CPU` (default 9), and run
   `python3 parity/0215-prune-points/measure.py`. It appends one fsynced record
   per finished sample, skips completed keys on restart and preserves unfinished
   artifacts. Each case uses three interleaved repetitions of all four arms.

The peer consumer retains its existing allocation policy and settings: prune
cutoff 0.5; points at half the vertex count, without colors. It removes the
inherited loop clock before building. Counter windows exclude parsing and
serialization and require 100% running `instructions:u`. No elapsed-time
performance metric is collected. Raw counter files can include perf's own
elapsed-time footer; it is not used or exported as a timing result.

The pre-registered three-real-mesh geometric-mean gate is candidate/optimesh
at most 0.99 and candidate/base at most 0.97 for SHIPS; only the second for
IMPROVES; otherwise NO-GAIN. Any individual case above 1.01 candidate/base
requires a revert. There are at most two campaigns, and no optimization edits
after the first begins; the owner lifted that rule once for the check-and-finish
pass, whose candidate is campaign 2 and the last. Development screens inherited from interrupted work
are retained separately and are not substituted for campaign samples.

Run the normal test matrix, clippy with warnings denied, formatting, both
source and package boundary audits, and the public-text check before commit.
Retain raw logs, binaries, inputs and failed attempts in scratch. Remove only
the dedicated target after all lane jobs exit. `REPORT.md` gives disposition;
`PROOFS.md` derives the bounds and records shared-helper scope.
