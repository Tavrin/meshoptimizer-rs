# RFC 113 clodBuild parity against Moss S2

**Verdict:** all 15 valid layout cases have byte-identical DAG, local indices, hierarchy, cluster and meshlet bounds. All 90 codec payload comparisons match. The nine-mesh paired cook ratio is **1.171×** scalar C++ and **1.159×** Moss-style SIMD C++; the scalar performance bar is met. Stride 32 remains invalid under the exact S2 protect mask.

## Setup and identity

- Worktree branch `parity/rfc113`, HEAD `6747c38b09e042fb212298398230c6584895403c`. No public API changes.
- C++ oracle: Moss S2 vendored `meshoptimizer` (master 717ca348), plus a hash-checked copy of `cluster_lod_bridge.cpp`.
- S2 config: `clodDefaultConfig(128)` with only `simplify_dilate_borders=false`; 3 normal weights of 0.5; protect bits 0..8 (plus 9..12 at stride 64); S2 `boundary_locks`; callback `clodLocalIndices`; hierarchy width 8 and bridge-derived levels.
- S0b inputs: all nine `.mesh` files actually present in the frozen directory (the brief says eight); stride 48 on all, plus 64/72/88 on pyramid and sponza_lionhead. Synthetic tangent/color/joints/weights are deterministic. The 72/88 layouts exercise the builder API although S2 candidate admission rejects skinned bases.
- Copied bridge SHA-256 `4d18edbac8730f2fdab119360c0217cf408aa8c580efa1ab99d57af9026578ba`; vendored `clusterlod.h` SHA-256 `71f95c1f97ef567235540c038b41906091fcef1ef68046f2933cd5989f94ea93`.
- DAG driver SHA-256: scalar C++ `9164100f3fd853cb46ffa0079bcdfae9fc3b3382ec07209c366ee7467161136f`, Moss-style C++ `16aab10d502e76cd35b2e7d2c6037c407726a06a935e577a7d39d74fa9189395`, Rust `895d0e99e4a7bc7f5ce900bc464ce9139deade1fc50f4ffc714599e0d33d4b7a`.
- Codec driver SHA-256: C++ `f0807f35418821a03c3139f1741962c21e4f2e202f17d55455a77ec6daf5b2b6`, Rust `7bb3b832d139caa0f15ed37adf25572c76f7b049e6dc85493ac95925a410b70f`.
- Full source and binary hashes: `/mnt/linux-extra/meshopt-artifacts/rfc113/result.json` and `/mnt/linux-extra/meshopt-artifacts/rfc113/codec.json`. Artifacts contain hashes and timings only; no source mesh copies or large payloads.

## Byte comparison

The complete S2 DAG is the output prefix; a parity-only trailer holds independently recomputed cluster and meshlet bounds. Equality of the full output also covers callback order, group and cluster records, errors, local index mapping, node records, and all floating-point bits. There was no first differing field among valid cases.

