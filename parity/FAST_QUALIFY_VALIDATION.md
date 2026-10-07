# Fast qualification validation

**Verdict: NOT RELEASE-GRADE — mandatory timing comparisons remain unverified.**

This review lands provisional tooling. It does not establish the planned
10–20 minute wall-time target, case/family agreement, or parallel-load noise.
Earlier exploratory records used superseded harness hashes and cannot
authorize release results. Local archive paths and operational notes are
kept outside the repository.

Acceptance requires all ten mandatory revision/profile comparisons:

| Revision label | Phase | Profiles |
|---|---|---|
| main | 0.1 | moss, default |
| main | 0.2 | default |
| main | 0.3 | consumer, release-defaults |
| main | 0.4 | moss, default |
| phase/0.1.x | 0.1.x preprocessing | moss, default |
| phase/0.1.x | 0.1 geometry | crate |

Labels alone are insufficient: each comparison must bind exact library,
oracle, dependency, build-profile and case-input identity. If the historical
source is unavailable, run both methods on a fresh exact-source export.
If full wall time is missing, retain a fresh same-source full run as its
provenance without replacing the archived verdicts.

Require zero family-verdict disagreements. A case disagreement is admissible
only when every changed timing gate lies inside the full record's own IQR.
Heap disagreements cannot be excused by clock noise. Report every
disagreement with both measurements, case/family agreement counts, full/fast
wall times and matching single-core/parallel smoke dispersion. Smoke never
qualifies a release. All required evidence must exist before issuing a receipt.

The unchanged bars are geometric mean <= 1.25, case maximum <= 1.50, memory
ratio <= 1.25 and the exact registered phase 0.2 decoder floors. Sequential
intervals assume independent stationary samples; family-mean acceptance is
an empirical comparison, not a confidence-sequence claim. There is no 0.5
adapter, so this method does not qualify that matrix.

Review verification on current main: cargo fmt --check, cargo clippy
--all-targets -- -D warnings and cargo test --locked each exited 0. The
27 untimed Python self-tests passed (exit 0), including synthetic worker
sequencing, raw-record comparison, stale-receipt rejection and cache identity.
These checks establish tooling behavior only. Timing agreement and speed
remain unmeasured.
