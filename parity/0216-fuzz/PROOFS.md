# 0.2.16 pre-release fuzz: receipt

Tree: `train/0.2.16` @ `ff38162` plus the fuzz sources in this directory and the
three repair targets in `parity/0216-repair/fuzz` (no change under `src/`).
Toolchain: nightly `rustc 1.101.0-nightly (282215592 2026-10-04)`, cargo-fuzz
0.13.2, libfuzzer-sys 0.4.13, `-O -a --sanitizer none`. Host: AMD Ryzen 9 7945HX
(x86-64, SSE2/SSSE3/SSE4.1/AVX2 present). Run through a CPU-only job-admission wrapper
with a private `CARGO_TARGET_DIR`.

**Verdict: 15 targets, 47,211 CPU-s, 0 crashes, 0 timeouts, 0 OOM, 0 slow units.
Every target reaches the new or changed code (table below). No source defect was
found, so no fix, regression test or exact-parity rerun was needed.**

## 1. Reachability from the seed corpus

Method: `cargo fuzz coverage <target> <seeds>` (llvm-cov `-C instrument-coverage`
build, seed corpus only, no mutation), `llvm-cov export` JSON and lcov, then every
function with a region on a line added or changed in `git diff -U0 v0.2.15..HEAD`
(`reach.py`). "Regions" are llvm-cov regions summed over generic instantiations;
"entries" are function entry counts over the seeds. Full per-function tables:
`receipts/reach.md`, raw: `receipts/reach/<target>.json`.

| Target | Changed executable lines hit | Functions reached (regions hit/total) |
|---|---|---|
| `weld_vertices` | 123/682 (new file `repair.rs`; the rest belongs to the other two targets) | `weld_vertices` 10/10, `weld_plan` 92/105, `weld_vertices_into` 5/5, `weld_vertices_bound` 3/3, `cell` 5/5 |
| `orient_faces` | 178/682 | `orient_faces` 5/5, `orient_faces_into` 5/5, `orient_faces_bound` 3/3, `orient_plan` 83/103, `edges` 44/50, `groups` 4/4, `apply_flips` 7/7 |
| `fill_holes` | 388/682 | `fill_holes` 5/5, `fill_holes_into` 5/5, `fill_holes_bound` 6/8, `fill_plan` 139/171, `triangulate` 86/156, `mark_disconnected_fans` 63/71, `intersects` 34/34, `fan_root` 11/11, `cross` 10/10 |
| `parallel_clod` | 205/235 | `parallel::compute` 35/46, `parallel::level` 36/40, `parallel::eligible` 5/5, `parallel::fits_serial_work` 20/33, `build_parallel_with_output` 7/14, `prepare_group` 37/42 |
| `sequence_stream` | 79/82 | `IndexSequenceDecoder::new` 13/16, `::push` 11/11, `::push_inner` 52/55, `::finish` 9/9 |
| `vertex`, `simd` | 36/36 | `vertex::decode_owned` 12/12, `simd::vertex_owned` 11/11, `simd::x86::vertex_owned` (SSSE3 stride 12) 9/9, `codec::allocate` 4/5 (`vertex_into` takes the slice path, so it does not reach these by design: 0/36) |
| `remap` | 65/77 | `remap_vertex_packed` 4/8/12/16 20/20, `remap_vertex_records` 4/8/12/16 32/36, `remap_index_gather` 19/26, `remap_index_buffer` 47/63 |
| `simplify` | 9/9 | `simplify_with_update` 5/6, `sparse_run` 4/4, `run` 2/2, `run_state` 2/2 |
| `bounds_diff` | 32/37 | `bounds::arch::roots` 9/9, `bounds::roots` 9/12, `meshlet_util::sqrt8` 4/6, `cluster_bounds` / `cluster_bounds_body` / `meshlet_bounds` 12/12 each (small, medium and large instantiations) |
| `meshlets` | 48/91 (the rest is bounds code, covered by `bounds_diff`) | `simd::spatial::x86_64::sweep` 34/34, `::area` 16/16, `::areas` 6/6, `simd::spatial::areas` 9/14, `Bvh::split` 5/15 |
| `clod` | 57/78 | `build_with_output` 14/21, `build_internal` 12/12, `prepare_group` 36/42, `build_in_context_mode` 22/24 |
| `lod_chain`, `simplify_extra` | 0 by design | they do not call the changed entry points (`lod_chain` uses the chain builder, `simplify_extra` the sloppy / prune / points family) and are included as unchanged-family differentials |

