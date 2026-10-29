# Cache restore: bounded scalar instruction campaign

Pinned input train: 68439a3; baseline: v0.2.14; S5: 5854223;
custom-table behavioral oracle: cc13453. Eight S5 fixtures (five corpus,
three real), verified against its input SHA-256 manifest. No new fixtures.

All builds and measurements run through the installed CPU admission runner,
declaring 4 GB, with an isolated target, incremental compilation disabled,
two compiler jobs, LC_ALL=C, and perf 6.17. All arms disable optional SIMD.
The peer consumer retains the S5 resident warm-workspace allocation policy;
input loading, first output and output serialization occur outside counters.
No wall-time result is used. C++ is meshoptimizer 1.3 via meshopt 0.7.0;
Rust peer is optimesh 1.1.0. Scalar safety refers to the library cache code;
the inherited C++ benchmark consumer remains outside the package.

First compare unchanged base/train/S5/C++/optimesh to attribute the premise.
Bound exploration to two candidate source variants. First variant prevents
cache_loop from being inlined into kernel, isolating its register allocation.
Run one final base/train/candidate/C++/optimesh campaign, median of three
instruction counts per case. Reverse arm order for the middle repetition.
Use 1000 calls for tiny, 3 for medium, 1 for million/real. Reject incomplete
or multiplexed counters (running percentage must be 100). Require identical
output hashes across arms and repetitions. Retain every sample and failure.

SHIPS requires candidate/base real geometric mean <=0.97 and every one of
the eight case ratios <=1.01. Otherwise NO-GAIN: retain the evidence and
leave the library unchanged. Report corpus geometric means separately.
Do not merge, tag or publish.

Proof: S5 exact.rs against v0.2.14, plus custom-table differential against
cc13453 and existing default-table corpus identity, all-features and
no-default-features. Compare full results, errors, Usage and caller buffers
including retained scratch and partial writes across every work limit on
selected small inputs. This is a finite boundary sweep, not exhaustive proof
of all meshes or floats. Run tests, clippy, fmt, source/package boundary,
and public-text before commit. Retain raw evidence outside the repository;
commit portable bindings, counts, results, commands and exit receipts.
