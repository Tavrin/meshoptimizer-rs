# Phase 0.3

Pure safe Rust translations of the pinned meshoptimizer 1.3 meshlet, partition
and spatial APIs. See the additive inventory in ../COVERAGE.md, decisions
D67–D77 in ../DECISIONS.md, and D78 onward for the performance work. The optional `clusterlod` feature reproduces the
pinned demo rather than a stable upstream output contract.

```sh
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/codex-meshopt-int03
export MESHOPT_ARTIFACTS=/mnt/linux-extra/meshopt-artifacts/p03
export MESHOPT_REFERENCE=/path/to/pinned/meshoptimizer
parity/run.sh --phase 0.3
parity/sweep.sh --phase 0.3 --cases-per-family 2000
python3 parity/p03/runner.py clusterlod
python3 parity/p03/benchmark.py --enforce
python3 fuzz/p03/smoke.py
```

The performance bar is enforced for two Rust profiles of the driver (D78):
the Moss-like consumer profile (thin LTO, `codegen-units = 1`, `opt-level = 3`)
and Cargo's release defaults. Select one with `MESHOPT_RUST_PROFILE`
(`consumer`, `release-defaults`, or `release` for the historical fat-LTO
build); each writes its own `benchmark-<profile>` record:

```sh
MESHOPT_RUST_PROFILE=consumer python3 parity/p03/benchmark.py --enforce
MESHOPT_RUST_PROFILE=release-defaults python3 parity/p03/benchmark.py --enforce
```

The combined-tree integration records detailed parity data in
`/mnt/linux-extra/meshopt-artifacts/p03`. Run
`parity/report.sh --phase 0.3 --verify-artifacts` to verify the slim summaries.
Build and compare jobs share the exact target: run them sequentially so active
reference drivers cannot be overwritten. Fuzz uses its native target subdirectory
and excludes both siblings of the benchmark core.

The test transport is unpublished and not part of the library. A request has a
48-byte little-endian MO03 header, packed positions, global indices, cluster
counts and optional radii. A response has MR03, a meaningful payload length,
payload, per-operation seconds and requested heap bytes. WASM repeats are zero.
Descriptor fields, used unpadded triangle bytes, vertex/index ordering and every
meaningful bounds float bit/signed byte are compared, excluding struct padding.

Records retain all source/executable identities, exact fixture inventory,
length-delimited inputs/expected outputs and their hashes. The sweep exercises
all allocating/caller-buffer outputs; all compared implementations must agree
before the expected bytes enter the corpus. Fixed stack storage, input transport,
caller destinations and allocator overhead are excluded from heap limits and
memory ratios on both sides. Default limits are 1 GiB owned output plus live
scratch/retained capacity and 2^34 record visits; D68 defines counted visits.
The shell sweep rejects fewer than 2,000 cases per family. Direct transport
diagnostics with shorter corpora cannot establish the sweep gate; the independent
evidence verifier checks every required operation, variant and case count.

`benchmark.py --quick` and `--families` are diagnostics only. The enforced full
matrix is 624 rows per profile, uses at least 20 interleaved same-core pairs,
increased to 30 when quartiles cross a bar, and has unchanged
GM <=1.25, maximum <=1.50 and heap <=1.25 bars per family/baseline. It reports
shared-load dispersion without claiming quiet qualification. SIMD output
identity is recorded separately; exact parity uses scalar-strict C++.

The cargo-fuzz smoke requires instrumentation counters, successful final stats,
unchanged source/binaries and at least 300 elapsed seconds per target. It uses
coverage plus trace compares and sanitizer none. It does not establish the RFC
release's four CPU-hours per target, 10,000-case sweeps, sustained nightly/weekly
runs, ARM execution, other-platform acceptance or Moss integration.