| Case | Input SHA-256 | DAG + bounds SHA-256 | Bytes |
|---|---|---|---:|
| `grass_a_01:48` | `4df417b005b75999212d2769df7b03f7a8ded0d4751593b96b59028fde4efad5` | `5e849abff2cc5728a0eab858d8b6eda20988dea13bd164e1c958163cbe6f5489` | 197,536 |
| `grass_b_01:48` | `2d279084f3aa62a72230cee2f1ad74d8fa0ade8b56a56d0494ab9a86702509d9` | `6928fe9d5665b62493ca365f6174a88a62b145a94577d8847569bf4d61ea05f0` | 26,912 |
| `modular_m4x4_03:48` | `7ebc85273af046cae944f48f12ae40b295185befbea2e8dd780c2cb8dae6f60f` | `84a58b016e9517bcc90a55cfb74b3ce3728d925d0667884d074522257b82e2a4` | 4,921,716 |
| `moss_01:48` | `77572a09c39b53e04c767e9ce7c950992c53eed0b42217af8dab2e810c24a670` | `a6b4cfd4b9d96161689a10a6a962036acb50e2a7053bd7483854bc8e445d43b7` | 6,780 |
| `pyramid:48` | `5b65b7ed590e817dc2a14ff355c17310a66805409091426f8db10a09fd088da1` | `1af121a1d2b7942ead4dd3f0f4c3239bc2146c2692190c29d0d25e7a3f3e100d` | 16,312,840 |
| `pyramid:64` | `c7d0b0e594ce733218528bab8d81a308fd906e08344d6c492360bf8f139060dc` | `1af121a1d2b7942ead4dd3f0f4c3239bc2146c2692190c29d0d25e7a3f3e100d` | 16,312,840 |
| `pyramid:72` | `2a44eacb9f689be4acf7cf400c301563a4e20cb5dd820fe8bc8fd594fb690755` | `1af121a1d2b7942ead4dd3f0f4c3239bc2146c2692190c29d0d25e7a3f3e100d` | 16,312,840 |
| `pyramid:88` | `d1aaab0cac5c8d241b2cad9a21f0cd63332ba80d6697497007831a334b2ce1e9` | `1af121a1d2b7942ead4dd3f0f4c3239bc2146c2692190c29d0d25e7a3f3e100d` | 16,312,840 |
| `riverforest_01_branches:48` | `7314d2309a47be398299f41dc5f6571ad5fd83f57910016d471529a6611c1f62` | `16b2e8b0788ab00e1b7d9f0d4210c94628b356eb2387f3d9e1356b7d5e08743c` | 8,410,596 |
| `riverforest_01_leaves:48` | `e4dd70ed0fd6e21fbde41cf0cc1b1d3dbec96db2de90f0dd6e0f226e67b55ef3` | `d28c97da2b7b9a7d0e15a536e902e400bbcad4a6da13362529df74e09798fa39` | 10,363,704 |
| `riverforest_01_trunk:48` | `315704601b714269004ddd7b45bc6b6e8e71e1f9cd79d6f2eb169d46427d43bc` | `e7a210bb829e072537a0cdd6a320a4f7675707a0ab3a1b6810fae9f1c30bd5a7` | 815,316 |
| `sponza_lionhead:48` | `83cecd65fbbd22ac064ac7e49b985d8ad8001511f070fe33dbedd25a0fac44c2` | `de2de6365a96bb3151d2639863c8bc00cdf0dc6045b3046dd621673156aa0c31` | 2,898,400 |
| `sponza_lionhead:64` | `89fa62e378b595d7fe816efeaa371da176dfeea20299d639e372026d934723dd` | `de2de6365a96bb3151d2639863c8bc00cdf0dc6045b3046dd621673156aa0c31` | 2,898,400 |
| `sponza_lionhead:72` | `f77c27d36198b9b280d57abadf7644918777ff50ef0b14c8f56dec6328fa766f` | `de2de6365a96bb3151d2639863c8bc00cdf0dc6045b3046dd621673156aa0c31` | 2,898,400 |
| `sponza_lionhead:88` | `da9b08d66647345d803d02bf89ad9bba77ae3ea9cf4fa81edbbd861cf860b27c` | `de2de6365a96bb3151d2639863c8bc00cdf0dc6045b3046dd621673156aa0c31` | 2,898,400 |

The 32-byte pos/nrm/uv setup is invalid for pyramid and sponza_lionhead. `clodBuild` asserts `attribute_protect_mask < (1 << (vertex_attributes_stride / 4))`; S2 specifies bit 8 even though stride 32 has only eight floats. With assertions disabled, the bit-8 scan reads outside the last vertex. The Rust API rejects the same mask as `InvalidParameter`. No byte parity or timing claim is made for stride 32. Reproduce with `python3 parity/rfc113/run.py --case pyramid:32 --pairs 0`. Resolving this exact-setup contradiction is outside this lane.

## Codec payloads

Each of the 15 valid cases compares the vendored C++ and Rust encoded bytes for IndexBuffer, IndexSequence, VertexBuffer, FilterOct, FilterQuat, and FilterExp: **90/90 identical**, all statuses zero. Both decoders agree on every C++ and Rust encoded stream; lossless vertex and sequence streams round trip. Triangle streams preserve oriented triangles under codec rotation. Vertex/index version and bound queries agree. Per-payload input, encoded, decoded, and bound hashes are in `codec.json`.

## Paired cook time

20 interleaved three-way pairs per mesh on pinned CPU 16; internal monotonic timers cover input conversion and DAG cooking/serialization, excluding process launch, pipe transfer, and parity-only detailed bounds. Scalar C++: `-O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD -fno-fast-math -ffp-contract=off`; Moss-style C++: `-O3 -DNDEBUG -fPIC -ffunction-sections -fdata-sections -m64; SIMD enabled, mirroring cc release defaults`; Rust: `consumer opt-level=3 thin LTO codegen-units=1`. The aggregate is the ratio of summed per-mesh medians.

Load average before/after: `[11.02783203125, 22.4375, 28.513671875]` / `[20.83837890625, 18.6279296875, 25.05126953125]`; core utilization over run: 94.9%.

