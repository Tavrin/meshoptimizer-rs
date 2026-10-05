# RFC 113 clodBuild parity against Moss S2

Historical review-fix result below. The subsequent recovery meets both performance bars; see [the recovery report](rfc113-clod-recover.md) and D153–D154.

**Verdict:** all 15 valid layout cases have byte-identical DAG, local indices, hierarchy, cluster and meshlet bounds. All 90 codec payload comparisons match. The nine-mesh paired cook ratio is **2.383×** scalar C++ and **2.399×** Moss-style SIMD C++; the consumer scalar performance bar is not met. Cargo-defaults scalar aggregate is **2.389×** (not met). Stride 32 remains invalid under the exact S2 protect mask.

## Setup and identity

- Worktree branch `parity/rfc113`, HEAD `36c4504bf8cfb9f56373fc9654519e3911439304`. No public API changes.
- C++ oracle: Moss S2 vendored `meshoptimizer` (master 717ca348), plus a hash-checked copy of `cluster_lod_bridge.cpp`.
- S2 config: `clodDefaultConfig(128)` with only `simplify_dilate_borders=false`; 3 normal weights of 0.5; protect bits 0..8 (plus 9..12 at stride 64); S2 `boundary_locks`; callback `clodLocalIndices`; hierarchy width 8 and bridge-derived levels.
- S0b inputs: all nine `.mesh` files actually present in the frozen directory (the brief says eight); stride 48 on all, plus 64/72/88 on pyramid and sponza_lionhead. Synthetic tangent/color/joints/weights are deterministic. The 72/88 layouts exercise the builder API although S2 candidate admission rejects skinned bases.
- Copied bridge SHA-256 `4d18edbac8730f2fdab119360c0217cf408aa8c580efa1ab99d57af9026578ba`; vendored `clusterlod.h` SHA-256 `71f95c1f97ef567235540c038b41906091fcef1ef68046f2933cd5989f94ea93`.
- DAG driver SHA-256: scalar C++ `9164100f3fd853cb46ffa0079bcdfae9fc3b3382ec07209c366ee7467161136f`, Moss-style C++ `16aab10d502e76cd35b2e7d2c6037c407726a06a935e577a7d39d74fa9189395`, Rust `e5fc4bea87a836340791844ee5935053aa9a18866e46f528a48bc9c8206f0aab`.
- Codec driver SHA-256: C++ `f0807f35418821a03c3139f1741962c21e4f2e202f17d55455a77ec6daf5b2b6`, Rust `8243bf4a9fcd8315efab0337a1aae525d2c67bd7c62157507695628da2829758`.
- Full source and binary hashes: `/mnt/linux-extra/meshopt-artifacts/rfc113/result.json` and `/mnt/linux-extra/meshopt-artifacts/rfc113/codec.json`. The timing and codec JSON retain hashes and timings without source mesh copies or large output payloads.

## Review-fix differential and memory sweep

- Archived `main` `a4b3c483e38c2484232c4f5b748bf0fbb0fff182` versus current Rust and vendored C++: 12 generated cases, all byte-identical. The moving near-threshold case changed 16 vertices.
- One-triangle, 32-attribute byte sweep: 517 limits; old peak 34,904 bytes, current peak 24,576 bytes; no old-success regression or shared failure-kind mismatch.
- Full records: `/mnt/linux-extra/meshopt-artifacts/rfc113/differential.json` and `/mnt/linux-extra/meshopt-artifacts/rfc113/budget.json`.

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

5–20 stage-1 interleaved three-way pairs per mesh, pinned to a sampled physical core within each burst; internal monotonic timers cover input conversion and DAG cooking/serialization, excluding process launch, pipe transfer, and parity-only detailed bounds. Scalar C++: `-O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD -fno-fast-math -ffp-contract=off`; Moss-style C++: `-O3 -DNDEBUG -fPIC -ffunction-sections -fdata-sections -m64; SIMD enabled, mirroring cc release defaults`; Rust: `consumer opt-level=3 thin LTO codegen-units=1`. The aggregate is the ratio of summed per-mesh medians.

