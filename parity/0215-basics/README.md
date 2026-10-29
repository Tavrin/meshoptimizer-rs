# Pipeline basics: 0.2.15 lane (remap family, index decode)

All builds, proofs and measurements run inside the shared CPU-only admission
runner with at most 4 GB declared; its adapter exports
`PARITY_RUNNER_ACTIVE=1` and `PARITY_RUNNER_GPU=0`. Use a dedicated
`CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0` and `LC_ALL=C`. No wall-time result
is used.

1. Supply the clean train head tree as `MESHOPT_BASELINE`, the eight frozen
   peer fixtures under `PEERS_ART/inputs` and their manifest as
   `PEERS_ART/inputs.json` (the toward-C++ lane's fixtures). `PEERS_ART` is a
   child of the lane scratch directory.
2. `python3 build.py base cpp rust candidate` freezes the base and candidate
   sources, adapts scratch copies of `parity/peers` (the contract-meshlets
   counting-only patch, then `prepare_driver.py`: remap ops, SIMD off), builds
   each arm and binds sources and binaries.
3. Set `PEERS_PERF` to perf 6.17 and optionally `PEERS_CPU` (default 9).
   `python3 measure.py baseline base cpp rust` measured the baseline;
   `python3 measure.py campaign-1 base candidate cpp rust` is the registered
   campaign. Samples append with fsync and resume by key; source, binary and
   input hashes are checked on every start. `python3 results.py CAMPAIGN`
   applies the registered gate.
4. `MESHOPT_LANE_SCRATCH=<dir> bash proof.sh` runs tests, clippy, fmt, the
   unsafe boundary, the public-text check, and `exact.rs` (base vs
   candidate, every work limit, both feature modes, plus a random sweep).
5. Raw logs, binaries, outputs and fixtures stay in scratch. Only the compact
   counts, results, bindings and these notes are committed.

`REGISTRATION.md` fixes arms, ops and the gate before results; `BASELINE.md`
is the stage-1 table; `PROOFS.md` argues each change and records the
verdicts and attribution.
