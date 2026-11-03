# 0217-P1: simplify metering and bounds checks against C++

Baseline (b0): train/0.2.17 head `ccc09cef` (simplify source equal to v0.2.16
plus the 0217 cache change). Reference: C++ meshoptimizer 1.3 (meshopt 0.7.0,
scalar arm, same build as `parity/0217-simpcache`). Method unchanged from that
round: `parity/0217-simpcache/measure.py`, exact `instructions:u` around the
resident calls of the `parity/peers` consumer, byte-identical output asserted
in every cell. Attribution by IBS op sampling (`ibs_op/cnt_ctl=1,swfilt=1/`,
period 100003, four resident calls on real-b6fd). Exact whole-call counts decide.

## Where the metering cost was

With every charge removed (round 0217-simpcache experiment e1, 1.132x), the
largest IBS difference against the baseline was collapse picking: about a fifth
of `pick` ops. `pick` charged one visit per corner inside a loop with four early
`continue`s; with the prepaid meter that is a counter increment per corner
which LLVM kept in the loop. Phases that already charged through `precharge`
or a single `scan` (adjacency, quadrics, the index rewrite) showed no metering
cost. Bounds checks: 3,177 of 46,037 IBS ops (6.9%) are fused bounds-check
compare-and-branch pairs, about two instructions each. The largest sites are
gathers and scatters through data-dependent indices: adjacency counting and
filling (about 1,400 ops), collapse picking, the update loop passes and the
index rewrite. None of them has an index the compiler can prove in range by
restructuring; removing them needs `unsafe` indexing.

## Change (kept)

1. `pick`: a prepaid meter (budget for the whole pass already proven) counts
   entered triangles and charges three visits per triangle once after the
   loop. The checked path keeps one charge per corner before any read.
2. `update`, positional path: the prepaid meter charges `n` once after the
   vertex pass, or the visited prefix `i + 1` before returning
   `NumericalFailure`; the two loop-remap passes charge `2n` once. The checked
   path is unchanged.

A prepaid meter cannot fail and only its total reaches `Work`, so the charged
total, the error and its precedence are those of visit-by-visit charging. The
`metered!` bound check, which selects between the two paths, is unchanged.

## Attempts

Real-mesh ratios to scalar C++ (214e / a082 / b6fd, then geometric mean).

| Attempt | What | 214e | a082 | b6fd | Real GM | Kept |
|---|---|---:|---:|---:|---:|---|
| b0 | train head | 1.165 | 1.207 | 1.211 | 1.1941 | - |
| p1 | pick: charge per triangle once; hash table sliced to `..=mask` | 1.139 | 1.160 | 1.163 | 1.1541 | pick part |
| vu | p1 + update closed-form charges | 1.1393 | 1.1570 | 1.1602 | 1.1521 | yes |
| vc | p1 + classify: pass charges once, opposite search by `position` | 1.1570 | 1.1712 | 1.1744 | - | no (+1.5%) |
| vr | p1 + position remap: probe count from the loop index | 1.1491 | 1.1657 | 1.1690 | - | no (+0.6%) |
| p2 | vu + vc + vr + quadric edge loop over a constant tuple table | 1.169 | 1.177 | 1.181 | 1.1757 | no |
| vq | vu + quadric edge loop reads the three kinds once per triangle | 1.1443 | 1.1614 | 1.1647 | - | no (+0.4%) |
| vqp | vq + pick reads remap and kind once per triangle | 1.1463 | 1.1656 | 1.1689 | - | no |
| vb | vu + position remap prepaid in batches of 1024 instead of 64 | 1.1559 | 1.1670 | 1.1596 | - | no |
| vt | vu without the `..=mask` table slice | 1.1376 | 1.1561 | 1.1593 | 1.1510 | yes (final) |

