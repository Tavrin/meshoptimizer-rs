# 0.2.17 pre-release fuzz: receipts

Tree: `train/0.2.17` @ `c08049f1` plus the `fuzz/` sources of this branch (no change under `src/`).
Toolchain: nightly `rustc 1.101.0-nightly (282215592 2026-10-04)`, cargo-fuzz 0.13.2,
libfuzzer-sys 0.4.13, `-O -a --sanitizer none`, `-rss_limit_mb=512`, `-timeout=60`. Host: AMD Ryzen 9
7945HX (SSE2 through AVX-512 VBMI/VBMI2 present, so every x86 dispatch level runs). Driver:
`fuzz/driver.py` (rule: 600 s chunks reusing the corpus; stop at the first chunk that adds no
coverage after at least 600 CPU-s; cap 7200 CPU-s; a libFuzzer timeout is a failure). CPU time is
the child's utime+stime from `/proc`, so multi-threaded targets reach the cap in less wall time.

**Verdict: 18 targets, 62,963 CPU-s, 0 crashes, 0 mismatches, 0 timeouts, 0 OOM; every chunk exited 0.**
Four targets stopped at the 7200 CPU-s cap with coverage still growing in the last chunk
(`character`, `process`, `parallel_equiv`, `parallel_clod`); the other 14 stopped on a flat chunk.

## Results

| Target | CPU s | Executions | Cov start | Cov end | Chunks | Stop reason | Failing chunks |
|---|---|---|---|---|---|---|---|
| `hidden` | 1203 | 467,151 | 825 | 1104 | 2 | coverage-flat | 0 |
| `dag_reader` | 2402 | 383,611,068 | 390 | 573 | 4 | coverage-flat | 0 |
| `character` | 7205 | 215,800 | 1365 | 2006 | 12 | max-cpu | 0 |
| `process` | 7764 | 139,129 | 3579 | 6074 | 13 | max-cpu | 0 |
| `parallel_equiv` | 12456 | 21,024 | 1741 | 2121 | 2 | max-cpu | 0 |
| `width_equiv` | 3602 | 678,912 | 4239 | 5373 | 6 | coverage-flat | 0 |
| `simplify_meter` | 1201 | 3,605,080 | 1305 | 1615 | 2 | coverage-flat | 0 |
| `lod_quality` | 2399 | 25,553,945 | 45 | 206 | 4 | coverage-flat | 0 |
| `repair4` | 1207 | 4,321,527 | 859 | 1458 | 2 | coverage-flat | 0 |
| `buffer_stream` | 1207 | 4,434,111 | 754 | 1056 | 2 | coverage-flat | 0 |
| `sequence_stream` | 1205 | 300,874,026 | 119 | 137 | 2 | coverage-flat | 0 |
| `vertex` | 1206 | 6,977,992 | 920 | 1301 | 2 | coverage-flat | 0 |
| `vertex_into` | 2412 | 20,155,906 | 798 | 1177 | 4 | coverage-flat | 0 |
| `simd` | 1208 | 11,749,533 | 1401 | 3529 | 2 | coverage-flat | 0 |
| `vertex_encode` | 1205 | 27,080,363 | 664 | 718 | 2 | coverage-flat | 0 |
| `vertex_encode_into` | 1203 | 24,582,918 | 664 | 724 | 2 | coverage-flat | 0 |
| `parallel_clod` | 12073 | 857,475 | 2236 | 3419 | 2 | max-cpu | 0 |
| `build_meshlets_spatial` | 1804 | 11,172,715 | 58 | 718 | 3 | coverage-flat | 0 |

Coverage is libFuzzer edge count (`cov:`); start is `INITED` on the seed corpus. Per-chunk records
(seed, binary hash, exit, crashes) are in `receipts.jsonl` in the run scratch directory; binary hashes in
`build-binding.json` there. No crash artifact exists in any target's `crashes/` directory.

## Targets and what they assert

