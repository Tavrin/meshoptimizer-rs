# Superseded checkpoint before the last campaign

# SPEC-0212 overdraw — BLOCKED / UNQUALIFIED

No implementation is qualified for acceptance. Three complete campaigns passed
the behavioral proof but failed the strict instruction regression gate. The
latest original-path repair has not built or run: operations reimposed a hold
until 2026-10-27 05:37:11 CET for a shared train rerun. The queued admission was
cancelled (143), the designated target was deleted (0, absence verified), and
all standalone source/binary/counter/output/proof evidence remains in private
scratch. No commit or PR was pushed; no merge, tag, publication or version bump.

## Latest source, unbuilt

The original 0.2.11 packed implementation and its original input reader remain
at their original module paths. Views delegate to a private fast module. The
pending tiny/million packed precheck must pass before a complete fresh proof
and 156-sample four-arm campaign. This precheck cannot clear any failed full
gate. Current-source tests, exact/C++ comparisons, fuzz and counters: NOT RUN.

Techniques prepared for views:
- src/input.rs:646: eight full stride-64 records, bitwise coordinate admission and exact unit-scan budget fallback.
- src/input.rs:915: shared maximum-index bounds for fixed records; original reader isolated.
- src/overdraw/fast.rs:207: validated coordinate bounds permit only proved-unreachable finite errors to be omitted; normal squared-norm overflow remains checked.
- src/overdraw/fast.rs:313: omit the already-proved finite target error on prepaid clustering; original checked work path and reservation retained.
- src/overdraw.rs:27: original packed module path retained; only views delegate.

No new unsafe, public API, sqrt backend, SIMD or other-family change. PROOF.md
contains the numerical and work-prefix arguments; these are not execution
receipts for the unbuilt repair.

## Most recent completed campaign — rejected

These numbers belong to the prior packed-submodule candidate, not the pending
repair. Median of three per arm, instructions:u only, scalar opt3/thin LTO,
CPU 9, perf 6.17, frozen rescreen driver/inputs. Every one of 156 samples exited
0 and every candidate output equaled baseline. Results rejection=1 because all
five packed overdraw cases exceed 1.01; worst million-smooth 1.056722. No gate
was relaxed and no sample was selected or replaced.

| Family | candidate/base | candidate/M7 | candidate/X | Case verdict |
|---|---:|---:|---:|---|
| real-214e | 0.774826 | 1.580393 | 1.154713 | PASS |
| real-a082 | 0.871829 | 1.381294 | 1.113929 | PARTIAL |
| real-b6fd | 0.871833 | 1.381253 | 1.113919 | PARTIAL |
| overdraw | 1.048064 | 1.039717 | 0.957738 | FAIL |
| analyze-overdraw | 1.000000 | 0.943495 | 0.977257 | WITHIN-REGRESSION-LIMIT |

The complete verdict is FAIL, including the real rows above. Per-case raw
medians and ratios are in rejected-results.json and rejected-instructions.tsv.
Final acceptance remains NOT RUN for the pending repair.

## Retained exits and proof

- First complete campaign: build/proof/156 samples=0; results=1, packed worst 1.020039.
- Second complete campaign: build/proof/156 samples=0; results=1, packed worst 1.034843.
- Third complete campaign: build/proof/156 samples=0; results=1, packed worst 1.056722.
- Each proof: locked all-features/no-default tests, clippy -D warnings, fmt,
  boundary, std/no_std exact comparison and pinned C++ differential=0.
- Each std/no_std exact sweep: 20,558,816 comparison cases per mode, each with
  cold/warm calls; complete Result, output/tails/partial writes and Usage.
- C++: 178 comparisons per complete campaign, all equal.
- Fresh differential fuzz CPU budgets: 300.010768 / 300.028914 / 300.078804 child
  CPU-seconds, all zero mismatches. CPU duration is a fuzz work budget.
- Scratch setup missing-document build=101 retained; two held configuration
  transitions=143, no builds or samples. Current held admission cancelled=143.
- Blocked target cleanup=0; verified absent. Current-source build/proof,
  packed precheck, final counters/audit and PR push: NOT RUN.

Private receipts remain under <scratch>/overdraw/attempt1-packed-regression/,
<scratch>/overdraw/attempt2-packed-validation/ and
<scratch>/overdraw/attempt3-packed-module/. Initial A/B/AB/C attribution (63
samples), safe-variant pilot (81), and block pilot (45) are also retained.
The pending source archive and WORK-IN-PROGRESS.json are in <scratch>/overdraw/.
Machine configuration and scheduling details remain private; public scripts
read configured environment paths. No wall-performance or GPU claim.

## Resume

After coordinator admission is available, configure the private environment
and invoke `admit.sh bash job.sh final` with this lane's relative script paths.
All builds/measurements use a CPU-only 4 GB runner. Rebuild all four arms,
require the packed precheck, then repeat the complete proof, fuzz and all
156 counters. Reject every >1.01 case. Audit and delete <target-dir>, update
Unreleased with accepted instruction ratios only, commit as Tavrin, push a
draft PR and append the actual result to the coordinator resume. Never merge,
tag or publish. No permission question is pending; the blocker is admission.
