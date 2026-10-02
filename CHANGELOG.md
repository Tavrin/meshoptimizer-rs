# Changelog

## Unreleased

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