| Target | Group | Covers | Kind |
|---|---|---|---|
| `hidden` | `fuzz/r0217` | `remove_hidden_triangles` (+ `_into`, u16), outside/point/box views, tolerance, pixel size | alloc vs `_into` vs u16 equality, report arithmetic, retained triangles are a sub-multiset of the input, workspace reuse, work metering (exact usage succeeds, one less fails with `LimitExceeded`) |
| `dag_reader` | same | cluster-LOD DAG `Reader::read`, `decode_page` (u32 and u16), record accessors | a file `read` accepts must decode every page; u16 equals u32; truncated pages error |
| `character` | same | `simplify_character`, `build_character_lod_chain` (plain and best-of), skin with 4 or 8 influences and up to 48 bones, blend shapes, joint poses | determinism, index bounds, best-of winner score never above the loser's, selections length |
| `process` | same | `process_mesh` with every stage toggle, skin, attributes, meshlets, clusters, index and vertex encoding | determinism (timings excluded), level bounds, encoded vertices decode to the level, exact counted work reproduces the result |
| `parallel_equiv` | same | `encode_vertex_buffer_parallel`, `decode_vertex_buffer_parallel` (valid and corrupted streams), `build_meshlets_spatial_parallel` (grids to 63x63) on the global pool and pools of 1, 2, 4, 8 | equality with the serial functions, results and counted work |
| `width_equiv` | same | `_generic::<u16>` vs `::<u32>` of simplify, sloppy, prune, points, vertex cache (plain, fifo, strip), overdraw, sanitize, analyze, orient, fill holes, meshlets, spatial meshlets, split non-manifold, stitch cracks, LOD chain | equal results and errors; cache optimizers keep the triangle set; in-place equals allocating |
| `simplify_meter` | same | `simplify`, `simplify_sloppy`, `simplify_prune` under limits | exact usage as the work limit gives the identical result and work, one less gives `LimitExceeded` |
| `lod_quality`, `repair4` | same | copies of the targets from `parity/0217-lodq`, `parity/0217-repair4` | invariants |
| `buffer_stream`, `sequence_stream` | `fuzz/codec` | streaming vertex, triangle and index-sequence decoders in chunks | equality with the one-shot decoders |
| `vertex`, `vertex_into`, `simd` | `fuzz/codec` | vertex decode forced to Scalar, SSE2, SSSE3, SSE4.1, AVX2 and AVX-512 | equality across levels (the harness previously stopped at SSE4.1; AVX2 and AVX-512 added) |
| `vertex_encode`, `vertex_encode_into` | `fuzz/codec` | parallel-changed encoder | encode then decode round trip |
| `parallel_clod`, `build_meshlets_spatial` | `fuzz/parallel-clod`, `fuzz/p03` | parallel vs serial cluster LOD; spatial meshlet invariants | equality / invariants |

## Reach on the seed corpus

`cargo fuzz coverage` on the seeds, `llvm-cov report` region coverage per source file (percent of regions;
only files the target is meant to reach):

| Target | New-code files reached |
|---|---|
| `hidden` | `hidden.rs` 90.4 |
| `dag_reader` | `clusterlod/dag_format.rs` 51.6 (error branches are the rest) |
| `character` | `character.rs` 77.5, `lod_quality.rs` 66.5, `lod_chain.rs` 60.2, `simplify.rs` 72.9 |
| `process` | `pipeline.rs` 84.3, `character.rs` 62.4, `repair/topology.rs` 55.6, `repair.rs` 61.6, `codec/index_encode.rs` 76.6, `codec/vertex_encode.rs` 61.1 |
| `parallel_equiv` | `meshlet_spatial.rs` 81.4, `simd/spatial.rs` 93.8, `codec/vertex_encode.rs` 96.9, `meshlet_division.rs` 94.1, `parallel.rs` 7.4, `codec/vertex_parallel.rs` 2.8 |
| `width_equiv` | `cache.rs` 51.6, `repair/topology.rs` 81.2, `overdraw.rs` 66.3, `simplify_extra.rs` 52.7, `meshlet.rs` 53.3 |
| `simplify_meter` | `simplify.rs` 53.9, `simplify_extra.rs` 45.2 |

## Not established

* The concurrent path of `decode_vertex_buffer_parallel` is not reachable by fuzzing: it needs at least
  48 MiB of output (32 MiB for strides of 8 or less), at least 4 workers and 8 blocks, which is too large per
  execution. The fuzz target only exercises the serial fallback of the decoder (equal by construction), so
  the concurrent decode has no fuzz evidence from this run.
  `codec/vertex_parallel.rs` reaches 2.8 percent in the table above for this reason.
* Four targets hit the 7200 CPU-s cap with coverage still growing; the rule's flat-coverage stop was not
  reached for them.
* No sanitizer (`--sanitizer none`), as in 0.2.16: debug assertions and overflow checks are on.
* NEON and wasm dispatch levels and non-x86 hosts were not run.
* Mismatch-detection power of `simplify_meter`/`hidden` metering claims was not mutation-tested.

## Reproduce

```sh
python3 fuzz/seeds.py $CARGO_TARGET_DIR/release/meshopt-0217-seedgen   # after cargo build --release in fuzz/seedgen
PARITY_RUNNER_ACTIVE=1 FUZZ_SCRATCH=... CARGO_TARGET_DIR=... python3 fuzz/driver.py build
PARITY_RUNNER_ACTIVE=1 FUZZ_SCRATCH=... CARGO_TARGET_DIR=... FUZZ_JOBS=4 python3 fuzz/driver.py run
```