| Mesh (stride 48) | Groups | DAG depth | Before scalar | Scalar ms | Moss ms | Rust ms | After scalar | Rust/Moss |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `grass_a_01:48` | 4 | 0 | 1.405× | 5.841 | 5.867 | 8.140 | 1.394× | 1.388× |
| `grass_b_01:48` | 2 | 1 | 1.476× | 1.158 | 1.154 | 1.465 | 1.265× | 1.269× |
| `modular_m4x4_03:48` | 134 | 7 | 3.689× | 295.018 | 292.449 | 306.701 | 1.040× | 1.049× |
| `moss_01:48` | 1 | 0 | 1.315× | 0.210 | 0.213 | 0.267 | 1.271× | 1.251× |
| `pyramid:48` | 439 | 5 | 7.888× | 1033.832 | 1037.047 | 1202.409 | 1.163× | 1.159× |
| `riverforest_01_branches:48` | 281 | 12 | 7.911× | 339.804 | 347.382 | 379.397 | 1.117× | 1.092× |
| `riverforest_01_leaves:48` | 300 | 1 | 5.789× | 679.342 | 691.772 | 862.926 | 1.270× | 1.247× |
| `riverforest_01_trunk:48` | 24 | 7 | 1.753× | 40.333 | 42.532 | 48.294 | 1.197× | 1.135× |
| `sponza_lionhead:48` | 89 | 7 | 2.240× | 174.200 | 178.176 | 198.964 | 1.142× | 1.117× |

**Sum of medians:** scalar C++ 2.570 s, Moss-style C++ 2.597 s, Rust 3.009 s; **1.171×** scalar and **1.159×** Moss-style.

## Root cause and scaling

The old five-pair consumer run measured **6.391×** scalar C++ (2.568 s C++, 16.409 s Rust). Its ratio increased with the S2 group count: Pearson correlation **0.960** across the nine meshes, versus **0.503** with DAG depth. The table above uses group counts and depth from the S2 output itself. The read-only S0b C++ Build/RSS tables confirm only the rough ordering of mesh difficulty: they use a different config and single-threaded wall time on a loaded host, with about ±30% variation. They are not timing baselines for these ratios.

Initial `perf record -g` captures put about **79%** of pyramid and **80%** of branches Rust samples in repeated full-source position/attribute validation and per-group simplification setup. Source inspection found that Rust copied every vertex's attributes and rebuilt flags for each group, rescanned every position in each meshlet/bounds call, and initialized source-sized adjacency for small flex groups. Vendored C++ uses sparse adjacency when the source vertex count exceeds group indices. Rust now validates and copies attributes once, reuses flags per level and the validated position magnitude, and compacts sparse groups to local indices before the flex builder, mapping output vertices back to source IDs. The public API and all compared bytes remain unchanged. The last scratch change initializes the compact map directly to its sentinel and reserves/pushes the local arrays without redundant zero fills.

An intermediate 20-pair run after validation/setup fixes but before sparse group compaction measured **1.319×** aggregate and retained 15/15 byte matches; it ran on CPU 0 with concurrent lane activity and is diagnostic only. In the short profiles, Rust `meshlet::adjacency` fell from **16.67% to 0.97%** of pyramid samples and from **15.65% to no samples** in branches after compaction. The final 20-pair ratio no longer grows with group count (Pearson **−0.388**) or depth (**−0.810**). These nine-point correlations are descriptive, not a model of other meshes.

## Stage attribution and SIMD

The table gives representative **self-sample percentages** from `perf record -g` on the same pyramid and branches requests, not whole-stage totals: C++ and Rust inline and split functions differently. The final profiles have 177–186 samples per pyramid build and 74–81 per branches build, so small differences are noisy. The profile binary hashes match the final consumer timing record.

| Mesh / stage | Scalar C++ | Moss-style C++ | Rust |
|---|---|---|---|
| Pyramid: meshlet build/search | `buildMeshletsFlex` 13.3%, `kdtreeNearest` 6.2% | 17.9%, 5.8% | `flex` 15.2%, `nearest` 8.2% |
| Pyramid: simplify | `simplifyEdge` 6.9%, `classifyVertices` 7.4% | 6.1%, 3.8% | `run_state` 13.7%, `quadrics` 5.1% |
| Pyramid: partition | `partitionClusters` 3.1% | 3.4% | `partition::adjacent` 2.4%, wrapper 1.3% |
| Pyramid: bounds | `computeClusterBounds` 3.4% | 3.3% | bounds 1.3%, cluster bounds 0.7% |
| Pyramid: callback / hierarchy | `onGroup` 20.2%, `clodBuild` 7.1% | 15.8%, 6.1% | `output` 12.8%, `position_remap` 1.7% |
| Branches: meshlet build/search | `buildMeshletsFlex` 7.1%, `kdtreeNearest` 9.3% | 8.4%, 8.8% | `flex` 9.4%, `nearest` 6.9% |
| Branches: simplify | `simplifyEdge` 7.6%, `fillAttributeQuadrics` 3.3% | 8.6%, 3.4% | `run_state` 16.3%, `vertex_ids` 3.5% |
| Branches: partition | `partitionClusters` 3.5% | unsampled | `partition::adjacent` 3.3%, wrapper 1.8% |
| Branches: bounds | `computeClusterBounds` 3.8% | cluster bounds 1.9%, sphere 1.9% | bounds 1.4% |
| Branches: callback / hierarchy | `onGroup` 15.8%, `clodBuild` 3.9% | 17.6%, 5.7% | `output` 18.7%, `lock_boundary` 3.4% |

