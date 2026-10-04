# Changelog

## 0.4 codec completion (unreleased milestone)

- Add vertex-buffer, index-buffer and index-sequence encoders with their bound
  functions and explicit per-call `VertexEncoding` / `IndexEncoding`
  configuration (versions 0/1, vertex levels 0-9), matching C++ 1.3 bytes.
- Add Oct, Quat, Exp (all four exponent modes) and Color filter encoders, and
  raw Color filter decoding outside the EXT helper.
- Add the meshlet codec: `encode_meshlet`, its bound, and both decoding forms
  (`decode_meshlet` with 2/4-byte vertices and 3/4-byte triangles, and
  `decode_meshlet_raw`), each with caller-buffer variants.
- Extend the codec harness to phase 0.4: upstream native and JS encoder
  vectors, malformed inputs, 2,000-case sweeps per operation, two-way
  cross-decoding, executed WASM identity, nineteen fuzz smokes and per-family
  benchmarks under the Moss and Cargo-default profiles.

## 0.1.0 (unreleased)

- Add the foundation API: checked strided inputs, vertex flags, errors,
  workspace storage and work limits, and `no_std` with `alloc`.
- Translate the standard vertex-cache and overdraw optimizers from
  meshoptimizer 1.3 with allocating, caller-buffer and in-place variants.
- Add offline C++ differential, wasm32 identity, seeded sweep, stable fuzz
  smoke and recorded single-thread benchmark tooling.

- Add `simplify`, `simplify_with_attributes`, their caller-buffer variants,
  `simplify_scale`, settings, result-error output and stable option masks.
- Support LOCK/PROTECT/PRIORITY, weighted attributes, Permissive, LockBorder,
  absolute error and both regularization strengths.
- Extend exact C++/wasm32 qualification to simplification and upstream fixtures.
- Remove hostnames from records and retain large buffer archives externally.
- Store per-function qualification summaries in the repository, with SHA-256
  identities for external per-case records and buffers. Verify available
  artifacts and identify absent historical evidence explicitly.
- Measure release performance with fat LTO, Moss's thin-LTO consumer profile,
  and Cargo release defaults; keep the RFC acceptance bars unchanged.
- Add CPU-accounted release fuzzing with at most eight workers, retained replay
  corpora, crash evidence and resumable source-bound records.
- Set the owner-amended release robustness budget to four CPU-hours per target.
- Add a local corpus continuation with a wall budget, core cap, low scheduling
  priority and SIGTERM handling, plus weekly and manual CI continuation.
- Add exact Rust execution checks on macOS arm64, Windows x86-64 and Linux
  arm64 against qualified native outputs recorded on Linux x86-64.
