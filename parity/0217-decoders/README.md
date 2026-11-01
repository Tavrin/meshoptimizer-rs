# Decoder qualification

The oracle remains meshoptimizer 1.3 at
`4c203430ca565cb59a468a91922c76c208169536`; the existing reference and
exact-output assertions are unchanged. `receipts/exits.json` records gate
exits. `identity.json`, `differential.json`, and `wasm.json` record identity
and timings when those steps complete.

The AVX-512 vertex group follows upstream `SIMD_AVX`: 128-bit VBMI
multishift and VBMI2 byte expansion. Runtime detection requires the complete
feature subset, including OS register-state support. Register-only assembly
preserves the Rust 1.88 minimum. Input/output windows and scalar error
semantics remain checked. The AVX2 diagnostic ceiling uses the existing
SSSE3 codec kernels; it does not designate a separate AVX2 implementation.
The pinned index and filter decoders have no AVX-512 specialization to port.

Run all builds and measurements through the required external CPU admission
runner with its declared reservation and fresh external target directory.
Required environment variables are `CARGO_TARGET_DIR`, `MESHOPT_REFERENCE`,
`DECODE_ART`, `DECODE_INPUTS`, `DECODE_MANIFEST`, and `MESHOPT_ARTIFACTS`.
The input manifest is the retained eight-case peer corpus. Keep `perf` and
`cargo-semver-checks` on `PATH`, and use `LC_ALL=C`. No commands in this lane
change Git state; semver's baseline is extracted with read-only `git archive`.

The wasm wall comparison reuses the existing asserting Node harness. It
uses `hrtime.bigint`, calibrates batches toward 150 ms for the fastest
arm, rotates after/C++/before order, discards two warmup rounds, and retains
seven rounds per corpus case. The first short-batch fast-path screen is
retained separately and is not a performance verdict. Copies and allocating output are included for all
arms. The reused harness's `scalar` label means the **before SIMD artifact**
in this comparison. Report medians per case and aggregate ratios separately;
these finite measurements do not establish a universal speed bound.