Gaps found first, and fixed in the targets or seeds (the earlier lane's targets
never reached these):

* `weld_vertices`, `orient_faces`, `fill_holes` called only the allocating form
  with a fixed policy. They now also call the `_into` and `_bound` forms with
  short destinations, the `AttributeEquality` weld policy, varying loop-size
  limits, and compare each against the allocating result and against a
  work-limited run (`parity/0216-repair/fuzz/fuzz_targets`). Seeds are hand-built
  meshes with holes, flipped faces and duplicate points (`seeds.py`).
* `remap` always used record size 12 with stride 12 / 16 / 20, so none of the
  tightly packed kernels (stride == size 4, 8, 16) ran. Record size and stride
  are now independent inputs (`diff/src.rs`).
* `simplify` seeds never set `SimplifyOptions::SPARSE`, so `sparse_run` (the
  changed `simplify_with_update` bounds path) had 0 entries. Seeds now cover the
  update family with SPARSE.
* No existing target exercised the SIMD bounds roots against the scalar
  v0.2.15 path: new differential target `bounds_diff`.
* The codec seeds were arbitrary bytes, which the decoders reject at the
  header; `seedgen` writes valid encoded vertex buffers (every stride class,
  counts either side of the 32-vertex kernel switch, versions 0 and 1) and index
  sequences with the crate's own encoders.

Vertex decode at every forced ISA level: `vertex`, `vertex_into` and `simd` run
each input at Scalar, SSE2, SSSE3 and SSE4.1 (`with_level`) and require
identical typed results and meaningful bytes (`fuzz/codec/src.rs`). AVX is not a
dispatch level in `src/codec/simd/dispatch.rs`; NEON and wasm levels are not
available on this machine. Level execution counts from the checkpoint files
(every level ran on every input): `vertex` 6,774,784 x 4 levels in chunk 2;
`vertex_into` 7,036,928 x 4 in chunk 2; `simd` 10,484,736 x 4 in chunk 3 (full
counts in `receipts/receipts.jsonl`, `level_counts_*`), NEON and wasm 0.

## 2. Fuzz under the pre-release rule

Rule: seed corpus, 600 s chunks reusing the corpus, stop when a whole chunk adds
no coverage after at least 600 CPU-s, cap 7,200 CPU-s, 0 crashes, a libFuzzer
timeout counts as a failure. Coverage is libFuzzer `cov` (edges); "initial" is
`INITED` of chunk 1 on the seeds. Seeds: 49 to 2,046 files per target.

| Target | Chunks | CPU s | Initial cov | Final cov | Executions | Crashes | Stop |
|---|---|---|---|---|---|---|---|
| `weld_vertices` | 2 | 1,203 | 292 | 366 | 7,594,716 | 0 | coverage-flat |
| `orient_faces` | 2 | 1,203 | 282 | 406 | 225,228,527 | 0 | coverage-flat |
| `fill_holes` | 2 | 1,202 | 557 | 816 | 69,488,865 | 0 | coverage-flat |
| `parallel_clod` | 2 | 9,705 | 2,200 | 3,317 | 618,880 | 0 | max-cpu |
| `sequence_stream` | 2 | 1,196 | 136 | 156 | 246,788,525 | 0 | coverage-flat |
| `vertex` | 2 | 1,202 | 557 | 743 | 12,079,133 | 0 | coverage-flat |
| `vertex_into` | 2 | 1,133 | 450 | 612 | 11,671,168 | 0 | coverage-flat |
| `simd` | 3 | 1,723 | 763 | 2,478 | 28,804,648 | 0 | coverage-flat |
| `remap` | 3 | 1,802 | 684 | 1,484 | 37,180,765 | 0 | coverage-flat |
| `simplify` | 8 | 4,756 | 2,304 | 5,072 | 3,835,280 | 0 | coverage-flat |
| `simplify_extra` | 4 | 2,361 | 939 | 3,886 | 31,203,336 | 0 | coverage-flat |
| `meshlets` | 4 | 2,364 | 1,135 | 2,783 | 1,665,276 | 0 | coverage-flat |
| `bounds_diff` | 13 | 7,790 | 1,497 | 4,105 | 94,443,115 | 0 | coverage-flat (chunk 13) after the cap was crossed |
| `clod` | 8 | 4,764 | 3,448 | 6,899 | 1,409,083 | 0 | coverage-flat |
| `lod_chain` | 8 | 4,807 | 746 | 2,408 | 7,034,146 | 0 | coverage-flat |

