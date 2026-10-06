# Measured release performance

## Latest 0.5 owner-stop handoff (D144; performance not qualified)

The validated combined tree retains D131 raster/strip, D128 remesh and the other
D129 changes. Strict root/consumer checks,79/74/79 feature-mode tests, fourteen
contracts and 90,000 native/libm/executed-WASM cases pass exactly. No new timing
is authorized at this stop. Remesh default owned diagnostic mean 1.264 still fails;
no failing mean or maximum is accepted as a residual. D142 is Moss-only/abandoned,
D143 untimed/abandoned. The fresh full320-row final matrix never ran. Current
verify exits1 with zero functional errors and 51 historical benchmark/profile
errors. Full per-family source-bound status, all tried/rejected hypotheses and
next diagnosis pointers are in [P05_PERFORMANCE.md](P05_PERFORMANCE.md) and D144
of [DECISIONS.md](DECISIONS.md). External source/binary/proof artifacts survive;
the exact lane Cargo target is deleted. All earlier timing tables and scheduling
statements below are historical and superseded by D120/D144.

Rust/C++ time ratios; each value is the family geometric mean of case median paired ratios, followed by the maximum case in parentheses.

| Function | Fat LTO (historical lane 3b) | Moss thin LTO | Cargo release defaults |
|---|---:|---:|---:|
| vertex_cache | 1.123 (1.201) | 1.184 (1.330) | 1.231 (1.331) |
| overdraw | 1.098 (1.417) | 1.101 (1.522) | 1.135 (1.471) |
| simplify | 1.212 (1.385) | 1.049 (1.304) | 1.060 (1.237) |
| simplify_with_attributes | 1.208 (1.342) | 1.072 (1.231) | 1.092 (1.315) |
| simplify_scale | 0.748 (0.884) | 0.915 (1.067) | 0.855 (1.010) |

RFC bars are unchanged: family geometric mean <= 1.25, maximum case <= 1.50, and requested output-plus-scratch memory <= 1.25 times C++.

Moss uses thin LTO, one codegen unit and opt-level 3. Cargo defaults use LTO disabled, 16 codegen units and opt-level 3. Both consumer records disable release debug. The crate retains its own fat-LTO profile; dependents do not inherit it.

The complete matrix has 204 allocating, caller-buffer and allocation-free cases, covering tiny, medium and million-triangle smooth, seam-heavy and sparse meshes. The single-thread resident drivers share one physical core, one warm-up each and 20 or 30 alternating pairs. Validation, required copies, allocation and scratch are timed.

These are shared-load measurements. Per-case timings, dispersion, memory, load telemetry and source/executable identities are in the external records. This table does not claim quiet-host performance or a universal speedup.

See results/benchmark.json, results/benchmark-moss.json and results/benchmark-default.json for per-function summaries and SHA-256 identities. report.sh --verify-artifacts verifies available data and identifies absent historical artifacts.

D146 final integration: the full stage-1 matrices retain their case maxima and family bars. Seven maxima received 30 fresh pinned-core pairs each; two passed the upper 95% confidence bound and five remain documented residuals (including three inconclusive). See results/stage2-0.1.json and results/requalification-0.1.json.

## 0.5 local benchmark verdict (not accepted)

Time ratio is Rust/scalar-strict C++ family geometric mean, with the largest representative case in parentheses. The unchanged RFC limits are 1.25 and 1.50. Each case has ten interleaved paired samples and exact output checks.

| Family and API | Moss-like | Cargo defaults |
|---|---:|---:|
| `stripify:allocating` | 1.346 (1.527) fail | 1.108 (1.363) pass |
| `stripify:caller` | 1.303 (1.712) fail | 1.266 (1.358) fail |
| `stripify_bound:allocating` | 2.803 (2.818) fail | 3.009 (3.498) fail |
| `unstripify:allocating` | 1.149 (1.472) pass | 1.154 (1.403) pass |
| `unstripify:caller` | 1.045 (1.184) pass | 1.105 (1.363) pass |
| `unstripify_bound:allocating` | 2.416 (2.419) fail | 3.287 (4.022) fail |
| `vertex_cache:allocating` | 1.817 (2.196) fail | 1.857 (1.916) fail |
| `vertex_fetch:allocating` | 1.432 (1.646) fail | 1.309 (1.372) fail |
| `overdraw:allocating` | 1.166 (1.269) pass | 1.770 (1.855) fail |
| `coverage:allocating` | 1.260 (1.322) fail | 1.725 (1.874) fail |
| `omm_measure:allocating` | 2.545 (3.128) fail | 2.152 (2.301) fail |
| `omm_rasterize:allocating` | 1.421 (1.636) fail | 1.706 (1.772) fail |
| `omm_rasterize:caller` | 1.434 (1.699) fail | 1.973 (2.155) fail |
| `omm_entry_size:allocating` | 1.877 (1.944) fail | 2.674 (2.717) fail |
| `omm_compact:allocating` | 1.390 (1.521) fail | 1.148 (1.189) pass |
| `tangents:allocating` | 2.838 (3.217) fail | 2.750 (2.991) fail |
| `tangents:caller` | 2.938 (3.252) fail | 2.869 (3.017) fail |
| `normals:allocating` | 3.530 (3.605) fail | 3.214 (3.277) fail |
| `normals:caller` | 3.450 (3.844) fail | 3.197 (3.364) fail |
| `remesh:allocating` | 1.613 (1.694) fail | 1.732 (1.833) fail |
| `remesh:caller` | 1.527 (1.602) fail | 1.528 (1.629) fail |

