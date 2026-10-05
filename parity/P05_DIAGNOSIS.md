# P05 diagfix: instruction diagnosis and ranked batch plan

Phase 0 closes at `74b7d16`, rebased onto `419dc7a`. All strict functional
gates passed before this diagnosis. No timing run occurred in Phase 1.
The old D127–D141 rejected candidates and D144 dead-hypothesis ledger were
read first. D142/D143 are unfinished hypotheses, not measured rejections.

## Evidence and limits

Fresh artifacts: `/mnt/linux-extra/meshopt-artifacts/p05-diagfix/before`.
`counts.json` binds source, scalar C++ reference and executable SHA-256s;
`sources.tar.gz`, both Rust executables, C++ library/runner, complete objdump
assembly, instruction-event samples and `assembly-metrics.json` are retained.
The reproducible controller is the sibling `instructions.py`.

Both consumers use opt-level 3, generic target, empty Rust flags; Moss uses
thin LTO/cgu=1 and default uses LTO=false/cgu=16. Reference is unmodified
`4c203430ca565cb59a468a91922c76c208169536`, O3, NO_SIMD, no fast math or FMA.
Every family is measured on frozen shapes 0 and 2, except remesh uses 0 and
4 (the D138 seam-heavy maximum); both owned/caller forms are covered. These
are instruction diagnostics, not the full timing matrix.

`perf stat instructions:u,branches:u,branch-misses:u` uses fixed CPU 0. Per-call
estimates subtract two whole-process batch counts and divide by their iteration
difference: setup, fixture generation, loading and output overhead are thereby
subtracted. The existing batch timer output is discarded; there are no elapsed
time ratios or timing admission requests. Counts include actual API validation,
fresh workspace, allocating output, scratch and destruction. Instruction-event
sampling reports describe the complete process and are approximate, especially
for tiny OMM compact: they are not exact per-API attribution. Branch-miss slopes
near zero can be negative from subtraction noise; raw counts are retained.
Static assembly counts include cold/error and alternate-layout blocks and are
never equated with retired instructions or directly compared as equal scopes.

## All-family comparison

Medium owned calls, retired instructions per call (rounded). Tiny ratio includes
the worst owned/caller observation on shape 0. M/D = Moss/default.

| Family | C++ instructions | Rust M / D | Ratio M / D | Tiny ratio M / D |
|---|---:|---:|---:|---:|
| stripify | 1,259,883 | 1,943,105 / 1,953,817 | 1.542 / 1.551 | 1.643 / 1.660 |
| stripify_bound | 24 | 19 / 19 | 0.792 / 0.792 | 0.793 / 0.797 |
| unstripify | 206,208 | 115,081 / 119,190 | 0.558 / 0.578 | 1.069 / 1.105 |
| unstripify_bound | 20 | 18 / 18 | 0.899 / 0.901 | 0.899 / 0.902 |
| vertex_cache | 111,817 | 107,455 / 108,379 | 0.961 / 0.969 | 1.423 / 1.953 |
| vertex_fetch | 166,465 | 158,478 / 158,494 | 0.952 / 0.952 | 1.843 / 1.860 |
| overdraw | 36,496,930 | 37,305,523 / 39,134,637 | 1.022 / 1.072 | 1.081 / 1.501 |
| coverage | 36,195,116 | 37,108,833 / 37,340,712 | 1.025 / 1.032 | 1.114 / 1.161 |
| omm_measure | 347,984 | 584,845 / 590,768 | 1.681 / 1.698 | 1.533 / 1.575 |
| omm_rasterize | 9,527 | 10,486 / 10,498 | 1.101 / 1.102 | 1.133 / 1.134 |
| omm_entry_size | 37 | 17 / 17 | 0.459 / 0.458 | 0.459 / 0.459 |
| omm_compact | 2,043 | 2,249 / 2,324 | 1.101 / 1.138 | 1.099 / 1.137 |
| tangents | 168,663 | 182,981 / 185,251 | 1.085 / 1.098 | 1.384 / 1.430 |
| normals | 161,571 | 204,253 / 206,260 | 1.264 / 1.277 | 1.431 / 1.461 |
| remesh | 6,461,084 | 9,296,634 / 9,646,512 | 1.439 / 1.493 | 1.470 / 1.508 |

