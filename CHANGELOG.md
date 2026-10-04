# Changelog

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
