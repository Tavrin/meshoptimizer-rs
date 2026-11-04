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

Second table, 2026-11-04, after the error-claim diagnosis below. Same crate output (the library is
unchanged), same views and artifacts. Ours is now built with the normals as attributes of weight 0.5,
like the reference (`ours`; `ours[normals=0]` is the first table's position-only build), and
`ours[measured=1]` is the proposed opt-in error rule, applied in the harness. Measured/claimed is in
scene units on both sides; "earlier form" is the first table's ratio, which mixed two depths (it
reproduces the first table exactly for the position-only build). The p95 deviation columns are over all
spans at each threshold. Regenerate with `python3 -I parity/vg-bench/report.py --cuts <work>/eval-*.jsonl`.

| Mesh | Builder | tris / bevy at p95 dev 5 px | at 8 px | measured/claimed median | p95 | earlier form (px) median | p95 max dev px at 0.5 / 1 / 2 / 4 px | nodes | clusters tested | drawn |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| bunny | ours | 0.60 | 0.81 | 1.15 | 1.62 | 1.34 | 0.9 / 3.0 / 4.7 / 10.8 | 38.1 | 362 | 176 |
| bunny | ours[measured=1] | 0.66 | 0.90 | 1.03 | 1.22 | 1.08 | 0.7 / 2.4 / 4.7 / 10.8 | 44.7 | 380 | 185 |
| bunny | ours[normals=0] | 0.49 | 0.62 | 1.71 | 1.86 | 1.73 | 2.0 / 3.6 / 6.9 / 19.7 | 31.9 | 214 | 105 |
| bunny | bevy | n/a | n/a | 1.16 | 1.77 | 1.31 | 2.8 / 4.6 / 7.4 / 10.6 | 10.7 | 372 | 179 |
| dead_tree_trunk_02 | ours | 0.70 | 0.85 | 1.13 | 1.44 | 1.44 | 0.8 / 2.0 / 6.7 / 12.2 | 56.1 | 573 | 277 |
| dead_tree_trunk_02 | ours[measured=1] | 0.67 | 0.71 | 1.04 | 1.13 | 1.11 | 0.6 / 1.5 / 5.4 / 10.4 | 56.9 | 593 | 286 |
| dead_tree_trunk_02 | ours[normals=0] | 0.71 | 0.83 | 4.00 | 10.63 | 4.59 | 3.7 / 7.6 / 15.8 / 34.2 | 35.4 | 293 | 142 |
| dead_tree_trunk_02 | bevy | n/a | n/a | 1.20 | 1.36 | 1.21 | 3.5 / 3.5 / 7.5 / 11.9 | 14.4 | 590 | 285 |
| namaqualand_boulder_02 | ours | 1.01 | 1.01 | 1.17 | 1.51 | 1.26 | 0.8 / 1.7 / 4.0 / 13.4 | 69.4 | 799 | 386 |
| namaqualand_boulder_02 | ours[measured=1] | 0.90 | 0.91 | 1.00 | 1.07 | 1.06 | 0.5 / 1.2 / 2.9 / 9.6 | 74.0 | 814 | 394 |
| namaqualand_boulder_02 | ours[normals=0] | 1.16 | 1.16 | 4.38 | 5.83 | 4.34 | 2.7 / 6.5 / 21.9 / 25.8 | 46.9 | 499 | 243 |
| namaqualand_boulder_02 | bevy | n/a | n/a | 1.10 | 1.38 | 1.14 | 3.4 / 3.4 / 3.8 / 12.7 | 18.0 | 821 | 395 |
| coast_rocks_01 | ours | 0.80 | 0.91 | 1.22 | 1.42 | 1.27 | 2.1 / 3.4 / 6.1 / 10.0 | 121.3 | 1,069 | 530 |
| coast_rocks_01 | ours[measured=1] | 0.66 | 0.74 | 1.01 | 1.21 | 1.27 | 1.5 / 2.9 / 5.0 / 8.6 | 136.9 | 1,148 | 568 |
| coast_rocks_01 | ours[normals=0] | 0.59 | 0.43 | 2.01 | 2.50 | 2.12 | 3.1 / 5.9 / 8.0 / 14.6 | 50.2 | 371 | 185 |
| coast_rocks_01 | bevy | n/a | n/a | 1.17 | 1.54 | 1.28 | 3.1 / 4.2 / 6.8 / 11.0 | 31.5 | 1,123 | 557 |
| dragon | ours | n/a | n/a | 4.73 | 14.24 | 5.67 | 11.1 / 37.2 / 37.7 / 45.9 | 115.2 | 1,088 | 544 |
| dragon | ours[measured=1] | 0.81 | 0.92 | 1.16 | 1.40 | 1.39 | 1.7 / 3.7 / 6.2 / 9.1 | 209.2 | 1,466 | 732 |
| dragon | ours[normals=0] | n/a | n/a | 14.78 | 29.23 | 13.91 | 60.7 / 65.4 / 65.4 / 65.4 | 58.8 | 466 | 233 |
| dragon | bevy | n/a | n/a | 1.44 | 3.02 | 1.75 | 3.7 / 5.7 / 14.2 / 14.2 | 36.0 | 1,146 | 574 |

