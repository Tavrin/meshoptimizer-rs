# Final qualification

Verdict: **SHIPS**. The registered thresholds are unchanged.

| Operation | Real / base | Corpus / base | Real / scalar C++ | Real / default C++ | Worst / base |
|---|---:|---:|---:|---:|---:|
| cache | 1.000000 | 1.000000 | 1.254056 | 1.254056 | 1.000000 |
| cluster-lod | 0.999981 | 0.999948 | 1.189170 | 1.189170 | 0.999986 |
| fetch | 1.000000 | 1.000000 | 0.951871 | 0.951871 | 1.000001 |
| index-decode | 1.000000 | 1.000000 | 1.024228 | 1.024228 | 1.000000 |
| index-encode | 1.000000 | 1.000000 | 0.770780 | 0.770780 | 1.000000 |
| meshlets | 1.000000 | 1.000003 | 1.001222 | 1.001222 | 1.000015 |
| meshlets-flex | 1.000000 | 0.999994 | 1.001812 | 1.001812 | 1.000001 |
| meshlets-spatial | 0.937251 | 0.952499 | 0.904651 | 1.082772 | 1.000185 |
| overdraw | 1.000000 | 1.000020 | 1.090151 | 1.090151 | 1.000111 |
| remap-index | 0.884625 | 0.882055 | 1.437443 | 1.437443 | 0.884663 |
| remap-vertex | 0.779320 | 0.802185 | 1.086538 | 1.086538 | 0.925220 |
| simplify | 1.000000 | 1.000008 | 1.136595 | 1.136595 | 1.000041 |
| simplify-attributes | 1.000000 | 1.000012 | 1.095286 | 1.095286 | 1.000062 |
| simplify-points | 1.000000 | 0.999996 | 1.013923 | 1.013923 | 1.000013 |
| simplify-prune | 1.000000 | 0.999995 | 0.995169 | 0.995169 | 1.000178 |
| simplify-sloppy | 1.000000 | 0.999992 | 1.275422 | 1.275422 | 1.000003 |
| simplify-update | 1.000000 | 0.999996 | 1.280125 | 1.280124 | 1.000001 |
| vertex-decode | 0.910983 | 0.879082 | 0.340020 | 1.144703 | 0.929216 |
| vertex-encode | 1.000000 | 0.999994 | 0.554267 | 0.554267 | 1.000001 |

Wall duration disclosure (real meshes; geometric means of per-case median ratios):

| Decoder | / base | / scalar C++ | / default C++ |
|---|---:|---:|---:|
| vertex-decode | 1.029276 | 0.381883 | 0.801101 |
| index-decode | 0.897311 | 1.144685 | 1.185729 |

Final proof: 21 steps, all exit 0. Including the rejected first experiment, targeted proof CPU is 1300.64 seconds, below 7,200 seconds.

TABLE.md discloses every median and wall duration. Wall time does not determine admission. The complete guard uses the same canonical build path, four required arms, eight fixtures, 19 operations and three interleaved samples. The separate control is measured for the two decoders only. Every sample has 100% counter running time. Rust base/candidate output hashes agree per cell; both C++ decode output hashes also agree.

Other-operation ratios include the starting integration changes and serve as guard evidence.

The original generic vertex dispatcher and slice monomorph remain unchanged. The owned-only selector instantiates existing stride-12 kernels behind a separate non-inlined owned entry. Earlier generic outlining and associated-constant experiments are rejected and retained under receipts/screen and receipts/screen2.

Exact resource-contract tests compare 5,114,700 observations per feature mode (owned/into, full buffers and tails, errors and Usage, every small-fixture work limit, byte boundaries, cold/warm/after-failure workspaces). Native forced-level differential coverage is 318,608 cases per std/no-std mode. Successful C++ bytes and statuses agree. Each mode records 74,863 inherited C++ failure-prefix differences; Rust malformed-write identity is established against the original Rust scalar and v0.2.15 contracts, not against upstream staging behavior.

All-feature/no-default/no-std SIMD tests, Clippy, formatting, independent unsafe boundary, pinned reference, package verification, unsafe-free check, Miri subset, fixed 20,000-run ASan fuzz and public-text checks pass. This lane changes x86 only; it makes no new NEON/wasm execution or performance claim.

Raw artifacts, frozen source trees and executables remain outside the repository. Public receipts bind source, binaries, inputs and individual samples. The dedicated build target is removed after retention. Git integration and publication remain operator-owned.