## Structural causes and dispositions

* **Remesh:** default marking/accumulation account for 54.88%/42.29% of
  instruction samples. Its medium owned call executes 1,033,479 branches vs
  190,275 in C++ (5.43x). Each sample computes two runtime-pitch products, then
  checked grid/row/voxel indexing; Rust's defined saturating float-to-int
  conversions add comparisons/selects to C++'s direct conversions. The dispatch
  marking body has 1,470 static instructions/136 conditional branches; accumulation
  1,679/126, including both packed/view and error paths. Three sample coordinates
  and their octant low bits must remain exact. Owned bound and output passes repeat
  marking. Do not repeat D127/D129–D141 charging, extraction, cast or blanket-inline
  candidates. Select D142/D143's representation hypothesis: constant pitch and
  bounded arrays remove address/check work without changing float operations.
* **Overdraw/coverage:** medium branches are 7.68M/7.31M vs 6.84M; samples are
  >99% in `raster::analyze`. The default body has 1,610 instructions and 126
  conditional branches. Source uses a 16-byte AoS Pixel vs C++'s two SoA arrays,
  but the full pixel loop is already a checked row slice; an AoS rewrite would
  add representation complexity without demonstrated necessity. Concrete excess:
  `reserve` initializes 65,536 pixels (1,048,576 bytes), followed by three more
  full clears. C++ initializes its uninitialized allocation only three times.
  Remove the first repeated clear (4 MiB -> 3 MiB initialization writes). Packed
  positions still go through `Positions::get` in both transform loops; select
  a reader once, retaining checked generic layouts and all numerical/fuel checks.
* **OMM compact:** medium default has 446 branches vs 391, three temporary arrays
  (old bytes, hash table, remap), and a separate `hash_bytes` call. The helper is
  70 instructions/6 conditional branches and uses four-byte chunk iteration,
  not a HashMap. Its 6.67% sampled bucket has only two samples and is weak evidence;
  the actual out-of-line call is firm assembly evidence. Inline this small helper
  and retain only if fresh counts/codegen support it. Preserve copies, collision
  order, partial writes and per-probe fuel; no broad hash/table replacement.
* **Stripify:** 98.60% of default samples are in SliceOutput core; 317,261
  branches vs 266,841. Checked valence/output accesses and dynamic eight-triangle
  buffer corner/search/removal paths remain. Default core is 1,276 instructions/
  156 conditional branches vs C++ entry 725/86 (different cold/inlining scopes).
  Initialized owned output and small helpers already reflect D129/D131. Abandon
  further edits in this campaign: no new bounded transformation demonstrated;
  do not repeat append ownership or inlining experiments. Historical edge timing
  passes, but six other frozen shapes remain unqualified.
* **Unstripify:** sequential carried reads, triangle chunks and branchless
  degeneracy already beat C++ medium retired instructions by ~42–44%; branches
  30,793/32,838 vs 45,139. Tiny caller still pays fallible validation/fuel/result
  overhead. D118 proves equal one-allocation owned behavior; D122 keeps Vec local.
  Abandon further ownership/dispatch edits; allocation count and iterator-vs-index
  are not supported explanations for a remaining full-family timing failure.
* **Cache/fetch:** medium loops beat C++ instructions. Tiny default cache is
  1.953x and fetch 1.860x, identifying fixed wrapper/validation/fallible scratch
  costs rather than scalable kernel work. Cache's no-charge warp/group kernel
  retains 13 conditional branches, including three indexed timestamp guards;
  fetch-small has 86 instructions/5 conditional branches and direct one/two-line
  handling. Fresh zeroing and quota checks are required. D123 kernel expansion
  regressed and was reverted; D125/D126 sinks/timestamp seam already retained.
  Abandon more edits here; no repeated kernel expansion or private unsafe indexing.
* **OMM measure:** 94.57% of samples are in packed `measure_triangles`; 41,539
  branches vs 32,450. Packed Rust body has 7 conversion instructions and 16 scalar
  compares/min/max vs C++ entry 3 conversions/10 compares (entry/helper scopes
  differ). Saturating quantization, checked table/source reads, finite validation
  and counted probes remain; six-integer hash is already inline. std uses native
  sqrt/log2 while no_std retains libm. Both use open-addressed arrays, not HashMap.
  Abandon bit-cast/clamp/vectorization changes: unrestricted valid UVs and exact
  key/level bits prohibit a speculative finite-range conversion shortcut; no
  evidence-based replacement has been demonstrated. Prior full diagnostic means
  pass despite higher instruction counts; counts do not establish timing verdicts.
