# SPEC-0212 public-text scrub

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
codec license attribution. Existing archive filenames remain available.

Set the required environment variables before running an archived script.
`CARGO_TARGET_DIR`, `WORKTREE`, and `REPO` designate the selected directory,
while `SCRATCH`, `REFS`, `LOCAL`, and `MESHOPT_ARTIFACTS` designate placeholder
roots. Shell replacements preserve quoting; Python replacements expand only
referenced placeholders and expand the registry's home prefix. Python heredocs
use the same expansion. Admission environment variable names remain compatible
with the external job runner.

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
