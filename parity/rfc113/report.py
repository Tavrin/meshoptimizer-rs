#!/usr/bin/env python3
"""Verify retained RFC113 evidence and write the small producer handoff."""
import hashlib
import json
import statistics
import subprocess
from pathlib import Path
import run as clod

def hash_file(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def main():
    dag_path=clod.ART/'result.json';codec_path=clod.ART/'codec.json'
    dag=json.loads(dag_path.read_text());codec=json.loads(codec_path.read_text())
    for record in (dag['identity']['sources'],codec['source_hashes']):
        for path,digest in record.items():
            if hash_file(path)!=digest:raise RuntimeError(f'source drift: {path}')
    for record in (dag['identity']['executables'],codec['executable_hashes']):
        for side,digest in record.items():
            path=(clod.TARGET/('consumer/rfc113-rust' if side=='rust' else 'rfc113-cpp')) if record is dag['identity']['executables'] else (clod.TARGET/('release/codec-driver' if side=='rust' else 'rfc113-codec-cpp'))
            if hash_file(path)!=digest:raise RuntimeError(f'binary drift: {path}')
    valid=[r for r in dag['cases'] if r['mismatch'] is None]
    invalid=[r for r in dag['cases'] if r['mismatch'] is not None]
    assert len(valid)==len(codec['cases'])==15 and len(invalid)==2
    assert all(not r['mismatches'] for r in codec['cases'])
    timing=dag['timing'];head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=clod.ROOT,text=True).strip()
    branch=subprocess.check_output(['git','branch','--show-current'],cwd=clod.ROOT,text=True).strip()
    lines=['# RFC 113 clodBuild parity against Moss S2','',
        '**Verdict:** all 15 valid layout cases have byte-identical DAG, local indices, hierarchy, cluster and meshlet bounds. '
        'All 90 codec payload comparisons match. The adoption condition is **not met**: stride 32 is invalid under the exact S2 protect mask, '
        f'and the nine-mesh paired cook ratio is **{timing["ratio"]:.3f}×** C++ (bar ≲1.2×).','',
        '## Setup and identity','',
        f'- Worktree branch `{branch}`, HEAD `{head}`. No crate API or algorithm changes.',
        '- C++ oracle: Moss S2 vendored `meshoptimizer` (master 717ca348), plus a hash-checked copy of `cluster_lod_bridge.cpp`.',
        '- S2 config: `clodDefaultConfig(128)` with only `simplify_dilate_borders=false`; 3 normal weights of 0.5; '
        'protect bits 0..8 (plus 9..12 at stride 64); S2 `boundary_locks`; callback `clodLocalIndices`; hierarchy width 8 and bridge-derived levels.',
        '- S0b inputs: all nine `.mesh` files actually present in the frozen directory (the brief says eight); '
        'stride 48 on all, plus 64/72/88 on pyramid and sponza_lionhead. Synthetic tangent/color/joints/weights are deterministic. '
        'The 72/88 layouts exercise the builder API although S2 candidate admission rejects skinned bases.',
        f'- Copied bridge SHA-256 `{hash_file(clod.HERE/"bridge.cpp")}`; vendored `clusterlod.h` SHA-256 `{hash_file(clod.VENDOR/"clusterlod.h")}`.',
        f'- DAG driver SHA-256: C++ `{dag["identity"]["executables"]["cpp"]}`, Rust `{dag["identity"]["executables"]["rust"]}`.',
        f'- Codec driver SHA-256: C++ `{codec["executable_hashes"]["cpp"]}`, Rust `{codec["executable_hashes"]["rust"]}`.',
        f'- Full source and binary hashes: `{dag_path}` and `{codec_path}`. Artifacts contain hashes and timings only; no source mesh copies or large payloads.','',
        '## Byte comparison','',
        'The complete S2 DAG is the output prefix; a parity-only trailer holds independently recomputed cluster and meshlet bounds. '
        'Equality of the full output also covers callback order, group and cluster records, errors, local index mapping, node records, and all floating-point bits. '
        'There was no first differing field among valid cases.','',
        '| Case | Input SHA-256 | DAG + bounds SHA-256 | Bytes |','|---|---|---|---:|']
    for r in valid:
        lines.append(f'| `{r["case"]}` | `{r["input_sha256"]}` | `{r["cpp_sha256"]}` | {r["bytes"]:,} |')
    lines+=['','The 32-byte pos/nrm/uv setup is invalid for pyramid and sponza_lionhead. '
        '`clodBuild` asserts `attribute_protect_mask < (1 << (vertex_attributes_stride / 4))`; '
        'S2 specifies bit 8 even though stride 32 has only eight floats. With assertions disabled, '
        'the bit-8 scan reads outside the last vertex. The Rust API rejects the same mask as `InvalidParameter`. '
        'No byte parity or timing claim is made for stride 32. Reproduce with '
        '`python3 parity/rfc113/run.py --case pyramid:32 --pairs 0`. Resolving this exact-setup contradiction is outside this lane.','',
        '## Codec payloads','',
        'Each of the 15 valid cases compares the vendored C++ and Rust encoded bytes for IndexBuffer, IndexSequence, '
        'VertexBuffer, FilterOct, FilterQuat, and FilterExp: **90/90 identical**, all statuses zero. '
        'Both decoders agree on every C++ and Rust encoded stream; lossless vertex and sequence streams round trip. '
        'Triangle streams preserve oriented triangles under codec rotation. Vertex/index version and bound queries agree. '
        'Per-payload input, encoded, decoded, and bound hashes are in `codec.json`.','',
        '## Paired cook time','',
        f'Five interleaved C++/Rust pairs per mesh on pinned CPU {dag["identity"]["core"]}; internal monotonic timers cover input conversion and DAG cooking/serialization, '
        'excluding process launch, pipe transfer, and parity-only detailed bounds. '
        f'C++: `{dag["identity"]["cpp_flags"]}`; Rust: `{dag["identity"]["rust_profile"]}`. '
        'The aggregate is the ratio of summed per-mesh medians.','',
        '| Mesh (stride 48) | C++ median ms | Rust median ms | Rust/C++ |','|---|---:|---:|---:|']
    for r in valid:
        if r['median_ratio'] is None:continue
        c=statistics.median(r['times_s']['cpp'])*1000;v=statistics.median(r['times_s']['rust'])*1000
        lines.append(f'| `{r["case"]}` | {c:.3f} | {v:.3f} | {r["median_ratio"]:.3f}× |')
    lines+=['',f'**Sum of medians:** C++ {timing["cpp_sum_medians_s"]:.3f} s, Rust {timing["rust_sum_medians_s"]:.3f} s; '
           f'**{timing["ratio"]:.3f}×**, fails the ≲1.2× adoption bar.','',
           '## Reproduction and limits','',
           'Run `python3 parity/rfc113/run.py --pairs 5` and `python3 parity/rfc113/codec.py` from the repository root. '
           'Use `--case mesh:stride` for one deterministic case. The comparison proves these frozen S0b inputs under the stated S2 setup; '
           'it does not qualify a Moss runtime or GPU swap. The 32-byte request needs a corrected upstream setup, and the cook-time bar needs separate performance work.','',
           '## Artifact inventory','',
           f'- `{dag_path}`: {dag_path.stat().st_size:,} bytes; SHA-256 `{hash_file(dag_path)}`.',
           f'- `{codec_path}`: {codec_path.stat().st_size:,} bytes; SHA-256 `{hash_file(codec_path)}`.','']
    out=clod.ROOT/'parity/results/rfc113-clod.md';out.write_text('\n'.join(lines)+'\n')
    print(out)
if __name__=='__main__':main()
