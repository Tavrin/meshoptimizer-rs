# 0.2.16 pre-release fuzz

Coverage-guided libFuzzer targets (cargo-fuzz 0.13.2, nightly, `-O -a`: release
optimisation with debug assertions and overflow checks, coverage and
trace-compares, no sanitizer) for the code added or changed since v0.2.15.
Results: `PROOFS.md`; raw per-chunk receipts: `receipts.jsonl`; reachability:
`reach/*.json`.

| Target | Group | What it exercises | Kind |
|---|---|---|---|
| `weld_vertices` | `parity/0216-repair/fuzz` | `weld_vertices` (positions-only and attribute-equality policy), `_into`, `_bound`, work limit | invariants + alloc/into agreement |
| `orient_faces` | same | `orient_faces`, `_into`, `_bound`, short destination, work limit | invariants + alloc/into agreement |
| `fill_holes` | same | `fill_holes`, `_into`, `_bound`, loop-size limit, work limit | invariants + alloc/into agreement |
| `parallel_clod` | `fuzz/parallel-clod` | `build_parallel_with_output` vs `build_with_output`, worker counts 1-32, limits, callbacks, reused workspace | differential (serial vs parallel) |
| `sequence_stream` | `fuzz/codec` | `IndexSequenceDecoder` chunked pushes vs `decode_index_sequence` | differential |
| `vertex`, `vertex_into`, `simd` | `fuzz/codec` | `decode_vertex_buffer` (+ `_into`) forced to Scalar, SSE2, SSSE3 and SSE4.1 and compared with scalar; `simd` also covers the other decoders | differential across ISA levels |
| `remap` | `diff` | `remap_vertex_buffer` (+ `_into`), `remap_index_buffer` (+ `_into`), `generate_vertex_remap`; record sizes 1, 4, 7, 8, 12, 16, 20, 24 and strides equal to or above the size | differential vs v0.2.15 |
| `simplify` | `diff` | `simplify`, `simplify_with_attributes` (+ `_into`), `simplify_with_update` incl. the SPARSE option | differential vs v0.2.15 |
| `simplify_extra` | `diff` | sloppy, prune, points | differential vs v0.2.15 |
| `meshlets` | `diff` | `build_meshlets`, `_flex`, `_spatial` (SIMD area sweep) | differential vs v0.2.15 |
| `bounds_diff` | `diff` | cluster, meshlet and sphere bounds with and without the cached positions handle (SIMD roots) | differential vs v0.2.15 |
| `clod` | `diff` | `clusterlod::build_with_output` | differential vs v0.2.15 |
| `lod_chain` | `diff` | `build_lod_chain` | invariants |

The `diff` group (`diff/`, copied from `parity/0215-qualify/fuzz`) builds against
two path dependencies: `candidate` (this tree) and `base` (v0.2.15 with the
package renamed `meshoptimizer-baseline`, `git archive v0.2.15`). Each input runs
on both, cold then warm on one workspace, and the Debug form of every result,
every caller buffer and the workspace usage must match. The `bounds_diff` target
is new (the cargo-fuzz binary name `bounds` is taken by the codec group's
bound-size target; both build into one target directory, so the names must be
distinct).

AVX is not a dispatch level of this crate (`src/codec/simd/dispatch.rs`: Scalar,
Sse2, Ssse3, Sse41, plus Neon and Wasm on other architectures), so the x86
levels exercised are exactly those four.

## Rule (owner, 2026-10-30)

Start from the seed corpus. Run in 600 s chunks, reusing the target's corpus.
Stop a target when a whole chunk adds no coverage after at least 600 CPU
seconds; cap 7,200 CPU seconds; zero crashes. A libFuzzer timeout counts as a
failure. CPU time is the child's utime+stime from `/proc` (so a multi-threaded
target such as `parallel_clod` consumes its cap in fewer wall seconds).

## Commands

Inside the CPU-only admission wrapper (`moss-heavy.sh 4`, `MOSS_HEAVY_GPU=0`),
with `FUZZ_SCRATCH` (holding `base/`) and a private `CARGO_TARGET_DIR`:

```sh
python3 parity/0216-fuzz/seeds.py SEEDGEN_BIN        # seed corpora (seedgen: cargo build --release in seedgen/)
parity/0216-fuzz/run.sh prepare [TARGET...]          # build every group, then `cargo fuzz coverage` on the seeds
python3 parity/0216-fuzz/reach.py TARGET...          # changed-line and per-function reachability (reach/)
parity/0216-fuzz/run.sh run [TARGET...]              # the rule above; receipts.jsonl, grown corpora in corpus/
```

`seeds/diff` holds the 174 cropped-mesh seeds of the 0.2.15 harness; `seedgen`
writes valid encoded vertex buffers and index sequences with the crate's own
encoders (the codec targets reject almost everything else at the header).
