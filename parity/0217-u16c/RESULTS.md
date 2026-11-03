# Concrete struct compatibility qualification

All 23 legacy index-bearing structs remain real, non-generic structs at their original paths, with unchanged fields and attributes. Width-aware APIs retain the separate `*Generic<I>` types. u32 conversions move owned index vectors; borrowed descriptors retain their original constructors and methods. The two LOD chain documentation links resolve.

The required cargo-semver-checks 0.51.0 check against 0.2.16 passes (202 applicable checks). An explicit patch-release check also passes (229 applicable checks). Static inventories preserve all 135 original free-function signatures and six original method signatures. Independent source and rustdoc inventories both count 143 additive width-aware entry points.

The exact pre-u16 test/example sources and original inline test modules compile against the current library in an isolated source-compatibility fixture. Its examples build and doctests pass. Current tests run separately with all features and without default features. The incoming character-metric merge remains included.

Formatting, Clippy with warnings denied, tests, doctests, examples, parallel-only, wasm32, no_std, rustdoc with warnings denied, API/source compatibility, and all 88 u16 corpus comparisons pass. All 22 locked C++ oracle source files match the retained upstream pin.

All 136 cells reproduce baseline/candidate/C++ output exactly. Three interleaved repetitions per Rust arm count resident-loop `instructions:u` with 100% counter coverage. Every cell is within the unchanged 1% ceiling.

| Metric | Result |
| --- | --- |
| Cells within 1% | 136 / 136 |
| Instruction-change range | -0.0351% to +0.2618% |
| Worst cell | tiny-smooth / fetch |
| Worst instructions/call | 3262.54 → 3271.08 |

Per-cell medians, all 816 samples, compiled source/binary identities, semver results, and final source hashes are retained with this report. The baseline is `b89cc4fe03efe346e4695594f1ca4e4688a8c8f1`; all 66 baseline library source files are hash-verified before and after its build. The original qualification receipts remain unchanged in their sibling directories.

All final gate exit codes are 0. The isolated build target and scratch-installed semver checker have been removed.
