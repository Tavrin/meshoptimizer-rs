# Intra-mesh parallel cluster LOD qualification

The baseline is v0.2.15, revision `1ecf69abe9f92c17e13dc38fa97be659aee44582`.
Only the two new opt-in entry points dispatch groups. Existing builders and
independent-mesh batches keep serial behavior within each mesh.

Run `bash parity/0216-par-clod/checks.sh` through the required CPU admission
wrapper, with a dedicated `CARGO_TARGET_DIR`, `MESHOPT_INPUTS` (the eight frozen
peer fixtures) and `MESHOPT_REFERENCE` (the pinned C++ 1.3 checkout). The runner
reads the baseline through `git archive`; it never changes Git state. Scratch
and executable files live below the dedicated target and are disposable after
retaining the logs and binding. `build.py` and `measure.py` are restartable;
`counts.jsonl` appends and fsyncs each completed sample. Source or binary drift
fails before measurements.

`exact.rs` compiles separately named baseline and candidate crates. SIMD-on and
SIMD-off modes both include `parallel` (and hence `std`) and `clusterlod`.
The no-default-feature crate is separately tested without parallel, since the
parallel feature requires std. The exact driver enumerates every integer work
limit on bounded meshes, storage boundaries, invalid input and configuration,
callback failures, cold/warm/after-failure workspaces, and larger multi-group
cases under 1/2/4/8/32 workers. `parity-internals` enables reverse and zigzag
worker delays and a forced speculative failure. Production builds omit these
hooks. LOD fixtures and the eight frozen peer meshes repeat the full worker
and successful schedule matrix, comparing every meaningful float bit, not
float equality or debug formatting.

The performance driver adapts the existing peer consumer in scratch. Its five
arms are baseline, serial candidate, four-worker candidate, scalar C++ 1.3,
and default C++ 1.3 (SIMD enabled). The C++ dependency's core and demo source
bytes must match the pinned reference. Input loading, warmup and serialization
are outside counters. Three repetitions of `instructions:u` alternate arm
order; `perf` inherits worker counters. Five interleaved wall repetitions on
the three real meshes use four distinct pinned physical cores for parallel
execution and one of those cores for single-thread execution. Wall time is a
disclosure, not a speed gate. The host is shared; pinning does not establish
exclusive core occupancy.

Fuzz source is provided under `fuzz/parallel-clod/`; this lane does not run it.

After the primary runner, run `bash parity/0216-par-clod/followup.sh` under CPU
admission. It compiles (but does not execute) the fuzz source, checks the true
no_std cluster-LOD configuration, and derives prefix observers from `exact.rs`
for every fixture and worker count. It also compares the full LOD corpus with
both C++ arms. `collect.py` retains normalized logs, emits the proof summary and
tables, and removes the dedicated target. Run the public-text check last.
Resume measurement samples only within an unchanged build/input campaign; a
new source revision requires a fresh evidence campaign.
