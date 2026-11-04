# Results: MDAG 1 (current crate) against Bevy meshlets

First table, 2026-11-04. Crate at `cf119a96` (0.2.17 plus the 0.2.18 impostor work), MDAG 1 container,
`clusterlod::build_parallel_with_output` plus `build_hierarchy` (width 8, 128 triangles per cluster).
Reference: `MeshletMesh::from_mesh` at Bevy commit `8cc060f01c3d4d5920781124cc8c32013bb47730`,
quantisation factor 4, saved with its own asset saver. Method and definitions: `README.md`.
Regenerate the tables with `python3 -I parity/vg-bench/report.py --summary <work>/*.jsonl`.

Host: AMD Ryzen 9 7945HX (16 cores, 32 threads), performance governor, shared with other jobs while
the runs were admitted through the machine queue. Single-thread rows are pinned to one core, the eight-thread
rows to cores 2-9 (four physical cores with SMT siblings). Five interleaved repeats per cell; the
parenthesised value is max minus min. Noise is large (up to 40% of the median on the small meshes, and
the largest rows moved by more than a second), so ratios below are indicative; the ordering is not
in doubt for build time, memory and file size.

Corpus: the public set in `CORPUS.md`, each fitted to 2 m (longest side), normals and box-projected UVs
derived where absent. Five of the seven corpus meshes are in the table; Lucy (28M triangles) is fetched and
hash-checked but not built: the reference needs roughly 0.8 KiB of resident memory per source triangle,
about 22 GiB for Lucy, above the lane's memory budget. Reversal cost: free (run `bench.sh` on it with
`VG_REPEATS=1`, more memory, and the `--no-cuts` evaluation). The Khronos multi-material asset is fetched
for the attribute and material lane and not used here. Dragon is the `dragon_vrip` reconstruction.

## Build time and memory

Build plus serialise (ours: DAG, hierarchy and MDAG 1 write; reference: `from_mesh` and save), median
of five, milliseconds. RSS is the whole-process peak from `/usr/bin/time -v` (input load included).

| Mesh | Triangles | ours 1T | bevy 1T | ours/bevy | ours 8T | bevy 8T | ours/bevy | ours RSS MiB (8T) | bevy RSS MiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 69,451 | 118 (+-53) | 516 (+-95) | 0.23 | 77 (+-9) | 511 (+-52) | 0.15 | 31 | 55 |
| dead_tree_trunk_02 | 83,128 | 143 (+-25) | 659 (+-46) | 0.22 | 92 (+-13) | 632 (+-34) | 0.14 | 35 | 74 |
| namaqualand_boulder_02 | 97,964 | 230 (+-60) | 993 (+-219) | 0.23 | 112 (+-19) | 783 (+-22) | 0.14 | 38 | 79 |
| coast_rocks_01 | 679,936 | 1,410 (+-448) | 9,464 (+-1,984) | 0.15 | 735 (+-205) | 7,571 (+-2,461) | 0.10 | 151 | 477 |
| dragon | 871,414 | 1,643 (+-236) | 11,661 (+-1,086) | 0.14 | 1,012 (+-147) | 11,344 (+-393) | 0.09 | 198 | 555 |

The reference gains little from eight workers (its simplification is parallel per group, the rest serial);
ours gains 1.5x to 2x because the MDAG 1 write is serial and counted. Build alone (without the write) is
roughly 0.12 to 0.19 of the reference at one thread and 0.07 to 0.11 at eight.

## Size and shape

Bytes per DAG triangle use the triangles over all levels (about twice the source count in both).
Ours resident is MDAG 1 tables plus decoded f32 geometry with source IDs, which is the number MDAG 2 is meant to
reduce; the reference stores quantised, bit-packed positions.

| Mesh | file B/tri ours | bevy | file zstd-19 B/tri ours | bevy | resident B/tri ours | bevy | total/leaf ours | bevy | tris/cluster mean ours | bevy | p5 ours | bevy | hierarchy depth ours | bevy |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 10.6 | 13.0 | 8.4 | 8.9 | 23.1 | 13.8 | 2.00 | 2.00 | 122.0 | 119.1 | 67 | 102 | 3 | 5 |
| dead_tree_trunk_02 | 10.8 | 13.8 | 8.7 | 9.1 | 23.7 | 14.5 | 2.00 | 1.99 | 119.6 | 119.4 | 53 | 101 | 3 | 5 |
| namaqualand_boulder_02 | 10.8 | 13.9 | 8.7 | 9.3 | 23.6 | 14.7 | 2.00 | 1.99 | 120.8 | 119.9 | 58 | 101 | 3 | 5 |
| coast_rocks_01 | 10.6 | 12.5 | 8.1 | 7.9 | 23.2 | 13.2 | 2.00 | 2.00 | 121.9 | 118.9 | 64 | 101 | 4 | 6 |
| dragon | 10.1 | 11.7 | 7.5 | 7.9 | 23.1 | 13.3 | 2.00 | 2.00 | 121.8 | 118.9 | 64 | 101 | 4 | 6 |