Self-checks: 60 run, 0 failed. The build-time, memory and size tables above are from the first run, with
the position-only build; simplifying with normals adds attribute work and changes the DAG, and those
tables were not rerun (reversal: run `bench.sh`, which now builds with normals).

First table (position-only build, ratio with mixed depths):

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

## Error-claim diagnosis

The first table's measured/claimed ratio (1.7 to 4.6, 13.9 on dragon) had three causes, two in
this harness and one in the library's default output, which is upstream behaviour.
`vg-bench diag` (see `README.md`) measured them without the pixel conversion: world-space cuts
at `t = radius x 2^-k`, k = 3..13, each measured one-sided in both directions against its largest
claimed error, and each group's single simplification step (its clusters against the clusters
simplified from them) against the recorded error.

1. **Harness input: no attributes.** Ours was built from positions only; the reference, and the
   upstream demo (`demo/nanite.cpp`), simplify with the normals as attributes of weight 0.5. The
   simplifier's position-only error under-reports the sampled distance of one step by a median of
   1.48 (bunny), 1.48 (dragon), 1.65 (trunk), 1.72 (boulder) and 1.51 (rocks), in both directions
   about equally. With normals at 0.5 the attribute error, clamped to the position scale, raises
   the claim and the per-step median becomes 0.94 to 1.00 (trunk, dragon). Fix: the harness builds
   ours with normals at 0.5 (`ours::NORMAL_WEIGHT`), so both builders see the same input.
2. **Harness ratio: two depths.** The ratio divided the deviation converted at the mesh's nearest
   depth by the claim projected at each drawn cluster's own depth, so a far cluster's claim was
   compared with a near conversion. Fix: both sides in scene units (largest self error among drawn
   clusters against the measured world maximum); the earlier form stays in the JSON as
   `median_measured_over_claimed_px`.
3. **Library: removed geometry priced at its own planes.** On dragon, with normals, the per-step
   median is 1.00 but the tail is long (p95 1.31, maximum 26.8): the worst groups lose geometry
   (source-to-level 23 mm against an error of 0.85 mm, level-to-source 0.9 mm). The quadric error of
   a collapse is the distance to the collapsed vertices' own planes, so a small or thin part that
   folds into the surface costs almost nothing while its points end far from what remains.
   Dragon (`dragon_vrip`) has 151 index-connected parts, 146 under 16 triangles, and the default
   cuts stop referencing 107 of them at `t = radius/256`. World-space cuts of the default build
   measure a source-to-level maximum of 3.1 times the claim (median over the eleven cuts, up to 16)
   while level-to-source stays at 1.14. `simplify_permissive` (on by default, as upstream) makes it
   worse: without it the median is 1.25, up to 3.0.

Not the cause: the sloppy fallback (`fallback_sloppy=0` gives identical cuts on bunny and dragon),
the error accumulation rule (`max(previous, current)`, the same as upstream `clusterlod.h` and as
the reference, which forces each group error to at least its meshlets' errors), and bound spheres
(the world-space cuts project every sphere at the same depth and still show cause 3).

The library's default output is unchanged. Its configuration (`default_config`) and error rule
are those of upstream `clodDefaultConfig` and `clodBuild`, and `parity/cluster-lod/differential.py`
compares the complete cluster-LOD result against the vendored C++ demo, so upstream has the same
claim. Changing the default would change output against upstream; the contract documents the
error as the simplification error, which it is.

**Proposed opt-in, prototyped in the harness (`--set measured=1`):** after simplifying a group,
measure the step with the crate's own `measure_lod_quality` (two-sided, about 4000 samples per
group) and take the group error as at least that distance, then restore monotonicity over the
DAG. In the library this would be a `Config` field (for example `simplify_error_measured`, default
off); `Config` is not `#[non_exhaustive]`, so adding a field breaks struct-literal construction
and belongs in a minor release (0.3) rather than a 0.2.x patch. A narrower rule that only priced
fully removed index-connected parts (`--set removed=1`) raised 6 to 11 groups on dragon and left
the median at 1.57: most of the lost geometry is not a whole disconnected part.

