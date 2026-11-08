# 0.2.20 disposition after Continuation 3

**BLOCKED / INCOMPLETE. Original qualification remains NOT QUALIFIED.**
No heavy job was admitted within the requested 45-minute bound. The single
submission was stopped before compilation: driver SIGTERM, subprocess exit -15,
shell convention 143. No cancel/resubmit loop or admission bypass was used.
The wrapper exited. Release cleanup timed out acquiring the queue lock after
20 seconds; subsequent reads verify no queued row or reservation for its PID.
No lane job remains intentionally queued.

## Applied candidate and decision

Applied the shading draft to source base `0058d05e60b0e1bbde7319cd2f2b4fea5f56ff75`. The new
`BuildMesh::normal_error_weight` prices `2 * weight * L * sin(theta / 2)` in
original-source regions, maximized with geometric and inherited floors. L is
the largest source/replacement triangle edge in the region. Builder revision 8
binds the weight in cook keys; MDAG 2 layouts remain compatible. Legacy APIs
are untouched in source; fresh byte-identity validation has not run.

Static review repaired the draft's public-error regression: undefined normal
interpolants still report `NumericalFailure` in the public quality API. The new
private metric uses the largest chord instead. The decoded-cut evaluator and
regressions are applied but uncompiled and untested. The prepared finite-budget
validation command's missing positional argument was also fixed.

**Keep zero disabled as an unmeasured placeholder. Do not accept this candidate
as qualified or select numerical normal/UV/tangent weights.** Every requested
cell in [the twenty-row table](shading/TABLE.md) is UNMEASURED.

## Exact stopping point and gates

The first submission was `$HEAVY_WRAPPER 4 cargo test --release --all-features
--lib normal_error_tests`, with GPU use disabled and the dedicated target.
The wrapper reported a 4 GiB reservation, 7 GiB cap and 4 GiB declaration.
Admission-lock contention, older priority jobs, push jobs and insufficient
admissible RAM and a protected landing budget kept it queued. The optional desktop offload path rejected the
selection; local admission continued. No compiler process or test started.

| Check | Exit / disposition |
| --- | --- |
| Source formatting / harness formatting | 0 / 0 |
| Public text | 0 |
| First shading test | NOT RUN; driver canceled 143 before admission |
| Clippy / release / all-feature release / rustdoc | NOT RUN |
| Legacy clusterlod / MDAG 1 identity | NOT RUN |
| Shading / attribute matrix / finite budgets | NOT RUN |

## Original bars: historical before, candidate after

| Bar | Prior retained evidence | Applied candidate |
| --- | --- | --- |
| Build <= half reference, 1T / 8T | 0/5 / 2/5 FAIL | UNMEASURED |
| Equal-deviation cut triangles, 5 / 8 px | 4/5 / 3/5 FAIL | UNMEASURED |
| File bytes per DAG triangle | 2/5 FAIL | UNMEASURED |
| Resident bytes per DAG triangle | 0/5 FAIL | UNMEASURED |
| Geometric measured/claimed median and p95 | 5/5 PASS | UNMEASURED |

Historical calibration: 35.70% of sampled 1T cycles in tree construction,
18.45% in nearest queries and 15.20% in closest-triangle arithmetic (69.35%
combined). The retained caller-floor variant's summed median timing ratios are
0.976438 at 1T and 0.992273 at 8T. These do not validate the shading candidate.
Rocks' prior 8 px cut ratio is 1.674574; dragon's is 1.0717. Prior reports and
rejected variants remain in [the archived disposition](REPORT-before-continuation3.md).

## Finish and remaining work

The dedicated target was deleted after the driver stopped and no process
retained its target environment. Caller-owned evidence and prepared scripts
remain. HEAD and index are unchanged; fuzz sources are unchanged. No new library
unsafe code or unwrap was added. [Status](shading/continuation3-status.json) and
[source hashes](shading/continuation3-source.json) bind this disposition.

Resume at compiling/testing the applied candidate with 4 GiB, then one
interleaved weight campaign and the attribute matrix. Successful measurements
must select the default and attribute set. Rebuild that selection, validate
finite budgets and actual-default byte identity, update documentation, then run
all final gates. Use 4 GiB for normal jobs; reserve 9 GiB only for all-features
doctests at the end. Stop after 45 minutes without admission. Further original
cut/storage work remains superseded by shading priority. No bar was relaxed.
