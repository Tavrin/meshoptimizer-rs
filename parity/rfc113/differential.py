#!/usr/bin/env python3
"""Deterministic pre-optimization/current/C++ cluster-LOD differential."""
import hashlib
import io
import json
import math
import os
import shutil
import struct
import subprocess
import tarfile
import argparse
from pathlib import Path

import run

ROOT = run.ROOT
ART = run.ART
TARGET = run.TARGET
ENV = run.ENV
VENDOR = run.VENDOR


def command(args, **kwargs):
    return subprocess.run([str(v) for v in args], check=True, env=ENV, **kwargs)


def build():
    ART.mkdir(parents=True, exist_ok=True)
    TARGET.mkdir(parents=True, exist_ok=True)
    baseline = command(['git', 'rev-parse', 'main'], cwd=ROOT, capture_output=True, text=True).stdout.strip()
    old_root = ART / 'pre-optimization-main'
    if old_root.exists():
        recorded = (old_root / '.source-commit').read_text().strip()
        if recorded != baseline:
            shutil.rmtree(old_root)
    if not old_root.exists():
        old_root.mkdir()
        archive = command(['git', 'archive', 'main'], cwd=ROOT, capture_output=True).stdout
        with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
            tar.extractall(old_root, filter='data')
        (old_root / '.source-commit').write_text(baseline + '\n')
    manifest = Path('parity/p03/Cargo.toml')
    command(['cargo', 'build', '--offline', '--release', '--manifest-path', old_root / manifest])
    old_bin = TARGET / 'rfc113-pre-optimization-rust'
    shutil.copy2(TARGET / 'release/meshopt-p03-driver', old_bin)
    command(['cargo', 'build', '--offline', '--release', '--manifest-path', ROOT / manifest])
    new_bin = TARGET / 'release/meshopt-p03-driver'
    cpp = TARGET / 'rfc113-p03-cpp'
    sources = sorted((VENDOR / 'src').glob('*.cpp'))
    command(['c++', '-std=c++17', '-O3', '-DMESHOPTIMIZER_NO_SIMD',
             '-fno-fast-math', '-ffp-contract=off', '-I' + str(VENDOR / 'src'),
             '-I' + str(VENDOR), ROOT / 'parity/p03/reference.cpp', *sources, '-o', cpp])
    return {'old': old_bin, 'rust': new_bin, 'cpp': cpp}, baseline


def grid(n, seed=0):
    points = [(float(x), float(y), float(math.sin(x * .4 + seed) * math.cos(y * .3)))
              for y in range(n) for x in range(n)]
    indices = []
    for y in range(n - 1):
        for x in range(n - 1):
            a = y * n + x
            indices.extend((a, a + 1, a + n, a + n, a + 1, a + n + 1))
    return points, indices


