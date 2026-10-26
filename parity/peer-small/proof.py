#!/usr/bin/env python3
"""Targeted existing C++ differential, frozen corpus and exact Result replay."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import array
import gzip
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import random
import struct
import subprocess
import sys

ART = Path(_public_path('<scratch>/meshopt-peer-c'))
ROOT = Path(__file__).resolve().parents[2]
assert os.environ.get('MOSS_HEAVY_ACTIVE')
sys.path.insert(0, str(ROOT / 'parity'))
import p01x
spec = importlib.util.spec_from_file_location('p03', ROOT / 'parity/p03/runner.py')
p03 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p03)
OPS = ['adjacency', 'optimize-meshlet', 'fetch', 'remap-indices']
CASES = ['tiny-smooth', 'medium-smooth', 'medium-seams', 'medium-sparse', 'million-smooth']
candidate = sys.argv[1]
build = json.loads((ART / 'proof-build.json').read_text())
assert build['source'] == {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
                           for p in (ROOT / 'src').rglob('*.rs')}
for path, expected in build['binaries'].items():
    assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == expected
record = dict(cases=[], mismatches=0, passed=False, build=build)
proof_dir = ART / 'exact-results'
proof_dir.mkdir(exist_ok=True)


class Client:
    def __init__(self, binary):
        self.p = subprocess.Popen([str(binary)], stdin=subprocess.PIPE, stdout=subprocess.PIPE)

    def run(self, data):
        self.p.stdin.write(struct.pack('<I', len(data)) + data); self.p.stdin.flush()
        size = self.p.stdout.read(4)
        assert len(size) == 4
        return self.p.stdout.read(struct.unpack('<I', size)[0])

    def close(self):
        self.p.stdin.close(); self.p.wait()
        assert self.p.returncode == 0


base, cand, cpp3 = (Client(ART / p) for p in ['proof-base.bin', 'proof-candidate.bin', 'cpp-p03'])


def compare(data, name, oracle=False):
    original, actual = base.run(data), cand.run(data)
    outputs = {'base': original, 'candidate': actual}
    if oracle and original[0] == 0:
        if data[:4] == b'MO03':
            reference = cpp3.run(data)[:-16]
        else:
            reference = subprocess.check_output([str(ART / 'cpp-p01x')], input=data)
        outputs['cpp'] = b'\0' + reference
    (proof_dir / f'{name}.input').write_bytes(data)
    for arm, output in outputs.items():
        (proof_dir / f'{name}.{arm}').write_bytes(output)
    match = len(set(outputs.values())) == 1
    record['cases'].append(dict(name=name, match=match, result='Err' if original[0] else 'Ok',
                               input_sha256=hashlib.sha256(data).hexdigest(),
                               outputs={a: hashlib.sha256(b).hexdigest() for a, b in outputs.items()}))
    if not match:
        record['mismatches'] += 1
        (ART / 'proof.json').write_text(json.dumps(record, indent=2) + '\n')
        raise AssertionError(name)


# Native C++ 1.3 frozen corpus proof, including the actual exported adapters.
for case in CASES:
    for op in OPS:
        outputs = []
        for tag, binary in [('base', str(ART / 'driver-base-1')), ('candidate', candidate), ('cpp', str(ART / 'driver-M7-0'))]:
            out = proof_dir / f'frozen-{case}-{op}.{tag}'
            p = subprocess.Popen([binary, op, str(ART / 'inputs' / f'{case}.bin'), str(out)],
                                 stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
            assert p.stdout.readline().strip() == 'READY'
            p.communicate('stop\n'); assert p.returncode == 0
            outputs.append(hashlib.sha256(out.read_bytes()).hexdigest())
        assert len(set(outputs)) == 1, (case, op, outputs)
        record['cases'].append(dict(name=f'frozen-{case}-{op}', match=True, hashes=outputs))

# Existing native/JS and geometry fixtures; include welding producer and
# tessellation because they share helpers with the adjacency/remap fixes.
for path in sorted((ART / 'p01x-inputs').glob('*.input')):
    if struct.unpack_from('<I', path.read_bytes(), 4)[0] in [9, 13, 19, 20, 22]:
        compare(path.read_bytes(), path.stem, True)
for i, (name, positions, indices) in enumerate(p01x.geometries()):
    for op in [9, 13, 19, 20, 22]:
        compare(p01x.message(op, positions, indices, i), f'geometry-{name}-{op}', True)
for path in sorted((ART / 'p03-inputs').glob('*.input')):
    if struct.unpack_from('<I', path.read_bytes(), 4)[0] & 0xffff in [9, 10]:
        compare(path.read_bytes(), path.stem, True)
for op in [9, 10]:
    for seed in range(20261004, 20261029):
        for into in [False, True]:
            data = bytearray(p03.case(op, seed))
            if into: struct.pack_into('<I', data, 4, op | 0x10000)
            compare(bytes(data), f'p03-{op}-{seed}-{into}', True)

for path in sorted((ART / 'fuzz-inputs').glob('*.input')):
    compare(path.read_bytes(), 'fuzz-' + path.stem)
# The exact existing p03 smoke corpus seed, plus deterministic mutations.
seed = bytes([20, 20, 0, 0, 0, 63, 63, 16, 64, 32]) + bytes(range(256)) * 4
rng = random.Random(20261004)
for i in range(257):
    data = bytearray(seed)
    if i:
        for _ in range(1 + i % 8):
            at = rng.randrange(len(data)); data[at] ^= 1 << rng.randrange(8)
    compare(b'MFUZ' + data, f'p03-fuzz-{i}')

for client in [base, cand, cpp3]: client.close()

# Exhaustive small limit transitions compare Results, usage and failure-side
# destinations byte-for-byte, with compressed raw output retained.
hashes = []
for arm in ['base', 'candidate']:
    process = subprocess.Popen([str(ART / f'proof-{arm}.bin'), 'limits'], stdout=subprocess.PIPE)
    digest = hashlib.sha256()
    with gzip.open(ART / f'limits-{arm}.gz', 'wb') as output:
        while chunk := process.stdout.read(1024 * 1024):
            digest.update(chunk); output.write(chunk)
    process.wait(); assert process.returncode == 0
    hashes.append(digest.hexdigest())
assert hashes[0] == hashes[1], hashes
record.update(limits_sha256=hashes, passed=True)
(ART / 'proof.json').write_text(json.dumps(record, indent=2) + '\n')
print('PASS', len(record['cases']), 'corpus/fixture/seed cases and exhaustive limit Results')
