# Meshlet attribution

Base: 8740a55 (0.2.13). The retained peer executables' source hashes match this
checkout. optimesh 1.1.0 declares MIT in its package manifest; no code was
copied from it. The existing peer report counts the resident call, median of
three. Sampling below uses `instructions:u`, period 1,000,003, on the same
three real inputs. It includes startup, preparation, warmup, serialization
and one measured call; therefore percentages are approximate self-instruction
shares, not exact per-function counts and not the final gate. Reports show no
lost samples; some report out-of-order events. Inlining merges functions into
callers and some C++ startup samples lack symbols.

| Workload | Input | Ours nearest | C++ nearest | optimesh nearest |
|---|---|---:|---:|---:|
| build meshlets | real-214e | 62.24% | 41.24% | 56.75% |
| build meshlets | real-a082 | 61.77% | 57.47% | 53.64% |
| build meshlets | real-b6fd | 45.60% | 56.46% | 51.82% |
| cluster LOD | real-214e | 23.39% | 21.43% | unavailable |
| cluster LOD | real-a082 | 22.14% | 20.83% | unavailable |
| cluster LOD | real-b6fd | 21.14% | 20.62% | unavailable |

Mapped functions: `nearest` (src/meshlet.rs:507), `kdtreeNearest`
(C++ vendor/src/clusterizer.cpp:583), `kdtree_nearest` (optimesh
src/clusterizer.rs:630). `kd_build` (src/meshlet.rs:456) maps to
`kdtreeBuild` (C++ :532) and `kdtree_build` (optimesh :556); on the first
real mesh their sampled shares are 13.70%, 20.80% and 19.71%, respectively.
The flex driver (src/meshlet.rs:1170) maps to C++ :1044 and optimesh :1323;
inlining prevents attributing its subloops independently. On cluster LOD,
nearest is the largest separately resolved function in both implementations;
ours also spends 10.32% in simplify run_state and 8.39% in meshlet optimization
on the first input. The measured C++ partition symbol is 2.40%; no structural
partition change is justified by this attribution.

The peer search takes direct roots and stores separate adjacency/live/used
arrays (optimesh src/clusterizer.rs:312, :630, :1346). Ours combines vertex
state (src/meshlet.rs:307), tracks checked work at each search node and leaf
(src/meshlet.rs:518), and uses conservative bounds plus memoized pinned roots
(src/meshlet.rs:530; src/math.rs:184). These are observed differences, not a
causal proof that any one of them accounts for the peer's aggregate advantage.
The lower-priority search and build functions must be measured after any change.
