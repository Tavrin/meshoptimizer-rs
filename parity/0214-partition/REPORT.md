# Partition instruction experiment

**Verdict: FAIL and revert.** The candidate exceeded the 1.01 candidate/base per-case limit on 80 of 241 cases. Its corpus candidate/base geometric mean was 1.010327, so it also missed the 0.90 PARTIAL threshold; candidate/M7 was 1.426411, above the 1.15 PASS threshold. The candidate and its temporary fuzz-target widening were reverted. The retained library source is identical to the registered base; no partition speedup is claimed.

## Attribution and attempted changes

The pre-edit comparison is in [ATTRIBUTION.md](ATTRIBUTION.md). Sampling identified adjacency construction and filtering as the dominant Rust symbols on the larger corpus cases, with a smaller repeated `pick` share. C++ inlined much of its corresponding work into its partition entry point; sample shares include process setup and are not isolated causal measurements.

The attempted candidate bound a filtering mark once, used an upper bound for adjacency-count accumulation with the checked, exact-budget fallback, and specialized `pick` by spatial scoring and a proven shared-sum bound. The checked `u32` sum remained the fallback. These were combined in one candidate, so this campaign does not isolate which technique caused the regressions. Candidate locations were `src/partition.rs:199`, `src/partition.rs:298`, `src/partition.rs:444`, and `src/partition.rs:482` in the retained candidate snapshot. The current source is reverted; those locations describe the experiment, not retained behavior.

## Instructions

Three `instructions:u` samples per arm and case were taken in the resident loop. Base, candidate and M7 were compared with identical inputs, and the warm-up output bytes matched for every arm and case. All counters reported 100% running. The table shows medians per call; the complete 241-row table includes all nine raw counters per case and the regression decision in [instructions.tsv](instructions.tsv). Chain identifiers in that table are anonymous; input hashes bind them to the retained captures. One complete campaign was run; no second candidate was submitted after the revert.

| Corpus case | Base | Candidate | M7 | Candidate / base | Candidate / M7 | Case gate |
|---|---:|---:|---:|---:|---:|---|
| tiny smooth | 10,005.126 | 10,129.126 | 6,951.126 | 1.012394 | 1.457192 | FAIL |
| medium smooth | 2,041,822.430 | 2,052,051.750 | 1,486,934.770 | 1.005010 | 1.380055 | PASS |
| medium seams | 1,584,380.750 | 1,615,370.780 | 1,163,721.050 | 1.019560 | 1.388108 | FAIL |
| medium sparse | 2,190,107.960 | 2,208,768.590 | 1,513,397.000 | 1.008520 | 1.459477 | PASS |
| million smooth | 273,218,089.000 | 274,917,546.250 | 189,676,370.750 | 1.006220 | 1.449403 | PASS |
| Corpus geometric mean | — | — | — | **1.010327** | **1.426411** | **NO-GAIN** |

The 236 captured real-chain calls had candidate/base ratios from 1.000653 to 1.024905; 78 failed the 1.01 limit. Together with two failing corpus cases, the campaign verdict is FAIL. The candidate's geometric mean and case results describe the rejected experiment, not the restored source.

All 41 selected chain replays matched their saved complete LOD output before their 236 partition calls were admitted to the measurement set. The trace snapshot recorded the exact indices, counts, positions and target for each call. Input, source, binary and adapter bindings are summarized in [binding.json](binding.json); complete captures, counter files, rejected candidate patch and admission receipts are retained in the run artifacts. The M7 partition source hash matches the pinned upstream 1.3 source.

## Proof and exits

The candidate proof ran both locked feature configurations, all-target all-feature clippy with warnings denied, formatting, the unsafe-boundary and public-text checks, and all exited 0. The independently built baseline comparison passed 10,882 exact observations in each feature mode, including result, count, assignments, work, storage and complete sentinel buffers around limits. The pinned C++ comparison passed 256 cases. A seeded exact differential ran 10,014,000 cases over 300 CPU-seconds, exit 0. The widened existing fuzz target then ran three coverage-guided AddressSanitizer batches for 338.34 CPU-seconds, exit 0; that widening was reverted with the candidate.

The restored source passed the same full gates: both Cargo test configurations, clippy, fmt, boundary and public-text checks, both exact builds and 10,882-observation sweeps, pinned reference check and 256 C++ comparisons, and the base-source identity check all exited 0. Restored-source differential fuzzing passed 10,326,000 cases in 300 CPU-seconds, exit 0. The enclosing restored-source gate exited 0.

| Operation | Exit | Outcome |
|---|---:|---|
| Candidate builds and full proof | 0 | All required checks passed before the campaign |
| Real-chain capture | 0 | 41/41 full outputs matched; 236 calls captured |
| Three-arm campaign | 0 | 241 cases and 2,169 samples; **FAIL** by the registered decision rule |
| Restored-source full proof | 0 | All checks passed; library and fuzz source identical to base |
| Instrumented widened-fuzz batches | 0 | Three batches, 338.34 CPU-seconds |
| Initial attribution recording | 1 | Rejected poll-file-descriptor capture; repaired recording exited 0 |
| Initial campaign-driver build | 101 | Adapter constructors missing fields; repaired build exited 0 |
| Initial widened-fuzz supervisor | 1 | Intentional interrupt produced target exit 72; rejected |
| Two partial instrumentation repairs | 130, 130 | Canceled to correct coverage flags and batch size |


Instrumentation history is retained: the initial `perf record` attempt exited 1 due to a poll-file-descriptor error; the repaired profiles exited 0 with no lost or throttled samples. The first widened-fuzzer runner consumed 300.82 CPU-seconds but treated its intentional interrupt exit 72 as an error, yielding wrapper exit 1 and no coverage guidance. Two subsequent partial runs were canceled (exit 130) to correct instrumentation and batch size. The final three naturally completed, instrumented batches above are the accepted fuzz evidence; the earlier outcomes are not erased.

The release branch advanced after this experiment was pinned. This draft remains based on `bd0c891` to preserve its source binding; the changelog entry will need reconciliation with the newer release history during integration.
