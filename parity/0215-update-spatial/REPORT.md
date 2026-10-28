# Update and spatial instruction qualification

## Disposition

Both functions are **NO-GAIN**. Both experimental source changes were reverted.
Final `src/`, `Cargo.toml` and `Cargo.lock` are identical to v0.2.14,
`dbb75e03ec011ae04645e6d40b0039561991286d`. The instruction targets remain
unmet. This PR retains the experiments, proof and reproducible negative result;
it makes no optimization or release claim. Shared-helper changes: none.

Campaign 1 tested the update bounds work scan plus spatial area-loop changes.
It stopped after 84 accepted samples: medium-sparse update was **1.011502315×**
baseline, above the mandatory 1.01 ceiling. The update change was reverted.
Campaign 2 tested spatial alone, with update restored to baseline. It completed
192 samples, exit 0. Neither function met candidate/base <= 0.97; spatial was
then reverted too. Exactly two campaigns ran. No implementation edits followed
results except these reverts. The rejected first campaign is retained, not
relabelled as a successful campaign or combined with the second campaign.

## Attribution and techniques

The inherited worktree was clean: useful partial work consisted of scratch
drivers, single-input screens, sampling profiles and assembly experiments.
No lane job remained active. Those artifacts were retained. The sampled spatial
profile placed 79.72% in BVH splitting; this is a diagnostic sample share, not an
isolated causal instruction attribution. The peer's box accumulator uses
explicit SSE/NEON dispatch, outside this lane's no-SIMD scope. No peer code was
copied. The contract-cost report does not isolate W/R/F shares for these two
functions, so shares from other functions are not assigned to them here.

- W, rejected: `src/simplify_extra.rs:737` replaced the post-solve bounds loop's
  per-visit charge with the existing fixed-count `Work::scan`. The baseline
  already has substantial covered-work specialization elsewhere in the solver.
  Correctness passed, but the registered instruction ceiling rejected this
  additional scan transformation. See `update-candidate.patch`.
- Scalar data flow, rejected: `src/meshlet_spatial.rs:25,253` borrowed box
  records, expressed extent products through fixed arrays and separated the
  independent forward/reverse area loops. Ordered comparisons and scalar sum
  order stayed unchanged. It did not reduce instructions. See
  `spatial-candidate.patch`.
- F/R: no additional finite check or root backend was removed. Existing std
  simplification already uses native roots. Solver attributes and mutable
  de-normalization do not supply a new universal finite envelope here.

`PROOFS.md` gives the accounting bound and observable-preservation arguments;
its file:line anchors refer to the retained patches. `REVIEW.md` records the
single-agent read-only source audit and its limitations.

## Registered instruction gate

Median of three `instructions:u` samples per cell. SHIPS requires the
three-real-mesh geometric mean candidate/optimesh <= 0.99 and candidate/base
<= 0.97; IMPROVES requires only candidate/base <= 0.97; otherwise NO-GAIN.
Any individual candidate/base > 1.01 forces a revert. All decisions use
unrounded values. All accepted counters reported 100% running.

Campaign 2 candidate ratios below describe the measured spatial experiment,
before its final revert. Update was already baseline-identical in that campaign.

| Function | Candidate/base GM | Candidate/optimesh GM | Candidate/M7 GM | Verdict |
|---|---:|---:|---:|---|
| simplify_with_update | 1.000000 | 1.076899 | 1.368246 | NO-GAIN |
| build_meshlets_spatial | 1.003880 | 1.104995 | 1.323781 | NO-GAIN; reverted |

Per-input candidate/base ratios, using medians:

| Input | Update | Spatial |
|---|---:|---:|
| tiny-smooth | 1.000000 | 1.000021 |
| medium-smooth | 0.999999 | 1.003604 |
| medium-seams | 1.000000 | 1.003594 |
| medium-sparse | 1.000001 | 1.003594 |
| million-smooth | 1.000000 | 1.003959 |
| real-214e | 1.000000 | 1.003892 |
| real-a082 | 1.000000 | 1.003893 |
| real-b6fd | 1.000000 | 1.003857 |

Maximum campaign-2 candidate/base: update 1.000001013, spatial 1.003958503.
`measurements.json` contains all four arms' median instructions per call and
candidate/base, candidate/optimesh and candidate/M7 ratios for all 16 cells.
`counts.jsonl` contains all 192 samples; all 16 canonical output identities
matched across all four arms and all repetitions. `rejected-counts.jsonl`,
`regression.json` and `rejected-build-binding.json` retain campaign 1.

Environment: rustc 1.98.1, perf 6.17.13, `LC_ALL=C`, CPU 9. Four arms use the
existing peers consumer's release/thin-LTO/one-codegen-unit profile, without a
target-CPU override. Peers are C++ 1.3 through meshopt 0.7.0 and optimesh 1.1.0.
Inputs and allocation policy are unchanged from the peers harness: five frozen
corpus meshes plus three real meshes, packed XYZ, four synthetic attributes,
half-index update target/error 0.5, spatial 64/41/124 with fill 0.5. Warmup,
parsing and serialization are outside counters; output allocation, required
copies and destruction remain counted. No wall-time, GPU, original-stride or
whole-chain throughput conclusion follows.

