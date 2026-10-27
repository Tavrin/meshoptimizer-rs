# registered protocol public-text scrub

Implemented as uncommitted file edits on `chore/scrub-paths`, based on
`23221072b44aef940f0512cbd0de67e4d2e8270f`. The coordinator owns committing.
No evidence files were removed and no library behavior was changed.

The existing `boundary` job lives in `.github/workflows/simd.yml`, rather than
`ci.yml`; its new step runs `check_public_text.sh`. The guard searches tracked
and untracked, nonignored text and excludes only its own source. Exit 1 from
`git grep` means clean; actual grep errors propagate as failures.

## Placeholder and environment map

| Original role | Archived text | Script environment |
| --- | --- | --- |
| Maintainer repository or named repository worktree | `<repo>` | `REPO` |
| Scratch root, retaining relative suffix | `<scratch>/…` | `SCRATCH` |
| Named Cargo build directory | `<target-dir>` | `CARGO_TARGET_DIR` |
| Named downstream worktree | `<worktree>` | `WORKTREE` |
| Artifact root, retaining relative suffix | `<artifacts>/…` | `MESHOPT_ARTIFACTS` |
| Reference root, retaining relative suffix | `<refs>/…` | `REFS` |
| Cargo registry under the maintainer home | `~/.cargo/registry/…` | Expanded user home |
| Other local paths, keeping terminal component | `<local>/…` | `LOCAL` |
| Job admission executable | `<job-runner>` | `JOB_RUNNER` |

Other private tool/directory prefixes become `downstream-`; the old storage
volume name becomes `storage`, and the private projects directory becomes
`projects`. The bare engine name becomes `a downstream engine`, preserving
codec license attribution. Evidence files remain available under neutral names; all committed references follow the renames.

Set the required environment variables before running an archived script.
`CARGO_TARGET_DIR`, `WORKTREE`, and `REPO` designate the selected directory,
while `SCRATCH`, `REFS`, `LOCAL`, and `MESHOPT_ARTIFACTS` designate placeholder
roots. Shell replacements preserve quoting; Python replacements expand only
referenced placeholders and expand the registry's home prefix. Python heredocs
use the same expansion. Archived scripts use the public admission environment below. A caller using
an external runner must export these names in its payload.

## Evidence and hashes

`python3 scrub_paths.py` performs the bulk rewrite and is idempotent after the
scrub. `scrub_hash_updates.json` lists every refreshed current-file binding,
its checking script, old/new SHA-256, and affected records (26 edited files,
30 record entries). These are registration/bar documents, the L2 harness
inventory checked by its audit, and the comment-only simplifier source binding.
Measurement input, executable and output hashes, and historical evidence/archive
inventory hashes remain unchanged. Their historical identities do not certify
the rewritten text; no old performance qualification is renewed by this edit.
All numeric, boolean and null JSON values were compared against the base and
are unchanged across 78 edited existing JSON files.

## Proof

| Check | Exit/result |
| --- | --- |
| Public-text guard | 0 |
| `bash -n` on all 54 edited/new shell scripts | 0 |
| `python3 -m py_compile` on all 123 edited/new Python scripts | 0 |
| `cargo test --locked --all-features` | 0 |
| `python3 parity/simd/check_boundary.py` | 0; 23 documented unsafe blocks |
| Base and current `cargo package --list --allow-dirty` | 0 each; identical 73 entries |
| `git diff --check` | 0 |
| Second bulk rewrite | 0; rewrote 0 files |
| Refreshed hashes verified against edited files and records | 26 valid bindings |

The one Cargo test used CPU-only job admission, declared 4 GB and one build
worker. The runner selected a 2 GB reservation and an automatic 8 GB cap;
actual recorded peak was 0.5 GB sampled / 1.3925 GB cgroup, below 4 GB. Its exit
was 0; six existing exhaustive/migration tests were ignored by the normal suite.
The dedicated target directory was removed after the test. Package-list
comparison used a temporary base archive with read-only repository metadata so
both lists included Cargo's generated VCS metadata entry.

## Public admission environment

| Public name | Meaning |
| --- | --- |
| `PARITY_RUNNER_ACTIVE` | Active admission identity; required by admitted scripts |
| `PARITY_RUNNER_GPU` | `0` for CPU-only work, `1` for GPU work |
| `PARITY_RUNNER_MEASUREMENT` | Measurement scheduling indicator |
| `PARITY_GPU_LEASE` | Held GPU lease indicator |
| `PARITY_GPU_LEASE_LABEL` | Lease attribution label |
| `PARITY_GPU_LEASE_DIR` | Lease state directory, when required |
| `PARITY_JOB` | Job attribution identity |

Other archived runner settings retain their suffix under `DOWNSTREAM_`.
`python3 parity/tools/scrub_names.py` rewrites tracked project and planning
names and performs evidence-preserving renames. Consumer labels and result
keys use `downstream`; Cargo's thin-LTO custom profile is `bench-release`.
Dynamic consumer builds map the consumer label to that Cargo profile.
Release selection uses `--release` throughout the archived parity CLI.
The public guard checks case-insensitively and exempts only the two exact
codec licence attribution lines and its own source.

The additional scrub leaves measurement values and historical input, binary,
output and archive hashes intact. New current-file hash refreshes are appended
to `scrub_hash_updates.json`; earlier refresh entries remain historical.
No historical performance qualification is renewed.

Additional scrub checks: shell/Python syntax, the all-features Cargo test,
the SIMD boundary check, and both package inventories exited 0. The package
inventories are identical (73 entries). Untimed fast-qualification tests pass
(27 tests). The cluster-LOD harness passes (4 tests) with its vendor source
configured; the initial unconfigured invocation reported a missing vendor
file. These mocked harness tests do not remeasure or renew qualification.
