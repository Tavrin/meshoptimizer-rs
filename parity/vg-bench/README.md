# Cluster-DAG benchmark against the Bevy meshlet builder

Not published and not part of the crate package (own Cargo workspace, outside the
package include list). It builds the same mesh with

- this crate: `clusterlod::build_parallel_with_output` plus `build_hierarchy`
  (per-level forest, width 8) and the MDAG 1 writer, with the vertex normals as
  simplification attributes of weight 0.5 (as the upstream demo and the reference do), and
- Bevy's `MeshletMesh::from_mesh`, pinned to commit `8cc060f01c3d4d5920781124cc8c32013bb47730`
  (feature `bevy`), saved with Bevy's own asset saver,

then measures both. Third-party source is data: the reference cut rule is a port of
`cull_bvh.wesl`, `cull_clusters.wesl` and `cull_shared.wesl` at that commit, and
the file reader parses the saved version-3 layout (`asset.rs`) from bytes.

## Run

```
bash parity/vg-bench/fetch_corpus.sh <corpus-dir>           # downloads, checks corpus.sha256
cargo build --release --features bevy --manifest-path parity/vg-bench/Cargo.toml
bash parity/vg-bench/bench.sh <corpus-dir> <work-dir> bunny  # one mesh, appends <work-dir>/bunny.jsonl
python3 -I parity/vg-bench/report.py <work-dir>/*.jsonl      # markdown tables
```

`bench.sh` reads its settings from environment variables listed at the top of the
script (worker count, pinned core lists, repeats, orbit directions).

## Commands

`vg-bench prep <mesh> <out.vgm> [--extent 2.0]` fits the mesh (longest side
`extent` metres, centred), derives normals and box-projected UVs when absent, and
writes the shared input both builders read. `build <ours|bevy> <in.vgm> --threads N
[--out file]` is one timed build plus serialisation; the process is wrapped by
`/usr/bin/time -v` and `taskset` in `bench.sh`. `eval <ours|bevy> <in.vgm>` prints
the static metrics, the self-checks, and the cut evaluation.

`diag <ours|bevy> <in.vgm> [--artifact file] [--set k=v,...] [--no-groups]` checks the
error claim without the pixel conversion. It selects world-space cuts (a camera so far
away that every sphere projects at the same depth, so the rule is `error < t`) at
`t = radius x 2^-k`, k = 3..13, and prints the one-sided source-to-level and
level-to-source maxima of each cut against its largest claimed error, plus the source
components the cut no longer references. For ours it also measures each group's single
simplification step (the group's clusters against the clusters simplified from it, both
directions) against the recorded error. `--set` (also accepted by `eval`, which then labels
the builder `ours[<set>]`) overrides configuration fields (`permissive`, `fallback_sloppy`,
`fallback_permissive`, `merge_previous`, `merge_additive`, `sloppy_factor`, `regularize`),
sets the weight of the normals as simplification attributes (`normals=<weight>`, default
0.5 as in the upstream demo and the reference; 0 simplifies positions only), or applies an
error rule prototyped in the harness (`removed=1`, `measured=1`, see `RESULTS.md`).

## What is measured

