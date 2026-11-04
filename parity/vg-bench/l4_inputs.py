#!/usr/bin/env python3
"""Pin the corpus subset and source inputs used by an admitted benchmark run."""
import hashlib
import json
import pathlib
import subprocess
import sys

corpus, evidence = map(pathlib.Path, sys.argv[1:3])


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            value.update(block)
    return value.hexdigest()


expected = dict(line.split()[::-1] for line in pathlib.Path('parity/vg-bench/corpus.sha256').read_text().splitlines())
inputs = {}
for name, sha in expected.items():
    if name == 'lucy.ply':
        continue
    actual = digest(corpus / name)
    if actual != sha:
        raise SystemExit(f'corpus hash mismatch: {name}')
    inputs[name] = actual
inputs['flighthelmet.LICENSE.md'] = digest(corpus / 'flighthelmet.LICENSE.md')
(evidence / 'inputs.json').write_text(json.dumps(inputs, indent=2, sort_keys=True) + '\n')
paths = subprocess.check_output(['git', 'ls-files', '--cached', '--others', '--exclude-standard', '-z']).decode().split('\0')
source = {}
for name in sorted(set(filter(None, paths))):
    path = pathlib.Path(name)
    if path.is_file() and path.suffix in {'.rs', '.wgsl', '.toml', '.lock', '.txt', '.sh', '.py'}:
        source[name] = digest(path)
(evidence / 'source.json').write_text(json.dumps(source, indent=2, sort_keys=True) + '\n')