Notes on the rejected attempts. Per-element counters that LLVM already folds
into the loop induction (classify searches, hash probes) cost nothing to keep,
and replacing them added instructions. Reading the three kinds or
representatives per triangle trades two range checks per edge for one per
corner, but most edges exit after the first kind (manifold) or the
representative test, so the eager loads cost more than the checks they
remove; the previous round's c4 failed for the same reason. The `..=mask`
slice removes the probe range check but measured 0.1% worse overall, within
the code-layout noise seen between equivalent builds (vb moved 214e by 1.5%
with a change outside its hot loops).

## Final: instructions per call, ratio to scalar C++

| Case | C++ | b0 | final | b0 ratio | final ratio |
|---|---:|---:|---:|---:|---:|
| tiny-smooth | 121,049 | 113,101 | 110,513 | 0.934 | 0.913 |
| medium-smooth | 11,065,466 | 12,488,685 | 12,135,188 | 1.129 | 1.097 |
| medium-seams | 14,329,336 | 15,929,248 | 15,400,914 | 1.112 | 1.075 |
| medium-sparse | 11,987,786 | 13,481,535 | 13,119,588 | 1.125 | 1.094 |
| million-smooth | 2,278,665,308 | 2,535,445,302 | 2,436,942,786 | 1.113 | 1.069 |
| real-214e | 2,853,783,794 | 3,323,584,557 | 3,246,608,828 | 1.165 | 1.138 |
| real-a082 | 5,092,141,421 | 6,148,153,206 | 5,886,956,144 | 1.207 | 1.156 |
| real-b6fd | 1,267,240,214 | 1,534,394,571 | 1,469,108,751 | 1.211 | 1.159 |
| **real GM** | | | | **1.1941** | **1.1510** |
| **corpus GM** | | | | **1.0796** | **1.0472** |

Target (real GM <= 1.08) not met; corpus improved. The default arm was not
re-measured: simplify has no SIMD path, and earlier rounds found default equal
to scalar to six digits.

## What remains, by rule

- Budget metering: with every charge removed the baseline measured 1.132x
  (round 0217-simpcache, e1); the final source is at 1.151x, so about 1.9
  points remain. IBS spreads them over the position remap batches and the
  quadric pass with no single site above the sampling noise.
- Safe indexing: about 7% of ops (roughly 12% of instructions) are bounds-check
  pairs on data-dependent indices. Removing them needs `unsafe` outside the SIMD
  modules, which the crate forbids.
- Rust-only numerical checks (`finite`, cached inverse weights in update) and
  the fused bounds pass are the rest.

## Gates

All on the final source (`src/simplify.rs`, `CHANGELOG.md`):

| Gate | Result |
|---|---|
| exact work-prefix driver (`parity/0216-simplify/exact.rs`, every work limit, cold/reused/after-failure, `Usage` work and bytes) vs renamed b0 source, all features | 8,970,468 observations, 0 differences, exit 0 |
| same, no default features | 5,769,468 observations, 0 differences, exit 0 |
| C++ differential (`parity/0215-toward-cpp/cpp-differential.rs`), all / no features | 1,932 cases each, 0 differences, exit 0 |
| oracle pin (`parity/check-reference.sh`) | exit 0 |
| `cargo fmt --check` | exit 0 |
| `cargo clippy --all-targets` all / no features, `-D warnings` | exit 0 / 0 |
| `cargo test` default / all / no features (doctests included) | exit 0 / 0 / 0 |
| `cargo doc` all / no features, `RUSTDOCFLAGS=-D warnings` | exit 0 / 0 |
| `cargo build --target wasm32-unknown-unknown --no-default-features` | exit 0 |
| `parity/check-unsafe-boundary.sh` | exit 0 (no `unsafe` added) |
| `parity/tools/check_public_text.sh`, `git diff --check` | exit 0 |
| cargo-semver-checks 0.51.0 vs 0.2.16, all features | 202 checks pass, no semver update required, exit 0 |
| measurement output identity: every case, every arm | equal |
