# RFC 113 cluster-LOD performance recovery

Implementation: `9589e0cab7fdf8eef50654a744a6f05a20a01047`; baseline: `3db42ccc81ecc9ac5e21d181d3deac735573f711`.

**Verdict: PASS.** Both correctness fixes remain intact. Ratios below compare safe scalar Rust to scalar C++ on the frozen S2 meshes. Consumer aggregate **1.196×**; Cargo-defaults aggregate **1.090×**.

| Mesh (stride 48) | Consumer / scalar | Defaults / scalar | Consumer / Moss | Defaults / Moss | Pairs consumer/defaults |
|---|---:|---:|---:|---:|---:|
| `grass_a_01` | 1.163× | 1.212× | 1.174× | 1.222× | 5/5 |
| `grass_b_01` | 1.080× | 1.234× | 1.235× | 1.269× | 5/5 |
| `modular_m4x4_03` | 1.123× | 1.178× | 1.064× | 1.145× | 5/5 |
| `moss_01` | 1.195× | 1.315× | 1.274× | 1.388× | 5/8 |
| `pyramid` | 1.147× | 1.000× | 1.141× | 1.079× | 5/5 |
| `riverforest_01_branches` | 1.269× | 1.108× | 1.144× | 1.133× | 5/5 |
| `riverforest_01_leaves` | 1.260× | 1.228× | 1.227× | 1.219× | 5/5 |
| `riverforest_01_trunk` | 1.197× | 1.254× | 1.238× | 1.228× | 9/6 |
| `sponza_lionhead` | 1.189× | 1.088× | 1.186× | 1.090× | 5/5 |

Moss-style aggregate: 1.160× consumer, 1.128× defaults. Scalar sum-of-medians seconds (C++ / Rust): 1.869586 / 2.235926 consumer; 2.363221 / 2.575799 defaults.

## Root cause and design

D149 restored bounded scratch by copying and zero-initializing all source attributes and converting all source locks before every sparse group simplification. This made setup scale with source vertices times group count. D150 is irrelevant to these S2 timings because dilation is disabled in that setup.

The recovery borrows the validated attribute view and stores the existing boundary locks directly in the private transparent one-byte flag type. The transient boundary marker is removed before borrowing those flags. The two per-group copies are removed. Existing boundary storage keeps its original lifetime, and attributes stay borrowed from the caller. No added heap allocation or changed arithmetic is required. Attribute layouts, explicit byte order, full-source validation, counted work and fallible child-budget accounting remain intact. Dilation still unconditionally invalidates the moderate-position fast path.

The diagnostic perf captures preceded implementation and were repeated afterward on exactly the same four affected inputs. They were collected under recorded shared-machine load while timing admission was blocked by a GPU lease, and establish stage attribution only. The exact baseline/optimized binaries, raw captures and core/load records are retained.

| Mesh | Wrapper before/after (%) | Bulk zeroing before/after (%) |
|---|---:|---:|
| `pyramid` | 45.94 / 2.86 | 9.60 / 1.19 |
| `riverforest_01_branches` | 46.24 / <0.3 reported threshold | 9.50 / 0.53 |
| `riverforest_01_leaves` | 43.30 / 1.92 | 8.39 / 0.95 |
| `modular_m4x4_03` | 25.98 / 5.06 | 5.01 / 0.83 |

## Correctness and memory

- All 15 supported RFC 113 layouts match scalar C++, Moss-style C++ and Rust byte for byte, including bounds; all 90 codec comparisons match. The two historical stride-32 setups remain invalid under S2’s bit-8 protect mask, as documented in the original report.
- All 12 archived-main/current/C++ differential cases match. The moving near-threshold case moves 16 vertices; threshold crossing is not claimed. Unconditional invalidation remains established directly by the unchanged source.
- All 517 byte limits pass the old-success/no-error-kind-regression checks. Old peak 34,904 bytes; current 24,576 bytes. Removing the two allocations and replacing u8 locks with an identically sized type preserves the general heap bound; no cache/high-water allocation is introduced.
- fmt, clippy with all features and no default features (-D warnings), and both test configurations pass. The new multi-level test compares output, mutated positions, work and exact byte boundaries across packed/padded floats and unaligned little-/big-endian bytes, with all three flag kinds, zero weights and both dilation settings.
- 0.1–0.4 fixture and 2,000-case-per-family seeded gates pass with executed wasm32 identity, including all 80 C++/native/WASM cluster-LOD cases. Source manifests were independently rechecked in gate-audit.json.

