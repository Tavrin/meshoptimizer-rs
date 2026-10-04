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
time. Both drivers stay pinned to one CPU and samples alternate C++/Rust order.
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
python3 parity/rfc113/run.py --pairs 5
python3 parity/rfc113/codec.py
```

Results are `result.json` and `codec.json` under
`/mnt/linux-extra/meshopt-artifacts/rfc113`. The small handoff is
`parity/results/rfc113-clod.md`. Pass `--case mesh:stride` to either runner
for a single deterministic reproduction.

S2's own `mesh_vertex_stride_for_layout` accepts 48, 64, 72, and 88 bytes.
The spec also requests 32-byte pos/nrm/uv coverage. S2's bit-8 protect mask
violates `clodBuild`'s `mask < 1 << (stride / 4)` assertion at stride 32 and
reads beyond the last vertex if assertions are disabled. The runner records
this setup as invalid and does not claim 32-byte parity or timing.