def message(points, indices, *, width=0, target=0, partition=16, triangles=32):
    attributes = []
    if width:
        for i in range(len(points)):
            for k in range(width):
                attributes.append(struct.unpack('<I', struct.pack('<f', ((i * 7 + k * 3) % 31) / 100))[0])
        target |= (width - 1) << 20
    return (struct.pack('<4s6I2f3I', b'MO03', 16, len(points), len(indices), partition,
                        max(1, triangles // 3), triangles, 0., .02, target, len(attributes), 0)
            + b''.join(struct.pack('<3f', *p) for p in points)
            + struct.pack('<' + 'I' * len(indices), *indices)
            + struct.pack('<' + 'I' * len(attributes), *attributes)
            + b'\0' * (4 * len(points)))


def cases():
    p, ix = grid(2)
    yield 'single-triangle-32-attributes', message(p[:3] + [(float(i), 100., 0.) for i in range(2048)], ix[:3], width=32)
    for n in (16, 32, 64):
        p, ix = grid(n, n)
        yield f'large-grid-{n}', message(p, ix, target=16 if n == 16 else 0)
    p, ix = grid(12, 4)
    yield 'sparse-8192', message(p + [(float(i), 200., 0.) for i in range(8192)], ix, width=3)
    p, ix = grid(9, 7)
    threshold = len(ix) * 4
    for count in (threshold, threshold + 1):
        extra = [(float(i), 300., 0.) for i in range(count - len(p))]
        yield f'compact-threshold-{count}', message(p + extra, ix)
    p, ix = grid(16, 2)
    for shift in (99_999_744., 99_999_936., 99_999_992., 100_000_008.):
        near = [(shift + (x - 15.) * 16., y * 16., z * 16.) for x, y, z in p]
        yield f'dilate-near-{int(shift)}', message(near, ix, target=16)
    p, ix = grid(12, 2)
    near = [(99_999_992. - abs(z) * 32., x * 8., y * 8.) for x, y, z in p]
    yield 'dilate-moving-below-threshold', message(near, ix, target=16)


def invoke(path, data):
    framed = struct.pack('<I', len(data)) + data
    result = command([path], input=framed, capture_output=True)
    if len(result.stdout) < 12:
        raise RuntimeError(f'{path}: short output')
    length = struct.unpack_from('<I', result.stdout)[0]
    payload = result.stdout[4:4 + length]
    if len(payload) != length or payload[:4] != b'MR03':
        raise RuntimeError(f'{path}: invalid response')
    n = struct.unpack_from('<I', payload, 4)[0]
    return payload[:8 + n]


def movement(output, source):
    body = output[8:]
    groups = struct.unpack_from('<I', body)[0]
    at = 4
    for _ in range(groups):
        count = struct.unpack_from('<I', body, at + 24)[0]
        at += 28
        for _ in range(count):
            indices = struct.unpack_from('<I', body, at + 28)[0]
            at += 32 + indices * 4
    vertices = struct.unpack_from('<I', source, 8)[0]
    before = source[48:48 + vertices * 12]
    after = body[at:at + vertices * 12]
    if len(after) != len(before):
        raise RuntimeError('short mutable-position trailer')
    changed = sum(before[i:i + 12] != after[i:i + 12] for i in range(0, len(before), 12))
    crossed = sum(abs(struct.unpack_from('<f', before, i)[0]) <= 1e8
                  and abs(struct.unpack_from('<f', after, i)[0]) > 1e8
                  for i in range(0, len(before), 12))
    return changed, crossed


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--reuse-binaries', action='store_true')
    args = parser.parse_args()
    if args.reuse_binaries:
        baseline = command(['git', 'rev-parse', 'main'], cwd=ROOT,
                           capture_output=True, text=True).stdout.strip()
        bins = {'old': TARGET / 'rfc113-pre-optimization-rust',
                'rust': TARGET / 'release/meshopt-p03-driver',
                'cpp': TARGET / 'rfc113-p03-cpp'}
        if not all(path.is_file() for path in bins.values()):
            raise SystemExit('missing binaries; run without --reuse-binaries')
    else:
        bins, baseline = build()
    records = []
    for label, data in cases():
        values = {name: invoke(path, data) for name, path in bins.items()}
        hashes = {name: hashlib.sha256(value).hexdigest() for name, value in values.items()}
        row = {'case': label, 'input_sha256': hashlib.sha256(data).hexdigest(),
               'bytes': len(values['cpp']), 'outputs': hashes,
               'match': len(set(values.values())) == 1}
        if label.startswith('dilate-'):
            row['moved_vertices'], row['crossed_threshold'] = movement(values['rust'], data)
        records.append(row)
        print(label, 'MATCH' if row['match'] else 'MISMATCH', flush=True)
    result = {'schema': 'rfc113-three-way/1', 'main_commit': baseline,
              'current_commit': command(['git', 'rev-parse', 'HEAD'], cwd=ROOT,
                                        capture_output=True, text=True).stdout.strip(),
              'source_hashes': {str(path): run.sha(path) for path in
                                [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock',
                                 ROOT / 'parity/p03/src/lib.rs', ROOT / 'parity/p03/src/main.rs',
                                 ROOT / 'parity/p03/reference.cpp', ROOT / 'parity/rfc113/differential.py',
                                 VENDOR / 'clusterlod.h',
                                 *sorted((ROOT / 'src').rglob('*.rs'))]},
              'executables': {name: run.sha(path) for name, path in bins.items()},
              'cases': records}
    (ART / 'differential.json').write_text(json.dumps(result, indent=2) + '\n')
    if not any(row.get('moved_vertices') and row['case'] == 'dilate-moving-below-threshold'
               for row in records):
        raise SystemExit('near-threshold open mesh did not dilate')
    if any(not row['match'] for row in records):
        raise SystemExit('three-way cluster-LOD differential failed')


if __name__ == '__main__':
    main()
