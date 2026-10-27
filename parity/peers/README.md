# Peer measurement consumer

Unpublished isolated crate; no library changes. REPORT.md contains the measured
matrix and source inventory. registry versions and transitive dependencies are
locked. Supply a dedicated CARGO_TARGET_DIR and a scratch directory PEERS_ART.
All build, fixture, validation and measurement commands require the shared
CPU admission runner with <=4 GB declared, GPU disabled and one compiler job.
The runner adapter exports PARITY_RUNNER_ACTIVE and PARITY_RUNNER_GPU from its
admission markers. Private paths and adapter scripts stay in scratch.

1. Under admission, run build.py. It builds ours/cpp/rust separately and copies
   each executable to scratch before building the next feature arm.
2. Set PEERS_INPUTS to the retained attribution input directory; run prepare.py
   under admission. It projects the existing packed input geometry and creates
   both decode fixtures with the new C++ 1.3 arm. No private assets are bundled.
3. Set PEERS_PERF and optionally PEERS_CPU (default 9); run measure.py under
   admission. Reuse the same scratch to resume only missing completed keys.
4. Set PEERS_STAGES to the retained stages manifest; run stages.py and
   measure_stages.py under admission for the eighteen-stage supplement.
5. Run validate.py and validate_stages.py under admission. NumPy is required
   for bounded-memory semantic validation. This cross-decodes all three
   encoders through all three decoders, checks oriented topology and fetch
   reconstruction, and parses complete cluster-LOD callback records.
6. Set PEERS_CPP and PEERS_RUST to the unpacked registry crate roots; run
   inventory.py. The source inventory includes line references and SHA-256s.
   Third-party sources are parsed as data.
7. Run report.py and audit.py under admission. After the final public-text check, receipts.py
   exports hashes of raw evidence and admission receipts. Commit only portable compact
   receipts, counts, identity records, sources' hashes, the consumer and report.
   Keep raw logs/binaries/inputs/outputs and excluded attempts in scratch.
8. Run the repository public-text check, remove only the dedicated target, and
   retain its cleanup receipt before committing and pushing a draft PR.

The C++ arm calls the published crate's public ffi module, except cluster LOD
which uses its public callback wrapper. Idiomatic wrappers are not separately
benchmarked; they may allocate or postprocess differently. Both Rust ports have
SIMD enabled; this is not a comparison of Cargo default feature sets.
REGISTRATION.md fixes settings and the counted allocation policy.