Pages (ours only): 11 to 128 pages of 128 KiB per mesh, mean fill 0.925 to 0.994, one terminal group per
mesh (bunny: 11 pages, fill 0.967; dragon: 128 pages, fill 0.994). Page decode: 660 to 710 MB/s of decoded
geometry on one core (median page; bunny's earlier smoke run read 603, the full run 680). The reference's whole-asset LZ4 decode runs at 2.8 to 5.6 GB/s of decompressed bytes
(bunny, boulder). Disk: ours is smaller as a file, nearly equal after `zstd -19` (7.5 to 9.3 B per DAG
triangle for both), so the file-size gap is the entropy coder, not the layout. Resident is 1.6x to 1.8x the
reference. MDAG 2 targets are not measured here.

## Cut efficiency and traversal

64 Fibonacci directions x spans 64, 128, 256, 512, 1024, 2048 px x thresholds 0.5, 1, 2, 4 px (viewport 2048 px,
60 degrees); deviation from `measure_lod_quality` converted at the nearest depth, so it is a worst case.
"tris ours/bevy at p95 dev" is the mean cut triangle count of ours divided by the reference's at equal p95
of the per-view maximum deviation, log-log interpolated along the four thresholds (n/a when the curves do
not overlap); below 1 favours ours. Traversal columns are the 1 px rows, averaged over all views, with
frustum culling and no occlusion culling.

| Mesh | tris ours/bevy at p95 dev 5 px | at 8 px | measured/claimed median ours | bevy | nodes ours | bevy | clusters tested ours | bevy | drawn ours | bevy | quantisation floor p95 px (bevy, source cut) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | 0.49 | 0.62 | 1.73 | 1.31 | 31.9 | 10.7 | 214 | 372 | 105 | 179 | 2.76 |
| dead_tree_trunk_02 | 0.71 | 0.83 | 4.59 | 1.21 | 35.4 | 14.4 | 293 | 590 | 142 | 285 | 3.52 |
| namaqualand_boulder_02 | 1.16 | 1.16 | 4.34 | 1.14 | 46.9 | 18.0 | 499 | 821 | 243 | 395 | 3.42 |
| coast_rocks_01 | 0.59 | 0.43 | 2.12 | 1.28 | 50.2 | 31.5 | 371 | 1,123 | 185 | 557 | 2.86 |
| dragon | n/a | n/a | 13.91 | 1.75 | 58.8 | 36.0 | 466 | 1,146 | 233 | 574 | 3.40 |

Self-checks: 30 run, 0 failed (threshold 0 selects exactly the source triangles and only leaves; a huge
threshold selects only the root set; the hierarchical traversal equals the flat cut at 1 px; per builder
per mesh). They guard the port of the reference cut rule and our selector.

## Against the section 6 bar

- Build time at most half of the reference at equal thread count: met on every mesh at 1 and 8 threads
  (0.09 to 0.23 including the MDAG 1 write).
- Memory bytes per DAG triangle at or below the reference: not met. Ours is 23 resident B per DAG triangle against
  13 to 15. File bytes per DAG triangle are below the reference (10 to 11 against 12 to 14).
- Fewer cut triangles at equal measured p95 deviation: met on bunny, trunk, rocks (0.43 to 0.83 at 5 and 8 px),
  not met on the boulder (1.16). Not decidable on dragon (see below).
- Measured over claimed error: ours 1.7 to 4.6 on the five small meshes against 1.1 to 1.3 for the reference.
  Ours claims less error than the cut measures, so a pixel threshold under-delivers by that factor; the
  reference's claim is much closer to the measure. This is the weakest point for the cut contract.

## Open findings

- Dragon: ours shows a constant p95 maximum deviation of 65 px across thresholds 1 to 4 at the 2048 px span
  and a measured/claimed median of 13.9, which is not what a monotone cut should do. Not diagnosed here: the
  candidates are removal of small disconnected fragments by the simplifier's pruning (error claimed against
  component size, measured against distance to the rest), or a non-monotone bound sphere in a group. The
  self-checks do not prove the cut covers the surface exactly once. The first step is to print the
  one-sided source-to-level and level-to-source distances for one dragon view; L1's containment checks should
  settle the second candidate. Neither is a builder change in this lane.
- The reference's position quantisation (1/16 cm, factor 4) sets a floor of 2.8 to 3.5 px at 2048 px span on
  these 2 m meshes (last column). Comparing deviations below that floor favours ours by construction;
  the equal-deviation rows at 5 and 8 px sit above it.
- Not measured: pinned bytes and MDAG 2 pages, wasm32 page decode (needs the MDAG 2 transcoder and wasmtime),
  the Khronos multi-material mesh, Lucy, alpha coverage, GPU runtime timing (the engine's gate).
