# 0.2.7 release results — 2026-10-26

Head: `8c0bd37`. Release edits remain uncommitted. No library or CI changes.
No commit, push, tag, merge or publication was performed.

## Fuzz

| Target | CPU seconds | Executions | Crashes |
|---|---:|---:|---:|
| `retention` | 912.30 | 4,582,547 | 0 |
| `vertex_fetch` | 325.94 | 2,844,535 | 0 |
| `overdraw` | 325.66 | 65,988,528 | 0 |
| `optimize_meshlet` | 326.48 | 10,588,481 | 0 |

Retention executions count random sequences of 8–32 calls. All targets meet
their CPU budgets; source and executable identities remain unchanged.
The meshlet run uses libFuzzer coverage with no sanitizer; the other targets
use stable seeded mutation. These are bounded release checks.

Raw receipts, source archives, binaries, seeds, corpora and logs:
`/mnt/linux-extra/moss-scratch/meshopt-rel027`.
Fuzz record SHA-256:
`91660677d17368a9758b50b93cc5288791b3b49d38d5a09d25cd56cd47b135a7`.
The supplemental `parity-source-identity.json` and `parity-sources.tar.gz`
retain the unchanged protocol dependency used by the fetch target.

## Checks

All checks ran in one script through CPU-only `moss-heavy`, declaring 4 GB.

| Check | Exit |
|---|---:|
| `cargo test --doc --all-features` | 0 |
| `cargo test --doc` | 0 |
| `cargo test --locked --no-default-features` | 0 |
| `cargo test --locked --all-features` | 0 |
| `cargo package --locked --allow-dirty` | 0 |
| `cargo publish --dry-run --locked --allow-dirty` | 0 |
| `cargo package --list --allow-dirty` | 0 |
| `package excludes REPORT/REGISTRATION` | 0 |

The first plain `cargo package --list` exited 101 because Cargo requires
`--allow-dirty` while the release files remain uncommitted. Its log remains
in `checks-initial/6.log`; no receipt was overwritten. The final complete
script passed using `--allow-dirty` for the listing. The six other commands
also passed in both script runs. No real upload occurred.

All 13 lockfile entries pin 0.2.7. The moved lane-C files are byte-identical
to their root originals. README changes are limited to the Lean usage line
and v0.2.7 links. Formatting and whitespace checks pass.

Cleanup: the designated target
`/mnt/linux-extra/moss-cargo-targets/meshopt-rel027` was deleted after saving
all receipts and executables; `/mnt/linux-extra/moss-scratch/meshopt-rel027/cleanup.json`
records removal. No other target directory was removed.
