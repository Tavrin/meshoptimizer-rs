# Toward C++: 0.2.15 lane

All builds, fixtures, proofs and measurements run inside the shared CPU-only
admission runner, declaring at most 4 GB. Its adapter exports
`PARITY_RUNNER_ACTIVE=1`, `PARITY_RUNNER_GPU=0`. Use a dedicated
`CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`, `LC_ALL=C`, and at most two
compiler jobs. No wall-time result is used.

1. Supply a clean unpacked v0.2.14 tree as `MESHOPT_BASELINE`, a pinned
   C++ 1.3 checkout as `MESHOPT_REFERENCE`, the eight retained peer fixtures
   under `PEERS_ART/inputs`, and their manifest as `PEERS_ART/inputs.json`.
   `PEERS_ART` must be a child of the lane scratch directory.
2. Set `MESHOPT_LANE_SCRATCH` to a fresh proof directory and
   `MESHOPT_PROOF_INPUTS` to the fixture directory. Run `bash proof.sh`
   from this directory. Proof steps append exit receipts and skip completed
   steps on restart; `bind.py` rejects source changes in a resumed proof.
3. Run `python3 build.py`; it snapshots base/candidate sources, adapts the
   existing peer consumer to add flex and remove elapsed-time collection,
   builds all four arms, and seals sources/binaries. Prior exploratory
   executables are retained separately.
4. Set `PEERS_PERF` to perf 6.17, optionally `PEERS_CPU` (default 9).
   Run `python3 measure.py campaign-1`, then `python3 results.py campaign-1`.
   Measurements append/fsync each completed sample and skip finished keys
   on restart, checking source, binary and input hashes. Interrupted sample
   files move aside. Counter running percentage must be 100.
5. Retain raw logs, binaries, outputs, profiles and private fixtures in
   scratch. Commit only portable compact bindings, counts, results and
   documentation. Recheck public text, then remove only the dedicated build
   target after all lane jobs finish and record cleanup.

`REGISTRATION.md` fixes cases, allocation policy, arms and gate interpretation.
`PROOFS.md` gives the bounds and floating-point argument. The geometry driver
crosses the candidate-list threshold and selected topology/attribute/lock/
configuration boundaries in addition to the inherited exact driver. These
finite domains are documented as finite domains, not exhaustive enumeration
of every possible float, layout or configuration product.
