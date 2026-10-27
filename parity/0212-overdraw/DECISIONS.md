# Decisions and retained attempts

- Treat c396a697 as post-#39 main; do not rebase or alter simplify work.
- Diagnostic finite/work ablations are scratch-only. Finite inputs can overflow
  during subtraction/cross product/centroid accumulation. No finite arithmetic
  guard may be removed solely because it reduced the attribution count.
- Reuse the frozen rescreen driver and inputs; measure no wall performance.
- Attribution setup attempt 1: base build 0; A build 101 because scratch copy
  omitted docs/USING.md needed by include_str!. Retained under
  attempt1-missing-doc/. Added docs to every isolated copy; no library change.
- First two admissions used the Python interpreter's shared wrapper class,
  which reserved 8 GB despite the explicit 4 GB declaration. Subsequent calls
  use the wrapper's supported lane-specific class to avoid inheriting unrelated
  Python histories. The declaration remains 4 GB and GPU remains disabled.
- Operations added a train-18 lane hold while attribution was running. The
  already-admitted measurement continues; the queued predicate build remains
  held. Do not bypass, clear or reinterpret the hold. No held build is evidence.
- Register safe predicate follow-ups before measuring: D = B plus bitwise AND
  of the three is_finite classifications; E = B plus maximum of the three
  sign-cleared IEEE bit patterns below infinity. Both preserve the same single
  NumericalFailure return before any centroid arithmetic. D/E retain every
  intermediate guard, threshold check, work charge and output write. Compare
  three real cases, median three, using the same source-bound peer binaries.
- Coordinator public-file rule: all machine paths are environment configuration;
  public notes use placeholders. Cancelled the held, pre-configuration predicate
  admission (143, no build or sample), then requeued it with a private env file.
  This preserves the hold and avoids starting with missing path configuration.
- The exact sweep explicitly pins every coordinate's nonfinite validation
  failure at each of nine vertex positions, and empty meshes whose supplied
  finite coordinates overflow the unused centroid sum. Empty topology must not
  gain a new centroid-overflow failure. Both cold and warm calls are checked.
- Provisional validation implementation: retain all finite predicates and combine
  their already-loaded Boolean results; reuse fixed records only when the entire
  stride-64 little-endian view has trailing padding. The original reader remains
  for every short, mapped, interleaved, big-endian or other-stride view. The
  iterator's exact length lets Work::scan select the same covered/exhaustion
  behavior. No new precharge or finite-intermediate proof is assumed.
- F's shared record bound uses the maximum after converting each index to usize,
  matching the original lookup conversions on every pointer width. The maximum
  dominates all three accesses. Each record is [u8; 64], so its first [u8; 12]
  exists without another runtime boundary test. Mapped and short views retain
  the generic reader; little/big-endian full records use the shared proof.
- No guarded arithmetic is regrouped: centroid additions, area/cross-product
  evaluation, cluster scoring, threshold targets and normalization retain
  the original arithmetic order. Guard removal is limited to the explicit
  invariants in PROOF.md; every reachable error retains its original point. Neither a favorable diagnostic guard-removal
  result nor finite inputs establish finite intermediate calculations.
- Register G before the held pilot: F plus removal of only two unreachable
  reciprocal errors and three unreachable normalized-component errors. The
  retained squared-norm check proves finite components; positive-root bounds
  prove a finite reciprocal; normalization remains below 16 by a loose IEEE
  bound. Finite area_sum plus nonnegative root accumulation proves its reciprocal
  finite, including subnormal results. PROOF.md gives the derivation. No other
  guard is removed. This candidate is not qualified until exact std/no_std,
  C++ and widened CPU-budgeted fuzz pass.
- Removed private admission environment names from public scripts as well as
  paths and executable names. The public runner contract uses a generic
  admission identity and CPU-only marker; a private launcher supplies the actual
  scheduling configuration. A second held configuration-transition command was
  cancelled (143, no build/sample) and requeued. All declarations remain 4 GB.
- Register H: fuse input finiteness with coordinate admission at magnitude 2^30,
  dispatch once to a bounded or general score kernel, and omit only checks whose
  errors are impossible under the proved coordinate bounds. The proof uses an
  accumulation interval, not an optimistic error estimate or face-count bound.
  The normal squared-norm check remains because bounded repeated faces can
  overflow it. Exact cases straddle admission by one ulp, include unused outlier
  vertices, and test 2/4 repeated bounded triangles at the normal-overflow
  boundary. Widened fuzz includes one such success/failure case per seed,
  alternating by seed parity, plus both bounded and general random paths.

