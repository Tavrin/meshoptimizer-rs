# RFC 113 clodBuild parity against Moss S2

**Verdict:** all 15 valid layout cases have byte-identical DAG, local indices, hierarchy, cluster and meshlet bounds. All 90 codec payload comparisons match. The adoption condition is **not met**: stride 32 is invalid under the exact S2 protect mask, and the nine-mesh paired cook ratio is **6.391×** C++ (bar ≲1.2×).

## Setup and identity

- Worktree branch `parity/rfc113`, HEAD `dee68822de94c9c8e7a49e58e2bb7d6350eafb3c`. No crate API or algorithm changes.
- C++ oracle: Moss S2 vendored `meshoptimizer` (master 717ca348), plus a hash-checked copy of `cluster_lod_bridge.cpp`.
- S2 config: `clodDefaultConfig(128)` with only `simplify_dilate_borders=false`; 3 normal weights of 0.5; protect bits 0..8 (plus 9..12 at stride 64); S2 `boundary_locks`; callback `clodLocalIndices`; hierarchy width 8 and bridge-derived levels.
- S0b inputs: all nine `.mesh` files actually present in the frozen directory (the brief says eight); stride 48 on all, plus 64/72/88 on pyramid and sponza_lionhead. Synthetic tangent/color/joints/weights are deterministic. The 72/88 layouts exercise the builder API although S2 candidate admission rejects skinned bases.
- Copied bridge SHA-256 `4d18edbac8730f2fdab119360c0217cf408aa8c580efa1ab99d57af9026578ba`; vendored `clusterlod.h` SHA-256 `71f95c1f97ef567235540c038b41906091fcef1ef68046f2933cd5989f94ea93`.
- DAG driver SHA-256: C++ `9164100f3fd853cb46ffa0079bcdfae9fc3b3382ec07209c366ee7467161136f`, Rust `ea57e711f0d4c0dd20b247a9513d6a6788856ad9c657400c7927c98074f230a5`.
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

Five interleaved C++/Rust pairs per mesh on pinned CPU 0; internal monotonic timers cover input conversion and DAG cooking/serialization, excluding process launch, pipe transfer, and parity-only detailed bounds. C++: `-O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD -fno-fast-math -ffp-contract=off`; Rust: `consumer opt-level=3 thin LTO codegen-units=1`. The aggregate is the ratio of summed per-mesh medians.

| Mesh (stride 48) | C++ median ms | Rust median ms | Rust/C++ |
|---|---:|---:|---:|
| `grass_a_01:48` | 4.802 | 6.747 | 1.405× |
| `grass_b_01:48` | 1.476 | 2.178 | 1.476× |
| `modular_m4x4_03:48` | 239.193 | 882.442 | 3.689× |
| `moss_01:48` | 0.220 | 0.289 | 1.315× |
| `pyramid:48` | 1038.914 | 8194.792 | 7.888× |
| `riverforest_01_branches:48` | 326.855 | 2585.827 | 7.911× |
| `riverforest_01_leaves:48` | 735.217 | 4256.099 | 5.789× |
| `riverforest_01_trunk:48` | 28.541 | 50.020 | 1.753× |
| `sponza_lionhead:48` | 192.371 | 430.869 | 2.240× |

**Sum of medians:** C++ 2.568 s, Rust 16.409 s; **6.391×**, fails the ≲1.2× adoption bar.

## Reproduction and limits

Run `python3 parity/rfc113/run.py --pairs 5` and `python3 parity/rfc113/codec.py` from the repository root. Use `--case mesh:stride` for one deterministic case. The comparison proves these frozen S0b inputs under the stated S2 setup; it does not qualify a Moss runtime or GPU swap. The 32-byte request needs a corrected upstream setup, and the cook-time bar needs separate performance work.

## Artifact inventory

- `/mnt/linux-extra/meshopt-artifacts/rfc113/result.json`: 24,903 bytes; SHA-256 `ab84efb39afff6dd161d6c23b8d5688591060fc1523f37ba5f736f8cc7c71187`.
- `/mnt/linux-extra/meshopt-artifacts/rfc113/codec.json`: 62,909 bytes; SHA-256 `48f39890acb694e964d97da3705593ff0f95b1525ede92dbb9aa84116679b5ad`.

