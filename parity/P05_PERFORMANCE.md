# Phase 0.5 performance follow-up — owner stop / diagnosis handoff

**Current verdict: functionally validated; performance NOT qualified.** The owner
stopped this lane before a new final matrix. No further timing is queued. This is
not an unreachable-family verdict, residual approval or release acceptance.

The retained tree (D144) combines D131 `opacity.rs`/`strip.rs`, D128 `remesh.rs`,
and the other validated D129 files. Rejected D129 remesh extraction is removed.
D127/D129/D130/D132–D141 remesh experiments are not promoted; D142/D143 are abandoned
artifact-side candidates. Existing caller-tail, fuel/error-prefix, exact-output
and requested-heap contracts remain intact. No unsafe, explicit SIMD or new API.

Fresh D144 validation exits0: root/consumer fmt and strict Clippy; root all-feature,
no-default and no-default+experimental tests 79/74/79; two resident consumer tests
per mode; fourteen phase fixture/fuel/storage contracts; native, libm and executed
WASM differential identity 30,000 cases each with zero mismatches. Expanded selected
family parity checks cover 38 shapes per std/libm mode. Receipts and immutable
proof binaries live outside the deleted build cache in
`/mnt/linux-extra/meshopt-artifacts/p05`. `D144-validate.done`,
`D144-functional-preflight.json` and `D144-per-family-status.json` are the entry points.
Historical fuzz receipts are preserved; fuzz was not rerun against D144.

`report.sh --phase 0.5 --verify-artifacts` still exits1: zero functional errors,
51 expected benchmark/profile identity/protocol errors from historical D73 final
summaries. Those summaries are intentionally not rewritten as current success.
The complete current 320 rows (160/profile,21 API groups) and final 5/10/20 screens /
unpooled D146 30-pair borderline resolution have **never run**. No residual accepted.

## Per-family status at handoff

Before values are the original short-corpus worst-API means. Latest evidence below
is diagnostic (five pairs) unless marked historical D73. Worst API mean/max is
shown per profile; maxima can belong to a different API than the worst mean.
These source-bound observations use different corpora/methods and do not imply
matched-case speedup or qualify the newly combined D144 source. Every family needs
a fresh complete final record. Helpers and compact still have only D73 evidence.

| Family | Before GM M / D | Latest applicable evidence GM (max), Moss | Default | Max heap ratio | Scope / status |
|---|---:|---:|---:|---:|---|
| `stripify` | 1.346 / 1.266 | 1.106 (1.176) | 1.176 (1.257) | 1.000 | D131 edge subset passes; full pending |
| `stripify_bound` | 2.803 / 3.009 | 0.424 (0.476) | 0.423 (0.478) | 1.000 | D73 historical pass; fresh final pending |
| `unstripify` | 1.149 / 1.154 | 1.010 (1.118) | 0.995 (1.152) | 1.000 | D122 edge subset passes; full pending |
| `unstripify_bound` | 2.416 / 3.287 | 0.638 (0.671) | 0.503 (0.521) | 1.000 | D73 historical pass; fresh final pending |
| `vertex_cache` | 1.817 / 1.857 | 1.057 (1.222) | 1.207 (1.219) | 1.000 | D129 edge subset passes; full pending |
| `vertex_fetch` | 1.432 / 1.309 | 0.932 (1.062) | 0.925 (1.064) | 1.000 | D125 full-family diagnostic passes |
| `overdraw` | 1.166 / 1.770 | 0.935 (1.096) | 1.005 (1.393) | 1.000 | D125 full-family diagnostic passes |
| `coverage` | 1.260 / 1.725 | 0.940 (1.140) | 0.964 (1.205) | 1.000 | D125 full-family diagnostic passes |
| `omm_measure` | 2.545 / 2.152 | 1.176 (1.381) | 1.154 (1.302) | 1.156 | D122 full-family diagnostic passes |
| `omm_rasterize` | 1.434 / 1.973 | 1.159 (1.286) | 1.127 (1.301) | 1.000 | D131 full-family diagnostic passes |
| `omm_entry_size` | 1.877 / 2.674 | 0.526 (0.540) | 0.488 (0.494) | 1.000 | D73 historical pass; fresh final pending |
| `omm_compact` | 1.390 / 1.148 | 1.246 (1.319) | 1.115 (1.181) | 1.000 | D73 historical pass; fresh final pending |
| `tangents` | 2.938 / 2.869 | 1.169 (1.330) | 1.195 (1.339) | 1.000 | D128 full-family diagnostic passes |
| `normals` | 3.530 / 3.214 | 1.189 (1.275) | 1.180 (1.318) | 1.000 | D123 full-family diagnostic passes |
| `remesh` | 1.613 / 1.732 | 1.248 (1.377) | 1.264 (1.348) | 1.000 | D128; default owned mean FAIL |

