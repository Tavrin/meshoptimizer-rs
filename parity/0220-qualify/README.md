# 0.2.20 qualification

Source is frozen; the runner never mutates the index or references. The baseline release (`QUALIFY_BASE_REF`, default v0.2.19) is extracted with `git archive`.

## Locations

Every directory comes from the environment; nothing below is written into the repository except the small JSON/Markdown receipts of the retained campaign.

| Variable | Required | Meaning |
|---|---|---|
| `CARGO_TARGET_DIR` | yes | disposable build target; delete it when the run is collected |
| `QUALIFY_LOGS` | yes | command logs (`logs/`), `exits.jsonl`, admission logs and large receipts (instruction counts and cache, stream case tables). Keep it outside the repository |
| `QUALIFY_ADMISSION` | for `execute.py`, `resume.py`, `deferred.py` | admission wrapper invoked as `<wrapper> <GB> <command...>` |
| `QUALIFY_RECEIPTS` | no | small receipts; defaults to this directory for the retained campaign |
| `QUALIFY_SOURCE` | no | source tree under test; defaults to this worktree (`deferred.py` sets it to an extracted commit) |
| `QUALIFY_BASE_REF` | no | baseline tag or commit, default `v0.2.19` |
| `QUALIFY_STEER` | no | admission steering file; a hold there stops new submissions |
| `VG_CORPUS` | bench | frozen virtual-geometry corpus |
| `MESHOPT_REFERENCE`, `QUALIFY_INPUTS`, `PEERS_PERF`, `CARGO_SEMVER_CHECKS` | full campaign only | C++ reference, frozen inputs, `perf` binary, semver tool |

No item uses the GPU; configure the admission wrapper for CPU-only jobs.

## Deferred items on one commit

```
python3 parity/0220-qualify/deferred.py <commit> [v2-exact] [determinism] [bench] [all-features]
```

With no item names it runs all four, in that order, each in its own admission:

| Item | GB | What it runs |
|---|---:|---|
| `v2-exact` | 4 | weight-zero v2 bytes against the baseline, grids 12/24/48, cases 0/1/2/4, all-features and `clusterlod`-only builds |
| `determinism` | 4 | the eight-variant determinism matrix (native/wasm32, std/no_std, release/debug, 1-16 workers); the summary pulls out the v2 normal-error weight 0 and 0.25 cases |
| `bench` | 4 | five-mesh virtual-geometry benchmark, then the two-weight shading rows |
| `all-features` | 9 | `cargo test --offline --release --all-features` with `RUST_TEST_THREADS=2` |

The commit is extracted with `git archive` into `$QUALIFY_LOGS/deferred-<commit>/source`; the worktree is not touched. Receipts go to `deferred-<commit>/receipts`, logs to `deferred-<commit>/logs`, and `deferred-<commit>/summary.json` records each item's exit code plus its key facts (v2 equal/total per feature mode, determinism equality for both weights, failed benchmark meshes, shading row count). An exit code of 75, 130 or 143 from the wrapper, or a hold in `QUALIFY_STEER`, stops further submissions. Rerunning the same commit reuses the extracted source and replaces the per-item results.

`v2-exact` exits 1 when any grid/case differs from the baseline; differences are listed with their first differing byte and never normalized. The driver is generated from `determinism.rs` into the target directory; each source rewrite must match an exact number of times, so drift in `determinism.rs` stops generation with a message rather than producing a driver that does not compile. `python3 parity/0220-qualify/v2_exact.py --build-only` compiles it without running.

## Retained full campaign

`python3 parity/0220-qualify/execute.py` runs the stages serially: preparation at 4 GB; formatting, other required gates, exact checks, instruction counts, determinism and benchmarks in one 4 GB admission; the all-features test/doctest and final recording at 9 GB. No stage runs concurrently. `resume.py` resumes the pending stages without repeating passed Clippy.

The campaign uses the retained eight inputs, 21 operations and six arms. It collects one instruction sample per cell, caching fixed arms by binary/input/driver/perf/bounds hashes and operation. Counters must have 100 percent runtime. No fuzz campaign is invoked.

Release tests and doctests use `RUST_TEST_THREADS=2` to bound concurrent compiler memory. The retained initial default-worker attempt was killed by the out-of-memory daemon; no test or timeout was changed.
