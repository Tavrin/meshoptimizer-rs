# Bet 0 peer screen — pre-build safety stop

Baseline O: meshoptimizer-rs 0.2.4, commit `775ff8cab4fb2ba01966acd1b277aa7019eb8937`. Peers: M7 meshopt 0.7.0; X optimesh 1.1.0. Branch: bench/peer-screen.

**Decision: STOP; build nothing.** meshopt's unconditional cc::Build::compile invokes an archiver (`ar`), outside the spec's compiler-or-rustc process allowlist. This is a literal admission failure, not a malicious-code finding. The optimesh default/SIMD build script is a no-op; its oracle branches and meshopt's optional source-tree binding writes are documented separately. Every resolved build.rs was read; no build scripts or proc macros executed.

Correctness labels for both peers: **not checked**. No output exists to classify as identical, different but valid, or invalid. No timing samples exist. F: **not measured**. No registered performance verdict can be applied.

| Peer | Families | GM peer/O | 95% interval | Max case | Verdict / status |
|---|---|---|---|---|---|
| M7 | Every existing harness family; real simplify/overdraw on 214e, b6fd, a082; added cluster LOD/browser scope | — | — | — | UNMEASURED: mandatory safety stop |
| X | Same complete family inventory, including meshlets, cluster LOD and 42-stream wasm SIMD decode | — | — | — | UNMEASURED: mandatory safety stop |

The per-operation inventory below applies to both peers, both native consumer profiles and planned scalar/SIMD configurations. Capability availability was not established by execution. The spec instructs M7 cluster LOD to be unavailable; however the downloaded 0.7.0 build.rs includes src/clusterlod.cpp and clusterlod bindings, so that assumption is flagged for owner review rather than silently tested or changed.

**Does X beat ours anywhere? Undetermined.** No measured cluster-LOD or wasm-decode comparison, quality comparison, or speed verdict exists. No historical results are substituted.

## Full unmeasured family inventory

