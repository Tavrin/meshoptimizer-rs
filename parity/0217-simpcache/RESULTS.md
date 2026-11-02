# 0217-P1P2: vertex cache and simplify against C++ (third attempt)

Baseline: train/0.2.17 head `3dccc1a5` (cache source identical to v0.2.16).
Reference: C++ meshoptimizer 1.3 (meshopt 0.7.0 vendored, pinned upstream
`4c203430`), built with `-DNDEBUG -fno-fast-math -ffp-contract=off`, scalar arm
with `-DMESHOPTIMIZER_NO_SIMD`. Consumer: `parity/peers`, warmed workspace,
owned output, `instructions:u` around resident calls only (`measure.py`).
Counts are exact and repeatable to a few hundred instructions per call.
Every measured cell asserts byte-identical output across all arms.

## Method: why the two earlier rounds found nothing

Both earlier rounds attributed instructions with `perf record -e instructions:u`.
That sampling skids onto long-latency loads: on real-a082 one cache-missing
load in the removal loop (`mov 0x4(%r8,%rax,4)`) received 8.4G of 17.6G sampled
loop instructions. The phase tables built from it pointed at removal and
rescoring bookkeeping, and the micro changes made there could not move the
total. This round used AMD IBS op sampling (`ibs_op/cnt_ctl=1,swfilt=1/`, period
100003), which tags the retired op itself, and read the hot loops side by side
with the C++ disassembly. Exact whole-call counts decide every result.

## optimize_vertex_cache

### Root cause

The real meshes are close to triangle soups (vertices about 0.8 x indices, 2.2
vertices per triangle). After each emitted triangle the cache holds 16-19
entries and almost all of them belong to vertices with no live triangles.
Upstream and the baseline both walk every entry: load the vertex, test its live
count, skip. Per entry the Rust loop costs 11 instructions against 10 in C++,
and the per-live-entry and removal paths carry bounds checks and a metering
counter kept in memory (stored through `&mut CacheWork` because a bounds panic
could observe it). The inner neighbour-update and removal search loops are
already at C++ length (13 vs 14 and 6 vs 6 instructions per element). The gap
is the sum of per-entry overheads on a loop that is mostly dead entries, plus
setup bounds checks (count, fill and triangle-score loops: about 25
instructions per triangle).

### Change (kept)

The covered path (budget proven to cover the whole kernel) now runs a dedicated
`cache_loop_covered`:

1. **Live bit per cache position.** A vertex's live count only falls when one of
   its triangles is emitted, and the emitted triangle's vertices always land at
   positions 0..3. So a 32-bit mask is exact: three fresh bits read after the
   removals, the rest carried with each kept entry through the merge. Rescoring
   visits set bits in position order (`trailing_zeros`) and never loads an
   exhausted vertex. Same entries, same order, same float operations, same
   strict `best_score < s` selection as upstream.
2. **Mask merge.** Old entries equal to a triangle vertex form a 16-bit mask
   from fixed-width comparisons (the compiler emits `pcmpeqd`/`pmovmskb` with
   baseline SSE2, no `unsafe`, no feature). Without a match, all 16 slots copy
   at once; otherwise the copy skips matched slots. Live bits are compacted per
   matched slot. Kept entries end at exactly upstream's positions.
3. **Visit count in a local.** The loop returns the exact visit total the
   limited path would charge one by one; the kernel prepays the proven bound and
   refunds the difference, as before. `Usage::work` is unchanged.

The limited path (`cache_loop_limited`) keeps every charge before its
observable mutation and is the previous `FAST = false` code with the
`CacheWork` wrapper removed (calls `Work::add/scan/search` directly, same
order). `CacheWork` is deleted.

### Attempts

| Attempt | What | Real GM vs scalar C++ | medium-smooth vs C++ | Kept |
|---|---|---:|---:|---|
| baseline | train head | 1.2416 | 1.226 | - |
| c1 | live-bit rescoring, merge carries bits per entry with variable shifts | 1.0447 | 1.187 | superseded |
| c2 | c1 + vector match mask + shift-close per matched slot | not measured (codegen inspected: closing pass scalarised into a jump table) | - | no |
| c3 | vector match mask, bulk copy without match, skip-copy otherwise | 0.7023 | 1.041 | yes |

### Final: instructions per call, ratio to C++ (same arm)

