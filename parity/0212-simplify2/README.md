# Simplify instruction qualification

Run from `<repo>`. Local paths come from the environment:

| Variable | Value |
|---|---|
| `MESHOPT_JOB_RUNNER` | `<job-runner>` executable accepting a GB declaration followed by a command |
| `MESHOPT_SCRATCH` | `<scratch>/meshopt-0212-simplify2`, initially absent |
| `MESHOPT_ATTRIBUTION` | `<scratch>/meshopt-0212-simp-attr`, retained attribution inputs/harness |
| `MESHOPT_REFERENCE` | Pinned C++ reference checkout |
| `MESHOPT_PERF` | perf 6.17 executable |
| `MESHOPT_M7_SOURCE` | The pinned M7 package source directory |
| `CARGO_TARGET_DIR` | `<target-dir>`, dedicated to this job |

The complete sequence after preparation is `bash parity/0212-simplify2/admit.sh bash parity/0212-simplify2/qualify.sh`.

Preparation writes isolated baseline/proof consumers and checks input hashes:

```sh
python3 parity/0212-simplify2/prepare.py
bash parity/0212-simplify2/admit.sh python3 parity/0212-simplify2/build.py
bash parity/0212-simplify2/admit.sh python3 parity/0212-simplify2/measure.py
python3 parity/0212-simplify2/results.py
bash parity/0212-simplify2/admit.sh bash parity/0212-simplify2/gates.sh
bash parity/0212-simplify2/admit.sh python3 parity/0212-simplify2/audit.py
python3 parity/0212-simplify2/report.py
```

Do not overwrite a retained campaign. Preserve an entire rejected attempt before
registering a refinement. The final audit checks source/input/binary hashes,
all raw counters and outputs, exact comparison logs and fuzz CPU budget before
emitting portable public bindings. Full machine-specific receipts stay in scratch.
Remove only the dedicated target directory after the final audit.