Remesh is the remaining observed full-family mean failure: D128 allocating
Moss 1.2484/default 1.2641, caller 1.1858/1.2488, heap 1.0. Default caller and Moss
owned means are near the bar; neither is qualified. Cache D126's full diagnostic
has default maximum 1.499; D129's updated two-edge subset passes but cannot replace
it with a full mean. D131 clears the prior Moss raster case around1.55:
its largest Moss owned/caller maxima are1.274/1.286. Strip/decoder edge evidence
covers shapes 0/7 only and leaves the six other shapes unmeasured for this method.

## Next diagnosis lane

Start with D128/D138 remesh perf evidence to identify the specific overhead.
D138's default worst case is seed 4, 256 vertices/512 triangles/style2: whole-driver
Rust/C++ instructions 4,383,683,293/2,929,328,885 (~1.50x); Rust cycle samples are
54.7% marking and 43.1% accumulating voxelization. These counts include fixture
setup and do not prove API causality. Bound and owned paths repeat marking;
allocation, cast/bounds and code-layout costs need measured attribution.

D142's source/binary-checked Moss-only diagnostic passes (GM 1.195/1.168,
max 1.264/1.261, heap 1.0), but its external worker assumed
`libmeshopt-p05-bench.so` after a broad prototype package rename produced
`libmeshopt-p05-padded-grid-prototype-bench.so`. It exits1 after completing Moss,
before profiling/default. Do not promote it or infer the default result. D143
extends the padded representation to accumulation, passes full semantic/strict
proof, and has no timing. Both remain outside production. Fix the external worker
basename handling or restrict package renaming before any separately authorized
future timing; source identity will require a fresh stream after harness edits.

For future authorized timing, use only `parity/p05/lease_run.py` (which invokes
`MOSS_HEAVY_GPU=1 /mnt/linux-extra/moss-coord/bin/moss-heavy.sh 4 timeout 840 ...`).
It verifies shared admission/holder ancestry and kernel flock; no nested/direct
lease, scoreboard or quiet-host gate. Grant soft checkpoint 600s, hard 840s,
retry 75/resume 76; touched rows/five pairs while iterating. Selected complete final
matrix uses 5/10/20 early screens and exactly 30 fresh unpooled D146 pairs only for
borderline-at-cap cases. Means<=1.25, maxima<=1.50, heap<=1.25 in BOTH profiles;
a failing mean cannot become a residual. All geometry/API allocation/initialization,
scratch, fresh workspace and destruction remain timed. Frozen D106 is unchanged.
The exact target directory is deleted at wrap-up; external archives survive.

## Candidate ledger and dead hypotheses at owner stop

Every source candidate is recorded in D71–D143 above. This index distinguishes
retained corrections from failed explanations, performance rejections, a compiler
no-op, and unfinished hypotheses. All artifacts are under
`/mnt/linux-extra/meshopt-artifacts/p05`; completed D128–D141 prototypes have
source/binary-bound `Dnnn-proof/baseline-*` archives. Earlier streams use
`baseline-Dnnn-*`, except frozen D106. D142 uses `D142-proof/baseline-D142-partial`.
No samples across sources/methods are pooled. Lower instruction counts or
semantic equivalence never establish a wall-time pass.