| Case | Base scalar | Final scalar | Base default | Final default |
|---|---:|---:|---:|---:|
| tiny-smooth | 1.223 | 1.048 | 1.223 | 1.048 |
| medium-smooth | 1.226 | 1.041 | 1.226 | 1.041 |
| medium-seams | 1.237 | 1.009 | 1.237 | 1.009 |
| medium-sparse | 1.223 | 1.040 | 1.223 | 1.040 |
| million-smooth | 1.225 | 1.045 | 1.225 | 1.045 |
| real-214e | 1.240 | 0.686 | 1.240 | 0.686 |
| real-a082 | 1.243 | 0.711 | 1.243 | 0.711 |
| real-b6fd | 1.243 | 0.711 | 1.243 | 0.711 |
| **real GM** | **1.2416** | **0.7023** | **1.2416** | **0.7023** |
| **corpus GM** | **1.2267** | **1.0366** | **1.2267** | **1.0366** |

Both targets (<= 1.05) are met in both arms. Final against base: real GM
0.566, corpus GM 0.845. The cache kernel is scalar in every arm, so scalar and
default agree. Smooth corpus meshes keep a 1-5% gap: there most cache entries
are live, so the mask saves little, and the setup bounds checks remain. The
table and strip APIs share the kernel; their outputs and charges are covered
by the exact driver, not by this table.

## simplify

### Measured cost of the crate rules

| Arm | Real GM vs scalar C++ |
|---|---:|
| baseline (train head) | 1.1941 |
| metering removed everywhere (experiment only, not shippable) | 1.1320 |

Removing every budget charge, check and tally (`Work::add/scan/precharge` and the
`Prepaid` meter made no-ops in a scratch copy) lowers the gap by 0.062 of a C++
call: **budget metering costs about 6.2 points**. Even without it simplify stays
at 1.132x, so metering alone cannot reach 1.05.

IBS attribution on real-b6fd (one resident call, ops not instructions):

| | Rust ops | C++ ops |
|---|---:|---:|
| user total | 14,450 samples | 12,621 samples |
| bounds-check compare and branch (to a panic block) | 939 samples (6.6%) | none |

Bounds checks cost an estimated 6-7% of Rust ops (more in instructions, since
each fused compare-and-branch is two instructions). They are what remains of
the safe-indexing rule; removing them needs `unsafe` outside the SIMD modules,
which the crate forbids. Metering plus bounds checks (about 6.2 + 6.6 points)
account for most of the 19-point gap. The remainder is spread over phases with
no single dominant loop (pick 2,818 vs 1,962 C++ samples; quadrics, position
remap and update within a few hundred samples each).

### Attempts

| Attempt | What | Real GM vs scalar C++ | Kept |
|---|---|---:|---|
| c4 | pick: read remap/kind once per corner, unrolled edge triple | 1.2090 (worse than 1.1941) | no, reverted |
| c4 | quadrics edge loop: same restructuring (built after c4's binary; not measured separately) | - | no, reverted |
| e1 | metering removed (rule-cost experiment) | 1.1320 | experiment only |

Decision: stop at the rule boundary as the spec directs. Simplify source is
unchanged from the train head.

### Final (unchanged source)

| Arm | Real GM vs C++ | Corpus GM vs C++ |
|---|---:|---:|
| scalar | 1.1941 | 1.0796 |
| default | not re-measured (no SIMD path in simplify; 0.2.16 campaign: equal to scalar to 6 digits) | 1.0796 |

Targets (<= 1.05) are not met. Rule costs: budget metering 6.2 points
(exact, measured); safe indexing an estimated 6-7% of ops (IBS, not exact).

## Gates

All on the final source (`src/cache.rs`, `CHANGELOG.md`; simplify unchanged):

| Gate | Result |
|---|---|
| exact work-prefix driver (0217-cache `exact.rs`: every work limit, cold/reused/after-failure, all cache APIs incl. table/strip, `Usage` work and bytes) vs train head, all features | 18,868,896 observations, 0 differences, exit 0 |
| same, no default features | 18,868,896 observations, 0 differences, exit 0 |
| C++ differential (`parity/0215-toward-cpp/cpp-differential.rs`), all / no features | 1,932 cases each, 0 differences, exit 0 |
| oracle pin (`parity/check-reference.sh`) | exit 0 |
| `cargo fmt --check` | exit 0 |
| `cargo clippy --all-targets` all / no features, `-D warnings` | exit 0 / 0 |
| `cargo test` default / all / no features (doctests included) | exit 0 / 0 / 0 |
| `cargo doc` all / no features, `RUSTDOCFLAGS=-D warnings` | exit 0 / 0 |
| `cargo build --target wasm32-unknown-unknown --no-default-features` | exit 0 |
| `parity/check-unsafe-boundary.sh` | exit 0 (no `unsafe` added) |
| `parity/tools/check_public_text.sh`, `git diff --check` | exit 0 |
| measurement output identity: every arm, every case, both modes | equal |
| cargo-semver-checks vs 0.2.16 | not run locally (tool not installed); only private items changed |
