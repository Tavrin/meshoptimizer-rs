# 0.2.10 release preparation — 2026-10-26

Base: `3e93b86`, branch `release/0.2.10`, after PRs #34 and #35.
Release edits remain uncommitted. Library code and CI are unchanged.
README edits only replace `blob/v0.2.9` with `blob/v0.2.10`.
The manifest and all 14 lockfile entries, including `parity/028-L1/Cargo.lock`,
pin 0.2.10.

Instruction ratios in the changelog are rounded from
`parity/0210-H1/REPORT.md` and `parity/0210-H2/REPORT.md`.
The 1.26× cache comparison uses the combined corpus and real-chain population;
the frozen-only population is 1.318×. No new measurements were run.

## Checks

`checks.sh` ran as one job through
`<job-runner> 4 bash parity/release-0210/checks.sh`,
with `MOSS_HEAVY_GPU=0`, `CARGO_BUILD_JOBS=2`, and
`CARGO_TARGET_DIR=<target-dir>`.
The job exited 0.

| Check | Exit |
|---|---:|
| `cargo test --locked --no-default-features` | 0 |
| `cargo test --locked --all-features` | 0 |
| `cargo test --doc --all-features` | 0 |
| `cargo test --doc` | 0 |
| `python3 parity/simd/check_boundary.py` | 0 |
| `cargo package --locked --allow-dirty` | 0 |
| `cargo publish --dry-run --locked --allow-dirty` | 0 |
| `cargo package --list` | 101 |
| `cargo package --list --allow-dirty` | 0 |
| Package includes migration guide, excludes REPORT/REGISTRATION files | 0 |
| Internal-reference check, excluding historical link URLs | 0 |
| Lockfile version verification, including the specified grep | 0 |
| `git diff --check` | 0 |

Both explicit doctest commands include the migration guide doctests.
Plain `cargo package --list` refuses uncommitted release edits. The refusal
is retained; the package contents were verified with `--allow-dirty`,
consistent with preparing an uncommitted release. No Git mutation was used
to avoid that refusal.

Logs, exit codes and extracted public text are retained at
`<scratch>/meshopt-rel0210/`.
The script's exit trap deleted the designated target directory;
`cleanup.txt` records its absence. No other target was removed.

No commit, push, tag, merge or publication was performed.
