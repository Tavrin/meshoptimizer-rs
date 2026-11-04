# Cluster-DAG benchmark against the Bevy meshlet builder

Not published and not part of the crate package (own Cargo workspace, outside the
package include list). It builds the same mesh with

- this crate: `clusterlod::build_parallel_with_output` plus `build_hierarchy`
  (per-level forest, width 8) and the MDAG 1 writer, and
- Bevy's `MeshletMesh::from_mesh`, pinned to commit `8cc060f01c3d4d5920781124cc8c32013bb47730`
  (feature `bevy`), saved with Bevy's own asset saver,

then measures both. Third-party source is data: the reference cut rule is a port of
`cull_bvh.wesl`, `cull_clusters.wesl` and `cull_shared.wesl` at that commit, and
the file reader parses the saved version-3 layout (`asset.rs`) from bytes.

## Run

```
bash parity/vg-bench/fetch_corpus.sh <corpus-dir>           # downloads, checks corpus.sha256
cargo build --release --features bevy --manifest-path parity/vg-bench/Cargo.toml
bash parity/vg-bench/bench.sh <corpus-dir> <work-dir> bunny  # one mesh, appends <work-dir>/bunny.jsonl
python3 -I parity/vg-bench/report.py <work-dir>/*.jsonl      # markdown tables
```

`bench.sh` reads its settings from environment variables listed at the top of the
script (worker count, pinned core lists, repeats, orbit directions).

## Commands

`vg-bench prep <mesh> <out.vgm> [--extent 2.0]` fits the mesh (longest side
`extent` metres, centred), derives normals and box-projected UVs when absent, and
writes the shared input both builders read. `build <ours|bevy> <in.vgm> --threads N
[--out file]` is one timed build plus serialisation; the process is wrapped by
`/usr/bin/time -v` and `taskset` in `bench.sh`. `eval <ours|bevy> <in.vgm>` prints
the static metrics, the self-checks, and the cut evaluation.

## What is measured

| Metric | Method |
| --- | --- |
| Build time | `build_ms` is the DAG build plus hierarchy (ours) or `from_mesh` (reference); serialisation is reported separately. Five interleaved repeats, one process each, pinned cores, 1 and N threads. Median, p95 and noise (max minus min) are in the report. |
| Peak memory | `Maximum resident set size` from `/usr/bin/time -v` for the whole process (input load, build, serialise). |
| Disk | Reference: saved `.meshlet_mesh` bytes (LZ4 frame), also after `zstd -19` of the file and of the decompressed stream. Ours: MDAG 1 file bytes and `zstd -19` of the file. |
| Resident | Reference: decompressed slice bytes. Ours (MDAG 1): metadata tables plus the decoded geometry of every cluster (vertices x 16 + indices x 4). |
| Pages | MDAG 1 page count, mean fill of the 128 KiB capacity, groups with the terminal error value. The reference has no pages. |
| DAG shape | total and leaf triangles, triangles and vertices per cluster (mean, p5), levels, hierarchy depth (dispatches per view). Reference leaves are meshlets with error 0. |
| Error per triangle budget | 64 Fibonacci orbit directions x spans of 64..2048 px (camera distance for a bounding sphere of that vertical span; viewport 2048 px, 60 degree FOV) x thresholds 0.5, 1, 2, 4 px. Each view selects a cut (no frustum culling, so the whole mesh is covered), then `measure_lod_quality` measures the cut against the source (200k samples on the source, level sampled at the same density). World deviation becomes pixels at the nearest depth (`distance - bounding radius`), so values are worst case. Identical cuts are measured once. Claimed error is the largest projected self error among drawn clusters; the ratio is measured maximum over claimed. |
| Traversal work | The hierarchical traversal with frustum culling (spheres for ours, the stored boxes for the reference; no occlusion culling): nodes visited, clusters tested, clusters drawn, averaged over the same views. |
| Decode | Ours: median `decode_page` rate. Reference: LZ4 frame decode of the whole asset. wasm32 decode is not measured here. |

Self-checks per builder: threshold 0 selects exactly the source triangles and only leaves;
a huge threshold selects only clusters whose parent error is the terminal value; the
hierarchical traversal equals the flat cut at 1 px. They guard the cut-rule port.

The reference quantises positions to 1/16 cm (`--quant`, default 4) in its saved asset;
its cut meshes use the decoded positions, so its measured deviation includes that
quantisation. Ours keeps f32 positions.
