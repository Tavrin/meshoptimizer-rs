# 0.2.17 qualification

Candidate: `train/0.2.17` at `c08049f1`. Baseline: v0.2.16. Reference: C++ meshoptimizer 1.3.
`src/`, `Cargo.toml` and `Cargo.lock` were frozen during qualification. Machine-readable results:
[qualification-summary.json](qualification-summary.json); the release table is [TABLE.md](TABLE.md).

## Verdict

- Exact equivalence with v0.2.16: 0 differences, all features and no default features.
- C++ 1.3 differentials: 0 differences. Malformed-input failed-buffer writes differ as already
  disclosed for 0.2.16.
- No function is more than 1% slower than v0.2.16 in any case or group (worst case 1.0038,
  vertex encode).
- wasm32 one-shot vertex decode is not slower than v0.2.16 (Node, 8 inputs, after/before
  geometric mean 0.92, worst 0.97).
- aarch64 one-shot vertex decode was not timed (no arm64 host). Its correctness runs in CI.

## Equivalence (exact drivers, all features / no default features)

| Driver | Observations |
|---|---|
| remap | 10,811,136 / 10,811,136 |
| simplify family | 8,970,468 / 5,769,468 |
| overdraw | 20,558,672 / 20,558,672 |
| vertex decoder (NEON lane driver) | 3,505,440 / 3,505,440 |
| vertex decode, stride 12 | 5,114,700 / 5,114,700 |
| bounds | 228,760 / 228,760 |
| spatial meshlets | 4 x 358,458 (forced levels) / 358,458 |
| LOD quality | 91,290 / 91,290 |
| hidden-surface removal and repair preprocessing | 87,272 / 87,272 |
| index-sequence streaming baseline comparisons | 164,442,780 / 164,442,780 |

All with 0 differences. `cargo test`: 455 passed (all features), 381 passed (no default features).
clippy with warnings denied, fmt, the unsafe-boundary check and the LOD-corpus identity test pass.

## C++ 1.3 differentials

- simplify, update, prune, points, cache, overdraw, fetch: 1,246 cases per mode, equal.
- meshlets (1,024), flex meshlets (1,024), spatial meshlets (2,000), bounds (512), cluster LOD
  (24), spatial extremes (28): equal across 7 clients.
- LOD quality oracle: 256 exhaustive cases per mode, 0 differences at 1e-7.
- vertex decoder: 318,608 cases across forced Scalar, SSE2, SSSE3, SSE4.1, AVX2 and AVX-512
  levels against C++ scalar, default, AVX2 and AVX-512 builds: 0 differences on successful
  decodes.

## Parallel, streaming and width identity

- Cluster LOD at 1/4/16 workers: 450 corpus checks and 80 real-input checks, 0 differences.
- Parallel vertex encode/decode and parallel spatial meshlets vs serial and vs v0.2.16: 240
  checks, 0 differences.
- Streaming vs one-shot decoders: 168 checks, 0 differences.

## Speed (instructions, ours / C++ 1.3, real-mesh / corpus geometric means)

See TABLE.md for every function. Highlights against v0.2.16: vertex cache 0.57 / 0.85, vertex
decode (default build) 0.85 / 0.90, simplify 0.96 / 0.97.

## Disclosures

- Streaming vs one-shot decode, instructions: vertex 1.26-1.32x at 4 KiB chunks, 1.04x at
  64 KiB; triangles 1.03-1.15x.
- `process_mesh` vs the same steps called by hand: 1.003x.
- Hidden-surface removal: 0.47-1.83 s on the evaluation meshes.
- Character LOD best-of mode: about 2x the time of one method.

## Not verified

aarch64 timing; the 0.2.15 merge driver ran in quick mode only; the timing was taken on a shared
host (decisions use instruction counts). Fuzzing is reported separately in
[../0217-fuzz/RECEIPTS.md](../0217-fuzz/RECEIPTS.md).