| Candidate / hypothesis | Outcome and disposition |
|---|---|
| D71–D73: batched helper timing, matched strict scalar reference, analyzer/generator loop reductions and adjacent normal pairs | Retained foundations. D73 full matrix still fails12/21 Moss and14/21 default groups. Historical data uses the earlier method. |
| D74–D85: sequential strip reads, cache bounds, scalar raster rows/setup, direct corner/voxel/hash reads, covered caller raster, eight-triangle removal, scalar log2 and inline OMM hash | Retained validated changes; caller witness D83. Individually untimed while gated, subsequently exercised by diagnostics. No individual speedup inferred. |
| D87/D89–D92: covered caller normal/tangent storage, fuel-specialized decoder/raster, once-only scratch initialization, retired remap removal | Retained semantics and requested-heap reductions. D88 adds executed libm proof. Superseded narrow emission choices are listed below. |
| D97/D98/D101/D102/D105: packed reads, disabled cache simulation, bounded OMM ranges, normal clear/charging, tangent accumulation and fetch bounds | Retained validated changes. Interim queues admitted no samples or partial samples; D102/D103 partial streams are preserved separately. |
| D99/D112: append owned strip/unstrip output, including the <=64 small decoder branch | Rejected as the final ownership layout. D118 replaces decoder append with initialized chunks; D131 replaces strip append with SliceOutput. Exact capacity/work/heap retained. |
| D103: branchless decoder degeneracy and checked triangle chunks | Retained, but D105 owned tiny/million maxima1.543/1.851 Moss and1.560/1.791 default still fail. Not sufficient alone. |
| D104: caller page commitment and owned output chunks | Retained matching correction; not a sufficient explanation for the remaining owned-only gap. |
| D106: resident immutable inputs and explicit decoder validation | Retained matching correction; focused default owned maximum1.756 still fails. Frozen D106 source/binaries preserved. |
| D107: matched conditional caller buffers | Retained correction. Discarded its first draft's unrelated untimed OMM parity branch edit. Does not alone clear tiny owned decoding. |
| D108/D115/D116: inline allocation, checked wrapper and emitting loop | Retained hints; successive Moss tiny maxima1.849/2.229/1.555 still fail. Removing opaque symbols alone is insufficient. |
| D110: warmed equal-batch calibration | Retained fix for concrete sub-ms batches. Corrected tiny Moss maximum2.889 still fails, rejecting undersized batches as the whole explanation. |
| D111: static API selection outside both loops | Retained method fix. Tiny owned Moss2.883 still fails. Wider dispatcher asymmetry is insufficient. |
| D112/D113: proven add/shift fetch arithmetic, no-restart decoder and <=64 fetch specialization | Retained semantics. D112 fetch mean1.290 Moss still fails; D114 matched method subsequently clears fetch. |
| D113: retain output pointer/count instead of making Vec ownership opaque | Retained matched correction; D114 tiny owned Moss2.145 still fails. |
| D117: matched standalone timer functions | Retained method correction; tiny owned Moss2.056 still fails. |
| D118: initialize all owned decoder bounds and emit fixed chunks | Retained; tiny Moss maximum1.551 still misses. Separate allocation-count experiment proves one108-byte malloc/free per owned call in both languages; extra allocation hypothesis rejected. Instrumented times excluded. |
| D119/D121: select owned/caller mode and prepare equal slice metadata outside timers | Retained matching corrections; tiny Moss maxima1.550/1.561 still fail. D121 default/profiles were not collected. |
| D122: keep Vec ownership local to fallible reservation | Retained. Decoder edge subsets finally pass both profiles; full family remains unqualified. |
| D123: cache kernel inlining, slicing and timestamp changes | Rejected: cache means1.364/1.402, maxima1.737/1.855. Restore kernel in D124. D123 normal merging/probe changes retained; tangent charging refined in D124. |
| D124: original cache kernel, fixed timestamp preparation, actual tangent probes | Retained; tangents pass, cache default tiny maximum1.613 still fails. |
| D125: make the complete statistics struct pointer opaque in both drivers | Retained matched sink correction. Fetch/overdraw/coverage pass; cache default max1.591 still fails. |
| D126: inline cache entry and fixed timestamp seam, keep kernel out of line | Retained. Full cache diagnostic passes but default max1.499 is borderline; D129 edge subset passes. No final maximum accepted. |
| D127: state-specialized raster and exact scalar floor | Retained; scalar proof33,426,938 cases. Raster default means1.302/1.361 still fail before D131. |
| D128: inline raster setup/entry, packed remesh reader, flat no-warp cache, strip wrapper and bounded tangent endpoint cache | Retain cache/tangent/remesh; strip/raster superseded by D131. Mixed candidate is not accepted: default raster/strip/cache subsets fail and remesh owned mean 1.264 fails. |
| D129: inline small strip/workspace helpers | Retained where still present; strip/raster replaced by D131. Per-triangle remesh extraction separately rejected below. |
| D131: nine direct raster edges in original order and initialized checked owned strip output | Retained in final tree. Raster full diagnostic and strip edge subset pass both profiles, heap 1.0; no complete final matrix. |