Moss's `build.rs` uses release `cc` with C++17 and leaves SIMD enabled; the second C++ driver mirrors its release flags. Both C++ drivers emitted identical bytes on all 15 valid cases. The scalar/Moss-style medians are 2.570/2.597 s, so there is **no measured SIMD speedup** in this S2 cook. Vendored `clusterizer.cpp` has explicit SSE/NEON box-merge and prefetch code in the **spatial** meshlet builder; S2's `clodDefaultConfig` sets `cluster_spatial=false` and runs the flex builder. Codec SIMD paths are outside the cook timer. Thus no active S2 stage can be assigned an explicit SIMD gap; residual stage differences between these two builds also include different floating-point and `cc` flags and sampling noise. Phase 0.7 can target spatial meshlets and codecs separately, but this S2 path's remaining costs are scalar flex, simplify, and callback work.

## Cargo defaults and correctness gates

The separate 20-pair Cargo release-defaults run was byte-identical on all 15 valid layouts and measured **1.071× scalar C++** and **1.096× Moss-style C++** on summed medians. Its nine scalar per-mesh ratios, in table order, were 1.414, 1.352, 1.070, 1.409, 1.031, 1.124, 1.008, 1.082, and 1.301; Moss-style ratios were 1.390, 1.336, 1.105, 1.371, 1.040, 1.087, 1.042, 1.585, and 1.244. That run selected CPU 23 after a 75.8% pre-run utilization sample, and load average rose from 38.3 to 77.7. Its C++ sum was 4.079 s, versus 2.570 s in the consumer run on CPU 16, so the cross-profile times are load-sensitive; within each run the three drivers were interleaved on one core. The consumer run's pre-run core utilization was 1.0%, with load average 11.0 to 20.8.

All listed commands exited zero on the final source: `cargo fmt --all -- --check`; clippy `-D warnings` with all features and with no default features; tests with all features, no default features, and no default features plus `clusterlod`; the no-default `clusterlod` wasm32 build; RFC 113 byte parity and `codec.py` (90/90); and the existing 0.1–0.4 parity runs and sweeps. Counts and summary hashes are retained in `gates.json`: 0.1 ran 279 fixtures and 10,000 seeded cases; 0.2 ran 287 fixtures, 3,637 malformed inputs and 14,000 seeded cases; 0.3 ran 47 upstream fixtures, 25 generated cases per 15 families, 30,000 seeded cases and 80 C++/native/WASM cluster-LOD cases; 0.4 ran 869 fixtures, 7,653 malformed inputs and 44,000 seeded cases. Every gate reported zero mismatches. Large temporary parity corpora reside in the lane target and are removed with it.

## Reproduction and limits

Run `python3 parity/rfc113/run.py --pairs 20` and `python3 parity/rfc113/codec.py` from the repository root. Use `--case mesh:stride` for one deterministic case. The comparison proves these frozen S0b inputs under the stated S2 setup; it does not qualify a Moss runtime or GPU swap. The 32-byte request needs a corrected upstream setup.

## Artifact inventory

- `/mnt/linux-extra/meshopt-artifacts/rfc113/result.json`: 39,795 bytes; SHA-256 `5314e70a55c6dcdbdb3d1da392b4038fbb54ab7b173413937763b816381c28c4`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/codec.json`: 62,910 bytes; SHA-256 `3c2717ecc3ccb8867cf5458f9715a6855ad299851834c2eb36aa00be5b60a896`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/result-defaults.json`: 39,820 bytes; SHA-256 `76acf6e6cbbdb79c5a0d0352d5dcc4d27257074c9a0dc79e8540f8b814ad182b`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/gates.json`: 4,094 bytes; SHA-256 `5d31bc08e7d0cc719a8dca21524675d5b22d2a4e7a74e2020cd786481ea25740`.
- `profile-identity.json`, `profile-pre-sparse-identity.json`, six final and six pre-sparse `profile-*.txt` call-graph summaries, and `result-pre-sparse.json` are in the same artifact directory. No `perf.data` is retained.
