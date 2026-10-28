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

- `src/simplify_extra.rs:5`: combine packed finite predicates and ordered bounds
  selects, replaying reference validation for bad data or insufficient fuel.
- `src/simplify_extra.rs:131`, `:144`, `:292`, `:349`: prepay fixed one-visit
  scans only when they fit; group four-float component records without changing
  arithmetic order.
- `src/simplify_extra.rs:160`, `:198`, `:220`, `:253`: bound union traversal
  batches by six monotone-parent walks per triangle, and one per root record.
- `src/simplify_extra.rs:426`, `:480`, `:509`, `:545`: pre-proven weld probe
  batches, packed source specialization and a checked fallback.
- `src/simplify_extra.rs:1110`, `:1126`, `:1232`: pre-proven point-cell probe
  batches; retain the old per-lookup accounting in the checked tail.
- Shared file changes are exactly `Work::covers_wide` at
  `src/workspace.rs:643` and `Work::settle` at `:651`. There are no changes in
  processing.rs or math.rs. Existing shared component helpers also affect
  simplify/update with PRUNE; bounds and cells also affect sloppy. These
  callers are included in the exact comparison, without performance claims.

No root implementation or arithmetic finite check was removed. Native roots
already existed in the baseline; color/reservoir checks remain necessary for
arbitrary finite weights and colors. See [PROOFS.md](PROOFS.md) for each bound,
first-failure ordering, overflow handling and storage/partial-write argument.

## Registered instruction campaign

One candidate, frozen before qualification; campaign 1 of at most 2. Median of
three interleaved repetitions, `perf stat -e instructions:u`, perf 6.17,
`LC_ALL=C`, CPU affinity 9 and 100% counter running in every accepted sample.
All builds and measurements used the required CPU admission with GPU disabled,
4 GB declared and one compiler job. The admission wrapper's process scope cap
was 6 GB; the declaration was 4 GB.

The five frozen corpus meshes and three retained real meshes match the peer
input hashes. The existing resident-loop consumer counts allocation, required
copies and destruction after warmup, excluding parsing and serialization.
Prune uses cutoff 0.5; points uses half the vertices without colors. No clock
was used around the measured loop. No wall-time performance conclusion follows.

Campaign exited 0 with 192 accepted samples (16 function/input cells, four
arms, three repetitions). Every repetition and every arm has identical output
bytes within each cell. No cell regressed above 1.01× base. No second campaign
or source revert was needed, and there were no optimization edits after the
campaign began.

The gate is applied verbatim: SHIPS if three-real-mesh geometric mean
candidate/optimesh ≤ 0.99 and candidate/base ≤ 0.97; IMPROVES if only
candidate/base ≤ 0.97; NO-GAIN otherwise. Any case candidate/base > 1.01
requires a fix or revert.

| Function | Real GM candidate/base | Real GM candidate/optimesh | Real GM candidate/C++ | Verdict |
|---|---:|---:|---:|---|
| simplify_prune | 0.687707 | 0.846367 | 1.103609 | **SHIPS** |
| simplify_points | 0.811646 | 0.853553 | 1.369870 | **SHIPS** |

Per-input ratios of instruction medians (below one means fewer instructions):

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
candidate/base ratio is 0.794622 for prune and 0.893155 for points.

## Contract proof and checks

| Command / proof | Result |
|---|---|
| `cargo test --offline --locked --all-features` | exit 0; 229 passed, 0 failed, 6 ignored across 10 suites |
| `cargo test --offline --locked --no-default-features` | exit 0; 183 passed, 0 failed, 6 ignored across 10 suites |
| `cargo clippy --offline --locked --all-targets --all-features -- -D warnings` | exit 0 |
| `cargo fmt --check` | exit 0 |
| `python3 parity/simd/check_boundary.py` | exit 0 |
| `cargo package --allow-dirty --list` piped to `python3 parity/simd/check_package.py` | exit 0 |
| `bash parity/tools/check_public_text.sh` | exit 0 |
| scratch `proof/exact-all` | exit 0; 14,957,296 observations; zero differences |
| scratch `proof/exact-none` | exit 0; 14,957,296 observations; zero differences |
| `bash parity/0216-qualify/run-protocol-modes.sh` | exit 0; 1,246 C++ comparisons per mode, including 356 prune/points comparisons per mode |

`MESHOPT_SCRATCH=proof python3 parity/0215-contract-simplify/fuzz.py` exited
0 after 1,712 seeds and 300.100 child CPU seconds, with zero differences.

The exact domain includes owned/full-buffer/empty-buffer APIs, destructive
update as a shared caller, packed/interleaved/byte views, both byte orders,
unaligned and short final records, colored and uncolored points, target and
cutoff variations, PRUNE on/off for shared callers, both retention modes,
storage failures, exceptional float patterns, colliding hashes, welded
duplicates, disconnected components and descending union chains. Every small
case sweeps integer work limits zero through unrestricted usage plus one and
compares two consecutive calls on each workspace. It compares complete caller
buffers, errors, float bits, work and storage usage.

The widened fuzz generator varies point targets and colors and keeps half its
generated geometry valid, reaching the changed kernels instead of mostly
zero-target or validation exits. Read-only adversarial review covered budget
boundaries, first failed charges, global batch offsets, sentinel validity,
overflow and shared consumers; no counterexample was found. This review was
performed by the implementing agent, not an independent external reviewer.

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

This lane establishes the tested host's instruction counts and finite exact
domain plus the written bounds. Cross-platform execution, authored colors on
the real meshes, strided-layout performance, GPU or whole-chain throughput,
wall-time speedups and independent external review were not verified here.
The branch remains a draft PR for coordinator integration, without merge,
tag or publication.