| Remesh candidate | Moss owned/caller GM | Default owned/caller GM | Decision |
|---|---:|---:|---|
| D127: whole-input charging | 1.467 / 1.358 | 1.482 / 1.304 | Rejected; at least one mean fails. |
| D129: extracted sample helpers | 1.538 / 1.449 | 1.562 / 1.445 | Rejected; at least one mean fails. |
| D130: covered/tight sample-row split | 1.514 / 1.392 | 1.510 / 1.407 | Rejected; at least one mean fails. |
| D132: bit-decoded coordinate clamp | 1.588 / 1.485 | 1.682 / 1.524 | Rejected; at least one mean fails. |
| D133: packed kernel boundaries / constant solve | 1.277 / 1.184 | 1.303 / 1.254 | Rejected; at least one mean fails. |
| D134: blanket entry/run inlining | 1.285 / 1.242 | 1.446 / 1.434 | Rejected; at least one mean fails. |
| D136: owned validation reuse + entry expansion | 1.277 / 1.282 | 1.395 / 1.437 | Rejected; at least one mean fails. |
| D137: equivalent unsigned saturating cast | 1.479 / 1.432 | 1.712 / 1.617 | Rejected; at least one mean fails. |
| D138: validation reuse only, original boundaries | 1.243 / 1.196 | 1.293 / 1.307 | Rejected; at least one mean fails. |
| D139: finite[0,506] grid conversion clamp | 1.406 / 1.256 | 1.388 / 1.357 | Rejected; at least one mean fails. |
| D140: twice-cutoff finite grid upper bound | 1.419 / 1.328 | 1.468 / 1.396 | Rejected; at least one mean fails. |
| D141: direct saturating half-cell byte conversion | 1.270 / 1.198 | 1.274 / 1.272 | Rejected; at least one mean fails. |
| D135: rounded base point per sample row | — | — | Rejected before timing: generated default marking/accumulation instructions, registers, branches and float constants identical to D128. Compiler already hoists it. |
| D142: fixed-pitch512-byte stack marking grid, resolutions4..8 | 1.195 / 1.168 | unmeasured | Abandoned at owner stop; Moss maxima 1.264/1.261, heap 1.0. Wrapper exits1 after Moss due renamed C++ library basename; no default or perf run. Not a demonstrated performance rejection. |
| D143: extend fixed-pitch stack grid/row map to accumulation | unmeasured | unmeasured | Abandoned without any timing submission; full strict/native/libm/executed-WASM proof passes. No performance inference. |

The operative rejection is failure against the unchanged bar; none of these
means is classified as a residual or as unreachable in safe Rust. D142/D143
remain usable hypotheses for the next diagnosis lane. D139/D140/D141 independent
native and executed-WASM conversion proofs each match134,615,444 cases; D132
native proof matches134,615,444 and D137 native cast proof33,558,016. These proofs
say nothing about speed. Source-extracted float/coordinate proof artifacts and
expanded std/libm corpus receipts remain in the respective proof directories.

Rejected scheduling assumptions are also closed: waiting for a quiet/load-free
host, periodic free-lease gaps, scoreboard polling, and direct lease submission
are obsolete. D114 introduced fair lease scheduling; D120 supersedes it with
visible shared heavy admission. None authorizes a new burst after this owner stop.

## Historical continuation record (superseded by the handoff above)

Current production is D129; matched statistic retention is D125, timer/
dispatch/input metadata changes are D117/D119/D121, and shared admission is D120.
D127 and artifact-side D128 strict root/consumer formatting,
Clippy and tests pass in all required modes. Native, libm and executed WASM each
match 30,000 cases. These receipts establish exactness/build health; the final
performance matrix has not started.

D107 matches conditional caller output allocations and exact bounds in both
drivers. D110 calibrates equal warmed batches for the unchanged 20ms target.
D113 retains data pointers/counts and adds safe no-restart decoding and narrow
fetch-line simplification. D115/D116 expose existing decoder helpers to inlining.
D122 keeps Vec ownership local to fallible exact reservation, removing a private
Result<Vec> ownership boundary. All fourteen phase contracts retain std/libm
values, errors, work and tails. D118 replaces tiny owned Vec append emission with initialized checked chunks,
as large owned outputs already use. D117 uses matched standalone timer functions;
D119 selects owned/caller before timing; D121 borrows prepared strip metadata
before timing in both drivers. All API work, owned allocation/initialization,
scratch, fresh workspace and destruction remain timed. No unsafe or explicit
SIMD is added. Exact errors, work prefixes, destination tails and values remain.

