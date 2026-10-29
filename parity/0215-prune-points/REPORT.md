# S3: prune and point simplification

Baseline: v0.2.14 (`dbb75e03ec011ae04645e6d40b0039561991286d`). Peers:
meshopt 0.7.0 / C++ meshoptimizer 1.3, and optimesh 1.1.0. No upstream or
peer code was copied. The optimesh implementation was read as MIT source data.

## Attribution and retained work

The interrupted implementation targeted repeated work-limit checks in position
welding, union-find traversal, component error passes and point-grid hashing,
plus branches in packed position validation. The contract-cost experiment
provides the general W/R/F method; it did not isolate these two functions, so
no attribution percentage from other functions is claimed here. Existing
development profiles and screens are retained separately from the registered
campaign and do not supply its medians.

Useful packed validation, numeric welding, grouped component records and
separate checked kernels were retained. Speculative full-pass accounting was
replaced with pre-proven batches before qualification: an insufficient budget
selects the original checked traversal before the first visit. The reference
hash order, equality rules, arithmetic and public write order are preserved.

Techniques and source locations:

- `src/simplify_extra.rs:5`, `:68`, `:119`: packed bounds in four-wide lanes
  with a finiteness sum and first-zero sign restoration, replaying reference
  validation for bad data or insufficient fuel (lanes added in the
  check-and-finish pass).
- `src/simplify_extra.rs:199`, `:212`, `:360`, `:417`: prepay fixed one-visit
  scans only when they fit; group four-float component records without changing
  arithmetic order.
- `src/simplify_extra.rs:228`, `:266`, `:288`, `:321`: bound union traversal
  batches by six monotone-parent walks per triangle, and one per root record.
- `src/simplify_extra.rs:494`, `:548`, `:577`, `:613`: pre-proven weld probe
  batches, packed source specialization and a checked fallback.
- `src/simplify_extra.rs:1178`, `:1194`, `:1300`: pre-proven point-cell probe
  batches; retain the old per-lookup accounting in the checked tail.
- `src/simplify_extra.rs:2128`, `:2182`: uncolored point reservoir and error
  passes prepay their fitting scans inline instead of calling a per-vertex
  closure, and drop the error pass's finite exit by a proven bound
  (check-and-finish pass).
- Shared file changes are exactly `Work::covers_wide` at
  `src/workspace.rs:643` and `Work::settle` at `:651`. There are no changes in
  processing.rs or math.rs. Existing shared component helpers also affect
  simplify/update with PRUNE; bounds and cells also affect sloppy. These
  callers are included in the exact comparison, without performance claims.

No root implementation changed. The only arithmetic finite check removed is
the uncolored point error exit, which a bound proves unreachable; color and
colored reservoir checks remain necessary for arbitrary finite weights and
colors. See [PROOFS.md](PROOFS.md) for each bound, first-failure ordering,
overflow handling and storage/partial-write argument.

## Check-and-finish pass

A second agent checked commit `cef747dc` against the spec and attacked each
batch's safety argument: whether a batch could charge different work, return
a different error or report different `Usage` than v0.2.14. It found no
counterexample. Every batch is selected from a complete bound that fits the
remaining fuel before its first visit, and `Work::batch` caps the bound at
`usize::MAX`. Union-find parents stay strictly smaller than their children,
since unions link the larger root and compression copies a smaller parent.
Topology rejects more than `u32::MAX` vertices, so the weld sentinel cannot
name a vertex. A failed general-view read settles its exact prefix. A
point-cell lookup that runs out of probes charges its whole table before
`NumericalFailure`, as the covered reference path does. Campaign-1 counts
reproduced exactly on one sample per real mesh, function and arm.

After this check the owner asked for further optimization under the same
contract. Profiles of the real meshes then attributed about 215 million
instructions per call to the sequential bounds scan in both functions,
against about 207 million for the C++ rescale (bounds and scaling together).
Two out-of-line per-vertex closures in the uncolored point reservoir and
error passes cost about 570 million instructions per call. The two new
techniques are listed above. They replace the pre-registered rule of no edits
after a campaign; campaign 2 below is the second and last campaign the spec
allows.

## Registered instruction campaign

Two campaigns, each on a candidate frozen before qualification: campaign 1 on
`cef747dc` (Codex), campaign 2 on the check-and-finish candidate. The
committed `results.json`, `counts.jsonl`, `binding.json`, `proof-results.json`
and `receipts.json` are campaign 2's and bind to the committed sources; the
campaign-1 tables below are kept for comparison. Median of
three interleaved repetitions, `perf stat -e instructions:u`, perf 6.17,
`LC_ALL=C`, CPU affinity 9 and 100% counter running in every accepted sample.
All builds and measurements used the required CPU admission with GPU disabled,
4 GB declared and one compiler job for the measured drivers. The admission
wrapper's process scope cap was 6 GB; the declaration was 4 GB.

