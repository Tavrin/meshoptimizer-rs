# P07 registered SIMD bars

Registered before candidate measurements on phase/0.7, base a4b3c483e38c2484232c4f5b748bf0fbb0fff182.
Oracle: meshoptimizer 1.3, 4c203430ca565cb59a468a91922c76c208169536.

| Bar | Cases | Geometric mean time Rust/C++ | Worst case |
|---|---|---:|---:|
| S1 | vertex v0/v1, views NONE, meshlet decode | 1.10 | 1.30 |
| S2 | varied filters, filtered views | 1.25 | 1.50 |
| S3 | each selected SIMD level / scalar Rust; varied filters only | no regression beyond paired 95% uncertainty | same |
| S4 | index/sequence | existing scalar 1.25 | 1.50 plus P02 minima |
| S5 | wasm simd128 / upstream shipped simd decoder | 1.25 | 1.60 |
| P1 | >=4 independent views, eight threads / serial C++ | 0.50 | report eight-thread C++ and wasm workers |

Linux x86-64 host: Ryzen 9 7945HX; generic C++ -O3 -DNDEBUG,
-fno-fast-math -ffp-contract=off, upstream SIMD enabled. Rust release,
fat LTO, one codegen unit, runtime detection, no target-cpu=native.
AArch64 dedicated host is unavailable: publish shared-CI ratios without gating.
Node version and hardware are recorded at execution. AVX2/AVX-512 deferred.

Corpus: the complete 81-case P02 matrix (parity/codec/measure.py corpus plus
candidate additions); both allocating and caller-buffer APIs. Meshlets: 1/1,
64/126 and 256/256 vertex/triangle records, vertex widths 2/4 and triangle
widths 3/4. Varied filters: seed 20261005, 17/4097/262145 records,
Oct strides 4/8, Quat stride 8, Exp strides 4/12/32, Color strides 4/8;
random directions/quaternions/exponents/colors encoded by the pinned oracle;
include EXT views for Oct/Quat/Exp. Inputs are generated and hashed before
any timing. Repeated Oct/Quat inputs remain reported, outside S3.

Binding owner timing directive 2026-10-05 supersedes amendment sample counts:
iteration only touched cases, five interleaved pairs; final full matrix once,
start at five pairs, stop on a paired 95% log-ratio interval wholly within or
over the applicable worst-case bar, cap at twenty. Borderline cases alone
receive D146's thirty fresh pairs and Student-t interval; inconclusive fails.
All raw samples retained, order rotated, one pinned core, telemetry before
and after each pair. Each timed burst <15 minutes, releasing cores between
bursts. Admission fails closed on a GPU lease or active moss-scoreboard unit;
a pair overlapping either is discarded and retained as interrupted evidence.
Family means and S3 are independently reported, never cleared by a case retry.
Missing platform/safety/release records prevent a release qualification claim.
