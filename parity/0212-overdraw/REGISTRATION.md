# SPEC-0212 overdraw registration

Baseline v0.2.11 peeled release commit 40dbf9e; candidate starts at post-#39
c396a697. All attribution variants are throwaway scratch copies of v0.2.11.
A removes geometry finite guards (diagnostic only, never shippable); B uses the
existing full-64-byte fixed-record iterator for validation; AB combines both;
C removes per-item Work accounting. None introduces unsafe code. Patches,
source copies and build receipts are retained. Attribution covers all three
frozen real inputs, median three instructions:u samples per arm, with M7/X.

Safe predicate follow-up: D combines B with the conjunction of three pure
is_finite classifications using boolean &, permitting a single branch. E
combines B with the maximum sign-cleared integer bit pattern below infinity.
Both are exactly equivalent on all IEEE f32 values, including signed zero,
subnormals, infinities and every NaN payload. Neither removes a finite guard.
F combines D with a maximum-index proof for full-record triangle reads,
replacing three individual checked record lookups by one checked maximum.
The same per-triangle Work::scan and IndexOutOfBounds error are retained.
G combines F with removal of the two reciprocal finite guards and three
normalized-component finite guards whose finiteness follows from the existing
area-sum and squared-norm checks and the positive root lower bound. See PROOF.md.
All overflow-capable sum, product, centroid, difference and final-score guards
remain. This experiment must pass the same exact/std/no_std/fuzz proofs.
They are measured on all three real cases with three samples per arm before
selecting the final candidate. All these samples remain pilot evidence.

Final frozen gate: arms base (v0.2.11), candidate, M7, X. Workloads overdraw and
analyze-overdraw on all five frozen corpus meshes, plus three real-overdraw
inputs, using the rescreen driver and exact frozen files. Median three per arm.
Real inputs: PASS <=0.80 candidate/base; PARTIAL <=0.95; NO-GAIN otherwise.
Any case >1.01 candidate/base is FAIL and must be fixed or the change reverted.
Report candidate/M7 and candidate/X for every cell. Corpus summary uses equal
case geometric means; no aggregate overrides a failed cell.

perf 6.17 instructions:u only, CPU9, acknowledged resident counter control;
loading/warm call/output validation outside counts, required copies/allocations,
scratch, validation and destruction inside. Fixed repetitions from rescreen;
no wall timings or calibration. Fresh builds for all final arms. Preserve all
failed/rejected attempts. All builds/measurements/proofs through admit.sh,
`<job-runner>` declaration 4 GB, MESHOPT_GPU=0 and dedicated target. Scratch only
`<scratch>/overdraw`; lane files only here.

Required proofs: locked all-features/no-default tests, clippy -D warnings, fmt,
unsafe boundary, independent v0.2.11 exact output/Result/Usage comparison under
std/no_std, every small-mesh work limit 0..terminal+1, all three entry points and
too-small buffers, thresholds and worst-case meshes; C++ overdraw differential;
widened differential fuzz for >=300 child CPU-seconds. No wall-performance
claim. Preserve finite arithmetic checks and failure prefixes.

Record decisions, full per-case ratios/verdicts and exits. Update Unreleased
with instruction ratios only; append RESUME under 0.2.12 overdraw; delete exact
Cargo target; commit as Tavrin, push draft PR, never merge/tag/publish. About
five hours active-work budget. No subagents.

H is a registered bounded-coordinate follow-up on G. The same validation
callback classifies all loaded coordinates against absolute magnitude 2^30;
passing proves finiteness, while a failing classification runs the original
finite predicate and selects the general geometry path. A single geometry
phase dispatch selects a const-generic score kernel. The bounded path omits
only area-sum, centroid-product, difference and score guards proven unreachable
by PROOF.md; the squared-normal overflow guard remains at its original point.
The general path retains those guards. Both existing work-accounting paths
and all arithmetic order are unchanged. H joins the three-real-input pilot,
with three samples and both peers, before final selection.

I joins the registered pilot on top of H. ClusterPrepaid omits only the finite
check on its already-proven target; Work retains that check at its original
point. No charge or reservation changes. The target proof is in PROOF.md.
The safe-variant pilot now has nine arms and 81 samples, all registered before
its first build or counter sample. The final four-arm gate remains unchanged.

After the first pilot (I: 214e 0.784011; b6fd 0.872667; a082 0.872663), register
J: groups of eight complete little-endian stride-64 records, a pure bitwise
admission reduction, an infallible Work::scan for admitted groups, and the
original checked per-vertex path otherwise. Work::scan's unit remains one:
insufficient group budgets retain each original failing prefix. Tails and all
other layouts retain checked reads. No heap allocation or new budget formula.
The block follow-up has base/I/J/M7/X, three real inputs, median three (45
samples). It reuses only identified baseline/peer/I binaries, takes all fresh
counters, and preserves the previous 81 pilot samples. Final gate unchanged.

## Coordinator final campaign ruling

This is the last campaign. Integrate origin/main 4068c7a before building the
pending original-path repair. Freeze all code before execution; no code edits
after observing results. The packed precheck is diagnostic: retain its exit
and continue the complete proof and 156-sample gate even if it rejects.
Run the independent audit and cleanup after a regression rejection too.
Any final case >1.01 requires restoring the overdraw changes in src/ to
origin/main, retaining parity evidence, recording NO-SHIP, and pushing an
evidence-only draft PR. No further repair, pilot, campaign or gate relaxation.