Load average before/after: `[8.59716796875, 15.67626953125, 20.5595703125]` / `[15.29296875, 15.35888671875, 17.90283203125]`; core utilization over run: 93.0%.

| Mesh (stride 48) | Groups | DAG depth | Before scalar | Scalar ms | Moss ms | Rust ms | After scalar | Rust/Moss |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `grass_a_01:48` | 4 | 0 | 1.405× | 4.502 | 4.491 | 5.374 | 1.194× | 1.197× |
| `grass_b_01:48` | 2 | 1 | 1.476× | 1.055 | 1.072 | 1.299 | 1.232× | 1.213× |
| `modular_m4x4_03:48` | 134 | 7 | 3.689× | 225.262 | 227.574 | 351.115 | 1.559× | 1.543× |
| `moss_01:48` | 1 | 0 | 1.315× | 0.197 | 0.201 | 0.249 | 1.263× | 1.242× |
| `pyramid:48` | 439 | 5 | 7.888× | 868.677 | 846.067 | 2394.767 | 2.757× | 2.830× |
| `riverforest_01_branches:48` | 281 | 12 | 7.911× | 281.055 | 286.499 | 728.967 | 2.594× | 2.544× |
| `riverforest_01_leaves:48` | 300 | 1 | 5.789× | 432.363 | 435.999 | 1042.847 | 2.412× | 2.392× |
| `riverforest_01_trunk:48` | 24 | 7 | 1.753× | 25.219 | 25.071 | 33.208 | 1.317× | 1.325× |
| `sponza_lionhead:48` | 89 | 7 | 2.240× | 146.671 | 144.853 | 171.825 | 1.172× | 1.186× |

The “Before scalar” column contains historical pre-optimization ratios. The final verdict uses the new adaptive samples.

**Sum of medians:** scalar C++ 1.985 s, Moss-style C++ 1.972 s, Rust 4.730 s; **2.383×** scalar and **2.399×** Moss-style.

## Cargo-defaults paired time

| Mesh (stride 48) | Rust/scalar | Rust/Moss |
|---|---:|---:|
| `grass_a_01:48` | 1.317× | 1.290× |
| `grass_b_01:48` | 1.266× | 1.294× |
| `modular_m4x4_03:48` | 1.653× | 1.636× |
| `moss_01:48` | 1.344× | 1.392× |
| `pyramid:48` | 2.702× | 2.642× |
| `riverforest_01_branches:48` | 2.569× | 2.581× |
| `riverforest_01_leaves:48` | 2.461× | 2.479× |
| `riverforest_01_trunk:48` | 1.404× | 1.410× |
| `sponza_lionhead:48` | 1.204× | 1.174× |

Cargo-defaults aggregate: **2.389×** scalar, **2.365×** Moss-style; bar not met.

## Owner-revised method and final checks

Admission requires no GPU lease holder and no scoreboard measurement. Load has no threshold. A scoreboard unit whose only child is `sleep` is waiting and is allowed; every check retains its MainPID and child process tree. A new blocker lets the current three-way pair finish, then all driver processes close before waiting. Failed checks poll every five minutes.

As recorded in D152, stage 1 starts with five interleaved pairs per mesh. The nominal two-sided 95% Student-t interval of paired log(Rust/scalar-C++) ratios is checked after each pair: stop once wholly within or over 1.5, cap at 20. Sequential intervals guide stopping and do not claim simultaneous confidence coverage. Only an interval still overlapping 1.5 at the cap receives D146 stage 2: 30 fresh pairs, df=29, upper bound ≤1.5 to PASS, lower bound >1.5 to FAIL, overlap INCONCLUSIVE and treated as FAIL. Stage 2 clears only a case maximum; all aggregate ratios retain stage-1 medians. The 1.2× aggregate and 1.5× per-mesh bars are unchanged.

