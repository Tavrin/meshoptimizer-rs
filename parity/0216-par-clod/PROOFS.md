# Parallel cluster LOD identity

## Scope and API

`clusterlod::build_parallel` and `build_parallel_with_output` require
`parallel` + `clusterlod`. They are opt-in extensions, not in C++ meshoptimizer.
The existing serial builders, flat-DAG builder, and independent-mesh batches
keep their execution mode. The using guide includes a four-worker recipe.

The parallel path requires `max_bytes == usize::MAX`, `max_work == u64::MAX`,
and no border dilation. Finite limits, including defaults, take the original
serial path. A level with one group, any speculative group or slot allocation
failure, or insufficient remaining work for the checked level bound also
falls back before its first callback. Callback failures propagate immediately
at their original ordinal. Speculation cannot recover or suppress a serial
error: it is discarded on failure, then the original group loop determines
that error and its prefix.

## Structural argument

1. Partitioning and boundary locking remain in the calling thread at the
   original per-level points. Input cluster order and immutable group ordinals
   are fixed before any worker starts. Workers borrow the same immutable mesh,
   locks, topology and clusters. Border dilation is excluded for the entire
   call; it therefore cannot create cross-group read/write dependencies.
2. `prepare_group` is the serial group's original merge, bounds, simplification,
   stuck test and inherited-error expression, extracted without arithmetic,
   allocation, member-order or charge changes. Workers and serial execution
   call this same helper. Clusterization calls the same existing serial kernel.
   No reduction, sphere expansion, tie comparison or simplifier loop is parallel.
3. Worker slots are indexed by ordinal. After every worker has joined, ordered
   assembly absorbs one group's preparation measurements, runs the original
   `output` and callback, then absorbs its clusterization measurements and
   appends topology. Completion order never controls output order. The callback
   runs on the caller and does not require `Send` or `Sync`.
4. `clusterize` only stores its `refined` argument in new pending records; it
   does not branch or calculate geometry from it. Speculation uses -1, then
   replaces that field with the actual callback ID before assembly. Bounds and
   index offsets are assigned at their original ordered assembly points. IDs
   therefore remain arbitrary caller-supplied identifiers, rather than worker
   ordinals or completion order.
5. The original context remains responsible for live serial storage. A worker
   measures two independent stages in a private context: preparation and
   clusterization. For a stage with relative peak P, work W and surviving
   capacities L, ordered absorption records `parent_live + P`, charges W, and
   adds L. Preparation survivors are merged indices, member bounds and
   simplified indices. Clusterization survivors are pending records and
   topology. The callback lies between these stages, with preparation live,
   and the original free order is retained. By induction on stage/group/level,
   successful work and logical peak storage equal serial execution. Initial
   retained-workspace costs occur before speculation; child workspaces are
   private and fresh on both paths.
6. Maximal limits alone do not prove sufficient remaining cumulative work.
   The level bound checked after speculation sums the successful stage work,
   one visit per potential allocating callback, and, for precise output
   bounds, `vertices + 6 * cluster_indices` per refined cluster. Bounds charges
   topology and corner scans (2I), triangle visits (I/3), corner sphere (<=2I),
   normal sphere and normal visits (<=4I/3), plus V position visits: <=V+6I.
   Checked arithmetic overflow or a bound above remaining work selects serial
   execution. Thus ordered absorption cannot collapse a work-limit failure
   into an incorrectly charged successful stage.
7. Failing speculation has no callbacks, position mutations or parent charges.
   All speculative buffers are dropped before fallback. Output-stage errors
   are still encountered in the original serial order; later speculative
   results never affect their observed prefix. Callback errors and panics
   prevent subsequent callbacks as in the serial API. Worker or pool panics
   follow the documented Rayon panic contract.

`Usage` deliberately describes the identical serial logical operation. Physical
parallel scratch, held speculative results and Rayon metadata are additional,
unmetered storage. This is explicitly documented and parallel execution is
unavailable under finite storage budgets. As with the existing parallel batch
API, physical allocator exhaustion and scheduling-dependent panics are not a
promise of equal environmental outcomes. Fallible speculative reservation
errors cause fallback; the subsequent serial operation selects the public error.

## Retained execution evidence

See `exits.tsv`, sanitized `logs/`, `build-binding.json`, `inputs.json`,
`environment.json`, `counts.jsonl` and `results.json`. The baseline revision is
`1ecf69abe9f92c17e13dc38fa97be659aee44582`; the C++ source binding requires core
and demo byte identity with the pinned 1.3 reference. Both exact-driver feature
modes enable `parallel`/`clusterlod` and toggle SIMD; the independent
no-default-feature gate additionally checks the crate without std/parallel.

The driver compares exact float words, cluster/index order, callback records
and custom IDs, returned errors, positions, and work/storage usage. Each bounded
case sweeps every work limit through the unrestricted charge + 1, then storage
boundaries and callback failures. Four successive calls exercise cold, warm,
failed-callback and after-failure workspace states. Larger multi-group cases
cover configuration switches, attributes and flags, all requested worker
counts, reversed/zigzag schedules, and forced speculative failure. Full LOD
corpus and three real meshes repeat every worker count and successful schedule.

This is structural reasoning plus finite differential evidence, not an
exhaustive proof over all floating-point meshes or allocator behaviors. The
separate fuzz target source is provided and is not executed in this lane.

## Final execution result

All 263,989 exact observation-vector comparisons passed; each targeted vector contains four successive calls. Breakdown:

| Driver | Comparisons |
|---|---:|
| targeted-simd | 86,371 |
| lod-corpus-simd | 720 |
| real-corpus-simd | 128 |
| targeted-scalar | 86,371 |
| lod-corpus-scalar | 720 |
| real-corpus-scalar | 128 |
| prefix-simd | 1,590 |
| prefix-scalar | 1,590 |
| exact-no-std | 86,371 |

Additional C++ LOD differential: 180 arm comparisons over 45 fixtures, zero differences. The eight peer fixtures include five synthetic and three real meshes; the LOD corpus has 45 fixtures. Full output matrices cover 1/2/4/8/32 workers and both SIMD modes; prefix matrices also cover all worker counts on every fixture. The truly no_std crate additionally passes the exhaustive targeted serial observer.

All final named checks exit 0. Initial test compilation failed with an incorrect test callback return type; the initial no-default doctest run failed due to a missing feature guard on the guide recipe. Both were fixed before the successful final full gates; original failure logs are retained. No fuzz input was executed (the new target was compiled only).

The bounded exhaustive target uses 29 tiny cases (grid side two, invalid configurations and geometry), plus 20 larger config cases on grid side ten. Its work limits are fully enumerated rather than sampled. This remains far below the two CPU-hour boundary-case allocation.

See [TABLE.md](TABLE.md) for instruction and wall-time disclosures. Identity passes: ship the feature as opt-in. The dedicated target is deleted after retaining receipts. Git index, branches, commits and references are unchanged.
