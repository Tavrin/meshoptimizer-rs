# Qualification evidence

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