Each stage-1 burst selects the least-busy physical core using a one-second sample of both siblings, excluding physical cores containing CPUs 0/1. D146 stage 2 keeps one selected physical core for its 30 fresh pairs, reopening on the same core after any admission pause. All three drivers share the selected CPU. Drivers close after each case and before every pause. Burst admission caps active work at 840 seconds before a new pair and then releases the core for a 60-second cooldown. Observed load, sibling utilization, source/binary identities, raw samples and pair boundaries are retained in JSON.

The earlier fixed-20 consumer run was interrupted by the binding sampling change. Its partial printed figures in `final-timing.log` are diagnostic and excluded; these records are the one complete final adaptive matrix under each profile. No implementation or committed harness source changed in this continuation. Only parity summaries, DECISIONS, and this report are uncommitted.

Fixes: `dab5e47` restores lazy per-simplification attribute/flag scratch; `demo_single_cluster_unused_attributes_do_not_consume_workspace` and the 517-limit old/current sweep pass. `2e2c568` invalidates the position-range fast path after dilation; `demo_dilation_near_position_range_threshold` and three-way near-threshold output/status checks pass. The moving case does not cross `1e8`; unconditional invalidation is established by source inspection. `36c4504` adds 12 old/current/C++ cases, makes Moss-only mismatch fatal, and creates the codec target before compilation; both harness tests pass.

Final-HEAD fmt, all-features/no-default-features clippy (`-D warnings`) and tests exited zero in the prior session; commands and completion output are in `package-checks.json`. All 0.1–0.4 runs and sweeps report zero mismatches, including wasm32 identity and 80 cluster-LOD C++/native/wasm32 cases. Refreshed summary hashes and verified source manifests are in `gates.json`.


### consumer opt-level=3 thin LTO codegen-units=1 stopping evidence

| Mesh | Stage-1 pairs | Nominal interval | Stop | Stage 2 |
|---|---:|---|---|---|
| `grass_a_01:48` | 5 | [1.123, 1.327] | PASS | not needed |
| `grass_b_01:48` | 5 | [1.207, 1.277] | PASS | not needed |
| `modular_m4x4_03:48` | 20 | [1.439, 1.611] | BORDERLINE | FAIL [1.606, 1.652] (30 fresh) |
| `moss_01:48` | 5 | [1.082, 1.358] | PASS | not needed |
| `pyramid:48` | 5 | [1.967, 2.754] | FAIL | not needed |
| `riverforest_01_branches:48` | 5 | [2.509, 2.599] | FAIL | not needed |
| `riverforest_01_leaves:48` | 5 | [2.294, 2.585] | FAIL | not needed |
| `riverforest_01_trunk:48` | 5 | [1.258, 1.486] | PASS | not needed |
| `sponza_lionhead:48` | 5 | [1.119, 1.223] | PASS | not needed |

Observed longest burst: 40.9 seconds. Each `measurement_bursts` record contains selected logical/physical core, before/after load and utilization for both siblings.

### Cargo release defaults: opt-level=3, 16 codegen units, no LTO stopping evidence

| Mesh | Stage-1 pairs | Nominal interval | Stop | Stage 2 |
|---|---:|---|---|---|
| `grass_a_01:48` | 5 | [1.239, 1.441] | PASS | not needed |
| `grass_b_01:48` | 5 | [1.255, 1.321] | PASS | not needed |
| `modular_m4x4_03:48` | 5 | [1.622, 1.695] | FAIL | not needed |
| `moss_01:48` | 10 | [1.268, 1.499] | PASS | not needed |
| `pyramid:48` | 5 | [2.171, 3.119] | FAIL | not needed |
| `riverforest_01_branches:48` | 5 | [2.525, 2.626] | FAIL | not needed |
| `riverforest_01_leaves:48` | 5 | [2.324, 2.627] | FAIL | not needed |
| `riverforest_01_trunk:48` | 5 | [1.316, 1.472] | PASS | not needed |
| `sponza_lionhead:48` | 5 | [1.163, 1.223] | PASS | not needed |

