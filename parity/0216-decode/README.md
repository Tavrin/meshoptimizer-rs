# Decoder lane

- PROOFS.md: identity argument, limitations, verdict and proof exits.
- BASELINE.md and TABLE.md: default-build measurements and final source.
- TUNING.md: rejected stride-12 experiment.
- REGISTRATION.md: unchanged gates and coverage.
- receipts/: text evidence, source/binary hashes and build manifests.

Builds remain externally admitted. Set CARGO_TARGET_DIR, DECODE_ART, DECODE_INPUTS, PEERS_PERF and MESHOPT_REFERENCE explicitly. Baseline extraction uses read-only git archive; no integration commands are part of this lane. canonical_fresh.sh forces recompilation and applies the fixed verdict before proofs. The coordinator owns Git integration and native arm64 CI acceptance.
