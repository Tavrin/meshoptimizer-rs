# Remap scalar qualification

`REGISTRATION.md` fixes the metric, eight inputs, four arms and ship gates.
`CAMPAIGN-2.md` registers the revision after the rejected first index attempt.
`PROOFS.md` gives the structural argument and final receipts.

Use a private scratch directory, a dedicated Cargo target and CPU-only
admission for all builds, tests and measurements. The caller supplies
`REMAP_SCRATCH`, `CARGO_TARGET_DIR`, and the admission wrapper. Put the
unmodified v0.2.15 source in `$REMAP_SCRATCH/base` and the eight previously
frozen peer fixtures in `$REMAP_SCRATCH/inputs`, with the manifest from
`../0215-basics/inputs.json` in `$REMAP_SCRATCH/inputs.json`.

1. Run `build.py CAMPAIGN` through admission. It freezes sources and the
   adapted peer consumer and builds baseline, candidate, scalar C++ and
   default C++ executables. Source and binary digests bind each campaign.
2. Set `PEERS_ART=$REMAP_SCRATCH/CAMPAIGN`, provide its `inputs` directory
   and `inputs.json`, and set `PEERS_PERF` and `PEERS_CPU`. Run
   `measure.py measured base candidate cpp cpp-default` through admission,
   then `results.py measured`. Completed samples are append-only and
   resumable. Timing is disclosed but does not affect the instruction gate.
3. Set `MESHOPT_LANE_SCRATCH` to a fresh proof directory and
   `MESHOPT_BASELINE=$REMAP_SCRATCH/base`, then run `proof.sh` through
   admission. It compares separately compiled baseline and candidate crates
   in both library feature modes and runs the C++ differential. Source
   binding rejects drift; failed attempts retain their own directories.
4. Retain raw inputs, outputs, logs, executables and frozen source trees in
   scratch. Copy compact portable receipts into the campaign directories.
   Delete the dedicated Cargo target after all final checks finish.

The root tests add one focused case each for error precedence, work
boundaries, duplicate vertex destinations, zeroed owned holes, and untouched
caller holes/tails. `exact.rs` extends the previous remap driver with all
small-case work/byte combinations and additional workspace states;
`cpp-differential.rs` checks valid width, stride, tail and hole cases using
safe oracle wrappers. No boundary allowlist changes are needed.
