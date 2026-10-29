# Cache gain restored: SHIPS

Input train: 68439a3 (train/0.2.15). One library line changes:
`src/cache.rs:389` puts `#[inline(never)]` on `cache_loop`. Both score-table
implementations, all arithmetic, traversal/tie order, resource accounting,
validation, error ordering and partial-write boundaries remain unchanged.

## Cause and intervention

The merge did **not** lose the batched tally. `git diff cc13453 68439a3 --
src/cache.rs` contains exactly the S5 tally/visit change. The cache file at
merge 733f39d and the input train has the same SHA-256:
`9f1a03ae8188b68013f7ab44eb53a7e9ee327af89a8e74d4e548984b18fb07a8`.
`git diff 5854223 68439a3 -- src/cache.rs` shows the table APIs and the
replacement of const-STRIP scoring with the generic `Scores` strategy.

This is a generated-code regression in the combined kernel after that
integration, rather than dynamic score dispatch: disassembly of the
unchanged train's default kernel has constant score-table addresses and
inlined scoring, with both covered and limited loops inside one kernel.
The controlled scalar S5 rebuild still achieves 0.9422 real / 0.9448 corpus,
whereas the train rebuild reproduces 0.9947 / 0.9660 under the same consumer,
compiler, features, inputs and counters. This rules out changed timing,
toolchain or optional SIMD as the explanation for the observed loss.

The single-attribute intervention separates each loop monomorph from setup
and from its limited/covered counterpart. `nm -SC` on retained binaries:

| Symbol | Train size, bytes | Candidate size, bytes |
|---|---:|---:|
| default kernel | 6437 | 2475 |
| default covered cache_loop | inlined | 1798 |
| default limited cache_loop | inlined | 3020 |

It recovers 3.98% of train instructions on real meshes (candidate/train
0.9602) without changing the algorithm. The exact LLVM heuristic/register
allocation decision is not isolated; the proven causal intervention is the
call boundary. Total code size increases. This partially recovers S5's
historical 0.9422 real ratio and meets the specified <=0.97 gate; it does
not claim to recover the entire historical gain.

## Measurements

Median of three resident-loop `instructions:u` counts per arm and case;
100% counter running in all 240 accepted samples (120 attribution, 120
candidate). Five frozen corpus meshes, three real meshes. Scalar arms:
v0.2.14, pinned input train, candidate, C++ meshoptimizer 1.3 via meshopt
0.7.0, optimesh 1.1.0. Attribution substitutes the S5 revision for candidate.
All arms and repetitions have identical output SHA-256 in every case.
No wall-time performance claim.

| Version/group | /v0.2.14 | /optimesh | /C++ |
|---|---:|---:|---:|
| Train real | 0.994663 | 0.956632 | 1.306013 |
| Candidate real | **0.955096** | **0.918578** | **1.254061** |
| Train corpus | 0.966014 | 0.933865 | 1.272652 |
| Candidate corpus | **0.942597** | **0.911228** | **1.241802** |

| Case | Train/base | Candidate/base |
|---|---:|---:|
| tiny-smooth | 0.973923 | 0.948138 |
| medium-smooth | 0.962900 | 0.941017 |
| medium-seams | 0.968040 | 0.941690 |
| medium-sparse | 0.963109 | 0.941352 |
| million-smooth | 0.962151 | 0.940811 |
| real-214e | 0.995541 | 0.956253 |
| real-a082 | 0.994224 | 0.954518 |
| real-b6fd | 0.994224 | 0.954519 |

**SHIPS**: real GM 0.955096 <=0.97; maximum case 0.956253 <=1.01.
`candidate-1/results.json` contains per-arm medians and complete ratios.

## Commands and proof

Commands below ran through the installed CPU admission runner, declaring
4 GB, with the required isolated target, two compiler jobs, LC_ALL=C and
perf 6.17. The Python build/measurement class selected a 9 GB reservation
and cap from shared history; the proof runner reserved 4 GB with a 6 GB cap.
No queued job was canceled. README.md documents environment and reproduction.

| Command (under admission) | Result | Exit |
|---|---|---:|
| `python3 parity/0215-cache-restore/build.py base train s5 cpp rust` | five frozen arms built | 0 |
| `python3 parity/0215-cache-restore/build.py candidate` | candidate built | 0 |
| `python3 parity/0215-cache-restore/measure.py attribution base train s5 cpp rust` | 120 samples, outputs equal | 0 |
| `python3 parity/0215-cache-restore/measure.py candidate-1 base train candidate cpp rust` | 120 samples, outputs equal | 0 |
| `bash parity/0215-cache-restore/proof.sh` | all 22 steps passed | 0 |

The proof script runs these exact gates (all exit 0):

- `cargo test --locked --all-features` and `--no-default-features`.
- `cargo clippy --locked --all-targets --all-features -- -D warnings` and
  the same command with `--no-default-features`.
- `cargo fmt --all -- --check`.
- `python3 parity/simd/check_boundary.py`.
- `cargo package --locked --allow-dirty --list | python3 parity/simd/check_package.py`
  under bash pipefail (direct equivalent of the boundary wrapper, without
  a verification-disabling flag).
- `bash parity/tools/check_public_text.sh`.
- Existing `default_vertex_score_table_identity_on_corpus` test, release,
  all-features and no-default-features: all eight cases passed in each mode.
- S5 `parity/0215-toward-cpp/exact.rs` against v0.2.14, all-features and
  no-default-features: **992,130 comparisons, zero differences per mode**.
- `table-exact.rs` against cc13453, both modes: **2,144,880 comparisons,
  zero differences per mode**. Covers default, zero, negative, extreme finite,
  mixed-sign and transient-maximum tables; empty, invalid, degenerate,
  repeated/high-valence and seeded meshes. Every work prefix through success
  plus one, selected byte limits, short/full caller destinations and scratch
  reuse compare complete results, errors, Usage and partial writes.

`results.py` evaluated both campaigns with exit 0. `audit.py` verified all
240 raw perf counters and output files, frozen sources, binary hashes and
22 proof exit receipts with exit 0. All raw evidence remains in the private
scratch directory; portable hashes/counts and receipts are committed here.
The dedicated target is removed after lane jobs finish (cleanup.json).
Final public-text and `git diff --check` passed, exit 0.

## Not verified

Other architectures, wasm, no_std performance, wall-time speed, custom-table
performance, effects on other function families and exhaustive allocation
failure/mesh/float domains were not measured. No independent review is
claimed. The exact drivers cover finite domains; the structural contract
argument is that only an inlining attribute changed. Existing unsafe/SIMD
code is unchanged; the cache change is safe scalar Rust. No merge, release,
tag or publication was performed.
