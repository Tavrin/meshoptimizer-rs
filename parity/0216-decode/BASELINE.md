# Default-feature baseline

Real-mesh geometric means of per-case medians (three samples). Ratios are Rust / C++ default.

| Decoder | x86 instructions:u | wasm Node wall time (disclosure) |
|---|---:|---:|
| vertex-decode | 1.2566 | 1.3174 |
| index-decode | 1.0242 | 1.3693 |

Every native cell and the scalar C++ arm are in TABLE.md; raw samples and the default wasm adapter identities are retained in receipts/.
