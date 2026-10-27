# Unresolved CI registration

The draft PR's boundary job failed with exit 1 at
`parity/simd/check_boundary.py:54`: the new unpublished C++ oracle consumer
`parity/peers/src/main.rs` is not in `PEER_FFI_DRIVERS`.
The public-text check passed immediately before that failure.

The existing allowlist contains the compare, encoder and 028-L1 oracle
consumers. The new consumer calls the same published C++ ffi API and uses
Linux perf control descriptors. Its unsafe code is confined to this
unpublished executable; the packaged library is unchanged.

The spec restricts lane files to `parity/peers/`. The allowlist is outside
that scope. `boundary-registration.patch` is an exact, unapplied integration
patch. It registers this one executable using the existing policy. It does
not change the library's audited SIMD block inventory or the independent
package audit. The CI failure remains unresolved until that patch is applied
by an owner authorized to change the checker.

Failed check: https://github.com/Tavrin/meshoptimizer-rs/actions/runs/40550436656/job/122352375158

The private raw job log is retained; receipts.json binds its SHA-256.
No measurement, threshold, library source, or boundary-checker source was
changed to clear this failure. Do not interpret the measurement audit as a
passing repository boundary gate.