D122 completes eight five-pair strip/decoder edge rows per profile, archived
in `baseline-D122-focused`. All subset groups pass: owned decoder maxima
0.999 Moss / 1.033 default, caller 1.118 / 1.152; owned strip maxima 1.407 /
1.459, caller 1.149 / 1.144. All requested-memory ratios are 1.0. The combined
admission lasted 78 seconds, measured process peak 0.5 GB. These are incomplete
family means and cannot qualify the full matrix. D122's subsequent 48 rows per
profile complete in a 217-second admission (`baseline-D122-pending`). OMM measure
passes both profiles (means 1.176/1.154, maxima 1.381/1.302, memory 1.156). Tangent
owned means 1.263/1.269 and default caller 1.291 fail; cache maxima 1.541/1.766
and Moss normal owned maximum 1.535 fail.

D123 normals pass all API/profile groups (`baseline-D123-focus`). D124 rejects
the regressing cache attempt, restores the D122 kernel and exposes fixed
timestamp-only buffer lengths through identical workspace preparation. Tangent
hashing uses bounded actual-probe charging. All tangent groups pass D124's
24-row diagnostics in both profiles (largest mean 1.248, max 1.351, memory 1.0).
Cache means pass 1.105/1.116; Moss max 1.450 passes, default tiny max 1.613 fails.
Both streams and fresh profiles are archived in `baseline-D124-focus`; its
visible grant lasted 117 seconds. No residual is accepted.

D125 matches cache/fetch/overdraw/coverage output retention: both drivers make a
pointer to the complete local statistics struct opaque. This replaces Rust's
by-value struct sink and C++'s single-field volatile XOR. All API work, return
production, workspace and allocation stay timed. Production Rust is unchanged;
strict checks and 30,000 exact native/libm/executed-WASM cases each pass. D125 completes all
four analyzer families in an 89-second admission (`baseline-D125-focus`).
Fetch/overdraw/coverage pass both profiles; cache means 1.147/1.149 pass, but
default tiny maximum 1.591 fails.

D126 exposes the identical fallible cache body and fixed timestamp preparation
to caller inlining; workspace begin/finish get inline hints. Kernels stay out
of line, preserving the rejection of D123's regressing kernel expansion.
Validation, quotas, work, allocations and result math stay exact. Full strict
checks and 30,000 native/libm/executed-WASM cases each pass. D126 completes cache
and pending raster/remesh diagnostics in an 86-second admission
(`baseline-D126-focus`). Cache passes both consumers (means 1.093/1.151, maxima
1.346/1.499, memory 1.0). Raster owned/caller means fail 1.366/1.439 Moss and
1.423/1.507 default; default remesh means fail 1.296/1.259.

D127 specializes validated raster state formats through recursion/emission,
uses exact scalar finite-floor bit rounding with the original non-finite
handlers, and proves/removes remesh per-record work checks when a whole-input
upper bound covers them. Float/voxel order, allocations, quotas, work errors
and caller tails remain exact. The source-extracted floor proof matches
33,426,938 patterns against std floor. Strict checks and each 30,000-case
native/libm/executed-WASM sweep pass. D127 completes in 97 seconds, archived
in baseline-D127-focus. Its remesh charging change regresses both profiles and
is rejected. Artifact-side D128 passes strict/native/libm/executed-WASM proof
and completes 50 five-pair rows per profile (D128-proof/baseline-D128-focused).
Tangents pass both profiles; Moss raster maximum falls to 1.246. Default
strip/cache/raster/remesh still fail some diagnostic groups. D129 promotes
D128, inlines their profiled small helpers and separates original per-triangle
sample charging. Its 34-row/profile focus completes; remesh extraction regresses and is
rejected. D130 sample-row work specialization and D132 bit coordinate conversion
also regress and are rejected. D131 direct edge calls/checked owned strip output
pass both profiles: raster means <=1.159/maxima <=1.302, strip edge subset means
<=1.176/maxima <=1.257, heap 1.0. D131 expanded strip/raster native/libm corpus
checks also match all shape controls. D133 restored remesh loop with separate
packed/strided kernels and constant solve dispatch passes strict/native/libm/
executed-WASM proof; its timing waits in visible shared admission. D134 entry
inlining is prepared and being proved as a follow-on if D133 still fails.
The required full final matrix launcher is ready but has not run. No residual is accepted; final matrices remain unrun.