World-space cuts, largest sampled deviation over the largest claim (median and maximum over the
cuts where the claim is positive; triangles of the k = 8 cut, `t = radius/256`):

| Mesh | ours, positions only | ours, normals 0.5 (new default) | ours, normals 0.5 + measured | reference | k = 8 triangles: normals 0.5 / + measured / reference |
| --- | ---: | ---: | ---: | ---: | ---: |
| bunny | 1.71 (7.00) | 1.14 (1.31) | 1.07 (1.22) | 1.44 (2.53) | 16,558 / 18,445 / 16,918 |
| dead_tree_trunk_02 | 4.60 (7.48) | 1.29 (2.14) | 1.08 (1.11) | 1.02 (1.35) | 23,448 / 26,256 / 28,936 |
| namaqualand_boulder_02 | 4.38 (5.83) | 1.16 (1.51) | 0.99 (1.36) | 0.94 (1.38) | 50,160 / 53,371 / 51,092 |
| coast_rocks_01 | 1.98 (2.55) | 1.03 (1.44) | 0.98 (1.22) | 1.35 (10.09) | 34,625 / 36,764 / 35,150 |
| dragon | 14.78 (127.48) | 3.11 (15.95) | 1.10 (1.34) | 1.50 (11.61) | 33,400 / 75,496 / 43,050 |

The reference's ratio grows at the finest cuts because its measured deviation includes its
1/16 cm position quantisation and its claim does not. The measured rule costs dragon 2.3 times
the triangles at `t = radius/256` (1.09 times at `radius/2048`) and the other meshes 1.06 to 1.12
times: that is the geometry the default claim hides.

## Against the section 6 bar

- Build time at most half of the reference at equal thread count: met on every mesh at 1 and 8 threads
  (0.09 to 0.23 including the MDAG 1 write).
- Memory bytes per DAG triangle at or below the reference: not met. Ours is 23 resident B per DAG triangle against
  13 to 15. File bytes per DAG triangle are below the reference (10 to 11 against 12 to 14).
- Fewer cut triangles at equal measured p95 deviation (second table, both builders simplifying with normals):
  met on bunny, trunk, rocks (0.60 to 0.91 at 5 and 8 px), even on the boulder (1.01). Not decidable on dragon
  with the default rule (the curves do not overlap); with the measured rule 0.81 and 0.92.
- Measured over claimed error (second table, scene units): ours 1.13 to 1.22 on bunny, trunk, boulder and
  rocks against 1.10 to 1.20 for the reference; dragon 4.73 against 1.44, because the default (upstream)
  error does not price removed thin or small geometry. The measured rule gives 1.00 to 1.16 on all five.
  The first table's 1.7 to 4.6 came mostly from the harness (position-only input, mixed depths).

## Open findings

- Dragon: diagnosed in "Error-claim diagnosis". The flat 65 px p95 deviation was geometry removed at a near-zero
  quadric error (source-to-level only), upstream behaviour made worse by `simplify_permissive`. With the default
  rule the p95 deviation is still flat between 1 and 2 px (37.2, 37.7); with the measured rule it rises with the
  threshold (1.7, 3.7, 6.2, 9.1 px). The library opt-in (`Config` field, 0.3) is proposed, not implemented.
  The self-checks still do not prove the cut covers the surface exactly once.
- The reference's position quantisation (1/16 cm, factor 4) sets a floor of 2.8 to 3.5 px at 2048 px span on
  these 2 m meshes (last column). Comparing deviations below that floor favours ours by construction;
  the equal-deviation rows at 5 and 8 px sit above it.
- Not measured: pinned bytes and MDAG 2 pages, wasm32 page decode (needs the MDAG 2 transcoder and wasmtime),
  the Khronos multi-material mesh, Lucy, alpha coverage, GPU runtime timing (the engine's gate).