## Timing method and stopping evidence

Only the four affected meshes received five diagnostic interleaved pairs during iteration. The final verdict uses one fresh adaptive pass per profile. It starts at five pairs, stops on a nominal paired-log 95% t interval wholly within/above 1.5, and caps at twenty. Only an unresolved interval receives thirty fresh D146 pairs. Sequential intervals guide stopping; they do not establish simultaneous confidence coverage. Aggregates retain stage-1 medians and the original 1.2 aggregate / 1.5 per-mesh bars.

Every pair was admitted with no GPU lease holder and no scoreboard measurement. A scoreboard with only a sleeping child was allowed. Drivers were pinned to a sampled physical core; both siblings and load are recorded for each burst. Load has no admission threshold. New blockers close all drivers after the current pair and before waiting. Bursts cap at 840 seconds before another pair, followed by cooldown. The retained measure-on-release.py adapter verifies the prepared source/binary manifest before using cached builds and watches holder process exits with pidfd notifications. A release triggers a fresh gate check; periodic failed checks retain the 300-second cadence. It acquires no lease and changes neither timers nor sampling. Its own source hash and prepared-builds.json hash are included in each result identity.

| Profile | Mesh | Stage-1 interval | Verdict | Stage 2 |
|---|---|---|---|
| consumer | `grass_a_01:48` | [1.145, 1.300] | PASS | not needed |
| consumer | `grass_b_01:48` | [0.889, 1.352] | PASS | not needed |
| consumer | `modular_m4x4_03:48` | [1.008, 1.257] | PASS | not needed |
| consumer | `moss_01:48` | [0.973, 1.391] | PASS | not needed |
| consumer | `pyramid:48` | [1.121, 1.184] | PASS | not needed |
| consumer | `riverforest_01_branches:48` | [1.135, 1.308] | PASS | not needed |
| consumer | `riverforest_01_leaves:48` | [1.068, 1.363] | PASS | not needed |
| consumer | `riverforest_01_trunk:48` | [0.965, 1.448] | PASS | not needed |
| consumer | `sponza_lionhead:48` | [0.994, 1.335] | PASS | not needed |
| defaults | `grass_a_01:48` | [1.146, 1.412] | PASS | not needed |
| defaults | `grass_b_01:48` | [1.142, 1.343] | PASS | not needed |
| defaults | `modular_m4x4_03:48` | [1.093, 1.256] | PASS | not needed |
| defaults | `moss_01:48` | [1.184, 1.500] | PASS | not needed |
| defaults | `pyramid:48` | [0.612, 1.393] | PASS | not needed |
| defaults | `riverforest_01_branches:48` | [1.002, 1.340] | PASS | not needed |
| defaults | `riverforest_01_leaves:48` | [1.188, 1.326] | PASS | not needed |
| defaults | `riverforest_01_trunk:48` | [0.985, 1.495] | PASS | not needed |
| defaults | `sponza_lionhead:48` | [1.084, 1.093] | PASS | not needed |

consumer: load before/after `[19.24658203125, 25.49609375, 24.546875]` / `[12.98779296875, 15.2861328125, 17.1357421875]`; longest active burst 22.728 seconds; source/binary identities and sibling utilization are retained in the result JSON.

defaults: load before/after `[12.98779296875, 15.2861328125, 17.1357421875]` / `[38.47021484375, 26.0087890625, 23.1162109375]`; longest active burst 23.405 seconds; source/binary identities and sibling utilization are retained in the result JSON.

## Identity and reproduction

Consumer Rust SHA-256 `afcb4e17f22160cb0becb173f3183abda690d4c63e6627beb0462950930f6625`; defaults Rust `5e54510ac2bc79a583f6adea9200f70d7d7fd55fde9172c708823c14a4c927b5`; scalar C++ `9164100f3fd853cb46ffa0079bcdfae9fc3b3382ec07209c366ee7467161136f`; Moss-style C++ `16aab10d502e76cd35b2e7d2c6037c407726a06a935e577a7d39d74fa9189395`.

