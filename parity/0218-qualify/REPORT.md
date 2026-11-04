# 0.2.18 qualification

Candidate: `train/0.2.18` at `c7dbb4df`. Baseline: v0.2.17. Reference: C++ meshoptimizer 1.3.
`src/`, `Cargo.toml` and `Cargo.lock` were frozen during qualification. The release table is
[TABLE.md](TABLE.md).

New since v0.2.17, all additive: alpha-card trimming (`trim_alpha_cards`, cost-aware card
trimming), impostor baking (`bake_impostor`), and parallax-corrected impostor reconstruction.

## Verdict

- Exact equivalence with v0.2.17: 0 differences, all features and no default features.
- C++ 1.3 differentials: 0 differences in every family; the protocol output is byte-identical to
  0.2.17's.
- No function is more than 1% slower than v0.2.17: 3,024 samples, worst case 1.0002 (vertex remap,
  default build). Output identity 168/168 against the baseline and 168/168 against C++.
- The new APIs give identical results across native release and debug, std and no_std, and
  wasm32 with and without simd128 (8 hash cases, 8 variants).
- cargo-semver-checks: no semver update required (202 checks passed, 58 skipped), default and all
  features.

## Equivalence (exact drivers, all features / no default features)

| Driver | Observations |
|---|---|
| remap | 10,811,136 / 10,811,136 |
| overdraw | 20,558,672 / 20,558,672 |
| buffer and sequence streams | 164,442,780 / 164,442,780 |

Decode and vertex decode, forced levels and C++ success paths, and the AVX2 and AVX-512 extended
drivers: 0 differences.

## Cost of the new APIs

Best of three on one pinned core, on the test assets of each lane.

| Operation | Time |
|---|---|
| `bake_impostor` | 0.08 to 1.43 s |
| `trim_alpha_cards` | 24 to 213 ms |
| Reconstruction, 3 parallax steps | 2.2 to 3.4 ms per view |

Three parallax steps raise the silhouette overlap (IoU) of a fern impostor at 8 x 8 views from 0.74
to 0.83.

## Fuzzing

Pre-release campaigns on the new targets, run to the release rule (coverage flat for 10 minutes),
0 crashes in the final runs:

| Target | CPU seconds | Executions |
|---|---|---|
| card trimming | 2,994 | 2.0M |
| impostor layout | 1,747 | 2.8M |
| impostor reconstruction | 1,165 | 10.6M |
| impostor baking | 1,776 | 237k |

The campaigns found two defects in card trimming, both fixed before this candidate: a fitted
corner on a triangle outline was shared by unrelated triangles, so folded cards could take the
wrong attributes, and a sliver card at threshold 0 could be reported as trimmed through rounding.

## Not verified

wasm32 timing of the new APIs, other CPU architectures (correctness runs in CI), and fuzzing on the
final candidate commit (the fixes touch card trimming only; its target ran on the fixed code).
