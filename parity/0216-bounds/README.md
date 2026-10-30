# Bounds instruction experiment

See `PROOFS.md` for final scope, rejected KD prototypes, limits and receipts;
`TABLE.md` contains instruction ratios and wall-time disclosure.

Run every build, profile, measurement and proof through the prescribed CPU
admission wrapper with a dedicated `CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`,
`CARGO_BUILD_JOBS=2`, `LC_ALL=C`, GPU admission disabled, and `BOUNDS_SCRATCH`
pointing to fresh external evidence. Input files are shared read-only through
`BOUNDS_INPUTS` (with adjacent `inputs.json`). Set `PEERS_PERF` to the prescribed
perf executable and `MESHOPT_REFERENCE` to the pinned C++ checkout. Admission
must remain active until the complete workload finishes.

1. Run `python3 parity/0216-bounds/build.py base-default base-scalar cpp-scalar cpp-default`.
   This freezes v0.2.15, builds reference arms, freezes actual baseline meshlet
   topology, and profiles the five target operations. Symbol and source-line
   reports stay beside raw perf data. No Git mutation occurs.
2. Make the profile-guided edit, then run
   `python3 parity/0216-bounds/build.py candidate3-default candidate3-scalar`.
   Existing arm directories are immutable: use new names for experiments.
3. Run `python3 parity/0216-bounds/measure.py final base-default base-scalar candidate3-default candidate3-scalar cpp-scalar cpp-default`.
   This collects three interleaved instruction samples for all five operations
   and eight frozen inputs, with identical output required across every arm.
   Both scalar arms allow separate packed-access and SIMD attribution.
4. Run `python3 parity/0216-bounds/results.py final` and
   `python3 parity/0216-bounds/gate.py final`. The gate rechecks raw perf counts,
   100% counter running, output identity, completeness and binary hashes.
5. Run `bash parity/0216-bounds/run.sh proof`. This performs final-source tests,
   exact baseline comparisons in both feature modes, larger geometry checks,
   unsafe/package boundary checks, pinned C++ differential, separately executed
   scalar/SIMD wasm, and an AArch64 compile check. Native execution needs its
   corresponding architecture.
6. Supplemental commands used here, within the same admitted CPU scope:

   ```sh
   cargo +1.88 test --offline --locked --lib codec::simd::bounds::tests
   cargo +1.88 check --offline --locked --no-default-features
   cargo +nightly miri test --offline --locked --lib codec::simd::bounds::tests
   RUSTFLAGS=-Zsanitizer=address cargo +nightly test --offline --locked --target x86_64-unknown-linux-gnu --lib codec::simd::bounds::tests
   ```

`exact.rs` compares errors, work/storage Usage, full caller buffers, callback
records, position bits, and every meaningful bounds field. Small-case sweeps
visit every work limit through success + 1 in cold, repeated-query,
after-failure and after-other-family workspaces. Larger cases cover lane,
stack-bucket and local-vertex boundaries. Finite testing supplements structural
arguments; compile-only evidence does not establish native execution parity.

Retain source/binary bindings, raw samples, all attempts and proof receipts
outside the target. Recheck final source bindings and run
`parity/tools/check_public_text.sh`, then delete only the dedicated Cargo target
directory. Leave all edits uncommitted for coordinator integration.
