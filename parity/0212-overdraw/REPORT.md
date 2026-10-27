# SPEC-0212 overdraw — NO-SHIP

The last registered campaign failed the strict per-case regression gate.
Four packed overdraw cases exceed 1.01; worst medium-smooth 1.016442517.
All overdraw source changes were reverted to origin/main. The committed src/
tree equals main; this is an evidence-only draft PR. No further repair or
campaign is authorized. No accepted performance claim or CHANGELOG entry.

Baseline v0.2.11: 40dbf9e42efc34d9292e439633e9ef52bc2248df.
The pending original-path repair was built after integrating main 4068c7a.
No library code was edited after observing results except the required revert.
Coordinator-authorized public script naming changes follow the archived run;
they do not change or replace its receipts. PREVIOUS-REPORT.md is a superseded
checkpoint, not the current outcome.

## Rejected techniques

These locations refer to the rejected candidate in the retained source archive:
- src/overdraw.rs:27: original packed algorithm and reader at their original paths; views delegate to a private fast module.
- src/input.rs:646: eight full stride-64 records, bitwise coordinate admission, exact unit-scan budget fallback.
- src/input.rs:915: shared maximum-index bound for full records; original reader isolated.
- src/overdraw/fast.rs:207: coordinate bounds omit only proved-unreachable finite errors; normal squared-norm overflow remains checked.
- src/overdraw/fast.rs:313: omit the proved finite target guard only on the existing prepaid path; original checked path and reservation retained.

No new unsafe, public API, root backend or SIMD. PROOF.md retains the numerical
and work-prefix arguments. Passing behavioral proof did not clear regression.

## Last campaign instruction results

Median of three per arm; instructions:u only, perf 6.17, CPU 9, scalar opt3,
thin LTO, one codegen unit. Base is v0.2.11, M7 is scalar meshoptimizer 1.3 via
meshopt 0.7.0, X is optimesh 1.1.0. The frozen rescreen driver, inputs and fixed
repetitions were reused. Real rows are case medians; corpus summaries are equal
case geometric means. Any individual ratio >1.01 rejects the entire candidate.
No wall-performance or GPU claim.

| Family | candidate/base | candidate/M7 | candidate/X | Case/family verdict |
|---|---:|---:|---:|---|
| real-214e | 0.786567 | 1.604342 | 1.172212 | PASS |
| real-a082 | 0.876417 | 1.388565 | 1.119791 | PARTIAL |
| real-b6fd | 0.876420 | 1.388521 | 1.119781 | PARTIAL |
| overdraw | 1.013773 | 1.005698 | 0.926401 | FAIL |
| analyze-overdraw | 1.000000 | 0.943495 | 0.977256 | WITHIN-REGRESSION-LIMIT |

The real PASS/PARTIAL labels do not override the overall NO-SHIP decision.

