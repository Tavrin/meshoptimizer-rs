# Generic u16 index qualification

Sealed `Index` implementations for `u16` and `u32` share the existing u32 kernels. The u32 adapter borrows input and moves owned output; u16 adaptation is fallible, storage-limited, and returns `Error::IndexOverflow` for an unrepresentable output. Remap sentinels, face identifiers, offsets and packed local triangle words retain their documented metadata types. Byte decoders retain explicit element-width parameters and reject overflowing two-byte output.

The all-feature public API has **143 migrated entry points**: 135 generic free functions, two aliases and six methods. Independent enumeration found 137 generated rustdoc function pages plus six methods. [API inventory](api.json) records both counts. Rustdoc and `docs/USING.md` explain type selection and integer literal annotations; the changelog records the addition.

## Results

- All required gates exited **0**, including all-feature/no-default tests, doctests, clippy with warnings denied, formatting, native no_std and wasm32 no_std builds, rustdoc and the public-text guard. [Gate receipt](gates.json).
- Four frozen inputs representable as u16 passed **88** narrow/wide comparisons, including simplification, generated topology, meshlets, cluster LOD, fetch and codecs.
- All **136** performance cells (eight inputs, 17 operations) produced exact baseline/candidate/C++ output. Three interleaved repetitions per Rust arm used resident-loop `instructions:u` counters at 100% event coverage. Every cell is within the unchanged 1% limit.
- Worst increase: `tiny-smooth` / `meshlets-spatial`, **47,015.17 → 47,038.90 instructions/call (+0.0505%)**. The full range is -0.2262% to +0.0505%. [Per-cell medians](performance.json), [raw samples](samples.jsonl).

The baseline is commit `b89cc4fe03efe346e4695594f1ca4e4688a8c8f1`; all 66 baseline Rust source files matched that commit before comparison. The compiled C++ oracle's 22 source/header files match the pin `4c203430ca565cb59a468a91922c76c208169536` exactly. [Binary/source identities](build-binding.json), [C++ identities](cpp-source-binding.json), [inputs](inputs.json), [final source identities](source-binding.json).

The first complete campaign exposed a point-simplification code-generation regression. Appending the new error variant preserves established error ordering and removed that regression without changing the point kernels. Its failed receipts and frozen binaries remain retained separately; the final complete campaign above qualifies the corrected source. No algorithms or Cargo features were added or changed.