The time bar fails in 18/21 Moss-like and 17/21 Cargo-default groups. Comparable C++ peak output-plus-scratch storage and the RFC million-triangle/geometry-variety corpus are absent, so the overall benchmark gate remains failed even if an individual time row passes. Raw paired samples and binary hashes are in the external artifacts linked by `results/benchmark-*-0.5.json`.

## 0.5 performance follow-up

The expanded, 20-pair, same-core D73 benchmark completed both 160-case profiles and comparable memory accounting. The unchanged combined bar fails in 12/21 Moss and 14/21 Cargo-default family/API groups; caller-buffer OMM rasterize, tangents and normals exceed the memory bar. See [P05_PERFORMANCE.md](P05_PERFORMANCE.md) for every family before/after, largest case, memory ratio and worst-case profile evidence. The result is evidence-complete with documented shortfalls, not performance-qualified. The measured source passes 30,000 native differential cases and 30,000 executed WASM identity cases with zero mismatches.

D74–D92 source changes pass native, no-default/libm and WASM differential
checks, plus all 114 expanded-corpus shapes. Their replacement timings wait
for all four coordinator conditions: after 02:30 Paris, load below 12, no GPU
lease holder and no active `moss-scoreboard-*` user unit. The D73 figures above
remain historical until both admitted matrices and their profiles finish.

The 2026-10-05 owner amendment (D94/D95) requires every 0.5 family mean to
pass for 0.2.0. Focused optimization now uses five pairs per touched case;
the final matrix runs once with bounded early stopping. A free GPU lease
and no actively measuring scoreboard are required; sleeping units and host
load do not block. D73 whole-family shortfalls remain historical evidence,
not acceptable residuals for the resumed candidate.


The P05 D97–D101 continuation targets the measured D95 cache/normal/tangent/
OMM-measure gaps and strip allocating output initialization. D99's fresh
native, libm and WASM differential sweeps have zero mismatches; D101 passed the same
validation before focused timing. D100 now verifies the owner's 5/10/20
screening and exactly 30 fresh D146 pairs only for borderline-at-cap cases.
Current-candidate timing remains pending lease/measurement admission. The
historical tables and final benchmark summaries do not qualify this source.

D107–D113 complete matched selected caller allocations, warmed calibration,
operation dispatch and output retention in both drivers, plus focused safe-Rust
decoder/fetch changes. Current D113 validation passes strict Clippy, 79/74/79
root tests and 30,000 exact cases each in native, libm and executed WASM. The
previous complete D112 focus still misses fetch means and the tiny owned
decoder maximum; its source-bound records are retained externally. Fresh
D113 five-pair measurements are waiting for lease admission. No current
performance verdict is available, and no residual is accepted.

D114 replaces lease-free gap waiting with the owner's shared queued lease:
benchmark/profile commands own meshopt-timing:p05-* grants, checkpoint after
ten minutes at completed-case/family boundaries, then release and requeue.
An 840-second foreground timeout preserves shared-wrapper cleanup and bounds
each grant below fifteen minutes. Scoreboard checks are removed. Fresh native,
libm and executed-WASM sweeps again match 30,000 cases each; source/API/bar and
pair decisions are unchanged. Current timings are queued, not yet qualified.

D120 supersedes the earlier admission rules: every timed burst now queues
visibly via `MOSS_HEAVY_GPU=1 moss-heavy.sh 4 timeout 840 ...`, with the shared
wrapper acquiring the GPU lease itself. No direct/nested GPU lease command,
lease-free gap wait, load gate or scoreboard poll remains. The declared peak
is 4 GB; observed completed bursts use approximately 0.5 GB. Actual receipts
verify admission, ancestor ownership and the matching kernel flock.

D122 strip/decoder edge subsets pass all eight API/profile groups, with memory
1.0, in a 78-second admission. D122 OMM measure full-family diagnostics pass
means/maxima/memory in both consumers. D123 normals pass all four API/profile
groups (largest mean 1.189, maximum 1.318, memory 1.0); three tangent groups
pass, but Moss tangent caller mean 1.275 fails. D123 cache changes regress and
are rejected. D124 restores the D122 cache kernel and specializes its unchanged
workspace preparation; tangents get proven bounded hash charging. Strict
Clippy and 30,000 native, 30,000 libm and 30,000 executed WASM cases pass.
Only cache/tangents are queued next (24 five-pair rows per profile), with fresh
failing profiles. These are separately source-bound diagnostics; historical
D73 final tables still do not qualify D124. Final full matrices remain unrun.


D127 completes two-family diagnostics but its remesh whole-input work dispatch
regresses and is rejected (baseline-D127-focus). Artifact-side D128 passes all
strict/native/libm/executed-WASM proof and both 50-row diagnostics. Tangents
pass both profiles; raster Moss maxima <=1.246. Default strip/cache/raster/
remesh retain diagnostic failures (D128-proof/baseline-D128-focused). D129
promotes D128 and targets those four families with private helper inlining and
per-triangle remesh charging specialization. D129 strict root/consumer checks,
contracts and 30,000-case native/libm/executed-WASM sweeps pass. The 34-row/profile
focus is admitted through the visible shared wrapper. An initial nine-second
admission failed before measurement because its external launcher omitted the
required target export; it produced no valid D129 stream. The corrected launch
retains the required shared target and source identity. No residual is accepted;
full final matrices remain unrun and historical tables do not qualify D129.