Per-chunk numbers, binary SHA-256 and seeds: `receipts/receipts.jsonl`
(`receipts/build-binding.json` binds the binaries to the build; every chunk of
every target ran the bound binary).

Notes on the stops:

* `parallel_clod` runs rayon pools of 1 to 32 threads in-process, so utime+stime
  grows about 6.6x faster than wall time (3,961 CPU-s in the first 600 s wall
  chunk). It reached the 7,200 CPU-s cap during chunk 2 with coverage still
  creeping (3,293 to 3,317), so it stopped on the cap, not on a flat chunk. The
  cap was overshot by one chunk (9,705 CPU-s) because chunks are not
  interruptible.
* `bounds_diff` kept adding a few edges per chunk (4,075 to 4,105 over chunks 8
  to 12) and was at 7,189 CPU-s after chunk 12, just below the cap, so chunk 13
  ran; it was flat.
* Chunk CPU time is a little below 600 for some chunks (for example `simd`
  chunk 1: 523 CPU-s) because the process was descheduled on the shared host;
  the minimum of 600 CPU-s holds for every target in total.

## 3. Crashes, timeouts

None. Every chunk exited 0; the crash directories are empty. No input needed
the timeout path. Because there was no crash there is no fix, no regression
test and no exact-parity driver rerun (the `src/` tree is unchanged by this PR).

## 4. Incidents (all recorded, none affects a result)

* **Binary-name collision, found and fixed.** The new differential target was
  first named `bounds`, the same binary name as the codec group's `bounds`
  (bound-size functions) target. Both build into one target directory, so the
  codec binary silently replaced it, and 2 chunks (1,204 CPU-s, 1,090,513,511
  executions, 0 crashes, 12 corpus files) fuzzed the codec bound-size functions
  with differential seeds instead. Symptom: 30 edges regardless of seed. The
  target was renamed `bounds_diff`, rebuilt, its reachability measured (32/37)
  and fuzzed in full. The misrun's receipts are kept in
  `receipts/receipts-bounds-codec-misrun.jsonl` and its corpus as
  `bounds-codec-misrun/`; it counts for nothing in the table above. The other 14
  binaries were checked by symbol name and by SHA-256 against the build binding.
* **Coverage builds overwrite fuzz binaries.** `cargo fuzz coverage` writes to
  `<target-dir>/<triple>/release/` too. `fuzz.py cover` now uses its own target
  directory (`$CARGO_TARGET_DIR/cov`); the fuzz binaries were rebuilt after the
  first coverage pass and before any fuzzing.
* **Driver restart.** Eight targets were run by a second driver process in
  parallel; the first driver then started duplicate chunks of weld_vertices and
  orient_faces, which were killed after about two minutes (no receipt, no
  effect; corpus files they added are in the corpora). `fuzz.py` now skips a
  target whose receipts already satisfy the rule.

## 5. Corpora

Grown corpora (never deleted): `corpus/<target>/` under the scratch directory,
56,917 files, 61 MB apparent size, copied to
`fuzz-corpora/meshoptimizer-rs-0.2.16/<target>/` on the NAS. No symlinks in
either tree. Verified: file count, apparent size per directory, and the sha256
of the sorted `(path, size)` list and of the sorted per-file sha256 list are
identical on both sides.

## 6. Not verified

* Only x86-64 levels Scalar, SSE2, SSSE3, SSE4.1 ran. NEON, wasm simd128 and
  the aarch64 vertex/bounds/spatial kernels (`src/codec/simd/aarch64.rs`) were
  not fuzzed. AVX is not a level of this crate.
* `bounds_diff` compares the candidate with v0.2.15 at the detected level (SSE4.1
  here); it does not force Scalar against SIMD for the roots kernel. The scalar
  v0.2.15 result is the reference, which is the stronger statement for exact
  parity.
* No sanitizer (ASan/MSan/UBSan) build; `-a` gives debug assertions and
  overflow checks only. Memory errors in the SIMD modules' `unsafe` would show
  as wrong output or a segfault, not as a sanitizer report.
* `parallel_clod` stopped on the CPU cap while coverage was still growing
  slightly; `bounds_diff` and `simplify` also grew until late chunks.
* No-default-features, no_std and wasm builds of the targets were not fuzzed
  (the codec group enables `std`, `simd`, `parity-internals`; the repair group
  builds with default features off).
* Reachability tables come from the seed corpus only; later mutation reaches more
  (final coverage above), and the lines still unhit were not enumerated.