Observed longest burst: 34.3 seconds. Each `measurement_bursts` record contains selected logical/physical core, before/after load and utilization for both siblings.

## Reproduction and limits

The committed `run.py --pairs 20` command is the historical fixed-count runner. The adaptive final method is `/mnt/linux-extra/meshopt-artifacts/rfc113/early-timing.py` (archive the existing admission/log files before a new final run), with D152 admission and stopping rules. Run `python3 parity/rfc113/codec.py` for codec parity. Use `--case mesh:stride` for one deterministic case. The comparison proves these frozen S0b inputs under the stated S2 setup; it does not qualify a Moss runtime or GPU swap. The 32-byte request needs a corrected upstream setup.

After target cleanup, rebuild the two timing profiles first from the repository root:

```sh
CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/codex-meshopt-clodfix python3 -c "import sys; sys.path.insert(0, 'parity/rfc113'); import run; run.build('consumer'); run.build('release')"
```

Then run the adaptive artifact script using the recorded D152 policy. This preparation does not collect timings.

## Artifact inventory

- `/mnt/linux-extra/meshopt-artifacts/rfc113/result.json`: 86,007 bytes; SHA-256 `9afd1d7238f0130f72e57e780e67b8cd8e85b3d79c81b7c0967943c748b72dd8`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/codec.json`: 63,418 bytes; SHA-256 `930dd5dafd40a9ac3b1417912ac43f72309228a049fdea96be35e5a6decda721`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/result-defaults.json`: 79,260 bytes; SHA-256 `167edd2665e35a30df89da3f5e0fa162690e72ed01f25cab4bb1f5019614c7ed`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/differential.json`: 11,860 bytes; SHA-256 `ad6b74848c45c6de9a3b55a64ed019feb6e2e693a3c7d1565de7bb5f0e770126`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/budget.json`: 653 bytes; SHA-256 `b4b21aac36d5e97399e4095bc6ca38b4445322a546c696263994b3667670385d`.

- `/mnt/linux-extra/meshopt-artifacts/rfc113/gates.json`: 9,101 bytes; SHA-256 `e6704913d6993179361e3e2f8342f6e5a2c5ff2020646d50c8bb8965fbc7bf06`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/package-checks.json`: 18,288 bytes; SHA-256 `826f8c56bee4870600c8c7942d1fb338204857ae695227e7322356ac33125aec`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/timing-early-admission.jsonl`: 442,500 bytes; SHA-256 `693bf135a01979ae064e1dfa32ddca2e8328badc76a44e0711b68bf7f303c524`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/early-timing.py`: 10,657 bytes; SHA-256 `a213b657ab07e57778306b2f54015d89a493ee33efca7ab4dc2cc6791da631ef`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/early-timing.log`: 6,224 bytes; SHA-256 `481fbfa037c92208ef80aafcddfbc2bff20816248b60f89ae875abbe5751c777`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/report-early.py`: 12,391 bytes; SHA-256 `3e43434191c1c944632c93017fc8cbf88413d2c96f72027e1ad651a54b8c9e89`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/finish-early-report.py`: 6,581 bytes; SHA-256 `1420c0f011ed0980d228868f3bd9f7fd8b6aae8981afb96f42bf77ac1a455b69`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/final-audit.json`: 4,323 bytes; SHA-256 `fc1616ff4160b4e72362e6a893748f69cbaa89ee0d900f6b79a1222e70ff9aa1`.

The designated Cargo target `/mnt/linux-extra/moss-cargo-targets/codex-meshopt-clodfix` was deleted after source and binary verification, as required by the spec. Retained JSON manifests preserve the checked identities; reproduction requires rebuilding the drivers as described above.
