# SPEC-03 finish handoff — external admission blocker

**Incomplete: final performance qualification is blocked.** Measured-source candidate: `8319f7d3624694f7f71f2fe1f45ff977c10e17d9`; the final documentation commit preserves its source and consumer hashes. PR [#6](https://github.com/Tavrin/meshoptimizer-rs/pull/6) remains draft.

The one-line regression-test loop now uses MSRV-stable `as_chunks::<4>()`. All exact CI test-job commands pass locally on pinned Rust 1.99.1, and both MSRV commands pass on 1.88. Global default toolchain is unchanged. [Source CI passed](https://github.com/Tavrin/meshoptimizer-rs/actions/runs/40001341141).

The retained build controller rebuilt all binaries in fresh `seen-frontier`, regenerating source guards and binary hashes. Previous receipts are intact in `seen-frontier-pre-ci-0f04876`. Both profiles pass 17,264 parity comparisons, 369,098,752 exhaustive Exp component comparisons and capacity/edge checks with zero mismatches. Earlier `a6fc95f` numbers and diagnostic scouts are superseded for final acceptance; none qualifies this candidate.

`measure.sh` was invoked headlessly without an outer heavy scope, with separate CPU-only burst admission and borrowing disabled. Its first historical-request phase never obtained admission. After an earlier delivery-gate hold released, a new hold for train `97762443c8` extended to **23:49:50 CEST on 2026-10-23**. The queued attempt was stopped with zero performance samples collected. No completed stream was repeated, and no partial measured case exists. This is an external scheduling block, not a performance pass/fail or an exhausted four-lane-day cap.

Resume from this worktree only after normal admission is available:

```sh
MOSS_HEAVY_USE_DECLARED=1 MOSS_HEAVY_NO_BORROW=1 bash /mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier/measure.sh
```

The controller runs exact historical requests, all 111 native encoder cases and all 40 shared peer cases in defaults/Moss order, with 690-second checkpoints and each burst under `moss-heavy.sh 3 timeout 840`. No quiet-machine wait. Source/binary guards must still match. Do not repeat completed streams or discard interrupted cases. After completion, apply unchanged GM ≤1.25 and every resolved upper95 ≤1.50 bars in both profiles, report unresolved A/A rows, refresh P03/CHANGELOG and portable evidence, then push and confirm final-head CI. No further optimization is authorized.

Durable receipts: `/mnt/linux-extra/moss-scratch/meshopt-enc03/seen-frontier`; blocker, CI-equivalent logs, extra-check cancellation disposition and cleanup: `/mnt/linux-extra/moss-scratch/meshopt-enc03/finish`. The fixed Cargo target was below 10 GB and deleted. Retained binaries require no Cargo rebuild to resume timing.

[P03](../P03_ENCODER_PERFORMANCE.md), [portable summary](../results/enc03-allocating-profiles.json), [compressed receipts](../results/enc03-allocating-profiles.json.gz). Reversal cost: revert one test-loop line, restoring the CI lint failure; prior epochs remain inspectable but cannot qualify changed source. Not verified: final native/peer timings, downstream runtime/cook, GPU/browser execution, release fuzz budgets or universal performance. No merge, publication or tag.
