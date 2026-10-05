# P05 current state — 2026-10-05

One fresh complete frozen performance matrix at `df96a78d6d1cd53ceb69ba81df107571f622a5db`: **Moss 20/21 API groups pass; Cargo-default 18/21 pass. Overall performance qualification FAILS.** Both profiles completed 160/160 cases with exact untimed output parity for every case. No implementation or harness changes.

Ratios are Rust/scalar C++ elapsed time. Each geomean is over the stage-one per-case median paired ratios for that family/API; worst is the largest such median. Bars are geomean ≤ 1.25 and every case maximum decision PASS at 1.50 using the frozen early-screen/D146 rule. Allocating and caller APIs stay separate. Values below are rounded; decisions use full precision. All memory ratios also pass the existing 1.25 bar (maximum 1.0 in both profiles).

| Family / API | Moss geomean | Moss worst | Verdict | Default geomean | Default worst | Verdict |
|---|---:|---:|---|---:|---:|---|
| stripify / allocating | 0.961954 | 1.220917 | PASS | 0.989104 | 1.459062 | PASS |
| stripify / caller | 0.973750 | 1.183414 | PASS | 0.966790 | 1.233087 | PASS |
| stripify_bound / allocating | 0.569651 | 0.572582 | PASS | 0.555714 | 0.579300 | PASS |
| unstripify / allocating | 0.825933 | 1.054376 | PASS | 0.860982 | 1.148611 | PASS |
| unstripify / caller | 0.713133 | 1.186131 | PASS | 0.815461 | 1.287156 | PASS |
| unstripify_bound / allocating | 0.538934 | 0.553314 | PASS | 0.495810 | 0.565139 | PASS |
| vertex_cache / allocating | 1.077028 | 1.233743 | PASS | 1.085304 | 1.301668 | PASS |
| vertex_fetch / allocating | 0.959305 | 1.084708 | PASS | 0.923835 | 0.996694 | PASS |
| overdraw / allocating | 0.997523 | 1.428598 | PASS | 1.063730 | 1.714627 | **FAIL max** |
| coverage / allocating | 1.002840 | 1.463045 | PASS | 1.019922 | 1.615840 | **FAIL max** |
| omm_measure / allocating | 1.171527 | 1.293070 | PASS | 1.176544 | 1.347339 | PASS |
| omm_rasterize / allocating | 1.109900 | 1.309512 | PASS | 1.095131 | 1.267948 | PASS |
| omm_rasterize / caller | 1.169040 | 1.312282 | PASS | 1.115069 | 1.267578 | PASS |
| omm_entry_size / allocating | 0.470318 | 0.472621 | PASS | 0.471002 | 0.475791 | PASS |
| omm_compact / allocating | 1.294428 | 1.307289 | **FAIL mean** | 1.299921 | 1.351852 | **FAIL mean** |
| tangents / allocating | 1.134720 | 1.258948 | PASS | 1.171413 | 1.342646 | PASS |
| tangents / caller | 1.145240 | 1.285017 | PASS | 1.127842 | 1.251835 | PASS |
| normals / allocating | 1.194135 | 1.281951 | PASS | 1.194366 | 1.309566 | PASS |
| normals / caller | 1.179244 | 1.259929 | PASS | 1.212749 | 1.301528 | PASS |
| remesh / allocating | 1.124983 | 1.256369 | PASS | 1.133404 | 1.237745 | PASS |
| remesh / caller | 1.100657 | 1.295486 | PASS | 1.123593 | 1.321010 | PASS |

**Failing cases grouped by cause** (interpretation grounded in [P05_DIAGNOSIS.md](P05_DIAGNOSIS.md); this timing pass does not independently prove instruction-level causality):

