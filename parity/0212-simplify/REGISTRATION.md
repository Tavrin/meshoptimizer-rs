# SPEC-0212 simplify — registration and continuation record

Historical status at initial registration: implementation and scripts written,
UNQUALIFIED. The operations hold was active. No <job-runner> admission, build, test, fuzz run, sqrt sweep, C++ build or
instruction measurement has been launched in this lane. Only source inspection,
code/script writes and a read-only Git diff review have been performed.

Branch: `perf/simplify-hot`. Entry HEAD:
`40dbf9e42efc34d9292e439633e9ef52bc2248df` (release merge). Exact baseline:
`v0.2.11^{}` = `40dbf9e42efc34d9292e439633e9ef52bc2248df`.
Annotated tag object: `4b0c94bcf57861d8f80bcfdedf7a482bd2e99ff3`.
The release merge and tag have the same tracked tree before these edits.
Read first: `<scratch>/meshopt-0212-simp-attr/REPORT.md`.
No subagents. No Git commit, push, PR, version bump, tag or publication during hold.

## Candidate decisions and proof obligations

- `src/simplify.rs::sqrt` uses `f32::sqrt` under std for bit patterns below
  `0x7f800000` (positive finite numbers and +0). IEEE correct rounding gives a
  unique representable result matching the correctly rounded portable backend.
  Negative operands, -0, infinities and NaNs retain the exact pinned backend,
  including its NaN payload/sign behavior. `simplify_sqrt_exhaustive_identity`
  compares the actual production adapter against that backend for all 2^32
  operand bit patterns, with `black_box` to prevent folding. This proof has NOT
  run; mathematical rounding arguments do not substitute for the required sweep.
- A simplify-local std `SqrtCache` calls the adapter directly, removing memo
  initialization/hash/traversal. no_std aliases the original portable memo.
  Mutable root call sites and all arithmetic evaluation order remain intact.
  Shared `src/math.rs`, overdraw, meshlet and clusterlod support are unchanged.
  Workspace Usage accounts heap capacities, so removing this inline memo does
  not change charged bytes. Exact cold/warm Usage comparisons remain required.
- `State::pick` reborrows remap/kind/forward/back at one active vertex length;
  `State::quadrics` similarly shares the positional-array length with its
  kind/forward/back slices. These are safe slices, with no new unsafe code or
  unchecked indexing. Existing per-corner work checks precede lookups.
- `run_state` dispatches pick through existing `metered!`, bounded by the current
  input index count m. Proof: indices consist of m/3 complete triangles; pick
  enters each triangle at most once and visits exactly three corners in an
  entered triangle. Each corner charges one unit, even if it creates no collapse.
  The capacity break occurs before entering a triangle and can only reduce
  visits. There are no other charges or fallible reader/arithmetic callbacks in
  pick. Hence visits <= m, without multiplying by degree, wedge count, number
  of candidates or capacity. Overflow is impossible for the local tally since
  m is already a slice length. `metered!` checks covers(m) without consuming
  work; the covered path tallies the exact visited total and settles once.
  If m does not fit, the original `Work` path preserves every failure prefix.
  The dedicated unit test compares both paths for all limits 0..13, welded and
  unwelded meshes, and capacities 0/2/3/6/12. The public exact sweep adds high
  valence, duplicate/reversed faces, degenerate/welded faces and disconnected
  triangles, including every budget up to unrestricted work+1.
- Keep all finite guards. Finite inputs do not prove finite intermediate
  subtraction, cross products, quadrics or error accumulation. No ablation from
  the attribution lane is treated as a shippable finite-guard proof.
- Defer new welding/hash algorithms and attribute-rank accounting bounds: the
  available evidence does not establish their error/read/work equivalence.

## Frozen instruction gate (apply verbatim)

Arms: base (v0.2.11), candidate, and M7. Workloads: the frozen corpus simplify
families (simplify, attributes, update, sloppy, prune, points) and the 18
real-chain simplify stages. Use the harness of the attribution lane. Take the
median of 3 per arm.

- **Real chains:** PASS if candidate/base ≤0.85; PARTIAL if ≤0.95; NO-GAIN otherwise.
- **Any case** with candidate/base >1.01 is FAIL: fix it or revert that change.
- Report candidate/M7 throughout.

Thirty corpus cells and eighteen real stages yield 432 fresh stat samples.
No attribution measurements are reused as new-arm evidence. Real-chain summary
ratios divide sums of the six stage medians; each stage is also reported.
Corpus summaries use the equal-case geometric mean, with every case shown.
No aggregate can mask a case above 1.01.

`prepare.py` freezes the attribution resident-loop driver, corpus inputs and
stage TSVs. Referenced real vertex/index blobs remain read-only and their SHA256
names are verified. Each arm is rebuilt using the attribution scalar profile:
opt3, thin LTO, one codegen unit, no SIMD or target-cpu flags. M7 uses Cargo
debug=0 and CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD -g', preserving NDEBUG without
forcing frame pointers. Rust debug=1 retains source symbols. Source/compiler,
manifest/lock, binary, input and output hashes and admissions are retained.

`measure.py` requires perf 6.17 and uses only `perf stat -e instructions:u`,
CPU9, acknowledged enable/disable and attribution iteration counts. Loading,
warm calls, validation and serialization stay outside counters. Destination
allocation, required input copies, scratch acquisition and destruction stay
inside. Arm order is base/candidate/M7, base/M7/candidate, base/candidate/M7.
All samples must have 100% counter running and valid topology/finite outputs.
Candidate output must equal base, including update positions and error bits.
Every repetition's complete output is retained separately. No wall calibration
or wall-time measurements. Timeouts are operational bounds only.

## Prepared proof

