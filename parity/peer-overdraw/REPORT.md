# registered protocol job B — overdraw peer gaps

Baseline v0.2.6 at 33556c48efc13f3f36a9293aa5fbf451c0884fe3.
Scratch: `<scratch>/meshopt-peer-b/`.
Target: `<target-dir>`.
Harness export: bench/peer-screen e8827e4de9b69c98b9a96c6f49f8192a6b045ff7.

The diagnosis below was recorded before edits. Final source passed targeted
proofs and is frozen for the single registered wall campaign.

## Setup decisions

Export the kept harness unchanged, then adapt only its scratch copy: point
the dependency to the baseline/candidate source and add a fixed-iteration
instruction mode that never calls the timer. The exported lock contains
meshoptimizer-rs 0.2.4; offline dependency resolution updates it to 0.2.6.
The initial --locked build rejection is retained, not a correctness failure.
Use the retained corpus byte-for-byte, verifying every input hash.

The first instruction job was queued under the machine's generic Python
class (9 GB historical reservation); canceled before execution, then submitted
through the job's named shell entry with its own measured class and the same
4 GB declaration. No samples were produced or discarded.
The first admitted counter probe could not launch: /usr/bin/perf is a kernel
version shim and lacks a tool for kernel 7.0.0-28. Use the installed real
`/usr/lib/linux-tools-6.8.0-110/perf` binary directly, with the same hardware
instructions:u event. Preserve the launch failure; no algorithm count existed.

## Diagnosis milestone — before source edits

Synthetic overdraw: the kernel routes every triangle norm and cluster normal
through a 512-entry hash cache and a software libm square root on misses;
X and M7 use a scalar hardware square root. The three final counting-sort
loops also update the fuel counter per record instead of batching a fixed scan.
The cost of validation and checked indexed reads remains part of our contract.

Real-overdraw: the same root path is especially expensive on varied captured
triangles, where the operand cache provides less reuse. The baseline retired
instruction ratios below establish the larger real-mesh instruction gap;
the root substitution will be isolated first with instruction counts. This is
a source-supported cause hypothesis, not a measured root-only attribution.
No approximation, reordering of FP expressions, or validation removal is proposed.

| Case | v0.2.6 instructions/call | X | M7 | v0.2.6/X |
|---|---:|---:|---:|---:|
| medium-seams | 2322078 | 1846270 | 1616957 | 1.257713 |
| medium-smooth | 2236105 | 1826514 | 1621138 | 1.224248 |
| medium-sparse | 2358792 | 1902684 | 1659152 | 1.239718 |
| million-smooth | 331072798 | 245277212 | 218536161 | 1.349790 |
| real-214e-overdraw | 1048738886 | 625047147 | 515249807 | 1.677856 |
| real-a082-overdraw | 2793568603 | 1530532570 | 1279677956 | 1.825226 |
| real-b6fd-overdraw | 698371600 | 382634597 | 319984565 | 1.825166 |
| tiny-smooth | 18428 | 18083 | 20475 | 1.019096 |

Counts use perf stat -e instructions:u, CPU 20, fixed iteration mode.
They include one process parse/setup/output and warm call amortized across
20 large/real, 300 medium or 10,000 tiny measured calls; do not interpret
them as a wall-time campaign or pure kernel-only counts. Raw perf files retained.

Candidate plan: direct correctly rounded scalar std square roots, preserving
the existing cached pinned backend in no_std; batch only the three fixed
counting-sort scans, retaining Work::scan failure prefixes. No public API change.

## Root-only probe and remaining cause

Root-only passes 1,544,416 exact observations (std) against v0.2.6.
Instruction ratios candidate/baseline: tiny .9293; medium .8495/.8683/.8751;
million .7806; real 214e .9022, a082 .8531, b6fd .8532. The isolated change
supports the root-path diagnosis, but candidate/X instructions remain
1.05–1.08 on medium/large synthetic and 1.51–1.56 on real geometry.
No wall samples have been taken.

The residual geometry loop dispatches Positions storage, checks scan capacity,
and creates triangle-scan closures separately for each soft cluster. Retain
one layout-specialized scan across all triangles when the whole geometry fuel
bound fits; defer only the known cluster-count charges and apply them even
on numerical failure. The tight-fuel path keeps its original visit order.
Also batch fixed counting-sort scans. Both changes preserve exact FP order,
per-call work, errors and partial destination behavior. This stays within the
same overdraw kernel and adds no allocation or API.

The all-layout geometry batch is rejected for packed storage: it adds an
extra cluster-end branch to each triangle, increasing medium instructions
about 14–15% versus root-only and million about 15%. It reduces real byte-view
instructions another 6–9%. Select geometry batching by storage layout:
strided/mapped views dispatch once when fuel fits; packed views retain the
original per-cluster loop. No case-name/size/threshold tuning. Keep the fixed
sort scan batching for both. The final campaign remains unstarted.


## Final implementation and proof

- `src/overdraw.rs:303`: scalar correctly rounded std roots; the original
  cached pinned libm backend remains in no_std.
- `src/overdraw.rs:148`: factor the original triangle/cluster arithmetic
  without changing expressions or evaluation order.
- `src/overdraw.rs:312`: one layout-specialized geometry scan for strided
  storage when the complete release fits; charge started clusters even on
  numerical error. Packed storage and tight fuel retain per-cluster scans.