D121's four-row Moss stream is retained in `baseline-D121-partial`: tiny owned
maximum 1.561 still fails, caller/million probes pass, memory is 1.0. Its first
visible admission held the lease for 26 seconds and measured a 0.5 GB process
peak. D121 default and fresh failing profiles were not collected; the queued
profile was stopped before admission. No live burst was interrupted.

D119's focused five-pair decoder probes remain separately source-bound in
`baseline-D119-focused`: tiny owned Moss maximum 1.550, default maximum 1.102;
caller and million-owned probes pass. The two-case decoder mean is incomplete,
and the Moss maximum still fails. No residual is accepted. Prior D106-D119
streams retain their matching sources, binaries and failing-family profiles.

D120 supersedes direct lease submission with visible shared admission:
`MOSS_HEAVY_GPU=1 moss-heavy.sh 4 timeout 840 ...`, dashboard lane
`meshopt-timing:p05-*`. The 4 GB declaration is adjustable if the measured peak
requires it. The shared wrapper obtains/releases the GPU lease; this lane never
calls or nests gpu-lease.sh. Receipts verify the admitted timeout, memory
reservation, heavy/lease ancestor PIDs, lane, and the holder's kernel flock for
the shared inode. No lease-free gap or scoreboard polling remains. Commands
checkpoint complete cases/families after ten minutes, release and requeue;
foreground 840-second timeout plus five-second kill grace bounds each grant.
Exit 75 retries admission and 76 resumes a completed checkpointed burst. Source,
binary identity, pair load/CPU receipts and all statistical decisions remain.

Final acceptance requires every family/API mean <=1.25, requested memory <=1.25
and the unchanged 1.50 maximum decision under both profiles. Final-only complete
160-case matrices screen at 5/10/20 pairs; only cases borderline at the cap
receive 30 fresh D146 pairs on a fixed fresh core, never pooled with selection
samples. Family means/memory retain stage-one data. The single final matrix
has not started; pending family diagnostics must pass before it does.

The figures below are the historical final D73 timed run after the safe-Rust optimization
passes. The measured source also passed exact native and WASM parity.

**Verdict: evidence verified; performance not qualified.** The unchanged scalar-strict C++ bar is geometric mean Rust/C++ time <= 1.25 per family/API, every case <= 1.50, and peak requested output-plus-scratch <= 1.25. Moss passes 9/21 family/API groups and Cargo defaults pass 7/21. Three caller-buffer groups fail memory in both profiles. No exception was approved.

Each profile has 160 cases with 20 alternating same-core pairs each, pinned to the measured least-busy physical core. Geometry-dependent functions include tiny, medium, five topology styles (random, connected, seam-heavy, sparse, explicitly disconnected) and a million-triangle connected case. Geometry-independent OMM operations use six varied cases. Load averages are stored before and after every pair; the driver never waits for a quiet host. Both profiles use opt-level 3 and debug 0; Moss uses thin LTO and one codegen unit, Cargo defaults disable LTO and use 16. The benchmark checks complete output bytes before timing.

Memory is peak requested output plus scratch, excluding caller buffers. The C++ driver tracks meshoptimizer scratch allocations and adds requested output vector sizes. Rust records per-call workspace usage, including staged outputs. These are requested bytes, not process RSS. In `omm_rasterize:caller`, C++ requests zero scratch while Rust requests a temporary output byte, so the ratio has no finite denominator.

