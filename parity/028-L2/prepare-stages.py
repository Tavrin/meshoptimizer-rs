#!/usr/bin/env python3
"""Freeze all calls from the retained r2 real-mesh stage harness, without timing."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib, json, os
from stage_vertices import recover
from pathlib import Path
ART = Path(_public_path('<scratch>/meshopt-028-L2'))
R2 = Path(_public_path('<scratch>/meshopt-simplify-perf/r2'))
assert os.environ.get('PARITY_RUNNER_ACTIVE')
rows = []
def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1 << 20), b''): h.update(chunk)
    return h.hexdigest()
for case in ['214e85c568a64e8a','b6fdb93d8a1709e2','a082ca1fd19bb86a']:
    folder = R2 / 'campaign' / case / '00-final-cpp' / 'result'
    name = 'jungle-scoreboard-' + case + '-entity'
    capture = folder / (name + '.json')
    doc = json.loads(capture.read_text())
    blobs = folder / doc['byte_blobs']
    record = doc['records']
    initial = record[0]['data']
    vertex = initial['vertex_bytes']
    vp = blobs / vertex['$bytes']
    assert vp.stat().st_size == vertex['length'] and sha(vp) == vertex['sha256']
    nv = initial['vertex_count']
    stride = vertex['length'] // nv
    lines = [f'{vp}\t{nv}\t{stride}']
    expected = []
    # r2's stage ratios include both the ordinary and legacy pass.
    for stream in ['records','legacy_records']:
        pruned=False
        for ri,r in enumerate(doc[stream]):
            if r['stage']=='prune_decision': pruned=r['data']['used_pruning']
            op = r['stage']; data = r['data']
            if op not in ['scale_raw','simplify','cache','overdraw']: continue
            active_vp=vp; reconstruction=None
            if op in ['cache','overdraw'] and pruned:
                following=doc[stream][ri:]
                od=next(item['data'] for item in following if item['stage']=='overdraw')
                packaged=next(item['data'] for item in following if item['stage']=='packaged')
                active_vp,reconstruction=recover(vp,blobs,od,packaged,stride)
            if op == 'scale_raw':
                op = 'simplify-scale'; ip = '-'
                result = int(data['scale_bits']).to_bytes(4, 'little')
            else:
                desc = data['input_indices']; ip = blobs / desc['$bytes']
                assert ip.stat().st_size == desc['length'] and sha(ip) == desc['sha256']
                out = data['output_indices']; result = (blobs / out['$bytes']).read_bytes()
                assert len(result) == out['length'] and hashlib.sha256(result).hexdigest() == out['sha256']
                if op == 'simplify': result += int(data['error_bits']).to_bytes(4,'little')
            lines.append(f"{op}\t{ip}\t{data.get('target_indices',0)}\t{data.get('target_error_bits',1056964608)}\t{data.get('options',0)}\t{data.get('threshold_bits',1065772646)}\t{active_vp}")
            expected.append(dict(operation=op, stream=stream, vertex_path=str(active_vp), vertex_sha256=sha(active_vp), reconstruction=reconstruction, input_path=str(ip), input_sha256=None if str(ip)=='-' else sha(ip), output_sha256=hashlib.sha256(result).hexdigest(), bytes=len(result)))
    manifest = ART / f'chain-{case}.tsv'
    manifest.write_text('\n'.join(lines)+'\n')
    rows.append(dict(case=case, capture=str(capture), capture_sha256=sha(capture), manifest=str(manifest), manifest_sha256=sha(manifest), vertex=vertex, stride=stride, calls=expected))
(ART / 'stages.json').write_text(json.dumps(rows,indent=2)+'\n')
print([(r['case'], len(r['calls'])) for r in rows])