- `src/overdraw.rs:361`: batch the three fixed sort scans with Work::scan,
  preserving exact error/fuel prefixes. No API, unsafe, encoder, SIMD or
  other-family implementation changes.

`instructions.json` retains the final instruction table; final/baseline
ratios are .903616 tiny, .791903/.808750/.818698 medium, .730017 million,
.844098 real 214e, .773402 a082 and .773426 b6fd. These are diagnostic
instruction counts, including amortized harness setup, rather than wall bars.

The final library hash manifest is `final-proof-source.json`, checked before
and after gates in `final-proof-identity.json`. Full source is retained in
`final-source/`; all eight final gate exits are in `GATES.tsv`. Earlier probe
logs and their mixed source epochs are retained under `probe-gate-logs/` and
are not substituted for final gates.

Exact proof in each of std and no_std: 1,544,416 observations comparing full
output vectors, Result values, partial destinations, untouched tails, work and
byte accounting. Coverage includes all eight frozen inputs, all four API
forms (including a too-short destination), four layouts, 4,096 mutations of
the existing seed 20261002 generator, and every work prefix 0..2199 on finite,
degenerate, subnormal, overflowing and nonfinite small geometry.
Frozen full-stride replay outputs are byte-exact against v0.2.6 and M7 on
8/8 cases; X is also byte-exact on those cases. The existing pinned C++
differential passes 178 overdraw cases. Every raw input/response is retained.

No long fuzz or 10k sweep was run; those remain nightly per this spec.
Results apply to this native x86-64 scalar campaign, without a WASM timing claim.

## Registered wall verdicts — final, no reruns

Exactly one completed campaign: 27 comparisons / 270 interleaved pairs,
1 warm-up batch + 10 pairs each, CPU 20, fixed iterations, a downstream engine scalar.
Bootstrap 10,000, seed 20261029. F = 0.164794109, giving case bar 1.264794109.
The largest A/A uncertainty is retained; both families also pass the case
bar with F=0, so the floor does not determine their acceptance.

| Family | ours/X GM | ours/v0.2.6 GM | Worst case upper95 vs v0.2.6 | Verdict |
|---|---:|---:|---:|---|
| overdraw | 1.116192 | 0.851051 | 0.941475 | PARTIAL |
| real-overdraw | 1.224010 | 0.834240 | 0.930378 | PARTIAL |

Both families miss the X bar and pass both v0.2.6 bars: PARTIAL verbatim.
Ship only as a speedup; no X performance-parity claim. No tuning or wall
reruns followed the verdict. Exact byte/error gates remain separately passed.
The residual X gap remains unclosed; instruction reductions alone are not
performance qualification. No other family was expanded into.

| Case | ours/X GM (95% CI) | ours/v0.2.6 GM (95% CI) | ours/M7 GM (95% CI), disclosure |
|---|---:|---:|---:|
| tiny-smooth | 1.055982 [1.051636, 1.059992] | 0.934649 [0.927211, 0.941475] | — |
| medium-smooth | 1.098843 [1.034228, 1.156975] | 0.902414 [0.889539, 0.914188] | — |
| medium-seams | 1.161872 [1.149208, 1.175274] | 0.850985 [0.845341, 0.857357] | — |
| medium-sparse | 1.165151 [1.148523, 1.182977] | 0.899941 [0.891466, 0.909809] | — |
| million-smooth | 1.102963 [0.965891, 1.278007] | 0.691174 [0.684379, 0.698941] | — |
| real-214e-overdraw | 1.282340 [1.273862, 1.291766] | 0.793256 [0.776912, 0.809037] | 1.240556 [1.217540, 1.264898] |
| real-b6fd-overdraw | 1.291443 [1.267306, 1.314242] | 0.926289 [0.921838, 0.930378] | 1.421515 [1.411688, 1.431872] |
| real-a082-overdraw | 1.107328 [1.041684, 1.168155] | 0.790157 [0.749207, 0.835233] | 1.158650 [1.066702, 1.292196] |

Real-family ours/M7 GM = 1.268937 [1.232867, 1.317176]; disclosure only.

`results.json` retains compact per-case/AA results, binary/source identity
and a hash of the full raw campaign. registered-audit.json recomputes every
case interval and family decision from all raw pairs, checks all 27 rows,
and verifies source/binary identity. No completed pair was excluded.

## Final gate exits

| Gate | Exit |
|---|---:|
| prepare-proof | 0 |
| exact-std | 0 |
| exact-no-std | 0 |
| frozen-exact | 0 |
| cpp-exact | 0 |
| test | 0 |
| clippy | 0 |
| fmt | 0 |
| wall-collection | 0 |
| registered-analysis-audit | 0 |

All-feature cargo tests: 212 passed, including the root/backend numerical
check. Clippy covers all targets and features with -D warnings. fmt passed.
All heavy jobs declared at most 4 GB with PARITY_RUNNER_GPU=0. Admission receipt
logs, including initial setup failures, are retained. The first successful
counter launch also exposed a CSV comment-header parsing error; its raw
count is retained, followed by the complete pre-edit baseline-r3 record.

## Retention and stop

The designated Cargo target was deleted after retaining both std/no_std
checkers and all four comparison arms; CLEANUP.json verifies removal.
Full evidence remains in job scratch. No merge, tag, publish or version bump.
Stop after the draft PR and resume entry; both registered PARTIAL verdicts stand.