## Correctness and exit codes

All builds and executions used CPU-only admission with 4 GB declared; the
wrapper selected a 6 GB scope cap. Exact comparisons were rerun after the update
revert and passed with the same totals; totals below are per feature mode per
successful run, not sums of repeated runs.

| Proof command/stage | All features | No defaults | Exit |
|---|---:|---:|---:|
| `cargo test --locked` with the corresponding feature flag | pass | pass | 0 / 0 |
| `run.sh exact`: update | 44,549,696 | 10,878,208 | 0 / 0 |
| `run.sh exact`: spatial | 406,760 | 406,760 | 0 / 0 |
| `run.sh cpp`: update protocol subset | 178 | 178 | 0 / 0 |
| `run.sh cpp`: spatial protocol subset | 400 | 400 | 0 / 0 |

`cargo clippy --locked --all-features --all-targets -- -D warnings`,
`cargo fmt --check`, `bash parity/check-unsafe-boundary.sh` and
`bash parity/tools/check_public_text.sh` passed, exit 0 each. The complete
inherited C++ helpers passed 1,246 simplify-family/overdraw protocol cases and
1,280 meshlet/cluster cases per feature mode; the table isolates the two lane
functions. The oracle was checked clean at the pinned 1.3 revision.

Update exact comparisons cover eight six-vertex numeric meshes, widths 0/3/32,
all 512 masks (128 accepted without experimental), every integer work limit
from zero through observed unrestricted usage plus one, result/error, full
mutable output bits and both Usage fields. Spatial covers eight meshes, four
settings and four spatial min/fill variants, both owned and sentinel-filled
caller buffers, and work limits extending beyond unrestricted usage. Both
drivers repeat calls and compare priming results. Spatial's inherited footer
has a stale literal `cells=140`; the actual product is 128, as explained in
REVIEW.md. Spatial has 406,760 priming attempts, 399,848 successful, per mode;
update has 11,131,280/9,914,768 all-feature attempts/successes and
2,718,016/2,413,888 without defaults. Failed priming does not establish a
successful-warm state. The bounds scan itself only changes local bounds and
counter bookkeeping; its proof does not depend on retained solver contents.

Differential fuzzing, exit 0 each: spatial **308,100 cases / 300.048629 child
CPU seconds**; update **1,083,300 / 300.024618 seconds**. The new spatial
generator explicitly selects the spatial API, which the inherited two-operation
meshlet stream did not reach. Generators include valid split-producing meshes,
numeric extremes, layouts, short final records, mutable attributes, flags and
selected limit boundaries. Fuzz binaries test the first candidate; spatial
implementation is identical in campaign 2, and the final update change is a
revert to baseline. Exact/C++ checks were rerun on the second candidate.

Preliminary failures remain failures: the inherited development build omitted
an included documentation file (101), the first new spatial exact harness had
a macro-hygiene generation error (101), and clippy rejected an indexed array
loop (101). These were corrected before campaign 1. Two generic queued jobs
were cancelled before admission and replaced by a dedicated shell entry point;
neither produced measurements. Earlier interrupted sampling profiles are
diagnostics, not gate evidence. Campaign 1's exit 1 is the retained performance
regression decision. Campaign 2 exited 0.

## Retention, replay and limits

The measured binaries, raw perf files, canonical outputs, source snapshots,
original diagnostics, failed logs and fuzz journals remain in the designated
scratch area. `build-binding.json`, `proof-binding.json`, input hashes and
`receipts.json` bind public evidence to retained artifacts. First-candidate
sources/proof were archived before preparing campaign 2. The executed
supervisor is retained separately; the public supervisor's final admission
interface reads `PARITY_RUNNER_*`, and its later selector/archive/interface
edits did not change measured driver or collector code.

Replay in an isolated v0.2.14 checkout: apply both patches for the first
candidate, or only the spatial patch for the second candidate; stage the hash-matching frozen inputs
under the scratch artifact input directory. Use `run.sh qualify`, `checks`,
`cpp`, `fuzz`, `measure`, and `summarize` under admission with the documented
environment. For a fresh standalone replay use the default campaign directory;
use `LANE_CAMPAIGN=campaign2` when continuing after the original first campaign.
Map the admitted active/GPU values into `PARITY_RUNNER_ACTIVE` and
`PARITY_RUNNER_GPU` before invoking the committed supervisor. See
REGISTRATION.md. Completed instruction and fuzz records are resumable.

After both reverts, final source identity gives exact baseline behavior for
every input, limit and workspace state. No third measurement campaign was run.
Final post-revert tests, clippy, fmt, boundary and public-text checks all exited
0. `audit.json` records successful source/binary/proof hash verification,
sample uniqueness, complete cells, counter validity and output identities in
both campaigns. Both replay patches passed `git apply --check` against the
restored baseline. The designated Cargo target was deleted and verified absent,
exit 0 (`cleanup.json`); retained scratch artifacts were preserved. Independent
external review, cross-platform execution, MSRV and whole-application acceptance
were not performed. The two performance targets remain open for a future lane.
