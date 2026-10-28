# optimize_overdraw: lower check and accounting cost

## Result

Campaign 1, `instructions:u`, median of three samples per arm. Base is v0.2.14.

| Case | candidate / base | candidate / optimesh 1.1.0 | candidate / C++ 1.3 |
|---|---:|---:|---:|
| tiny-smooth | 0.973 | 0.867 | 0.771 |
| medium-smooth | 0.955 | 0.872 | 0.987 |
| medium-seams | 0.943 | 0.864 | 0.991 |
| medium-sparse | 0.903 | 0.849 | 0.978 |
| million-smooth | 0.959 | 0.872 | 0.988 |
| real-214e | 0.741 | 0.880 | 1.088 |
| real-a082 | 0.759 | 0.887 | 1.091 |
| real-b6fd | 0.759 | 0.887 | 1.091 |
| three production LOD-chain meshes, geometric mean | 0.753 | 0.884 | 1.090 |

Verdict under the pre-registered gate: SHIPS (candidate/optimesh ≤ 0.99 and candidate/base ≤ 0.97; no case above 1.01 against base). Output bytes matched base in all 96 samples. The first measurement run stopped before writing a record because the performance tool printed a decimal comma under the host locale; it was resumed with `LC_ALL=C`, with the runner, binaries and inputs unchanged.

## Techniques

Only `src/overdraw.rs` changed; no `unsafe`, no unchecked indexing, no API change. Proofs are in `PROOFS.md`.

- F1 (lines 115–127): packed positions are summed without per-value checks; a finite sum proves every coordinate finite, otherwise the checked scan is replayed.
- F2 (lines 216–238): a cluster score checks 3 values instead of 13, because non-finite values propagate to the dot product. Only a reciprocal can turn an infinity back into a finite value, so its inputs are checked directly.
- F3 (line 315): the soft-cluster target check is skipped on the prepaid path, where the threshold bound already rules out failure.
- W (lines 422–460, 525–531): when positions are packed and the budget covers every remaining charge, geometry and emission charge exact totals. The sort and copy code moved unchanged into emission. A one-triangle cluster uses a fixed three-index copy.

Attribution is taken from the earlier contract-cost experiment, not re-measured here: per-step finite checks were 68% of the real-mesh gap to C++, work accounting 11.5%. Only the combined change was measured.

## Proof

Every step exited 0:

- tests with all features and with no default features;
- clippy with `-D warnings`, fmt, the unsafe-boundary check and the public-text check;
- exact v0.2.14-versus-candidate comparison: 20,558,816 observations each under std and no_std, covering every work limit through success+1, every entry point and layout, and the real meshes;
- `exact-extra.rs`: 136,288 observations per mode, 77 of them failing cases;
- pinned C++ differential: 178 equal;
- fuzz: 2,906 seeds over 300 CPU-seconds.

Not verified: the per-technique split of the gain; non-packed storage receives only F2 and its speed was not measured; the coverage argument assumes `usize` is at most 64 bits.
