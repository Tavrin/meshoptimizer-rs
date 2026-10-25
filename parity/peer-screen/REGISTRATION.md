# Bet 0 registration

Recorded 2026-10-25T15:02:51.863728+00:00; no builds, correctness runs or samples have occurred.
Baseline: 775ff8cab4fb2ba01966acd1b277aa7019eb8937 (v0.2.4).

## Registered protocol (write REGISTRATION.md before any sample; apply verbatim)
- **Arms:**
  - O, ours at 0.2.4;
  - M7, meshopt 0.7.0 through `meshopt::ffi`, as the existing adapters do;
  - X, optimesh 1.1.0 through its public API.
  Use the same frozen inputs, Moss's thin-LTO consumer profile and the Cargo-default profile, scalar
  and SIMD builds as the harness already defines, and the same CPU pinning.
- **Families:** every family the harness covers today, plus:
  - meshlet building and cluster LOD where X offers them (M7 has no cluster LOD: mark it
    unavailable);
  - the three real meshes 214e, b6fd and a082, for simplify and overdraw, through the a082-diag
    replay inputs at /mnt/linux-extra/moss-scratch/meshopt-a082-diag/, if X and M7 accept them;
  - browser decode on the 42 frozen streams: O simd128, X wasm SIMD, and upstream JS SIMD as the
    reference.
- **Correctness before timing:**
  - Each arm's output for each family is checked against scalar C++ 1.3. Record one of: identical;
    different but valid (the harness's independent validity checks pass, with triangle count and
    reported error recorded); or invalid.
  - M7 is expected to be identical; X ports 1.1 and may differ.
  - A family whose outputs are not identical is still timed, but labelled "not output-equivalent".
    Its timing table rows carry the output metrics next to the ratio.
  - An invalid output is not timed.
- **Timing:** use the harness's existing paired protocol, unchanged: five alternating pairs, extended
  once to twenty when the paired interval straddles equality. Report each family as the geometric
  mean of case ratios (peer time / ours) with its 95% interval, plus the max case.
- **Noise floor:** one same-session A/A family set, O against a second O build from identical source
  in a different path, on the three densest families. F is the largest |family GM − 1|.
- **Verdict per (peer, family):**
  - PEER FASTER if the upper bound of peer/ours is < 1 − F − 0.02;
  - OURS FASTER if the lower bound is > 1 + F + 0.02;
  - otherwise EQUAL WITHIN NOISE;
  - or UNAVAILABLE.
  A verdict applies only to output-equivalent families; others get "NOT EQUIVALENT" plus the
  numbers.
- **"Beats ours anywhere":**
  - Yes, if any family with an X cluster-LOD or wasm decode arm is PEER FASTER.
  - Also yes, if any non-equivalent family shows X faster beyond the same bound with no worse
    reported error at the same triangle count. That case is flagged for owner review, not counted
    as a quality win.
- **Infrastructure:** a harness timeout or crash allows one rerun of that family, which is recorded.
  No retuning, and no reruns to change a verdict.


Safety admission is a prerequisite. A safety stop produces no timing verdict and no invented F.

## Run 2 amendment registered 2026-10-25T15:06:15.399761+00:00

 The first run (campaign-20261025T150110Z,
SAFETY.md) stopped because meshopt's build runs `ar` through the `cc` crate. That was the spec's
error, not a finding: the allowlist named only "the C/C++ compiler or rustc".
- The allowed processes are the C/C++ toolchain that `cc` drives (compiler, archiver `ar`/`ranlib`,
  and the MSVC equivalents), rustc, and rustc/autocfg version probes. Writes stay under OUT_DIR.
- Network access, any other process, and writes outside OUT_DIR still stop the screen.
- Keep optimesh's oracle features and meshopt's `generate_bindings` feature disabled. The first
  review cleared only the default and `simd` paths.
- Reuse the first review instead of repeating it. Confirm the resolved dependency tree and source
  hashes match its SHA256SUMS.json and cargo-tree.txt, and re-review only what differs.
- Append this amendment to REGISTRATION.md before any sample. No sample was taken in the first run,
  so the measurement rules stand as registered.


## Run 2 implementation details, before any timing sample

The three densest native A/A families are simplify, meshlets and meshlets-spatial;
their five existing geometry cases run once in the Moss SIMD consumer configuration.
The second O source tree is an exact copy under this campaign's aa-source path.
F is the largest absolute GM deviation from one among these three families. This
single floor applies unchanged to every native/browser verdict, as the spec directs.

Ratios are peer/O. A case estimate is exp(mean(log paired ratios)); its nominal
95% interval is the existing harness's Student-t log interval. A family GM is the
geometric mean of its case estimates. Its nominal log interval propagates the
independent case mean-log variances (sum of variance/n divided by case_count^2),
using t=2.776445 when any case has five pairs, otherwise 2.093024. This does not
adjust for sequential stopping or multiple comparisons. Max is the largest case GM.
Every output check precedes all native timings; resident output hashes are checked
again before calibration. Allocation, required copies, scratch and destruction stay
timed; meshlet padding normalization remains outside timing, as in the harness.

Real-case inputs use the first simplify and first overdraw replay records referenced
by a082-diag. Retain the complete stride-64 vertex records and exact recorded target,
error bits, options and threshold bits. XYZ bits are additionally packed in the fixture
prefix solely for the independent validators; timed position access uses the original
stride-64 bytes. The former packed position preflight is retained separately and is
not used as a real-replay timing result. Frozen synthetic fixtures are unchanged.

Browser extension: reuse REPORT-3's frozen 42 streams, RustArm/JsArm transports and
headless-only muted Chromium CDP driver. Replace its unpaired 15-call medians with
the registered five alternating pairs / once-to-twenty rule. Batch calls to at least
25 ms during calibration to avoid its documented 0.1 ms timer zero samples. X and O
both allocate decoded output then copy to the caller buffer; source is resident.
The upstream shipped JS helper remains the reference transport, including its copy.
Each browser output must match fresh scalar C++ 1.3 bytes before any browser sample.
Browser bytes and timing runs are separate. Each pair is durably posted to a loopback
receipt server. A waiting train produces PAUSE between streams and releases admission.

Before any sample: O and X Wasm SIMD modules receive REPORT-3's successful
`wasm-opt -Oz --enable-simd` postprocess in both consumer profiles. Raw modules,
commands, version and both hashes remain retained. This matches the referenced
browser arm's existing build treatment; upstream JS keeps its shipped SIMD blob.

Pre-sample correction: the earlier implementation draft expanded A/A to four
configurations. That expansion is rejected: the spec calls for one A/A family set.
No timing or calibration sample preceded this correction. Earlier draft retained as
REGISTRATION-implementation-v1.md; no threshold or original protocol text changed.

Before any sample: consumer symbol stripping removes Wasm target_features metadata.
wasm-opt 108 therefore receives explicit enable flags corresponding to REPORT-3
raw module feature declarations (bulk-memory, sign-ext, nontrapping-float-to-int,
multivalue, mutable-globals, reference-types, SIMD). -Oz is unchanged for every arm.
The initial preprocessing rejection is retained; see WASM-PREPROCESS.md.

Pre-sample input-fidelity correction: no timing sample exists. The initial packed
real preparation is retained in packed-preflight/. Real timing now uses full captured
records and call arguments. Native drivers and the second-path replica are rebuilt.
Synthetic correctness hashes remain canonical references; every new resident output
must match its validated hash before calibration. Real output checks are regenerated.