* **OMM rasterize:** 1,234 branches vs 754, but only 1.102x instructions in
  the owned medium path. Texture's four checked corner loads, saturating fixed
  coordinates, recursive state-specialized traversal and fuel fallback account
  for extra control work. D127 scalar floor and D131 nine ordered direct edges
  already address the identified hot paths; std/libm branches remain explicit.
  Abandon further edits rather than repeat the rejected state/float experiments.
* **Tangents/normals:** packed readers already dispatch once. Default tangent
  samples: accumulation 34.32%, merge 20.45%; normal merge 31.10%, accumulation
  17.73%, smoothing 14.53%. Their branches are 28,446/38,189 vs 18,530/18,603,
  with negligible miss slopes: checked indirect group/adjacency/union-find accesses
  and counted search paths, not branch unpredictability. Packed accumulation
  bodies have 45/54 static conditional branches vs 225/228 in generic view bodies;
  the layout specialization is already effective. Native scalar sqrt is emitted
  (6/12 static sites across paths); no_std uses libm. Ordered dot/cross/angle sums
  and merging forbid reassociation/FMA or parallel reductions. Arrays already
  replace hash maps. Abandon further broad merge/inlining/layout rewrites: existing
  full-family diagnostic means pass and no novel safe win is established.
* **Bound/entry helpers:** only 17–19 Rust instructions per varied call vs 20–37
  C++; no allocation or hashing. Their historical failing maxima precede corrected
  matched batching. Abandon additional library edits; fresh full qualification
  is outside this touched-family campaign, not presumed successful.

Abandonment above means no further optimization in this scoped campaign, supported
by current diagnosis and the rejection ledger. It is not a lower-bound proof,
an approved residual, or acceptance of an unmeasured family.

## Ranked batch plan (commit each fix separately; no interim timing)

| Rank | Fix | Expected effect | Parity risk / effort |
|---|---|---|---|
| 1 | Fixed 8-pitch 512-byte marking grid for resolution 4..8 | Remove two runtime products and grid guard in repeated hot marking; target several percent of total instructions | Low/medium: copy successful interior back; retain borders, heap quotas, fuel and all float operations. Existing resolution-16 fallback goldens. ~1h |
| 2 | Same bounded representation plus 64-entry row map for accumulation | Remove runtime products/grid-row guards from the other ~42% sampled path | Medium: identical cell numbering/voxel and octant bits, original higher-resolution fallback. ~1h |
| 3 | Omit initial redundant raster clear | One fewer 1 MiB clear/call; 25% fewer initialization writes | Low: newly initialized positive-zero buffer, clear between axes. ~15min |
| 4 | Packed raster transform reader selected once | Fewer per-position layout/Option checks, primarily large-input/default codegen | Low/medium: preserve finite checks, scalar operation order and fuel prefixes; generic layout witness. ~30min |
| 5 | Inline OMM compact byte hash | Remove call boundary; expect small instruction saving and expose chunk codegen | Low: retain exact arithmetic and probes. Abandon if assembly/counts show a no-op or regression. ~15min |

Each retained change requires fresh two-profile instruction counts/objdump plus
strict native/libm/executed-WASM 0.5 sweeps and root tests/Clippy. Existing frozen
inputs and all error/tail/work/heap contracts remain. After all edits, one lean
timing campaign covers only touched families, both consumers, complete frozen
shapes and owned/caller forms, 5/10/20 screens and 30 fresh unpooled D146 pairs
only at borderline cap. No all-family matrix, lower bars, unsafe or SIMD.

The current harness offers only fixed-five diagnostics or an all-family final
mode. Add a scoped `lean` mode using the existing final screening/D146 policy;
retain full-matrix `passed=false` when untimed families are absent and separately
report selected-scope completeness/verdict. Historical before times are explicitly
unmatched diagnostics; only Phase 3's new stream decides the touched-family bar.