Artifacts: `/mnt/linux-extra/meshopt-artifacts/clodrec`. Source archives, perf captures, raw pairs, protocol output hashes, fixtures/sweeps and validation logs are retained. The frozen input meshes are referenced by hashes rather than copied. These results establish this corpus and Linux x86-64/WASM parity; no Moss runtime or GPU integration claim is made.

```sh
export CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/codex-meshopt-clodrec
export MESHOPT_RFC113_ART_DIR=/mnt/linux-extra/meshopt-artifacts/clodrec-new
python3 parity/rfc113/timing.py
```

Use a fresh artifact directory for a new final pass. `--profile consumer --pairs 5 --case pyramid:48` selects a short diagnostic. The runner builds current binaries before admission. Correctness commands and exits are retained in gates.json; D153–D154 record the scoped decisions.

## Artifact hashes

- `result.json`: SHA-256 `1284fa471c057e5ee9be9a6df61efdbc46c8aa7d5a1e4f7cf796179e05a8059a`.
- `result-defaults.json`: SHA-256 `d7b5853ae0d22ded6f3ca94e2e47285b56fb0d24641cb0b85f1a2a7486fa5e33`.
- `diagnostic-consumer.json`: SHA-256 `c7819a9f3fa81983487338996bedcd6f88761814e820d31b0c706bd1fafe8125`.
- `codec.json`: SHA-256 `7ccf92274a5a89a095338c470bf15cabd5cef8627e26f408d56d65f61bf790d3`.
- `differential.json`: SHA-256 `c4e35638c3e9361fde561f873206f268794bba041e5d4b3c2d8c3c391ddd83cb`.
- `budget.json`: SHA-256 `bce094232a85293deb83e5c6628dc5f1e215537b64bc6ed6714fccfa85da6127`.
- `gates.json`: SHA-256 `7bad67f972464cf54bf69b5ac2d0aea0b9b9055c2095d297053aa589a036bcf4`.
- `gate-audit.json`: SHA-256 `504ffd35079df0a4334de70ec073b98077471d4e0986f800b1e980202f6dacfd`.
- `baseline-source.json`: SHA-256 `9e8974ba413eff5d47932a39b70640d205786ad11f87270a388b010425b276f3`.
- `baseline-profile.json`: SHA-256 `9d469e1ba719871f40c30038e7b2d0cff57711a88d75e6b6a157bb8068ba88e7`.
- `optimized-profile.json`: SHA-256 `a48d02302ec02cb21896066f6839717f0227c3be6d1c85c64671eab0764fd5a3`.
- `stage-attribution.json`: SHA-256 `d3bd21015463e412632c367bb95b72de0e8bf81a9b349a1e09e5458e49a80730`.
- `timing-early-admission.jsonl`: SHA-256 `fa9b13d606b56a4cb65ad87a52a0eaac4180a7be351bdd5151e773e821479387`.
- `measure-on-release.py`: SHA-256 `803c07a37aa1c6f892bea56a1c00f1278fe60f3a1930dc849a49aee15de47e61`.
- `prepared-builds.json`: SHA-256 `9b75cfd8872778370f9499acf0caed2ea0647f564a2b68e30ccb8a31223e273a`.
- `implementation-source.tar`: SHA-256 `db17b72e825e55f7ff8105b41eab0a305c39bdbd2b0e1a73899eda9c3d4aca6e`.
- `baseline-source.tar`: SHA-256 `bf86fb79f2ebcc15834c3cfd5b0e864163c3419a6bf21751defa217bab52ac4b`.

The required Cargo target was deleted after verification. Twenty required executable identities remain archived and verified. Cleanup SHA-256 `c918cd8c69827262ca4aef24cca0567678ba331c3010d625ea937e840ddd4c82`; binary inventory SHA-256 `7a43719a62ee3cf8e1597d4b36e91174f314084e832b2c8c42efd1e64711dc67`.
