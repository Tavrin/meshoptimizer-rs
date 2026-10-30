# Qualification evidence

The **0.2.16 combined train qualification** against v0.2.15 is recorded in
`REPORT.md`, `TABLE.md`, and `combined-ff38162/`. Its registration, exact
counts, C++ differentials, source-identical arm64 CI receipts, raw counters,
cost disclosures, and final verification are retained together there.

The files described below belong to the **earlier qualification against a
different baseline**. Its report is preserved in `REPORT-pre-train.md`.
Those historical proof gaps and measurements are not current train receipts.

`REGISTRATION.md` specifies the campaign and its retained continuation after an
adapter failure. `results.json`, `counts.jsonl` and the two campaign bindings
record the 88 measured cases. The raw inputs, binaries, logs, outputs, failed
attempts and admitted job receipts remain in the lane scratch directory.

The exact drivers here are finite subsets; `ENUMERATION.md` and
`enumeration.json` describe what remains missing. `reach.json` records
coverage evidence and the LLVM warning. Run all builds and executions under
the prescribed CPU-only admission wrapper with a dedicated target directory
and `CARGO_INCREMENTAL=0`. The frozen v0.2.13 source and candidate must be
built as separately named dependencies; the overdraw exact driver requires a
directory of peers-format small inputs as its first argument. The simplify
exact driver accepts `fuzz SEED` for its bounded fuzz stream. No success of
these subsets substitutes for the full exact proof.

For SPEC-0216b, `enumeration-0216b.json` records the corrected release domain
and distinguishes retained subset counts from the still-unknown required
per-function sizes. `rerun-affected.sh` reruns the corrected simplify and
overdraw subset drivers in both modes; it is not a complete proof supervisor.
`run-overdraw-strides.sh` adds a separate four-mesh stride-by-work sweep, and
`run-protocol-modes.sh` compares the pinned C++ protocol in both feature modes.
`overdraw-fuzz.py` measures five minutes of child CPU time for the overdraw
fuzz stream. Set `MESHOPT_SCRATCH`, `MESHOPT_QUALIFY_TARGET`, and, for the
protocol run, `MESHOPT_REFERENCE`; execute each through the prescribed
CPU-only admission wrapper. The complete release enumeration still needs to
be implemented and executed before any SHIPS verdict.
The additional `run-meshlet-strides.sh`, `run-meshlet-strides-owned.sh`,
`run-simplify-flags.sh` and `run-clod-flags.sh` drivers record bounded
cross-products; their receipts state the exact cases covered and must not be
confused with complete release enumeration sizes.
`run-clod-flags-structured.sh` uses direct record and position equality for
the same domain; the earlier formatting-based run was interrupted.