| Case / operation | base instructions | candidate instructions | M7 instructions | X instructions | candidate/base | candidate/M7 | candidate/X | Verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| medium-seams / analyze-overdraw | 6397691.80 | 6397689.10 | 7322110.90 | 6586404.90 | 0.999999578 | 0.873749276 | 0.971347677 | WITHIN-REGRESSION-LIMIT |
| medium-seams / overdraw | 1674882.30 | 1702082.70 | 1593088.40 | 1827145.50 | 1.016240186 | 1.068416982 | 0.931552906 | FAIL |
| medium-smooth / analyze-overdraw | 9426547.00 | 9426545.40 | 9694952.90 | 9631884.70 | 0.999999830 | 0.972314719 | 0.978681296 | WITHIN-REGRESSION-LIMIT |
| medium-smooth / overdraw | 1651821.30 | 1678981.40 | 1599085.30 | 1808888.00 | 1.016442517 | 1.049963626 | 0.928184277 | FAIL |
| medium-sparse / analyze-overdraw | 9578648.20 | 9578645.60 | 9787904.50 | 9750404.20 | 0.999999729 | 0.978620664 | 0.982384464 | WITHIN-REGRESSION-LIMIT |
| medium-sparse / overdraw | 1765896.40 | 1793059.20 | 1628383.20 | 1876453.00 | 1.015381876 | 1.101128530 | 0.955557746 | FAIL |
| million-smooth / analyze-overdraw | 618104397.00 | 618104378.00 | 771204972.00 | 723594421.00 | 0.999999969 | 0.801478725 | 0.854213852 | WITHIN-REGRESSION-LIMIT |
| million-smooth / overdraw | 197576586.00 | 200808909.00 | 191784604.00 | 217329085.00 | 1.016359848 | 1.047054377 | 0.923985434 | FAIL |
| real-214e / overdraw | 578994175.00 | 455417930.00 | 283865779.00 | 388511547.00 | 0.786567378 | 1.604342488 | 1.172212084 | PASS |
| real-a082 / overdraw | 1577532448.00 | 1382575785.00 | 995686828.00 | 1234672990.00 | 0.876416702 | 1.388564904 | 1.119791067 | PARTIAL |
| real-b6fd / overdraw | 394395300.00 | 345655962.00 | 248938320.00 | 308681863.00 | 0.876420084 | 1.388520506 | 1.119780601 | PARTIAL |
| tiny-smooth / analyze-overdraw | 2990558.73 | 2990558.71 | 2665351.66 | 2676540.74 | 0.999999996 | 1.122012813 | 1.117322323 | WITHIN-REGRESSION-LIMIT |
| tiny-smooth / overdraw | 16024.41 | 16096.41 | 20235.42 | 18009.29 | 1.004492820 | 0.795456888 | 0.893783391 | WITHIN-REGRESSION-LIMIT |

## Proof and exits

All four arms built fresh (0). The diagnostic packed precheck rejected
million-smooth at 1.01636 (1); its twelve samples remain separate from the
156 final samples. The full campaign proceeded as registered before execution.
All 156 samples exited 0, counter running fractions met >=99%, and every
candidate output equaled base. No samples were replaced or selected.

Locked all-features/no-default tests, clippy -D warnings, fmt, boundary,
std/no_std exact builds and sweeps, pinned C++ comparison, and fuzz all exited 0.
Each exact sweep compared 20,558,816 cases, each with cold/warm calls: Result,
complete output including tails/partial writes, and Usage. All entry points,
thresholds, every small-mesh work limit through success+1, record/budget/admission
boundaries and worst-case meshes are covered. C++ comparisons: 178, all equal.

Widened differential fuzz: 3,675 seeds, 705,600 generated cases, 300.062058 child CPU-seconds, zero mismatches. CPU duration is a fuzz work budget.

Campaign exits: build=0, packed-precheck=1, proof=0, measure=0, results=1, audit=0, cleanup=0. Runner exit=1 (regression rejection).

The independent audit passed before reverting source: 156 samples and 3,097
bound files. binding.json describes the measured rejected candidate, not the
restored shipping tree. Raw commands, counters, inputs/outputs, standalone
binaries, source archive, exact/fuzz logs and admission receipts remain in
`<scratch>/overdraw/`. final-lane-before-public-renames/ preserves the actual
executed scripts; their hashes precede the public-only naming migration.
The designated target was deleted and absence verified (cleanup=0;
915,852,499 apparent file bytes). Public-text guard passes after main integration. Public binding paths use
neutral aliases for historical cluster-LOD file names; hashes are unchanged.

Prior rejected complete campaigns remain retained: worst packed ratios
1.020039, 1.034843 and 1.056722; all their proofs passed, all their instruction
gates failed. See PREVIOUS-REPORT.md and rejected-* evidence for the previous
checkpoint. Superseded blocked binding/cleanup are replaced by the final audit
and cleanup receipts. No merge, tag, publication or version bump.

## Decision

NO-SHIP. The final regression gate failed, so overdraw remains as on main for
this release. Preserve evidence, commit as Tavrin, push an evidence-only draft
PR, append the actual result to the coordinator resume, and stop. No pending
permission, repair, acceptance campaign or implementation task remains.
