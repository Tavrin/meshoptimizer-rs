# Registration: 0.2.15 toward C++ (vertex cache, vertex decode, meshlets, cluster LOD)

Fixed before any campaign result.

Arms: base (v0.2.14), candidate, M7 (C++ 1.3 through meshopt 0.7.0), optimesh
1.1.0 (no cluster LOD). Peer consumer: the existing `parity/peers` driver, with
`parity/0215-contract-meshlets/prepare_driver.py` applied (adds
`meshlets-flex`; counting only). Ops: `cache` (`optimize_vertex_cache`),
`vertex-decode`, `meshlets` (`build_meshlets`, 64/124, cone weight 0),
`meshlets-flex` (`build_meshlets_flex`, 64/41/124, weight 0, split 2) and
`cluster-lod` (default config 128). Workloads: the frozen five-mesh corpus and
the three real meshes (`inputs.json`). Metric: median of three
`instructions:u` samples per cell, 100% counter running, CPU 9, interleaved
arm order (reversed on the second repetition). No wall timings.

Verdict per op on three-real-mesh geometric means:

- SHIPS if candidate/base <= 0.97 and candidate/C++ is at least 0.05 below
  base/C++ (the spec replaces the optimesh condition for these functions,
  which already lead optimesh);
- IMPROVES if only candidate/base <= 0.97;
- NO-GAIN otherwise.

Any case with candidate/base > 1.01 is fixed or its change reverted. Candidate
and base outputs must be byte-identical in every cell. No code edits after a
campaign's results except reverts; at most two campaigns.
