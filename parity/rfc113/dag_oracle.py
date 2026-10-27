#!/usr/bin/env python3
"""Replay the frozen S1 manifest; never write oracle inputs or references."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import argparse
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess

SCRATCH = Path(_public_path('<scratch>/meshopt-cluster-dag'))

def field(data, offset):
    names = ['magic','version','source_vertex_count','source_index_count','flags','vertices','indices','clusters','groups','nodes','level_count','bvh_depth']
    if offset < 48:
        return 'header.' + names[offset // 4]
    sizes = struct.unpack_from('<5I', data, 20)
    at = 48
    for name, count, width, fields in zip(
        ['vertex','index','cluster','group','node'], sizes, [16,4,32,32,32],
        [['source','x','y','z'],['value'],['first_index','index_count','group','refined','x','y','z','radius'],
         ['x','y','z','radius','error','depth','first_cluster','cluster_count'],
         ['x','y','z','radius','error','group','child_offset','child_count']]):
        if offset < at + count * width:
            return f'{name}[{(offset-at)//width}].{fields[((offset-at)%width)//4]}'
        at += count * width
    return 'end-of-file'

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--milestone', action='store_true')
    args = parser.parse_args()
    subprocess.run(['python3', str(Path.home()/'dev/specs/meshopt/bin/check-uz67q-corpus.py')], check=True)
    manifest = json.loads((SCRATCH/'MANIFEST.json').read_text())
    for path, expected in manifest['pins'].items():
        assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == expected, path
    files = manifest['files']
    if args.milestone:
        files = [min((f for f in files if f['dilated'] == d), key=lambda f:f['size']) for d in [False, True]]
    binary = Path(os.environ['CARGO_TARGET_DIR'])/'release/rfc113-dag-oracle'
    rows = []
    for f in files:
        for path, expected in [(f['source'], f['source_sha256']), (f['oracle'], f['oracle_sha256'])]:
            assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == expected, path
        output = SCRATCH / ('milestone' if args.milestone else 'oracle') / (f['mesh']+'.clod')
        output.parent.mkdir(exist_ok=True)
        assert not output.exists(), f'refusing to overwrite evidence: {output}'
        command = [str(binary),f['source'],str(output)] + (['--dilate'] if f['dilated'] else [])
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        stats = json.loads(result.stdout)
        actual, expected = output.read_bytes(), Path(f['oracle']).read_bytes()
        exact = actual == expected
        row = {'mesh':f['mesh'],'byte_exact':exact,'dilated':f['dilated'],**stats}
        rows.append(row)
        print(json.dumps(row), flush=True)
        if not exact:
            offset = next((i for i,(a,b) in enumerate(zip(actual,expected)) if a!=b), min(len(actual),len(expected)))
            (SCRATCH/'DIVERGENCE.md').write_text(f"First differing byte: {offset}\n\nField: {field(expected,offset)}\n\nCause: pending diagnosis; mesh {f['mesh']}.\n")
        if not exact or stats['collisions']:
            (SCRATCH/'failed-oracle.json').write_text(json.dumps(rows,indent=2)+'\n')
            raise SystemExit(1)
    name = 'MILESTONE' if args.milestone else 'ORACLE'
    (SCRATCH/(name+'.json')).write_text(json.dumps(rows,indent=2)+'\n')
    report = f"# {name}\n\n{len(rows)}/{len(rows)} byte-exact; {sum(r['collisions'] for r in rows)} hash collisions.\n\n| Mesh | Exact | Bytes | Dilated | Collisions |\n|---|---|---:|---|---:|\n"
    report += ''.join(f"| {r['mesh']} | {r['byte_exact']} | {r['size']} | {r['dilated']} | {r['collisions']} |\n" for r in rows)
    (SCRATCH/(name+'.md')).write_text(report)

if __name__ == '__main__':
    main()
