# Instruction campaign registration (written before the campaign)

Function: `simplify_with_attributes` only (the `simplify-attributes`
operation of the parity/peers consumer: four synthetic floats per vertex,
unit weights, half the index count, error 0.5).

Arms: base (v0.2.14), candidate (this branch), C++ 1.3 (`meshopt` 0.7.0
FFI), optimesh 1.1.0. The C++ and optimesh drivers are the retained 0216
continuation binaries (`driver-cpp` sha256
`1c96e2dd84bf3aaad71730582641739ef945e9e2eb3656b1d58be69268223800`,
`driver-rust` sha256
`b79483a33c932fb6858086ee9d3664f3a36550f3186be238c8a0fd5a1a986064`); they
do not link this crate. Base and candidate are built by `build.sh` from the
v0.2.14 tree and the candidate tree, each with the consumer source the 0216
continuation measured (`parity/peers/src/main.rs` after `prepare_driver.py`
and `additional_driver.py`).

Workloads: the eight frozen inputs of `parity/0216-qualify/inputs.json`
(five corpus meshes, three real LOD-chain meshes). Metric: `perf stat -e
instructions:u` (perf 6.17) over the counted loop only (`-D -1` with control
descriptors), CPU-pinned, median of three interleaved repetitions (arm order
reversed on the middle repetition). Runner: `measure.py`, resumable, one
record per sample.

Gate, applied verbatim:
- SHIPS if the three-real-mesh geometric mean of candidate/optimesh is at
  most 0.99 and that of candidate/base is at most 0.97.
- IMPROVES if only candidate/base is at most 0.97; NO-GAIN otherwise.
- Any case (all eight inputs) with candidate/base above 1.01: revert the
  change responsible.
- Base and candidate output hashes must match in every cell.
- No code edits after the campaign except reverts. One campaign.