The five frozen corpus meshes and three retained real meshes match the peer
input hashes. The existing resident-loop consumer counts allocation, required
copies and destruction after warmup, excluding parsing and serialization.
Prune uses cutoff 0.5; points uses half the vertices without colors. No clock
was used around the measured loop. No wall-time performance conclusion follows.

Each campaign exited 0 with 192 accepted samples (16 function/input cells,
four arms, three repetitions). Every repetition and every arm has identical
output bytes within each cell, and campaign 2's outputs equal campaign 1's.
No cell regressed above 1.01× base in either campaign, and no source revert
was needed. There were no edits after campaign 2 began.

The gate is applied verbatim: SHIPS if three-real-mesh geometric mean
candidate/optimesh ≤ 0.99 and candidate/base ≤ 0.97; IMPROVES if only
candidate/base ≤ 0.97; NO-GAIN otherwise. Any case candidate/base > 1.01
requires a fix or revert.

| Function | Campaign | Real GM candidate/base | Real GM candidate/optimesh | Real GM candidate/C++ | Verdict |
|---|---|---:|---:|---:|---|
| simplify_prune | 2 (final) | 0.620134 | 0.763203 | 0.995169 | **SHIPS** |
| simplify_prune | 1 | 0.687707 | 0.846367 | 1.103609 | SHIPS |
| simplify_points | 2 (final) | 0.605547 | 0.636813 | 1.022023 | **SHIPS** |
| simplify_points | 1 | 0.811646 | 0.853553 | 1.369870 | SHIPS |

Campaign 2 per-input ratios of instruction medians (below one means fewer
instructions):

| Input | Function | Candidate/base | Candidate/optimesh | Candidate/C++ |
|---|---|---:|---:|---:|
| tiny-smooth | prune | 0.779630 | 0.908326 | 1.199992 |
| tiny-smooth | points | 0.778029 | 0.730716 | 1.095217 |
| medium-smooth | prune | 0.709593 | 0.854931 | 1.048778 |
| medium-smooth | points | 0.637328 | 0.650047 | 1.033544 |
| medium-seams | prune | 0.697359 | 0.839862 | 1.043117 |
| medium-seams | points | 0.818406 | 0.685005 | 1.219801 |
| medium-sparse | prune | 0.664181 | 0.804168 | 1.023640 |
| medium-sparse | points | 0.582575 | 0.635680 | 0.971694 |
| million-smooth | prune | 0.710145 | 0.856479 | 1.047412 |
| million-smooth | points | 0.677122 | 0.660483 | 1.079547 |
| real-214e | prune | 0.617931 | 0.760192 | 0.994248 |
| real-214e | points | 0.575711 | 0.628368 | 0.985445 |
| real-a082 | prune | 0.621455 | 0.765147 | 0.995761 |
| real-a082 | points | 0.575477 | 0.627948 | 0.985924 |
| real-b6fd | prune | 0.621021 | 0.764280 | 0.995498 |
| real-b6fd | points | 0.670212 | 0.654483 | 1.098770 |

The maximum campaign-2 candidate/base ratio is 0.779630 for prune and
0.818406 for points.

Campaign 1 per-input ratios (Codex, `cef747dc`):

| Input | Function | Candidate/base | Candidate/optimesh | Candidate/C++ |
|---|---|---:|---:|---:|
| tiny-smooth | prune | 0.794622 | 0.925789 | 1.223062 |
| tiny-smooth | points | 0.884111 | 0.830352 | 1.244540 |
| medium-smooth | prune | 0.747209 | 0.900243 | 1.104363 |
| medium-smooth | points | 0.822776 | 0.839188 | 1.334229 |
| medium-seams | prune | 0.739170 | 0.890208 | 1.105633 |
| medium-seams | points | 0.893155 | 0.747571 | 1.331191 |
| medium-sparse | prune | 0.716833 | 0.867914 | 1.104776 |
| medium-sparse | points | 0.801350 | 0.874392 | 1.336570 |
| million-smooth | prune | 0.747521 | 0.901557 | 1.102539 |
| million-smooth | points | 0.838189 | 0.817592 | 1.336338 |
| real-214e | prune | 0.686254 | 0.844245 | 1.104180 |
| real-214e | points | 0.799757 | 0.872906 | 1.368946 |
| real-a082 | prune | 0.688582 | 0.847796 | 1.103320 |
| real-a082 | points | 0.799706 | 0.872622 | 1.370080 |
| real-b6fd | prune | 0.688288 | 0.847064 | 1.103327 |
| real-b6fd | points | 0.836010 | 0.816390 | 1.370586 |

`results.json` retains all four raw instruction medians per cell;
`counts.jsonl` retains every unrounded sample and output identity. The maximum
campaign-1 candidate/base ratio is 0.794622 for prune and 0.893155 for points;
its compact receipts remain in `cef747dc`.

