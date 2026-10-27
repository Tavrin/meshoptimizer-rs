# Overdraw qualification lane

Use an admission runner that accepts a memory declaration in GB followed by
a command, enforces a CPU-only job, and preserves its admission identity in
MESHOPT_ADMISSION_ID. All builds and measurements must use admit.sh (4 GB).
Configure paths in a private shell file outside the repository:

```sh
export MESHOPT_OVERDRAW_SCRATCH='<scratch>/overdraw'
export MESHOPT_RESCREEN_SCRATCH='<scratch>/rescreen'
export MESHOPT_REFERENCE='<reference>'
export MESHOPT_PERF='<perf-6.17>'
export MESHOPT_JOB_RUNNER='<job-runner>'
export MESHOPT_PROFILE='<scalar-profile>'
export CARGO_TARGET_DIR='<target-dir>'
```

The repository root is inferred from the script location. Preparation writes
isolated baseline/variant crates, drivers and proof consumers into scratch;
those generated manifests contain local paths and must remain private.
Run prepare.py and proof-prepare.py once, preserving prior evidence. Run
build.py attribution then measure.py attribution through admit.sh. See
REGISTRATION.md for the registered predicate follow-up and final sequence.
After selecting and formatting the candidate, invoke `admit.sh bash job.sh
final` (with paths relative to this lane or the full relative repository paths).
That runs all final builds, proofs, counters, audit, target cleanup and report
sequentially, preserving logs and stopping on any nonzero exit.

Machine configuration and operational logs must never be copied into public
lane files. The report and compact results use placeholders for all private
locations. The JSON binding has hashes under these placeholder locations.

The campaign is terminal NO-SHIP; the scripts retain the historical protocol.
Do not run another acceptance campaign. MESHOPT_PROFILE names the configured
scalar profile in the frozen private harness. Executed scripts are archived
before the coordinator-requested public naming migration.