`gates.sh`: locked all-feature/no-default tests; all-target/all-feature clippy
with -D warnings; fmt check; unsafe boundary check; explicitly ignored release
exhaustive sqrt test; independent std/no_std exact consumers; pinned C++ simplify
and attributes differential; widened exact differential fuzz for >=300 child
CPU seconds. CPU fuzz duration is a work budget, not a performance result.

`exact.rs` compares typed Result names and result float bits, all output words
including failure prefixes and unused tails, update position/attribute bits,
and complete Usage (work and bytes), twice per workspace. It covers allocating,
into, too-small into and destructive update entry points across all six
families, all 512 experimental option masks, every small-mesh work budget
0..unrestricted-work+1, both retention modes on worst-case meshes, byte limits,
invalid topology, and overflow/non-finite/subnormal/signed-zero inputs. The
fuzz generator additionally varies attribute width through 32, target, weights,
flags, raw f32 patterns and exact work-boundary limits; unlimited calls ensure
new native-root and covered-picker paths are reachable. It compares against a
freshly archived, source-identical v0.2.11 renamed only for Cargo coexistence.

## Registered release sequence — unexecuted at registration

Only after explicit coordinator steering that train 17 has landed, acknowledge
that steering in the environment and invoke each phase through the wrapper:

```sh
export MESHOPT_SIMPLIFY_TRAIN17_RELEASED=yes
bash parity/0212-simplify/admit.sh bash parity/0212-simplify/job.sh prepare
bash parity/0212-simplify/admit.sh bash parity/0212-simplify/job.sh build
bash parity/0212-simplify/admit.sh bash parity/0212-simplify/job.sh proof
bash parity/0212-simplify/admit.sh bash parity/0212-simplify/job.sh measure
bash parity/0212-simplify/admit.sh bash parity/0212-simplify/job.sh report
```

Every admission declares 4 GB, MOSS_HEAVY_GPU=0, one Cargo job and the dedicated
`<target-dir>` target. Artifacts go
only under `<scratch>/meshopt-0212-simplify/`. Preparation
refuses to overwrite that directory. Scripts retain failed exits and stop;
retain any failed campaign before separately registering a repair/rerun.

After gates and instruction review: fix/revert every >1.01 regression; record
all ratios/verdicts/exits in REPORT.md; update CHANGELOG [Unreleased] with
instruction ratios only; append the measured result under "0.2.12 simplify" in
`~/dev/specs/meshopt/RESUME-2026-10-24.md`; delete only this lane's designated
Cargo target. Commit as Tavrin with no AI trailer and push a draft PR. Never
merge, tag or publish. Stop at about five hours active work excluding the hold.
The hold does not authorize inventing ratios, claiming passed proof, or pushing
an unqualified candidate as a finished result.

## Hold handoff — historical

Instruction ratios/verdicts: NOT RUN / UNQUALIFIED for every case.
Build/test/proof/fuzz/measurement exit codes: NOT RUN (not zero).
PR URL: none; deferred under the code/scripts/registration-only hold.
No Cargo target was created by this turn, so there is no lane build target to
clean up now. The next dependency is coordinator release, followed by validating
and repairing this unbuilt candidate and its scripts.

## Released continuation and repair 1

The user explicitly confirmed train 17 landed and the operations hold RELEASED
on 2026-10-26. CPU-only <=4 GB admissions are authorized. The first preparation
admission exited 2 before Python or any build: an apostrophe inside Bash's
`${parameter:?message}` caused an unmatched quote in job.sh. Removed that
apostrophe; the admission's failed receipt remains in coordinator history and
this registration. No campaign directory was created or overwritten. Rerun
preparation, then the unchanged frozen gate and proof sequence.

## Repair 2: test-only Result context

The initial proof phase exited 101 at test-all compilation. The picker test
expanded `metered!` inside a tuple-returning helper, but its `?` operators need
a Result-returning context. Wrapped that invocation in a Result closure. No
production code changed. Retained initial three successful arm builds, source
hash binding, binaries, failed test log and exit TSV under
`attempt1-test-compile/`. Rebuild all three arms and restart the complete proof
sequence; no instruction samples were taken before this repair.

## Repair 3: test formatting

Second proof: test-all=0, test-no-default=0, clippy=0, fmt=1. Applied the exact
rustfmt diff to the new test only. Initial successful gates and second build
receipts/binaries are retained under `attempt2-test-format/`. Rebuild the arms
and restart all proof gates with the final formatted source binding. No
instruction samples have been taken or selected.

## Final continuation result (2026-10-27)

The released sequence completed: final builds, complete proof, all 432 fresh
instruction samples and report generation exited 0. All three real chains are
PARTIAL: 214e 0.892071/1.670312, a082 0.867441/1.549117, b6fd
0.864697/1.554694 (candidate/base and candidate/M7). No case exceeds 1.01.
Keep the qualified gains; the <=0.85 PASS target remains unmet. No extra
optimization campaign or altered gate is justified by this partial result.

The independent retained-evidence audit runs through job.sh audit after report.
REPORT.md, results.json, instructions.tsv, proof-exits.tsv and binding.json hold
the final evidence. The historical unqualified handoff above is superseded by
this continuation and REPORT.md. Commit/push a draft PR after audit and target
cleanup; never merge, tag or publish.

Baseline clarification: the initial registration mislabeled the annotated tag
object as the peeled commit. The actual tag peels to entry HEAD 40dbf9e;
ENTRY.json and the complete exact/archive audit bind that correct commit and
tree 3ede4f74b0f5b560e74de50e47ec008c20709d7d. No baseline changed.

Final evidence audit=0. The designated target was deleted and verified absent;
see cleanup.json. The final continuation supersedes the historical hold status.