## Contract proof and checks

| Command / proof | Result |
|---|---|
| `cargo test --offline --locked --all-features` | exit 0; 230 passed, 0 failed, 6 ignored across 10 suites |
| `cargo test --offline --locked --no-default-features` | exit 0; 184 passed, 0 failed, 6 ignored across 10 suites |
| `cargo clippy --offline --locked --all-targets --all-features -- -D warnings` | exit 0 |
| `cargo fmt --check` | exit 0 |
| `python3 parity/simd/check_boundary.py` | exit 0 |
| `cargo package --allow-dirty --list` piped to `python3 parity/simd/check_package.py` | exit 0 |
| `bash parity/tools/check_public_text.sh` | exit 0 |
| scratch `proof/exact-all` | exit 0; 15,517,656 observations; zero differences |
| scratch `proof/exact-none` | exit 0; 15,517,656 observations; zero differences |
| `bash parity/0216-qualify/run-protocol-modes.sh` | exit 0; 1,246 C++ comparisons per mode, including 356 prune/points comparisons per mode |

`MESHOPT_SCRATCH=proof python3 parity/0215-contract-simplify/fuzz.py` exited
0 after 1,677 seeds and 300.233 child CPU seconds, with zero differences.
These are the campaign-2 gate results on the final sources. On `cef747dc`
they were 229/183 tests, 14,957,296 exact observations per mode, the same
C++ counts and 1,712 fuzz seeds over 300.100 CPU seconds, all exit 0.

The exact domain includes owned/full-buffer/empty-buffer APIs, destructive
update as a shared caller, packed/interleaved/byte views, both byte orders,
unaligned and short final records, colored and uncolored points, target and
cutoff variations, PRUNE on/off for shared callers, both retention modes,
storage failures, exceptional float patterns, colliding hashes, welded
duplicates, disconnected components, descending union chains and (added in
the check-and-finish pass) signed-zero extreme coordinates across the
four-vertex blocks of the bounds pass, with and without colors. Every small
case sweeps integer work limits zero through unrestricted usage plus one and
compares two consecutive calls on each workspace. It compares complete caller
buffers, errors, float bits, work and storage usage.

The widened fuzz generator varies point targets and colors and keeps half its
generated geometry valid, reaching the changed kernels instead of mostly
zero-target or validation exits. Read-only adversarial review covered budget
boundaries, first failed charges, global batch offsets, sentinel validity,
overflow and shared consumers; no counterexample was found. This review was
performed by the implementing agent, not an independent external reviewer.
The check-and-finish agent repeated that attack on `cef747dc` (see above) but
then wrote the two later techniques itself, so they have no review by a
second agent.

Retained non-passing attempts: the preliminary format check failed before
formatting; a check supervisor exited 2 after its script was edited while Bash
was reading it (both test suites had completed); the first exact build failed
with compiler exit 101 / supervisor exit 1 because the baseline copy omitted
an included documentation file. The corrected check tail and exact harness
were run separately. These receipts remain retained and are not relabeled as
passes. No registered measurement came from the failed attempts.
The first final-audit admission request (1 GB declared, 6 GB scope cap) was
canceled while queued, exit 143, before executing any audit. Its replacement
used the normal 4 GB declaration and completed with exit 0.
In the check-and-finish pass, a development clippy run failed on the
intentional `v - v` (`clippy::eq_op`, now allowed locally with a reason), and
the first campaign-2 audit exited 1 because the exact logs had been written
beside the other step logs; after copying them to `proof/` it exited 0.

## Evidence and limits

`counts.jsonl`, `results.json`, `inputs.json`, `binding.json`,
`proof-results.json` and `receipts.json` contain portable counts, identities,
source/binary bindings and raw-log hashes. Raw logs, all binaries, source
copies, complete outputs, inputs, profiles and rejected attempts remain in
caller-provided scratch. The library was source-hash matched to the frozen
candidate after the campaign.

`python3 parity/0215-prune-points/results.py` revalidated all 192 raw counters,
all complete output files, input hashes, peer binaries, library source binding,
exact and C++ receipts and the 300-second fuzz total; exit 0. Final formatting,
diff whitespace and public-text checks also exited 0. After every lane build
and measurement job exited, the dedicated target was removed and verified
absent; `cleanup.json` records exit 0. Raw scratch evidence remains retained.
The same audit was rerun on campaign 2 and exited 0; the check-and-finish
pass removed its own dedicated target in the same way.

This lane establishes the tested host's instruction counts and finite exact
domain plus the written bounds. Cross-platform execution, authored colors on
the real meshes, strided-layout performance, GPU or whole-chain throughput,
wall-time speedups and independent external review were not verified here.
The branch remains a draft PR for coordinator integration, without merge,
tag or publication.
