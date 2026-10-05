# P06 parallel batches

Enable `parallel` for `meshoptimizer_rs::parallel`; it enables `std` and pins
Rayon 1.11.0. Cluster DAG/forest batches also require `clusterlod`. See the
module rustdoc and README for descriptors, ordered errors and per-item budgets.
The default and no-default dependency trees still contain only libm.

## Reproduce

```sh
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p06
export MESHOPT_ARTIFACTS=/mnt/linux-extra/meshopt-artifacts/p06
export MESHOPT_REFERENCE=/home/etienne/dev/refs/meshoptimizer
export CARGO_BUILD_JOBS=2
cargo test --offline --locked --all-features --test parallel -- --test-threads=1
cargo build --offline --locked --release --manifest-path parity/p06/Cargo.toml
python3 parity/p06/measure.py
# If admission pauses it, resume the same source/binary epoch:
python3 parity/p06/measure.py --resume
```

The existing phase 0.1/0.1.x, codec 0.2/0.4 and meshlet/cluster-LOD 0.3
harnesses run unchanged, with records isolated under the external p06 directory.
Phase 0.5 is not in the published 0.1.0 baseline or its dispatcher; this lane
cannot establish that unmerged phase's parity. It does not alter p05.

The nine integration tests compare every meaningful output byte and float
bit at 1, 2, 8 and N threads (N = 32 on this host). They cover both simplifiers,
attributes/flags, all four meshlet builders, mixed vertex/triangle/sequence
encoding, EXT filters and errors, cluster DAGs, hierarchy fields and dilation
including an asserted `LimitExceeded` after curved-boundary position mutation,
with successful neighbours and repeated calls at every registered thread count.
Hierarchy tests cover both sides of the small-batch dispatch cutoff. Tight byte/work budgets, failure isolation, empty
input, scheduling histories, custom pools and concurrent global-pool calls are
included. Whole-chain tests enforce cumulative work and live output storage.
Default/no_std builds exclude the batch module; WASM compile checks also pass
with all features, but parallel WASM execution is not qualified.

## Measured speed curve

One final curve, five interleaved sequential/parallel Rust pairs per row, with
one warm-up per API. The owner specified short, adaptive measurement; p06 has
no hard bar, so no extra pairs or D146 second stage are needed. No iterative
or legacy timing matrices were run. Every measured output is compared exactly
outside timing and retained with SHA-256. The single burst lasted **56.93 s**,
releasing worker processes after every case. Every admission before/after each
pair found no GPU lease holder; the running scoreboard wrapper had only a
`sleep` child. The driver pins its serial caller and each pool worker to recorded
physical cores selected by one `/proc/stat` interval. Build jobs are capped at 2.

CPU: AMD Ryzen 9 7945HX, 16 physical cores / 32 logical CPUs. One-minute load
was **15.46–18.00**: these are shared-machine observations. Corpus: sixteen
deterministically translated/scaled variants of the pinned upstream
`demo/pirate.obj`, **2,889 vertices / 5,010 triangles per mesh**. Pirate is by
Clint Bellanger, [CC-BY-SA 3.0](https://opengameart.org/content/pirate); the asset
is external, not republished in this crate. This is a single authored model,
not a diverse asset suite or a Moss cook benchmark. The driver uses Cargo's
release defaults (opt-level 3, 16 codegen units, LTO false).

Parsing, input generation, pool creation, serialization and hashing are outside
timing. Validation, required copies, result/scratch allocation and execution
are inside. Cluster-LOD timing includes fresh position copies on both APIs.
Encode runs three views per mesh; decode runs one attribute view per mesh.
Hierarchy input groups are prebuilt before timing. Speed-up is the median of
five **paired sequential seconds / parallel seconds** ratios; values below
one mean batching is slower. The serial milliseconds column is the median
baseline in the one-thread row; all raw per-row baselines are retained.

| Family | Serial ms | 1 thread | 2 | 4 | 8 | 16 |
|---|---:|---:|---:|---:|---:|---:|
| LOD chains | 32.898 | 1.026× | 2.001× | 3.211× | 6.216× | 8.201× |
| Mixed encoding | 3.421 | 1.207× | 2.173× | 3.453× | 5.445× | 7.599× |
| EXT view decoding | 0.715 | 1.193× | 2.094× | 3.031× | 4.261× | 3.284× |
| Standard meshlets | 30.022 | 1.112× | 1.553× | 4.102× | 3.872× | 6.136× |
| Cluster LOD | 126.427 | 0.991× | 1.911× | 3.913× | 5.505× | 7.818× |
| Hierarchy forests | 0.086 | 0.845× | 0.817× | 0.852× | 0.988× | 0.623× |

The small hierarchy workload does not amortize dispatch. More threads also
reduce decoding's gain after eight. Dispersion is material: at sixteen threads,
cluster LOD ranges **1.82–10.23×** and meshlets **4.71–9.81×** across the five
pairs. These medians are not universal gains or C++ algorithmic comparisons.
The full curve, CPU affinities, load, GPU/scoreboard observations, raw pairs,
input/source/dependency/binary identities and serialized output hashes are in
`/mnt/linux-extra/meshopt-artifacts/p06/parallel/curve.json`.

## Final gate record

Fmt, both strict Clippy modes, all/no-default tests, the feature-only parallel
tests, strict rustdoc, both WASM builds and driver checks pass. Rust 1.88 tests
also pass. Every available legacy exact fixture/sweep gate is green, including
C++ and executed WASM. The external verifier checks current source/dependency
hashes, every ZIP buffer/member, full corpus inventories and the timing curve.
The slim [MEASURED_P06.json](../MEASURED_P06.json) points to its retained records.
Phase 0.5 remains the explicit missing integration prerequisite; no 0.5, release
fuzz, historical performance-bar, remote CI or Moss integration pass is implied.


## Review fix follow-up (2026-10-05)

[D150](../DECISIONS.md) records the hierarchy dispatch profile and late-mutation
witness. Small hierarchy batches now run sequentially within the API (<=256 total
groups and <=2048 group-by-level visits); larger batches retain Rayon. This is a
conservative heuristic covering the measured family, not an optimal crossover.
The new failure witness asserts changed position bits and `LimitExceeded`, plus
two successful neighbours, at 1/2/8/N threads over repeated calls.

The hierarchy-only diagnostic retained five pairs at 1/2/4/8 threads before a
new GPU lease paused it; its 16-thread row was not measured. Dispatch alone cost
14–19 us against 53–64 us serial work. The final check retained five pairs at
1/2/4/8/16 threads, with paired medians **0.878/0.836/0.775/0.545/0.725x**.
Caller-side `pool.install` remains timed, including its scheduling overhead;
these residual slowdowns are retained rather than claimed as gains. Both bursts
lasted under eight seconds; admission observations and source/binary identities
are in `/mnt/linux-extra/meshopt-artifacts/p06fix/{before,after}/parallel/curve.json`.
The earlier full-family curve and qualification record above are historical.