- **Tiny fixed-workload overhead — OMM compact, allocating, both profiles.** All six cases contribute to the failing family mean; each individually passes the 1.50 maximum rule. Actual timed input is always four entries (levels 0–3), six references, and 12 or 22 bytes, so every case is **tiny**. The outer fixture labels range from 8 vertices/4 triangles to 512/512 but do not enlarge the compact workload (`parity/p05/bench.cpp:152`, `parity/p05/src/main.rs:722`). Diagnosis identifies validation, scratch/copy/probe structure and a small hash cost; the timed Rust batch also builds its data with Vec pushes versus C++ pre-sized storage. This is fixed small-call overhead in the frozen harness, not evidence of a scalable compact slowdown. Ratios by case (Moss / default): `s0` tiny 1.289832 / 1.269462; `s1` tiny 1.302014 / 1.344966; `s2` tiny 1.278150 / 1.251736; `s3` tiny 1.307289 / 1.351852; `s4` tiny 1.298985 / 1.277542; `s5` tiny 1.290507 / 1.307236.
- **Size-dependent raster work — Cargo-default overdraw, allocating, `s7`, large (600,000 vertices / 1,000,000 triangles): 1.714627×.** Stops at ten pairs, interval [1.605981, 1.862368], FAIL. Shapes s0–s6 pass. The gap appears at large size, rather than as tiny fixed overhead. The diagnosis identifies transform/pixel work, checked accesses and profile-sensitive code generation; the earlier clear/reduction fixes are already present. This pass does not isolate the remaining large-input mechanism.
- **Size-dependent raster work — Cargo-default coverage, allocating, `s7`, large (600,000 vertices / 1,000,000 triangles): stage-one median 1.615840×.** Borderline at 5, 10 and 20 pairs; the only D146 case. Thirty fresh, unpooled pairs on a freshly selected fixed core give median 1.745995× and interval [1.718684, 1.777992], FAIL. Shapes s0–s6 pass. Classification and causal limits are the same as overdraw. D146 decides the maximum only; the table retains stage-one geomean/worst as required by the harness.

Size vocabulary: tiny ≤ 16 work items, small 17–256, medium 257–65,535, large ≥ 65,536; geometry uses triangle count, compact uses actual entries/references. No small/medium compact scaling test exists in this frozen matrix. No other family/API or case fails the requested timing bars.

**Protocol and evidence.** Unmodified scalar reference `4c203430ca565cb59a468a91922c76c208169536`; C++ O3, NO_SIMD, no fast math/FMA. Rust opt-level 3 with empty Rust flags: Moss thin LTO/cgu=1, Cargo-default LTO=false/cgu=16. Alternating Rust/C++ pairs use matched batches on one selected physical core, with warmup and per-pair CPU/load/admission receipts. Source, compiler environment, reference and retained executable hashes are recorded.

Moss used 159 five-pair cases and one twenty-pair case; default used 149 five-pair, eight ten-pair and three twenty-pair cases, plus the single D146 stream above. Each profile completed in one admitted burst (Moss 3m09s, default 3m33s), checkpointing every completed row; no restart or repeated completed row. Both commands exit 1 because the performance bar fails, not because collection failed. Load-1m ranges were 9.952–10.805 and 11.270–24.786 respectively; these are shared-machine paired measurements, not an isolated-machine lower-bound claim.

Entry point (run once per profile, substituting `moss` / `default`):

```sh
CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/codex-p05-measure \
MESHOPT_ARTIFACTS=/mnt/linux-extra/meshopt-artifacts/p05-measure-20261005 \
RUSTFLAGS= CARGO_ENCODED_RUSTFLAGS= \
python3 parity/p05/lease_run.py --lane p05-measure-PROFILE --memory-gb 4 -- \
  python3 parity/p05/benchmark.py --consumer-profile PROFILE --families all --mode final
```

The existing launcher submits `MOSS_HEAVY_GPU=1 moss-heavy.sh 4 timeout --foreground --signal=TERM --kill-after=5 840 ...`; shared admission chooses the reservation/cap and owns the lease. No direct lease acquisition or admission bypass.

Artifacts: `/mnt/linux-extra/meshopt-artifacts/p05-measure-20261005`. Full `benchmark-0.5-{moss,default}.json`, binaries, scalar library, build manifest, `sources.tar.gz`, environment, logs, timing receipts, `audit.py`/`audit.json`, and cleanup receipt are retained there. Checked-in summaries are `results/benchmark-{moss,default}-0.5.json`; `results/p05-current-state.json` binds the artifact root and inventory hashes.

The audit recomputed all case inventories, medians/geomeans, early screens, D146 separation/decision, memory ratios, CPU/load receipts and shared admission coverage, and verified current source/reference and retained executable identities: PASS for both profiles. Historical functional/fuzz/WASM records and historical perf profiles were not refreshed; this measurement-only task does not claim a fresh full cross-phase release verifier pass. The mandated Cargo target was deleted after validation; retained executables remain in the artifact directory.
