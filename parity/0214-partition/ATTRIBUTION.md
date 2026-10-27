# Instruction attribution before implementation

Base `bd0c891` and scalar C++ 1.3 were built with thin LTO and debug information. Five frozen corpus cases were profiled on one CPU with `instructions:u`, first with a resident-loop `perf stat` and then `perf record`/`report`/`annotate`. All ten valid records and both builds exited 0; each sampling check found no lost or throttled records. The first `perf record` attempt exited 1 with a poll-file-descriptor mismatch and is retained as rejected evidence. The repaired recording includes setup and teardown, so its sampled shares are diagnostic rather than exclusive function counts or campaign verdicts. In particular, input preparation is visible in the largest case. Inline line attribution can duplicate periods and does not establish an isolated cost.

| Frozen case | Base instructions/call | M7 instructions/call | Base / M7 |
|---|---:|---:|---:|
| tiny smooth | 10,005 | 7,337 | 1.364 |
| medium smooth | 2,041,822 | 1,615,454 | 1.264 |
| medium seams | 1,584,381 | 1,261,714 | 1.256 |
| medium sparse | 2,190,111 | 1,641,930 | 1.334 |
| million smooth | 273,218,093 | 205,425,422 | 1.330 |

These are single diagnostic samples, not gate medians. Their serialized outputs match across arms.

Rust's `adjacent` accounts for about 30–40% of sampled process instructions on the medium and million cases; `partition_clusters`, including filtering, geometry and the merge loop, accounts for about 31–48%. `pick` is about 2–10%. On the million case the hottest resolved Rust source locations include the filtering table lookup at `src/partition.rs:439` and adjacency slot handling at `src/partition.rs:262`. Upstream's inlined `meshopt_partitionClusters` is dominated by the corresponding filtering and adjacency construction (`partition.cpp:42,171`); debug inline stacks prevent assigning its full symbol to one function. The root backend and `pick` remain relevant, but the sampling does not justify ranking them above adjacency/filtering for these inputs. Root hit/miss rates have not been measured.

The rewrite order follows the observed hotspots: preserve filtering order and charges while exposing common indexed accesses, then reduce checked adjacency work only when an upper bound and budget prove it safe. The smaller but repeated `pick` share also motivates specialized scoring and a proven shared-sum bound, with the checked fallback retained. No allocation is added or removed; no error, accounting or partial-write boundary is intentionally moved. Actual results require the registered complete campaign and differential proof.
