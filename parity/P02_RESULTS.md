# P02 local lane result

PASS: the complete P02 local done-when gates exit 0. The published core remains safe Rust with optional std and pinned libm. No package is published and Moss is not migrated.

| Check | Result |
|---|---|
| Required format/lint/build/test commands | 10 pass; 56 root tests per feature configuration, adapter transport test pass |
| Upstream native and awaited JS fixtures | 287, zero mismatches |
| Malformed streams | 3,637, zero classification/identity mismatches |
| Seeded differential | 2,000 each raw decoder, filter and checked view; 14,000 total, both APIs and executed WASM |
| Complete benchmark corpus identity | 81 inputs, scalar C++/native/WASM exact on both APIs; complete bytes retained |
| ASan/libFuzzer smokes | 13 x at least 300 seconds; 64,266,473 executions; no crashes |
| Geometry regression | 279 cases and 65,543 square-root bit results pass |

| API | Raw Rust/scalar time geometric mean | Maximum | Frozen minima |
|---|---:|---:|---|
| allocating | 1.009760 | 1.375315 | All pass |
| caller_buffer | 0.972634 | 1.266216 | All pass |

Raw scalar limits remain 1.25 geometric mean and 1.50 maximum. The decoder bar was registered before the first candidate measurement; its numeric minima and byte hash remain unchanged. The failed 1.542310 triangle timing and all its samples remain in before-triangle-chunks. The final bounded triangle record writes resolve that miss.

Keep the safe scalar implementation under RFC 6.2. P02_PERFORMANCE.md publishes every absolute throughput, Rust/SIMD ratio and scalar ratio. Oct/Quat timing uses repeated encoded records and exact last-record reuse; varied-filter throughput is not established. Shared-host timings do not establish a quiet-host result or Moss integration performance.

Raw vertex/index/sequence v0/v1 support is separate from the EXT attribute v0 boundary. Parent metadata checks are explicit. Zero Oct normals return a typed numerical error; canonical filters follow strict scalar 1.3, with SIMD conformance checked separately. Raw caller-buffer operations use no heap scratch; output and retained Workspace storage are accounted.

Artifacts: /mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p02. The requested /mnt/linux-extra/meshopt-artifacts/p02 is read-only in this environment; exact placement remains the filesystem residual. The exact isolated build target is removed after archive verification; cleanup.json retains the proof. Git metadata and HEAD remain unchanged.

Local scope is Linux x86-64 plus executed Node WASM. Broader native/platform release qualification, 10,000-per-family release sweeps and the four-CPU-hour-per-target release fuzz gate remain separate. Decisions are in DECISIONS.md; raw records, bytes, samples, corpora, executable copies and source archives are retained outside the build target.