| Metric | Method |
| --- | --- |
| Build time | `build_ms` is the DAG build plus hierarchy (ours) or `from_mesh` (reference); serialisation is reported separately. Five interleaved repeats, one process each, pinned cores, 1 and N threads. Median, p95 and noise (max minus min) are in the report. |
| Peak memory | `Maximum resident set size` from `/usr/bin/time -v` for the whole process (input load, build, serialise). |
| Disk | Reference: saved `.meshlet_mesh` bytes (LZ4 frame), also after `zstd -19` of the file and of the decompressed stream. Ours: MDAG 1 file bytes and `zstd -19` of the file. |
| Resident | Reference: decompressed slice bytes. Ours (MDAG 1): metadata tables plus the decoded geometry of every cluster (vertices x 16 + indices x 4). |
| Pages | MDAG 1 page count, mean fill of the 128 KiB capacity, groups with the terminal error value. The reference has no pages. |
| DAG shape | total and leaf triangles, triangles and vertices per cluster (mean, p5), levels, hierarchy depth (dispatches per view). Reference leaves are meshlets with error 0. |
| Error per triangle budget | 64 Fibonacci orbit directions x spans of 32..2048 px (camera distance for a bounding sphere of that vertical span; viewport 2048 px, 60 degree FOV) x thresholds 0.5, 1, 2, 4 px. Each view selects a cut (no frustum culling, so the whole mesh is covered), then `measure_lod_quality` measures the cut against the source (200k samples on the source, level sampled at the same density). World deviation becomes pixels at the nearest depth (`distance - bounding radius`), so values are worst case. Identical cuts are measured once. Claimed error is the largest self error among drawn clusters. The ratio `measured_over_claimed` is the measured maximum over that claim, both in scene units: each drawn cluster's self error is meant to bound its deviation, so the cut's maximum should stay under the largest one. (`measured_over_claimed_px`, the first table's form, divided the deviation converted at the mesh's nearest depth by the claim projected at each cluster's own depth, which mixes two depths and overstates the ratio.) |
| Traversal work | The hierarchical traversal with frustum culling (spheres for ours, the stored boxes for the reference; no occlusion culling): nodes visited, clusters tested, clusters drawn, averaged over the same views. |
| Decode | Ours: median `decode_page` rate. Reference: LZ4 frame decode of the whole asset. wasm32 decode is not measured here. |

Self-checks per builder: threshold 0 selects exactly the source triangles and only leaves;
a huge threshold selects only clusters whose parent error is the terminal value; the
hierarchical traversal equals the flat cut at 1 px. They guard the cut-rule port.

The reference quantises positions to 1/16 cm (`--quant`, default 4) in its saved asset;
its cut meshes use the decoded positions, so its measured deviation includes that
quantisation. Ours keeps f32 positions.

## MDAG 2 build-policy and attribute rerun

`build ours` now uses the upstream builder with normals and writes MDAG 2 with
normal/UV payloads. `build ours-v2` uses `clusterlod::v2::build`, including its
measured error floor, then writes the same payload layout. `eval ours --set v2=1`
uses that library path; the `measured=1` override remains the historical harness
prototype. `materials <mesh.gltf>` checks the real asset's normal and UV errors
against their bounds and every material range against the builder's slots.

After building this harness in an admitted job, `l4.sh <corpus> <evidence>` runs
five alternately ordered timing repeats at 1 and 8 threads, pinned to cores 2
and 2-9, then the 64-direction cut tables for both policies on all five meshes.
It never waits for machine quiet. Every build, cut table and resource receipt
has a separate file so reruns can use a fresh evidence directory.

## Foliage coverage

Use `l5_inputs.py <gltf> <buffer> <alpha.raw> <width> <height> <out.input>` to
prepare a caller-owned single-material foliage primitive (the CC0 fern in
`CORPUS.md`). The raw alpha bytes must match the texture's original row order.
Run the crate example `vg_foliage_measure <out.input>` in an admitted CPU scope
with the `clusterlod` feature. It compares off/on 1 px cuts at five distances,
reports all nine fixed sphere-grid views as ratio extrema and maximum blurred
difference, and reports triangle cost and sampled fallback quality. Raster
framing is held fixed against the snapped source. Grid exponent is -16 and
cluster capacity is 32 triangles, partition size 2, UV weights zero. The cut
uses repaired spheres and error values read from actual MDAG 2 files (including
the quantisation error floor); every page is validated. The example performs
no timing.

## 0.2.20 partition qualification

`plateau ours <input.vgm> --set v2=1 --jobs 8` reports per-level group sizes,
triangle counts, pinned triangles, achieved simplification ratios, inherited
errors, and stored error percentiles. `diag ours <input.vgm> --set v2=1
--groups-only --jobs 8` measures the local steps without repeating world cuts.
These local ratios are diagnostics, not substitutes for the original-source
error floor or the full orbit quality gate.

`qualify0220.py` runs one qualification campaign in an admitted CPU job. Set
`VG_CORPUS` to the frozen corpus and `VG_EVIDENCE` to a fresh evidence directory
containing `baseline-vg-bench` and `candidate-vg-bench`. Both binaries must be
built from the locked harness; retain their source and binary hashes separately.
The script creates `final/`, rotates and reverses the candidate, baseline, and
reference build order over five repeats at 1 and 8 threads, pins the first one
and eight CPUs in its admitted affinity, checks complete candidate and baseline
file identity between thread counts, and runs the original 64-direction,
six-span, 200,000-sample quality campaign. An existing `final/` fails closed.
The `shape` rows additionally break resident bytes into metadata, descriptors,
positions, triangles, and attributes; page headers account for the remainder.


`limits ours <input.vgm> --bytes <cap> --work <visits> --jobs 8` builds through
`v2::build` with fresh finite workspace limits and reports accounted peak bytes,
counted work and per-source-triangle ratios. Normal weight is 0.5 and grid -22.
The JSON `pass` and `result` fields describe the build outcome; command exit 0
means the diagnostic ran, including an intentionally budget-limited build.
Finite limits use serial accounting despite the configured worker pool. Use `--suggested` instead of explicit ceilings to validate the empirical
`v2::suggested_limits` helper. The completed calibration and the earlier hold
are recorded in [the sizing record](0220/HOLD.md).


`continue0220.py` measures one byte-preserving calibration experiment. Set
`VG_EVIDENCE` to the original campaign and `VG_CONTINUATION` to a fresh directory
containing the experimental `candidate-vg-bench`. It counterbalances five repeats
of the previous weighted policy, experiment and pinned reference at 1/8 threads,
then requires complete output identity with the original artifacts at both thread
counts. Only after that check does it reuse the original shape and quality rows.
It also runs each finite build using `suggested_limits`. Existing output fails
closed. Timing and quality execution remain separate evidence.

## Error saturation diagnosis

`cooked <file>` opens the first MDAG 2 container embedded in a consumer asset
with the crate reader, transcodes every page, and prints one `cooked_group` row
per group (depth, stored error, triangles, parent count), one `cooked_level` row
per depth (groups, triangles, pinned groups, error min/median/max) and
`far_field` rows: the mean drawn triangles over 64 orbit directions at a
one-pixel threshold for camera distances giving the bounding sphere 32, 64 and
128 px of vertical span. `raw-prep <mesh> <out.vgm>` caches a source without the
`prep` extent fit, so a rebuild stays in the consumer's coordinates. `rebuild
<in.vgm> <out.mdag> <grid-exponent> [--no-normals]` builds through `v2::build` at
the consumer's grid, prints the `plateau` rows, one `error_terms` row per group
(simplifier estimate, sloppy fallback, sampled step and source floors,
inherited child error, acceptance status), the crate `health` verdict, the
`diag_step` rows and then the `cooked` analysis of the written file.

`cooked <file> --steps` also measures each local replacement against the
input triangles decoded from the container, independently of source import
or build configuration. `cooked_health` records the reader health verdict.

In the original cooked asset, error 3.424 first appears at depth 4 on a
1,237-to-761-triangle replacement. Independent decoded-geometry sampling finds
1.305 source-to-replacement and 0.950 reverse deviation; inherited error is
0.755. Later ancestors inherit 3.424. The container does not retain simplifier
flags, so it cannot prove which fallback produced that first estimate. The
revision-8 rebuild identifies a sloppy estimate of 3.290 versus a sampled
step of 1.990; measuring only sloppy steps still leaves a regular estimate of
3.216 versus a sampled step of 0.942.

Revision 9 measures both paths, retains progressing partial reductions, and
lets weighted parent partitions grow to twice the largest replacement region.
Without the partition change the corpus dragon still had three equal-error
chains and two non-root pins, and the rock mesh had one non-root pin. Enlarging
the parent target lets partial regions merge and release shared boundary locks
while retaining indivisible siblings. Errors are neither artificially raised
nor stripped of monotone inheritance. Reverting this policy requires another
builder revision and recooking affected assets; the container layout is unchanged.

### Error saturation measurements

Consumer source: 91,216 triangles; normals supplied, normal-error weight 0, 128-triangle clusters, grid exponent -16. The release and revision-8 rebuilds use the same cached source as revision 9. The original cooked asset has a different depth table and is reported separately.

Per-depth cells are groups / triangles / error min, upper median, max. Errors exclude terminal pins; `root` means no finite errors.

| Depth | 0.2.19 rebuild | Revision 8 rebuild | Revision 9 rebuild |
| ---: | --- | --- | --- |
| 0 | 47 / 91216 / 0.014,0.019,0.111 | 47 / 91216 / 0.014,0.019,0.111 | 47 / 91216 / 0.012,0.018,0.111 |
| 1 | 21 / 45576 / 0.034,0.042,0.111 | 26 / 45576 / 0.034,0.043,0.111 | 17 / 45576 / 0.032,0.040,0.111 |
| 2 | 9 / 22772 / 0.073,0.088,0.119 | 13 / 22768 / 0.069,0.091,0.111 | 7 / 22772 / 0.060,0.073,0.114 |
| 3 | 4 / 11378 / 0.145,0.195,0.375 | 7 / 11374 / 0.179,0.213,0.447 | 3 / 11376 / 0.117,0.120,0.191 |
| 4 | 2 / 5684 / 0.280,0.375,0.375 | 4 / 5684 / 0.438,0.762,3.290 | 1 / 5684 / 0.272,0.272,0.272 |
| 5 | 2 / 2840 / 1.091,9.403,9.403 | 2 / 3092 / 0.806,3.290,3.290 | 1 / 2842 / 0.551,0.551,0.551 |
| 6 | 1 / 1420 / 9.403,9.403,9.403 | 1 / 1544 / 3.290,3.290,3.290 | 1 / 1420 / 0.831,0.831,0.831 |
| 7 | 1 / 710 / 9.403,9.403,9.403 | 1 / 772 / 3.290,3.290,3.290 | 1 / 710 / 1.048,1.048,1.048 |
| 8 | 1 / 354 / 9.403,9.403,9.403 | 1 / 386 / 3.290,3.290,3.290 | 1 / 354 / 2.463,2.463,2.463 |
| 9 | 1 / 176 / 9.403,9.403,9.403 | 1 / 192 / 3.290,3.290,3.290 | 1 / 176 / 2.991,2.991,2.991 |
| 10 | 1 / 88 / root | 1 / 96 / root | 1 / 88 / root |

Original cooked asset, decoded with the crate reader:

| Depth | Groups | Triangles | Error min | Median | Max |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 49 | 91216 | 0.033 | 0.040 | 0.058 |
| 1 | 23 | 45580 | 0.067 | 0.091 | 0.114 |
| 2 | 10 | 22770 | 0.149 | 0.199 | 0.298 |
| 3 | 6 | 11374 | 0.298 | 0.742 | 1.151 |
| 4 | 3 | 5684 | 0.853 | 1.151 | 3.424 |
| 5 | 1 | 3010 | 3.424 | 3.424 | 3.424 |
| 6 | 1 | 1504 | 3.424 | 3.424 | 3.424 |
| 7 | 1 | 752 | 3.424 | 3.424 | 3.424 |
| 8 | 1 | 376 | 3.424 | 3.424 | 3.424 |
| 9 | 1 | 188 | 3.424 | 3.424 | 3.424 |
| 10 | 1 | 94 | root | root | root |

Far-field mean cut triangles at 1 px, using stored MDAG 2 errors and 64 orbit directions. Screen span is the bounding sphere vertical diameter.

| Span px | Original cooked | 0.2.19 rebuild | Revision 8 | Revision 9 |
| ---: | ---: | ---: | ---: | ---: |
| 32 | 8325.3 | 2840.0 | 4836.0 | 2842.0 |
| 64 | 13048.0 | 5822.0 | 7956.0 | 5684.0 |
| 128 | 22770.0 | 11682.0 | 11374.0 | 10858.0 |

Bench comparison: revision-9 results use 64 directions; revision-8 corpus rows reuse the existing 16-direction baseline for the same source inputs and build policy. Consumer rows use 64 directions on both sides. The different corpus view counts are not a matched-view performance comparison. Both use seven spans (32–2048 px), 200,000 source samples per unique cut, same density on the selected level. These rows use build geometry, the source origin-enclosing radius, and the origin-centered orbit used by `eval`. They are separate from the stored-container rows, which use its sphere radius and center.

| Mesh | Span px | Revision 8 mean triangles at 1 px | Revision 9 |
| --- | ---: | ---: | ---: |
| consumer | 32 | 4166.0 | 1420.0 |
| consumer | 64 | 6570.0 | 2842.0 |
| consumer | 128 | 11374.0 | 6724.0 |
| consumer | 256 | 21602.0 | 17594.9 |
| consumer | 512 | 37502.3 | 35840.0 |
| consumer | 1024 | 63126.1 | 60206.9 |
| consumer | 2048 | 91173.8 | 91120.9 |
| bunny | 32 | 2708.0 | 1082.0 |
| bunny | 64 | 2708.0 | 2167.0 |
| bunny | 128 | 5655.0 | 4336.0 |
| bunny | 256 | 8668.0 | 8673.0 |
| bunny | 512 | 17956.9 | 17739.1 |
| bunny | 1024 | 35207.9 | 36179.9 |
| bunny | 2048 | 65834.0 | 65884.5 |
| dragon | 32 | 22080.0 | 1688.0 |
| dragon | 64 | 25194.0 | 11042.0 |
| dragon | 128 | 33872.0 | 24016.0 |
| dragon | 256 | 46123.0 | 43180.0 |
| dragon | 512 | 66317.2 | 72140.6 |
| dragon | 1024 | 119311.2 | 127957.9 |
| dragon | 2048 | 221443.2 | 228312.8 |
| coast_rocks_01 | 32 | 14708.0 | 164.0 |
| coast_rocks_01 | 64 | 17595.0 | 662.0 |
| coast_rocks_01 | 128 | 20350.0 | 7149.0 |
| coast_rocks_01 | 256 | 33494.0 | 16746.4 |
| coast_rocks_01 | 512 | 65639.3 | 48432.0 |
| coast_rocks_01 | 1024 | 120423.4 | 119617.7 |
| coast_rocks_01 | 2048 | 247183.1 | 248130.2 |
| namaqualand_boulder_02 | 32 | 3050.0 | 1526.0 |
| namaqualand_boulder_02 | 64 | 5549.0 | 3054.0 |
| namaqualand_boulder_02 | 128 | 12220.0 | 11092.0 |
| namaqualand_boulder_02 | 256 | 27411.1 | 27329.2 |
| namaqualand_boulder_02 | 512 | 54925.2 | 55060.1 |
| namaqualand_boulder_02 | 1024 | 97964.0 | 97922.5 |
| namaqualand_boulder_02 | 2048 | 97964.0 | 97964.0 |
| dead_tree_trunk_02 | 32 | 1286.0 | 1296.0 |
| dead_tree_trunk_02 | 64 | 2894.0 | 2592.0 |
| dead_tree_trunk_02 | 128 | 6292.0 | 5184.0 |
| dead_tree_trunk_02 | 256 | 15539.8 | 13235.0 |
| dead_tree_trunk_02 | 512 | 34350.5 | 32869.6 |
| dead_tree_trunk_02 | 1024 | 73669.8 | 65200.1 |
| dead_tree_trunk_02 | 2048 | 83128.0 | 83128.0 |

Aggregate cut table across all seven spans. Each cell is mean triangles / measured-to-claimed median / p95 / mean maximum deviation in pixels. Source-only cuts have no nonzero simplification claim and are excluded from the ratio, as in the harness.

| Mesh | Threshold px | Revision 8 | Revision 9 |
| --- | ---: | --- | --- |
| consumer | 0.5 | 45323.7 / 0.982 / 1.086 / 0.326 | 42910.7 / 1.000 / 1.098 / 0.343 |
| consumer | 1 | 33644.9 / 1.000 / 1.089 / 0.852 | 30821.2 / 1.000 / 1.128 / 0.915 |
| consumer | 2 | 22033.8 / 1.000 / 1.139 / 2.214 | 19150.3 / 1.000 / 1.139 / 2.341 |
| consumer | 4 | 13007.5 / 1.000 / 1.000 / 3.865 | 10444.1 / 1.000 / 1.068 / 4.405 |
| bunny | 0.5 | 28953.2 / 1.000 / 1.046 / 0.422 | 28835.1 / 1.000 / 1.066 / 0.412 |
| bunny | 1 | 19819.7 / 0.989 / 1.041 / 1.085 | 19437.4 / 1.000 / 1.030 / 1.093 |
| bunny | 2 | 11775.3 / 0.967 / 1.026 / 2.149 | 11257.2 / 1.000 / 1.026 / 2.085 |
| bunny | 4 | 6967.1 / 0.955 / 1.021 / 3.930 | 6829.9 / 1.000 / 1.029 / 3.924 |
| dragon | 0.5 | 130626.0 / 0.999 / 1.017 / 0.672 | 129101.5 / 1.000 / 1.023 / 0.655 |
| dragon | 1 | 76334.4 / 0.976 / 1.000 / 1.231 | 72619.6 / 1.000 / 1.092 / 1.222 |
| dragon | 2 | 49175.7 / 0.958 / 1.000 / 2.223 | 41653.8 / 1.000 / 1.092 / 2.228 |
| dragon | 4 | 34680.8 / 0.905 / 1.000 / 4.144 | 25072.7 / 1.000 / 1.092 / 4.006 |
| coast_rocks_01 | 0.5 | 142218.3 / 0.914 / 1.036 / 0.626 | 131032.7 / 1.000 / 1.031 / 0.655 |
| coast_rocks_01 | 1 | 74199.0 / 0.866 / 1.000 / 1.179 | 62985.9 / 1.000 / 1.006 / 1.231 |
| coast_rocks_01 | 2 | 41019.3 / 0.866 / 1.000 / 2.250 | 29961.0 / 1.000 / 1.006 / 2.199 |
| coast_rocks_01 | 4 | 24544.6 / 0.866 / 1.000 / 4.133 | 12904.2 / 1.000 / 1.006 / 3.652 |
| namaqualand_boulder_02 | 0.5 | 56022.0 / 0.970 / 1.000 / 0.259 | 55602.4 / 1.000 / 1.000 / 0.257 |
| namaqualand_boulder_02 | 1 | 42726.2 / 0.942 / 1.000 / 0.637 | 41992.5 / 1.000 / 1.000 / 0.634 |
| namaqualand_boulder_02 | 2 | 29711.8 / 0.942 / 1.000 / 1.639 | 28523.6 / 1.000 / 1.000 / 1.720 |
| namaqualand_boulder_02 | 4 | 16924.4 / 0.942 / 1.000 / 4.280 | 15880.3 / 1.000 / 1.000 / 4.312 |
| dead_tree_trunk_02 | 0.5 | 42397.2 / 0.966 / 1.075 / 0.360 | 40071.1 / 1.000 / 1.101 / 0.361 |
| dead_tree_trunk_02 | 1 | 31022.9 / 0.951 / 1.061 / 0.856 | 29072.1 / 1.000 / 1.058 / 0.848 |
| dead_tree_trunk_02 | 2 | 19366.8 / 0.957 / 1.045 / 2.370 | 17938.0 / 1.000 / 1.052 / 2.362 |
| dead_tree_trunk_02 | 4 | 9438.9 / 0.941 / 1.000 / 4.452 | 8821.9 / 1.000 / 1.052 / 4.456 |

Revision-9 structural health, minimum chain length 3:

Accuracy columns are the worst median and p95 measured-to-claimed ratios over all evaluated non-source cut rows; limits are 1.3 and 1.5. Aggregate measured deviation rises strictly with threshold on every mesh.

| Mesh | Pass | Equal-error chains | Pins (final roots) | Worst median | Worst p95 |
| --- | --- | ---: | ---: | ---: | ---: |
| consumer | true | 0 | 1 | 1.076 | 1.139 |
| bunny | true | 0 | 1 | 1.066 | 1.081 |
| dragon | true | 0 | 1 | 1.092 | 1.095 |
| coast_rocks_01 | true | 0 | 1 | 1.006 | 1.082 |
| namaqualand_boulder_02 | true | 0 | 1 | 1.017 | 1.017 |
| dead_tree_trunk_02 | true | 0 | 1 | 1.094 | 1.119 |

### 0.2.20 candidate rerun (builder revision 9)

Measured on the revision-9 candidate (a56d033b), before builder revision 10
restored the regular quadric floor for regular steps. These rows characterize
revision 9; they were not re-measured for revision 10. One release-profile
execution per row. Far-field cuts use 64 orbit directions
at a one-pixel threshold. Shading cuts use 16 directions at six projected
spans and 16,384 probes in each direction per unique cut.

All previously reported revision 9 far-field triangle means reproduced to the displayed precision.

| Mesh | Normal-error weight | Mean cut triangles | Normal p95 / max degrees | Geometric ratio p95 |
| --- | ---: | ---: | ---: | ---: |
| bunny | 0 | 22,431.25 | 8.29 / 39.04 | 1.0559 |
| bunny | 0.25 | 33,583.99 | 4.11 / 33.57 | 0.8869 |
| dead_tree_trunk_02 | 0 | 33,659.88 | 28.40 / 175.70 | 0.9360 |
| dead_tree_trunk_02 | 0.25 | 68,407.50 | 6.76 / 83.34 | 0.8495 |
| namaqualand_boulder_02 | 0 | 48,721.73 | 20.73 / 147.77 | 0.9551 |
| namaqualand_boulder_02 | 0.25 | 77,556.04 | 7.06 / 72.22 | 0.8616 |
| coast_rocks_01 | 0 | 73,566.82 | 41.31 / 168.25 | 0.9828 |
| coast_rocks_01 | 0.25 | 240,955.26 | 15.58 / 108.55 | 0.4814 |
| dragon | 0 | 84,398.65 | 10.51 / 179.87 | 0.9263 |
| dragon | 0.25 | 340,491.17 | 3.78 / 132.17 | 0.4270 |