- Before any H build/sample, tightened its triangle-norm proof and admitted the
  largest supported power of two, 2^30. Three rounded squares sum to at most
  3*2^126 < f32::MAX; area <=2^64, raw centroid <=2^121, final score <=2^67.
  This improves reach without changing an acceptance gate. Retained the H24
  prototype and binding in private scratch. At the new bound, two stress faces
  succeed, three overflow the norm sum, and four overflow individual squares.
  All prefixes are now affordable in the exact sweep; fuzz cycles all three
  outcomes across seeds rather than spending its CPU budget on huge repeats.
- Register I before pilot execution: the existing prepaid target bound makes its
  target finite check unreachable. A statically dispatched meter method omits
  it only on ClusterPrepaid; Work retains the original check. The reservation,
  all charges, threshold conditions and arithmetic order remain unchanged.
- First safe-variant pilot passed all 81 samples/output checks. I was best:
  214e 0.784011 (PASS), b6fd 0.872667 and a082 0.872663 (PARTIAL). Preserve it.
- Register one bounded follow-up J before its counter campaign: eight-record
  admission reduction plus infallible unit scans for proven groups; rejected
  groups, tails and other layouts keep checked visits. Every group has exactly
  eight visits, with original low-budget granularity. The eight-record path
  permits restoring scale_full_records' original opaque Iterator signature,
  leaving the simplify helper's public-to-the-crate type unchanged. PROOF.md
  records the budget/error argument and the expanded boundary exact sweep.

## Final source selection

Select J. Its block pilot passed all 45 samples/output checks: 214e 0.775204
(PASS), b6fd 0.869227 and a082 0.869223 (PARTIAL). It improves on I on every
real input. Stop optimization; run the complete proof and independent fresh
four-arm final campaign. No pilot count is final acceptance evidence. Retain
J only if every required proof passes and no final case exceeds 1.01. The
<=0.80 target remains unmet for the two PARTIAL pilot inputs. Final outcomes
and exact counts are recorded by REPORT.md after execution.

## Rejected first complete campaign and packed-path repair

All first proof gates passed (20,558,816 comparison cases per std/no_std mode,
178 C++ cases, 300.010768 child CPU-second differential fuzz), and all 156
counter samples completed. The instruction verdict is nevertheless FAIL:
medium-seams overdraw 1.014435581, medium-smooth 1.017515090 and million-smooth
1.020038574 exceed 1.01. Retain the entire rejected campaign, source archive,
proof binaries/logs, raw samples and outputs in the private attempt directory.
No passing proof or focused retry clears it.

Repair: packed positions use their original finite/centroid reduction and the
general score kernel; the real stride-64 path keeps the qualified optimizations.
No magnitude admission is needed on the packed path. Restart all four builds,
all proof gates including five CPU-minutes fuzz, and all 156 counters. The
thresholds remain verbatim, and no old sample is reused as acceptance evidence.

## Rejected second complete campaign and packed compatibility kernel

The narrow packed-validation repair passed the entire proof again, including
fresh 300.028914 child CPU-second fuzz, but every packed overdraw case exceeded
1.01 (worst million-smooth 1.034843). Retain all 156 samples, binaries, source
archive, proof receipts and logs as a second rejected complete campaign.

Decision: packed calls dispatch to a private source-identical v0.2.11 kernel
(only visibility differs). Preserve its algorithm and validation together;
keep all new optimizations on the view path. This uses no workload-size cutoff
or relaxed gate. The compatibility code remains safe Rust and introduces no
public API. Restart all builds, proof gates, fuzz and 156 counters once more.

## Rejected third campaign and original-path isolation

The copied packed module passed the full proof but regressed packed counters
(worst 1.056722). Source identity alone did not establish instruction identity.
Retain all receipts as the third rejected complete campaign. Restore the
baseline implementation at its original module path and restore its original
input reader. Isolate optimized views and their reader in private fast paths.
Run a tiny/million packed counter precheck before the complete proof, then all
156 fresh four-arm counters. A precheck cannot clear any failed full gate.

## Last campaign: NO-SHIP

All proof gates passed, but four final packed cases exceeded 1.01; worst
medium-smooth 1.016442517. The coordinator's last-campaign ruling is terminal.
Reverted all overdraw src/ changes to origin/main, retained all evidence, and
made no additional repair or measurement. Public script naming migration is
coordinator-authorized and postdates the archived execution. Commit and push
only parity evidence as a draft PR; no accepted CHANGELOG performance claim.
