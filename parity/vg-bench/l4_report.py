#!/usr/bin/env python3
"""Summarize immutable L4 receipts; fail when the measured-policy acceptance bar fails."""
import json
import pathlib
import statistics
import sys

root = pathlib.Path(sys.argv[1])
meshes = ['bunny', 'dead_tree_trunk_02', 'namaqualand_boulder_02', 'coast_rocks_01', 'dragon']


def rows(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def cuts(mesh, policy):
    return [r for r in rows(root / f'{mesh}.{policy}.cuts.jsonl')
            if r.get('kind') == 'cut' and r['span_px'] == -1 and r['threshold_px'] >= 1e-6]


def cell(mesh, policy):
    return next(r for r in cuts(mesh, policy) if r['threshold_px'] == 1.0)


print('## MDAG 2 measured-policy rerun\n')
print('Both paths snap before building, use normals at weight 0.5, and write MDAG 2')
print('with Oct16x2 normals and UnormRange16x2 UVs. After uses the library v2 builder;')
print('before uses the unchanged upstream builder. Grid exponent -22, extent 2 m,')
print('128 triangles/cluster. Cut rows: 64 directions, six spans, 200k source samples;')
print('measured/claimed is the median over views at the 1 px threshold in scene units.\n')
print('| Mesh | Before median | After median | Before cut triangles | After cut triangles |')
print('| --- | ---: | ---: | ---: | ---: |')
for mesh in meshes:
    a, b = cell(mesh, 'baseline'), cell(mesh, 'v2')
    print(f"| {mesh} | {a['median_measured_over_claimed']:.3f} | {b['median_measured_over_claimed']:.3f} | "
          f"{a['mean_triangles']:.0f} | {b['mean_triangles']:.0f} |")
    assert b['median_measured_over_claimed'] <= 1.3, mesh
    for policy in ['baseline', 'v2']:
        assert all(r['pass'] for r in rows(root / f'{mesh}.{policy}.cuts.jsonl') if r.get('kind') == 'check'), (mesh, policy)

print('\nDragon p95 maximum deviation (px), thresholds 0.5 / 1 / 2 / 4 px:\n')
for policy in ['baseline', 'v2']:
    curve = sorted(cuts('dragon', policy), key=lambda r: r['threshold_px'])
    values = [r['p95_max_dev_px'] for r in curve]
    print(f"- {policy}: " + ' / '.join(f'{v:.3f}' for v in values))
    if policy == 'v2':
        assert all(a < b for a, b in zip(values, values[1:])), values

print('\nBuild plus write milliseconds, median / p95 / max-minus-min of five repeats.')
print('Pinned to core 2 (1T), cores 2-9 (8T), alternating policy order, shared machine.\n')
print('| Mesh | Policy | Threads | ms median / p95 / noise | Peak RSS MiB | File bytes |')
print('| --- | --- | ---: | ---: | ---: | ---: |')
for mesh in meshes:
    for builder in ['ours', 'ours-v2']:
        for threads in [1, 8]:
            data = [rows(root / f'{mesh}.{builder}.{threads}.{i}.jsonl')[0] for i in range(1, 6)]
            times = sorted(r['build_ms'] + r['serialise_ms'] for r in data)
            rss = statistics.median(r['vmhwm_kib'] for r in data) / 1024
            sizes = {r['file_bytes'] for r in data}
            assert len(sizes) == 1, (mesh, builder, threads, sizes)
            print(f'| {mesh} | {builder} | {threads} | {statistics.median(times):.1f} / {times[-1]:.1f} / {times[-1]-times[0]:.1f} | '
                  f'{rss:.1f} | {next(iter(sizes))} |')

print('\nSize rows include normal and UV payloads. Resident bytes mean resident prefix plus all memory pages.\n')
print('| Mesh | Policy | DAG triangles | File B/triangle | Resident B/triangle | Pages | Pinned bytes |')
print('| --- | --- | ---: | ---: | ---: | ---: | ---: |')
for mesh in meshes:
    for policy in ['baseline', 'v2']:
        shape = next(r for r in rows(root / f'{mesh}.{policy}.cuts.jsonl') if r.get('kind') == 'shape')
        triangles = shape['total_triangles']
        print(f"| {mesh} | {policy} | {triangles} | {shape['file_bytes']/triangles:.3f} | "
              f"{shape['resident_bytes']/triangles:.3f} | {shape['pages']} | {shape['pinned_bytes']} |")
print('\nMulti-material asset receipt:\n')
print('```json\n' + (root / 'materials.jsonl').read_text().strip() + '\n```')
print('\nAcceptance assertions: passed. These are sampled CPU quality measurements; no GPU timing or continuous error certificate is claimed.')
