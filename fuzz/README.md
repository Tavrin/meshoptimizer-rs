# Stable fuzz smoke

`parity/fuzz.sh --seconds 600 --seed 20261002` builds and runs the five stable
bounded mutation targets concurrently for ten elapsed minutes each. The
machine has no installed cargo-fuzz/nightly toolchain, so this is the spec's
stable fallback. It has no libFuzzer coverage guidance or sanitizer
instrumentation and does not satisfy the later 24-CPU-hour release gate.

Inputs vary triangle lengths, indices, finite and arbitrary float bits,
thresholds, layouts, flags and storage/work limits. Successful calls must
preserve optimizer triangle multisets, simplifier index bounds and error
finiteness, resource limits, caller-buffer tails and variant output identity. Failed in-place calls must preserve their input.
The harness itself bounds allocation to small buffers.

The record in `parity/results/fuzz.json` includes per-target execution and
success counts, elapsed/user/system CPU time, maximum RSS, seed, captured
output, and pre/post source, dependency and executable identities. Crashes
and invariant failures stop the target and retain its stderr. The panic hook
prints the target, seed and iteration. Reproduce the prefix with the built
`vertex_cache` or `overdraw` binary's arguments: elapsed seconds, seed, and
maximum executions. A nonzero initial seed is required.


The cluster milestone adds the safe `build_dag_into` target. Run it with the
same CPU-accounted release harness:

```sh
parity/fuzz.sh --target build_dag_into --cpu-hours-per-target 4 --jobs 8 --seed 20261025
```

This target mutates indexed meshes, finite/arbitrary position bits, canonical
configs, hierarchy widths, workspace limits and each caller-buffer capacity.
It includes too-small buffers and checks successful prefixes, index/source and
record ranges, untouched tails, external guards and workspace accounting.
The stable seeded harness retains replay seeds and execution counts; it has no
coverage guidance or sanitizer instrumentation.

`parallel-clod/target.rs` is the separate `parallel_clod` libFuzzer source for
intra-mesh cluster LOD identity. It bounds meshes and topology, varies float
bits, limits, dilation, callbacks and worker counts, and compares exact output
words, errors, usage and mutable positions on reused workspaces. This feature
lane adds the source only; execution belongs to the dedicated fuzz lane.

## 0.2.17 coverage-guided targets

`fuzz/r0217` (new targets for hidden-surface removal, the DAG reader, character simplification,
`process_mesh`, parallel-vs-serial equality, u16-vs-u32 equality and simplify metering, plus copies of
`lod_quality` and `repair4`), `fuzz/codec` (streaming decoders, ISA levels up to AVX-512) and
`fuzz/driver.py` (build, cover, run under the 0.2.17 rule). Seeds: `python3 fuzz/seeds.py SEEDGEN`
regenerates `fuzz/seeds/` (the codec seeds come from `fuzz/seedgen` and are large, 26 MB; they are
reproducible and need not be committed). Results: `parity/0217-fuzz/RECEIPTS.md`.

## 0.2.20 MDAG 2 reader

`fuzz/r0220` holds `dag_format_v2`: each input goes through `v2::Reader::open` (and the transcoding
of every page its records name) and `v2::Reader::read_all`, once as given and once with its page
and resident hashes recomputed so that mutations reach the structural rules. Accepted files must
reopen from their resident prefix and transcode every page from exactly its bytes. For every file
`open` accepts, the pinned prefix, the whole page table and two input-derived dependency-closed page
sets must give valid cuts (`cut::refine_sequential` never `Incomplete`, equal to `cut::select_cut`,
every parentless group resident, exactly one selected cluster along every source-to-root chain) at
`Threshold::Finest`, orthographic thresholds, and perspective thresholds at and immediately below
projected group errors, with input-derived ordinary/far cameras and the far-camera regression.
Each input that opens is also rerun with three resealed edits: one `PDEP` entry replaced, one parent
count raised and one `CREF` entry changed. Seeds in
`fuzz/seeds/dag_format_v2/` come from
`DAG_V2_SEED_DIR=fuzz/seeds/dag_format_v2 cargo test --release --features clusterlod --test dag_format_v2 empty_mesh_and_seeds`.
Build and run with `FUZZ_GROUPS=r0220 FUZZ_SANITIZER=address fuzz/driver.py build` and
`ASAN_OPTIONS=quarantine_size_mb=16 fuzz/driver.py run dag_format_v2` (same rule as 0.2.17).
