# Combined exact proof

Candidate source: `ff38162`; base: v0.2.15
(`1ecf69abe9f92c17e13dc38fa97be659aee44582`). All rows below completed with
exit 0 and zero baseline differences. Counters retain each lane driver's
original meaning; overlapping observations are not presented as independent
coverage. Full logs and process exits are in `logs/` and `exits.jsonl`.

| Lane driver | All features | Scalar/no defaults |
|---|---:|---:|
| remap | 10,811,136 | 10,811,136 |
| repair prerequisites | 68,872 | 68,872 |
| simplify family | 8,970,468 | 5,769,468 |
| decoder (NEON lane's exact driver) | 3,505,440 | 3,505,440 |
| stride-12 decoder exact driver | 5,114,700 | 5,114,700 |
| bounds | 228,760 | 228,760 |
| supplemental geometry | 1,020,255 | 1,020,255 |
| spatial | 4 × 358,458 | 358,458 |
| streaming: unchanged one-shot baseline comparisons | 164,442,780 | 164,442,780 |
| parallel targeted contracts | 86,371 | 86,371 |
| parallel original worker grid, LOD corpus | 720 | 720 |
| parallel 1/4/16 workers, LOD corpus | 450 | 450 |
| parallel 1/4/16 workers, eight frozen inputs | 80 | 80 |

Spatial's four all-feature runs force Scalar, SSE2, SSSE3, and SSE4.1.
The scalar bounds/geometry consumers enable `clusterlod` as required by those
drivers; their `std` and `simd` features are disabled. The scalar parallel
consumer enables `parallel`, `clusterlod`, and `parity-internals`, which also
enables the mandatory `std` dependency; it disables `simd`. See the retained
consumer manifests and binary hashes in `exact-binding-*.json` and
`par-spec-binding-*.json`.

Each streaming run also completed 9,990 C++ wire checks, 2,703,362 chunkings,
and 2,284,337 streaming budget checks. Each remap C++ run completed 2,592
comparisons with zero differences. Each repair property run passed all 45
shared generated meshes.

Cargo tests (including doctests), Clippy, and the LOD identity test on both
the generated corpus and all eight frozen inputs returned 0 in both modes.
The all-feature test invocation reports 350 passed / 0 failed / 8 ignored;
no defaults reports 285 passed / 0 failed / 8 ignored. The required ignored
LOD identity test was separately executed on each input set in each mode.
Formatting and the package/unsafe boundary gate returned 0. The package audit
found exactly one unsafe-code allowance and all 37 audited blocks.

The core job used **2,726.259658 CPU seconds**, below its 6,000-second size
estimate and the 7,200-second maximum. Queue waiting is excluded from CPU
usage. The executable supervisor copy is retained by SHA-256 in `runners/`.
The production source, Cargo manifest and lockfile remained unchanged.

The source-identical Linux/Apple arm64 CI receipts are documented separately
in [CI.md](CI.md). Measurement and the remaining merged C++ differentials are
separate stages; this proof record does not assert their acceptance.
