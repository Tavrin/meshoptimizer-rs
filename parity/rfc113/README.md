# RFC 113 S2 cluster LOD parity

`run.py` compares the exact S2 bridge output with a safe Rust equivalent. Its
`bridge.cpp` is a byte-for-byte copy of S2's `cluster_lod_bridge.cpp`, checked
before each build; the C++ build links Moss's read-only vendored sources. Both
drivers receive the same S0b mesh bytes, synthesized layout, and S2
`boundary_locks` array. The Rust callback emits S2's DAG records in callback
order and uses `clusterlod::local_indices`. It passes the 3 normal weights to
the first three attribute components and zero weights for the remaining
components so the public Rust API can express S2's wider protect mask.

The `R11B` parity request appends independent cluster and meshlet bounds to
the complete S2 DAG. Every output byte is compared. The `R11T` timing request
executes the S2 bridge without extra bounds and returns internal monotonic
time. All three drivers stay pinned to one CPU and samples alternate scalar
C++, Moss-style SIMD C++, and Rust order. The runner samples `/proc/stat` for
one second and chooses the least busy allowed core unless `--core` is supplied.
It excludes cores 0 and 1, used by concurrent benchmark lanes, by default;
`MESHOPT_RFC113_EXCLUDE_CORES` changes that list.
It records load average and selected-core utilization over the run.
Input preparation, pipe transfer, and process launch are outside this timer;
decoding the framed input and serializing the DAG are inside it.

`codec.py` uses the existing 0.4 parity drivers against the same vendored C++
codec sources. For every supported layout case it compares IndexBuffer,
IndexSequence, VertexBuffer, FilterOct, FilterQuat, and FilterExp payloads,
their bounds and version queries where applicable, and cross-decodes the
encoded bytes on both sides. It records hashes rather than retaining large
payload files.

Run from the repository root:

```sh
python3 parity/rfc113/run.py --pairs 20
python3 parity/rfc113/codec.py
python3 parity/rfc113/differential.py
python3 parity/rfc113/budget.py
python3 parity/rfc113/profile.py --core 21
```

Results are `result.json`, `codec.json`, `differential.json`, `budget.json`, and small perf call-graph summaries under
`/mnt/linux-extra/meshopt-artifacts/rfc113`. The small handoff is
`parity/results/rfc113-clod.md`. Pass `--case mesh:stride` to either runner
for a single deterministic reproduction.

Set `MESHOPT_RFC113_MOSS_DIR` to the Moss cooker directory,
`MESHOPT_RFC113_MESH_DIR` to the frozen `.mesh` directory,
`MESHOPT_RFC113_ART_DIR` to the artifact directory, and `CARGO_TARGET_DIR`
to the build target. Their current paths are the defaults in `run.py`; this
performance lane uses
`CARGO_TARGET_DIR=/mnt/linux-extra/moss-cargo-targets/codex-meshopt-clodfix`.

`differential.py` archives the exact `main` commit into the artifact directory,
builds its phase 0.3 Rust driver, then builds the current Rust driver and the
vendored scalar C++ demo driver. It compares the complete cluster-LOD result,
including post-dilation positions, on deterministic large, sparse, 32-attribute,
compact-threshold, and near-`1e8` cases. The archive is never checked out into
this worktree. Its JSON records all three binary hashes and the source commit.
`budget.py` uses the archived `main` source to build a separate bounded-memory
driver. It sweeps each byte around the old success boundary, checks that every
old success still succeeds, and compares error kinds where both sides fail.
The scalar C++ build uses `MESHOPTIMIZER_NO_SIMD` and strict scalar float
flags. The second C++ build mirrors Moss's release `cc` flags, including
`-O3` and SIMD enabled; it is the adoption reference, while the scalar build
remains this lane's performance bar.

S2's own `mesh_vertex_stride_for_layout` accepts 48, 64, 72, and 88 bytes.
The spec also requests 32-byte pos/nrm/uv coverage. S2's bit-8 protect mask
violates `clodBuild`'s `mask < 1 << (stride / 4)` assertion at stride 32 and
reads beyond the last vertex if assertions are disabled. The runner records
this setup as invalid and does not claim 32-byte parity or timing.
