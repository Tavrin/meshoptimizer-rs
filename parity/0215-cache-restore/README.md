# Cache restore evidence

See REPORT.md for attribution, results and limits; REGISTRATION.md fixes the
bounded experiment. Set RESTORE_SCRATCH to a fresh private artifact directory,
RESTORE_INPUTS to the eight retained S5 peer fixtures, RESTORE_CORPUS to the
cache-table corpus (real inputs have the `-simplify.bin` suffix), and
CARGO_TARGET_DIR to an isolated build directory. Use LC_ALL=C,
CARGO_INCREMENTAL=0, CARGO_BUILD_JOBS=2, the installed CPU admission runner
with 4 GB declared and GPU disabled, and export PARITY_RUNNER_ACTIVE=1,
PARITY_RUNNER_GPU=0. Set PEERS_PERF to the specified perf 6.17 binary and
PEERS_CPU=9. Preserve the runner's actual reservation/cap in local logs.

Through admission, from the repository root:

```sh
python3 parity/0215-cache-restore/build.py base train s5 candidate cpp rust
python3 parity/0215-cache-restore/measure.py attribution base train s5 cpp rust
python3 parity/0215-cache-restore/measure.py candidate-1 base train candidate cpp rust
bash parity/0215-cache-restore/proof.sh
```

Copy `parity/0215-toward-cpp/inputs.json` to RESTORE_SCRATCH/inputs.json
before measuring. The build helper freezes each source, adapts the inherited
peer consumer only to disable optional SIMD and elapsed-time collection,
and retains build logs and standalone executables outside the build target.
Do not modify a frozen source or reuse a scratch directory for another variant.

Evaluate and audit (these commands only read retained measurement files):

```sh
python3 parity/0215-cache-restore/results.py "$RESTORE_SCRATCH/attribution"
python3 parity/0215-cache-restore/results.py "$RESTORE_SCRATCH/candidate-1"
python3 parity/0215-cache-restore/audit.py
bash parity/tools/check_public_text.sh
git diff --check
```

Delete only the dedicated CARGO_TARGET_DIR after every admitted lane job has
finished; retain scratch fixtures, frozen sources, logs, binaries, raw perf
counters and outputs. No merge, release, tag or publication is performed.
