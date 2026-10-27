# downstream-qx5g3 release check v2 registration

UTC: 2026-10-25T11:05:57.557437+00:00

Registered before all qualification and measurement. Candidate f40f5af; baseline afe803b; fallback a7688ef. Initial HEAD f080cb512b1ee3dd0ad80a3a74ae917bde68211d. Clean worktree. Required src diff d74b914..f40f5af is empty.

The complete approved protocol below is binding. Alignment qualification will be appended and frozen before step 1. No timing samples have been collected. Archive copies use unchanged release parity/src, identical in B/B-prime. Normal arms have empty Rust flags; perturbation flags apply only to Bp/Cp.

# SPEC — downstream-qx5g3 release check v2: no regression vs 0.2.3, with a measured noise floor and replication
Kind: proof
Effort: medium
Heavy: yes
Status: APPROVED by the coordinator on 2026-10-25, before the downstream-z9kkq repro result was known.
Registered when the job writes release-v2/REGISTRATION.md, before any sample.

Repo: meshoptimizer-rs worktree ~/dev/meshoptimizer-rs-wt/simplify-perf (branch
perf/simplify-large-mesh, draft PR #10). You are a fresh agent. You may commit records under
parity/release-check-v2/ on this branch and push to PR #10, with author
`Tavrin <4936492+Tavrin@users.noreply.github.com>` and no AI trailers. Never merge, tag, publish, or
change src/. Build variants from `git archive` copies under
<scratch>/meshopt-simplify-perf/release-v2/src-*, each with its own target dir
under <target-dir>. Delete those target dirs at the end,
after saving the binaries. Heavy work goes through <job-runner>;
measurement jobs set PARITY_RUNNER_MEASUREMENT=1. **Train first:** between cases, never mid-pair,
yield to any waiting <worktree> job and record a PAUSE. Do not
spawn subagents. Never delete evidence.

## Context
- Release check v1 (parity/release-check/REPORT.md) gave NO RELEASE on case-level rules: the
  candidate had 6 regressing cases, the fallback 4. No family regressed.
- The downstream-z9kkq diagnosis (<scratch>/meshopt-simplify-perf/z9kkq/DIAGNOSIS.md):
  - Builds of identical source differ on this host by up to 3.4–3.8% per case. The binaries are not
    bit-identical, and both A/A controls trip v1's 1.02 lower-bound case rule.
  - The worst v1 case (vertex_cache million/smooth allocating, 1.131) runs no changed code. Its
    kernel bytes are identical after relocation masking; only its alignment moved.
  - Repro addendum (z9kkq/repro/RESULT.md): context only. It does not feed F, T or any case's
    verdict.
- Disclosure: these rules were revised after v1's NO RELEASE, in light of the measured A/A floor.
  The candidate code is unchanged since v1.
- Candidate: f40f5af. Confirm `git diff --stat d74b914..f40f5af -- src/` is empty. Fallback:
  a7688ef. Baseline: afe803b (B).

## Pre-registered measurement and decision (apply verbatim)
- **Builds:** all arms use the parity release profile (fat LTO, codegen-units = 1), empty
  RUSTFLAGS, and the release check's unchanged driver.
  - B and B′ are two builds of afe803b from identical source in different archive paths.
  - C is the candidate.
  - Bp and Cp are the same two sources built with the qualified alignment option (see Layout
    perturbation).
- **Session:** one measurement job, CPU 8. Every comparison is 1 warm-up pair, then exactly 12 ABBA
  pairs per case, with no adaptive stopping. Bootstrap 95% intervals use 10,000 within-case pair
  resamples, seed 20261025. A case's statistic is the median of its paired ratios; a family's is
  the geometric mean of its case medians.
- **Step 1, main:** C/B on every case of the frozen matrix.
- **Step 2, same-session A/A:** B′/B on every case. The floor F is the largest absolute deviation
  of any case median from 1. Then T = ceil_to_0.001(1 + F + 0.02).
- **Step 3, replication:** every case whose C/B lower bound is > T is re-measured as Cp/Bp, same
  protocol.
- **Layout class:** for each case flagged in step 1, compare the hot kernel symbols of C and B,
  meaning every function holding ≥10% of the case's samples in one `perf record` per arm. If their
  bytes are identical after relocation masking and only their addresses or alignment differ, the
  case is LAYOUT.
- **Rules:**
  - A family regresses if its C/B GM point estimate is > 1.02.
  - A case regresses only if all three hold: its C/B lower bound is > T, its Cp/Bp lower bound is
    > T, and it is not LAYOUT.
  - Replication verdict: a flagged case is a REGRESSION only when its Cp/Bp lower bound also exceeds
    T and the LAYOUT test does not classify it. Otherwise it is listed as NOT REPLICATED or LAYOUT.
  - LAYOUT cases and cases that don't replicate are still listed in the report, with their numbers.
- **Infrastructure:** a harness timeout or crash, not a measured value, allows one full rerun of that
  job, which is recorded. A second one means NO RELEASE.
- **Verdict:**
  - RELEASE: no family regresses and no case regresses.
  - FALLBACK: otherwise, run steps 1–3 once on a7688ef, with a fresh A/A in that session. It
    releases if that passes. Otherwise the verdict is NO RELEASE.
  - No retuning, and no reruns to change a verdict.
- **Absolute bars:** these come from r2's complete frozen-final matrix (all families pass), as in
  v1.
- **Last rule set for f40f5af:** if v2 plus its single fallback says NO RELEASE, the candidate is
  closed. Any later attempt needs new code and a fresh registration, with no further rule revision
  for this code.

## Layout perturbation (qualified before step 1; recorded in REGISTRATION.md)
Try these options in order. The first that qualifies is used, and it applies to both arms:
1. `-C llvm-args=-align-all-functions=6`
2. `-C llvm-args=-align-loops=64`

An option qualifies if all of these hold:
- both C and B build with it;
- `parity/run.sh --release 0.1 --profile scalar-strict` and the three real-case hash checks pass on
  the C build;
- the cache kernel and the overdraw kernel start on the alignment the option promises, in both
  arms, checked in the symbol table.

If none qualifies, Bp and Cp are B and C each rebuilt with the same no-op perturbation: one unused
private `#[inline(never)]` function in src/simplify.rs, as z9kkq's A3. Record which path applied.
Put the qualifying option in REGISTRATION.md and the report together with the raw symbol-table check
output (kernel symbol, address, address mod 64, for each arm).

## Proof (falsifiable)
The check fails if any of these is true:
- REGISTRATION.md post-dates a sample;
- any comparison has other than 1 + 12 pairs;
- T is not computed from this session's A/A;
- a replication or layout test is skipped for a flagged case;
- the decision is not applied verbatim.

## Done-when / Stop condition
- release-v2/REPORT.md holds the family table, T and F, the flagged, replicated and LAYOUT cases,
  the verdict and the raw samples.
- On RELEASE: add the README known-exceptions line and the CHANGELOG entry, as in v1 Goal 3.
- Push to PR #10, and run `br comments add downstream-qx5g3`.

Stop there; the owner runs the review and the publish.

## Expansion rule / Non-goals
Never here: no src/ changes, no fixes, no bar or corpus changes, no repo harness changes, no
version bump, tag or publish. Record any evidence and report it.

## Final message
Token-lean:
- F and T;
- the family table;
- flagged, replicated and LAYOUT cases;
- the alignment option used;
- the verdict;
- wall time;
- receipts;
- not verified.

Never end on a question.


## Qualification frozen before step 1

UTC: 2026-10-25T11:56:19.951665+00:00

Selected first option: `-C llvm-args=-align-all-functions=6`. Both arms built; release 0.1 scalar-strict parity and all three real-case exact hash checks passed on Cp. `cmp` verified the parity qualification executable equals the retained Cp driver. The scratch-only runner preserves RUSTFLAGS and removes the empty encoded-flags override. Its inherited descriptive rust_profile sentence still says flags are empty; the retained Cargo fingerprints, explicit build flags and symbol checks establish the actual alignment flag; no algorithm or driver changes. Real-case checker is the retained r2 exact checker with archive dependency/output paths adapted. Initial unqualified perturbation binaries and build incidents retained in build-preflight-incident.json. The real-case checker dependency build hit its 7 GiB cap and <job-runner> queued a retry at 11 GiB, which refused to overwrite the immutable completed parity receipt. The unfinished real-checker build was resumed alone at 11 GiB; the original OOM log is retained; no measurement occurred.

Raw kernel symbol table checks (every required address mod 64 = 0):

```text
Bp meshoptimizer_rs::cache::kernel::<false> 0x36180 mod64= 0
Bp meshoptimizer_rs::cache::kernel::<true> 0x38580 mod64= 0
Bp meshoptimizer_rs::overdraw::kernel 0x50a40 mod64= 0
Cp meshoptimizer_rs::cache::kernel::<false> 0x36ac0 mod64= 0
Cp meshoptimizer_rs::cache::kernel::<true> 0x38ec0 mod64= 0
Cp meshoptimizer_rs::overdraw::kernel 0x52980 mod64= 0
```

Full raw nm outputs: Bp-symbols.txt and Cp-symbols.txt. Build/profile/source identities: identities.json. Qualification logs: parity-qualification.log, real-qualification.log, real-Cp/hash-checks.json.

Controller uses unchanged performance.Paired and parity/src. ABBA is 12 alternating paired ratios (AB/BA), identical to v1. Bootstrap uses NumPy PCG64 seed 20261025 and 10000 within-case resamples. A single logical measurement session covers main, fresh A/A and replication. A train PAUSE releases admission at a completed-case boundary; resumption keeps that session and never repeats a completed case. Any waiting train job causes PAUSE regardless of queue reason. Infrastructure crash/timeout permits one complete session restart only. Layout profiling is diagnostic and cannot feed F, T, or the comparison statistics. No prior diagnosis/repro timings feed decisions.

Protocol script manifest SHA-256: `840d826c270d679cf41ebc795f508977f2aa144fa2d146d7a638551371712630`. Exact scripts retained in protocol-frozen/.
