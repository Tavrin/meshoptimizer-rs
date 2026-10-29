# Decoder lane

- PROOFS.md: identity argument, limitations, verdict and proof exits.
- BASELINE.md and TABLE.md: default-build measurements and final source.
- TUNING.md: rejected stride-12 experiment.
- REGISTRATION.md: unchanged gates and coverage.
- receipts/: text evidence, source/binary hashes and build manifests.

Builds remain externally admitted. Set CARGO_TARGET_DIR, DECODE_ART, DECODE_INPUTS, PEERS_PERF and MESHOPT_REFERENCE explicitly. Baseline extraction uses read-only git archive; no integration commands are part of this lane. canonical_fresh.sh forces recompilation and applies the fixed verdict before proofs. The coordinator owns Git integration and native arm64 CI acceptance.

On arm64 the decoder differential asserts Rust NEON equals Rust scalar for
every status and destination byte, and uses C++ scalar for status and
successful-output parity. Inherited C++ failure-write differences are counted.
C++ NEON/default divergences from C++ scalar are receipt disclosures only;
x86 still requires zero C++ ISA divergences. See PROOFS.md for native CI
counts and an example malformed stream.
