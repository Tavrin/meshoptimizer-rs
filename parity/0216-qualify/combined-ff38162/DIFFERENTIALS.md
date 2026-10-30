# Merged C++ 1.3 differentials

All stages returned 0. The reference is clean at
`4c203430ca565cb59a468a91922c76c208169536`. The bundled meshopt 0.7.0 core
and cluster-LOD header were checked byte-for-byte against that reference;
source hashes are retained in `environment.json`.

| Family / protocol | Cases | Result |
|---|---:|---|
| simplify/update/prune/points, cache, overdraw, fetch protocol | 1,246 per feature mode | equal |
| remap C++ driver | 2,592 per feature mode | equal |
| index-sequence wire driver | 9,990 per feature mode | equal under its wire/error contract |
| meshlets, owned/into | 1,024 | equal across seven clients |
| flexible meshlets, owned/into | 1,024 | equal across seven clients |
| spatial meshlets, owned/into | 2,000 | equal across seven clients |
| spatial finite extremes / signed zeros | 28 | equal across seven clients |
| spatial production meshes | 3 | equal across seven clients |
| cluster bounds | 512 | equal across seven clients |
| meshlet bounds | 512 | equal across seven clients |
| sphere bounds | 512 | equal across seven clients |
| cluster LOD | 24 | equal across seven clients |
| decoder and stride-12 lane matrices | 318,608 × 4 | see malformed-output disclosure |

The seven geometry clients are scalar/default C++ 1.3, no-default Rust with
`clusterlod`, and all-feature Rust forced to Scalar/SSE2/SSSE3/SSE4.1. The
geometry matrix totals 5,636 synthetic messages plus three production spatial
messages. The transport's message cap is raised from 128 to 512 MiB solely to
admit those frozen production records; algorithm settings and comparisons are
unchanged. The synthetic input/output corpus hash and production output hashes
are in `p03-differential.json`.

Each decoder lane ran its original fixed differential matrix with std/SIMD
and no-std/SIMD, forcing all four x86 ceilings. All four receipts report zero
forced-level differences and zero C++ successful-byte differences. Status
agreement is asserted for every message. Each reports **74,863 malformed
failure-write differences** across the scalar/default C++ references; failed
destination prefixes are disclosed rather than claimed byte-identical.
The NEON-lane x86 receipts additionally report zero C++ default/scalar ISA
divergences. See `decode-*/` and `vdecode-*/`.

Repair APIs have no matching C++ 1.3 entry points. Their inherited prerequisites
and new-API corpus properties are covered by [PROOF.md](PROOF.md), along with
parallel-versus-serial identity. Arm64 uses the documented CI rule and retained
source-identical receipts in [CI.md](CI.md).

All peer builds, cost-driver build and C++ differentials together used
**717.1837 CPU seconds**, below the 6,000-second estimate and 7,200-second
maximum. No measurement result is inferred from these proof timings.