- `vertex-decode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `vertex-encode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `vertex-encode-v0` — M7 and X unmeasured; mandatory pre-build safety stop.
- `vertex-encode-default` — M7 and X unmeasured; mandatory pre-build safety stop.
- `index-decode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `index-encode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `sequence-decode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `sequence-encode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `oct-decode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `quat-decode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `exp-decode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `color-decode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `oct-encode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `quat-encode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `exp-encode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `color-encode` — M7 and X unmeasured; mandatory pre-build safety stop.
- `cache` — M7 and X unmeasured; mandatory pre-build safety stop.
- `cache-strip` — M7 and X unmeasured; mandatory pre-build safety stop.
- `cache-fifo` — M7 and X unmeasured; mandatory pre-build safety stop.
- `overdraw` — M7 and X unmeasured; mandatory pre-build safety stop.
- `fetch` — M7 and X unmeasured; mandatory pre-build safety stop.
- `fetch-remap` — M7 and X unmeasured; mandatory pre-build safety stop.
- `simplify` — M7 and X unmeasured; mandatory pre-build safety stop.
- `stripify` — M7 and X unmeasured; mandatory pre-build safety stop.
- `unstripify` — M7 and X unmeasured; mandatory pre-build safety stop.
- `meshlets` — M7 and X unmeasured; mandatory pre-build safety stop.
- `meshlets-scan` — M7 and X unmeasured; mandatory pre-build safety stop.
- `meshlets-flex` — M7 and X unmeasured; mandatory pre-build safety stop.
- `meshlets-spatial` — M7 and X unmeasured; mandatory pre-build safety stop.
- `vertex-version` — M7 and X unmeasured; mandatory pre-build safety stop.
- `index-version` — M7 and X unmeasured; mandatory pre-build safety stop.
- `vertex-bound` — M7 and X unmeasured; mandatory pre-build safety stop.
- `index-bound` — M7 and X unmeasured; mandatory pre-build safety stop.
- `sequence-bound` — M7 and X unmeasured; mandatory pre-build safety stop.
- `strip-bound` — M7 and X unmeasured; mandatory pre-build safety stop.
- `unstrip-bound` — M7 and X unmeasured; mandatory pre-build safety stop.
- `meshlet-bound` — M7 and X unmeasured; mandatory pre-build safety stop.
- `remap` — M7 and X unmeasured; mandatory pre-build safety stop.
- `remap-multi` — M7 and X unmeasured; mandatory pre-build safety stop.
- `remap-custom` — M7 and X unmeasured; mandatory pre-build safety stop.
- `remap-vertices` — M7 and X unmeasured; mandatory pre-build safety stop.
- `remap-indices` — M7 and X unmeasured; mandatory pre-build safety stop.
- `shadow` — M7 and X unmeasured; mandatory pre-build safety stop.
- `shadow-multi` — M7 and X unmeasured; mandatory pre-build safety stop.
- `position-remap` — M7 and X unmeasured; mandatory pre-build safety stop.
- `adjacency` — M7 and X unmeasured; mandatory pre-build safety stop.
- `tessellation` — M7 and X unmeasured; mandatory pre-build safety stop.
- `provoking` — M7 and X unmeasured; mandatory pre-build safety stop.
- `simplify-attributes` — M7 and X unmeasured; mandatory pre-build safety stop.
- `simplify-sloppy` — M7 and X unmeasured; mandatory pre-build safety stop.
- `simplify-prune` — M7 and X unmeasured; mandatory pre-build safety stop.
- `simplify-points` — M7 and X unmeasured; mandatory pre-build safety stop.
- `simplify-update` — M7 and X unmeasured; mandatory pre-build safety stop.
- `analyze-cache` — M7 and X unmeasured; mandatory pre-build safety stop.
- `analyze-fetch` — M7 and X unmeasured; mandatory pre-build safety stop.
- `analyze-overdraw` — M7 and X unmeasured; mandatory pre-build safety stop.
- `analyze-coverage` — M7 and X unmeasured; mandatory pre-build safety stop.
- `cluster-bounds` — M7 and X unmeasured; mandatory pre-build safety stop.
- `sphere-bounds` — M7 and X unmeasured; mandatory pre-build safety stop.
- `meshlet-bounds` — M7 and X unmeasured; mandatory pre-build safety stop.
- `optimize-meshlet` — M7 and X unmeasured; mandatory pre-build safety stop.
- `partition` — M7 and X unmeasured; mandatory pre-build safety stop.
- `spatial-remap` — M7 and X unmeasured; mandatory pre-build safety stop.
- `spatial-triangles` — M7 and X unmeasured; mandatory pre-build safety stop.
- `spatial-points` — M7 and X unmeasured; mandatory pre-build safety stop.
- `quantize-unorm` — M7 and X unmeasured; mandatory pre-build safety stop.
- `quantize-snorm` — M7 and X unmeasured; mandatory pre-build safety stop.
- `quantize-half` — M7 and X unmeasured; mandatory pre-build safety stop.
- `dequantize-half` — M7 and X unmeasured; mandatory pre-build safety stop.
- `quantize-float` — M7 and X unmeasured; mandatory pre-build safety stop.
- `simplify-scale` — M7 and X unmeasured; mandatory pre-build safety stop.
- Added scope: cluster LOD; 214e/b6fd/a082 simplify and overdraw; Chromium headless decode of all 42 frozen streams — all unmeasured for the same stop reason.

## Receipts and limits

Campaign: `/mnt/linux-extra/moss-scratch/meshopt-peer-screen/campaign-20261025T150110Z`. `REGISTRATION.md` retains the unchanged measurement/decision rules before any sample; `SAFETY.md` records the stop. Archive hashes, full dependency sources, tree, metadata, lockfile, review inventory, and `unmeasured.json` are retained there. No admission, timing rerun, PAUSE, binary or target cleanup was needed. The old meshopt-cmp campaign, src/, manifests and README files were untouched.

Not verified: output validity/equivalence, runtime/API compatibility, A/A noise, native/browser performance, peer capability completeness. The registered screen is incomplete and has no performance acceptance claim. No dependency retuning or gate relaxation was attempted.
