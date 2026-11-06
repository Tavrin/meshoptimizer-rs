# 0.2.19 candidate qualification runner

Run from the repository root inside the brief's CPU admission wrapper. Source files are frozen.
Set `CARGO_TARGET_DIR`, `QUALIFY_LOGS`, `MESHOPT_REFERENCE`, `QUALIFY_INPUTS`, `PEERS_PERF`,
`CARGO_SEMVER_CHECKS`, and `VG_CORPUS` to caller-owned locations. Optional `QUALIFY_DISK_FLOOR_GB`
controls the scratch free-space floor. No fuzz campaign is run.

1. `python3 parity/0219-qualify/run.py prepare`
2. `python3 parity/0219-qualify/campaign.py`
3. `python3 parity/0219-qualify/supplement.py`
4. `python3 parity/0219-qualify/bench.py`
5. `python3 parity/0219-qualify/matrix_default.py` (the recorded repeat with optimization enabled)
6. `python3 parity/0219-qualify/finalize.py` (removes the disposable target and writes the final report)

The campaign preserves the 0.2.18 qualification's frozen eight instruction inputs,
21 operation families, scalar/default Rust and C++ arms, and three counterbalanced samples.
The reused exact and C++ drivers remain in their original repository directories.
The baseline is extracted with `git archive`; no checkout or Git index mutation is needed.
Supplemental drivers compare preserved 0.2.18 APIs and MDAG 1, then hash new virtual-geometry APIs
across feature and target variants. Native all-feature variants also compare 1/2/4/8/16 workers.

Opaque and material determinism cases enable cluster optimization; the strict alpha retention fixture disables it. `matrix_default.py` repeats that matrix and wasm timing.
