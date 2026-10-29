# Simplify and sloppy: instruction qualification

Baseline v0.2.14; peers are optimesh 1.1.0 and C++ meshoptimizer 1.3.
The registered campaign used `instructions:u`, three samples per arm, and
the frozen corpus and three real meshes. Ratios are candidate divided by
the named arm; the real-mesh summary is a geometric mean.

| Function | Base | optimesh | C++ | Instruction verdict |
|---|---:|---:|---:|---|
| simplify | 1.0000 | 0.9767 | 1.2364 | NO-GAIN |
| simplify_sloppy | 0.8446 | 0.7063 | 1.2754 | SHIPS |

| Case | simplify base / optimesh / C++ | sloppy base / optimesh / C++ |
|---|---|---|
| tiny-smooth | 1.0000 / 0.9570 / 0.9265 | 0.9224 / 0.9036 / 1.2573 |
| medium-smooth | 1.0000 / 0.9377 / 1.1096 | 0.9224 / 0.9185 / 1.2509 |
| medium-seams | 1.0000 / 0.9246 / 1.0903 | 0.9459 / 0.7982 / 1.3827 |
| medium-sparse | 1.0000 / 0.9477 / 1.1265 | 0.8778 / 0.8334 / 1.2433 |
| million-smooth | 1.0000 / 0.9067 / 1.0787 | 0.9128 / 0.9605 / 1.2093 |
| real-214e | 1.0000 / 1.0172 / 1.2600 | 0.8461 / 0.6899 / 1.2820 |
| real-a082 | 1.0000 / 0.9571 / 1.2230 | 0.8361 / 0.7286 / 1.2572 |
| real-b6fd | 1.0000 / 0.9570 / 1.2266 | 0.8517 / 0.7011 / 1.2873 |

No case exceeded 1.01 against base. All 192 sample outputs matched across
arms and repetitions. Raw counts and compact source bindings accompany this
report; instruction verdicts are distinct from contract qualification.

The real-b6fd baseline profile attributed 27.0% of simplify samples to
quadrics and 27.4% to its enclosing state runner; sloppy attributed 45.6%
to its kernel closure, 15.3% to vertex IDs and 8.4% to triangle counting.
These are sampled symbols, not per-change causal attribution. The earlier
contract-cost experiment attributed 44.2% of the pooled simplify gap to
finite checks conditional on W/R removal, and 40.4% of the sloppy gap to
work accounting; its interventions removed checks without preserving the
contract and are not a qualification of this candidate.

The changes are the packed bounds scan and covered cell-probe batches in
`src/simplify_extra.rs`; their bounds and checked fallbacks are in PROOFS.md.
These helpers also serve point simplification. No workspace.rs, processing.rs
or math.rs helper changed. The sample does not isolate the individual gains.

## Proof status

The all-features and no-default tests, all-target clippy, formatting,
unsafe-boundary and public-text checks each exited 0. The pinned C++
differential exited 0 for 1,246 cases in each feature mode. A first candidate
build exited 1 because its scratch copy omitted an included documentation file;
after repairing the scratch copy the build exited 0. The failed log is retained.
The exact baseline comparison exited 0 in each mode: 1,200,243,710
observations with all features and 273,906,930 without default features.
It compares complete outputs, results and usage, including every work limit
through success plus one for its finite small-mesh cases and the larger
batch-bound cases. The independent fuzz run exited 0 after 1,522 seeds and
300.034 child CPU seconds. These finite inputs do not exhaust all possible
meshes or platforms.

## Check lane: review and second campaign

The review found no observable change in the two `simplify_extra.rs`
routines; PROOFS.md records why. They are byte-identical to the packed
bounds and cell-table routines on the prune/points branch, so the two
branches merge with a conflict only in CHANGELOG.md. That branch also adds a
unit test of the zero-sign restore.

Attribution used unthrottled `instructions:u` samples (period 500009) on
real-b6fd, mapped to inlined source functions, with candidate and C++
1.3 phases side by side, in samples:

| Phase | Candidate | C++ |
|---|---:|---:|
| Quadric construction | 640 | 603 |
| Collapse picking | 374 | 353 |
| Edge adjacency, initial and per pass | 485 | 406 |
| Position welding | 381 | 343 |
| Vertex classification | 280 | 234 |
| Position normalization | 144 | 124 |
| Quadric and edge-loop update | 216 | 135 |
| Input validation | 72 | 0 |

Quadric construction is the largest phase but runs close to C++: face,
vertex and edge loops have the same arithmetic, and the difference is
indexing checks and one byte-slice search. The gap is spread over the
phases below it. The cheapest fixes that keep every observable were the
per-vertex readers and checks in validation and normalization, which run
on every vertex of the unwelded real meshes.

Changes, all in `src/simplify.rs` (bounds in PROOFS.md):

- `:30`, `:569`: packed finite validation by lane sums of `x - x`, charged
  once when the scan fits; otherwise the counted scan.
- `:2503`: packed normalization without attributes under a finite scale,
  precharged when its visits fit; otherwise the per-vertex loop.
- `:2673`: one charge for a fitting compaction pass.
- `:1847`: a pattern test in place of a byte-slice `contains`.

Not tried: the remaining adjacency, classification and update gaps are
mostly bounds checks on data-dependent indices; removing them needs
`unsafe`, which the spec excludes.

Campaign 2 used the registered harness and verdict rule (three samples per
arm, the same inputs and peer drivers; receipts in `campaign-2/`):

| Function | Base | optimesh | C++ | C++ gain | Verdict |
|---|---:|---:|---:|---:|---|
| simplify, campaign 1 | 1.0000 | 0.9767 | 1.2364 | 0.0 points | NO-GAIN |
| simplify, campaign 2 | 0.9192 | 0.8978 | 1.1365 | 10.0 points | SHIPS |
| simplify_sloppy, campaign 2 | 0.8446 | 0.7063 | 1.2754 | 23.5 points | SHIPS |

| Case | simplify base / optimesh / C++ |
|---|---|
| tiny-smooth | 0.9914 / 0.9488 / 0.9186 |
| medium-smooth | 0.9839 / 0.9226 / 1.0917 |
| medium-seams | 0.9851 / 0.9108 / 1.0741 |
| medium-sparse | 0.9682 / 0.9176 / 1.0907 |
| million-smooth | 0.9929 / 0.9002 / 1.0710 |
| real-214e | 0.8888 / 0.9041 / 1.1198 |
| real-a082 | 0.9349 / 0.8947 / 1.1433 |
| real-b6fd | 0.9348 / 0.8946 / 1.1466 |

No case exceeds 1.01 against base, and all outputs match across arms. The
sloppy ratios are unchanged from campaign 1. The changes were measured
together; the sample does not isolate each one.

Proofs for the final source, each exit 0: all-features and no-default tests
(412 passed in total), all-target clippy, formatting, unsafe-boundary and
public-text checks; the exact v0.2.14 comparison with 1,200,243,710
observations with all features and 273,906,930 without; the pinned C++
differential with 1,246 cases per mode; fuzzing with 1,966 seeds and 300.126
child CPU seconds. Receipts are in `campaign-2/proof-results.json`.