| Family (worst API) | Moss old -> final | Cargo default old -> final | Final max case M / D | Max memory |
|---|---:|---:|---:|---:|
| `stripify` | 1.346 -> 1.146 | 1.266 -> 1.094 | 2.098 / 2.022 | 1.000 |
| `stripify_bound` | 2.803 -> 0.424 | 3.009 -> 0.423 | 0.476 / 0.478 | 1.000 |
| `unstripify` | 1.149 -> 1.419 | 1.154 -> 1.384 | 5.534 / 5.403 | 1.000 |
| `unstripify_bound` | 2.416 -> 0.638 | 3.287 -> 0.503 | 0.671 / 0.521 | 1.000 |
| `vertex_cache` | 1.817 -> 1.456 | 1.857 -> 1.503 | 1.871 / 1.977 | 1.000 |
| `vertex_fetch` | 1.432 -> 1.238 | 1.309 -> 1.332 | 1.338 / 1.518 | 1.000 |
| `overdraw` | 1.166 -> 0.986 | 1.770 -> 1.101 | 1.175 / 1.819 | 1.000 |
| `coverage` | 1.260 -> 0.913 | 1.725 -> 1.011 | 1.186 / 1.344 | 1.000 |
| `omm_measure` | 2.545 -> 1.351 | 2.152 -> 1.450 | 1.487 / 1.607 | 1.156 |
| `omm_rasterize` | 1.434 -> 1.629 | 1.973 -> 1.766 | 2.128 / 2.269 | unbounded |
| `omm_entry_size` | 1.877 -> 0.526 | 2.674 -> 0.488 | 0.540 / 0.494 | 1.000 |
| `omm_compact` | 1.390 -> 1.246 | 1.148 -> 1.115 | 1.319 / 1.181 | 1.000 |
| `tangents` | 2.938 -> 1.556 | 2.869 -> 1.709 | 1.894 / 2.097 | 2.122 |
| `normals` | 3.530 -> 1.317 | 3.214 -> 1.311 | 1.438 / 1.479 | 1.909 |
| `remesh` | 1.613 -> 1.293 | 1.732 -> 1.294 | 1.454 / 1.380 | 1.000 |

The old and final geometric means use different corpora; these arrows show the lane outcome, not a matched-case speedup. The detail JSONs retain per-API groups, raw samples, memory bytes, load, input hashes and binary SHA-256 identities. `parity/results/benchmark-{moss,default}-0.5.json` are the compact summaries.

## Profile evidence

`perf stat` instruction ratios below compare whole profiling drivers at the selected worst benchmark case. They include input setup, so they are diagnostic rather than per-call instruction counts. `perf record` identifies the Rust hot path. Every failing family in each profile has a retained stat log, record data file and report under `/mnt/linux-extra/meshopt-artifacts/p05/profiles-0.5-{moss,default}`.

| Failing family | Moss instructions Rust/C++ | Default instructions Rust/C++ |
|---|---:|---:|
| `stripify` | 1.07x | 1.07x |
| `unstripify` | 1.56x | 1.53x |
| `vertex_cache` | 1.72x | 1.77x |
| `vertex_fetch` | passed | 1.42x |
| `overdraw` | passed | 1.97x |
| `coverage` | passed | passed |
| `omm_measure` | 1.38x | 1.49x |
| `omm_rasterize` | 1.79x | 2.05x |
| `omm_compact` | passed | passed |
| `tangents` | 1.37x | 1.90x |
| `normals` | 1.58x | 1.54x |
| `remesh` | 1.46x | 1.49x |

Moss `perf record` places the largest Rust samples in `unstripify_core`, the vertex cache analyzer, OMM measurement/rasterization, tangent accumulation, normal grouping and remesh voxelization. Stripify's selected profiling driver spends material time in `memmove`, so its whole-driver instruction ratio must not be read as a pure algorithm ratio. D73's normal merging skips unrelated pair dot products; its Moss seam-heavy median fell from 1.63 to 1.34, and default fell from 1.71 to 1.48. These are paired benchmark observations on the same case, while the profile summaries select each run's worst case and need not select the same input across revisions. Exact symbol percentages and raw counts are in the profile artifacts.

The safe-Rust changes removed repeated position reads in tangent/normal accumulation, used a direct scalar square root in standard builds while preserving the `libm` path for `no_std`, simplified checked bounds, batched exact work charges, and reduced OMM UV mapping and cache analyzer work accounting. The three tiny helper gaps were measurement artifacts: the final harness varies valid inputs, batches at least 100,000 calls, dispatches outside both loops, and matches the result compiler barriers. D73 caller-buffer OMM rasterization, tangents and normals preserved the destination on any error by staging a complete result. D81/D87 subsequently removed this staging when a conservative work bound proves completion after all fallible setup. The other D73 failing rows retain their historical measured ratios and worst-case profiles; no exception to the bar is claimed. Fresh measurements of the resumed candidate remain pending admission.

At D73, `parity/report.sh --phase 0.5 --verify-artifacts` verified those historical records and reported `performance_qualified: false`. The current verifier exits1 for obsolete benchmark/profile identities and protocol; the owner no longer accepts whole-family shortfalls.
