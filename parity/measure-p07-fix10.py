#!/usr/bin/env python3
"""Fix10: frozen full matrix, symmetric JS allocation and A/A resolution.

One final R/C epoch; unfinished accepted pairs survive bounded heavy bursts.
A/A is frozen before qualification and is never selected from final failures.
"""
import base64
import hashlib
import json
import math
import os
from pathlib import Path
import statistics
import struct
import subprocess
import sys
import time

sys.path[:0] = [str(Path(__file__).resolve().parent / 'simd'), str(Path(__file__).resolve().parent)]
import qualify as q
from p07_lease import admission

platform, kind = sys.argv[1:3]
assert platform in ['native', 'wasm'] and kind in ['aa', 'final']
A = q.ART
path = A / f'{platform}-{kind}.json'
began = time.monotonic()
record = json.loads(path.read_text()) if path.exists() else {
    'rows': [], 'bursts': [], 'sources': q.sources(),
    'binaries': {str(p.relative_to(A)): q.m.sha(p) for folder in ['bin', 'node-bin', 'control-bin', 'control-node-bin'] for p in (A / folder).iterdir()},
    'controller_sha256': q.m.sha(__file__),
    'node_controller_sha256': q.m.sha(q.ROOT / 'parity/wasm-p07-fix10-bench.mjs'),
    'admission_sha256': q.m.sha(q.ROOT / 'parity/p07_lease.py'),
    'epoch': 'fix10-native-block-init' if platform == 'native' else 'fix10-js-allocating',
    'policy': 'full frozen corpus; 4ms calibration; rotated persistent slots; 5-20 pairs; borderline-only 30 fresh pairs; paired 95% Student-t log-ratio; no repeat-to-pass',
    'resolution': 'both A/A upper bounds <=1.25; all rows in family means; max upper <=1.5 on resolved rows',
}
assert not record.get('complete')
assert record['sources'] == q.sources()
assert record['controller_sha256'] == q.m.sha(__file__)
assert record['node_controller_sha256'] == q.m.sha(q.ROOT / 'parity/wasm-p07-fix10-bench.mjs')
assert all(q.m.sha(A / p) == digest for p, digest in record['binaries'].items())
cases = [(name, b) for name, b in q.corpus() if platform == 'native' or struct.unpack_from('<I', b, 4)[0] in [1, 2, 3, 7]]
record['scope_case_names'] = [name for name, _ in cases]
old_aa = json.loads((A.parent / 'p07-aa/native-aa.json').read_text())['rows']
reusable = {(r['case'], r['api']): r for r in old_aa if r['side'] == 'cpp'}
# Only the native C++ comparator is byte-identical and its wrapper is unchanged.
old_identity = json.loads((A.parent / 'p07-aa/identity.json').read_text())
assert q.m.sha(A / 'bin/cpp-simd') == q.m.sha(A.parent / 'p07-fix9/bin/cpp-simd')
cache = {}
nodes = {}
done = {(r['case'], r['api'], r['side']) for r in record['rows']}


def save():
    temporary = path.with_suffix('.tmp')
    q.save(temporary, record)
    temporary.replace(path)


class Checkpoint(Exception):
    pass


def budget():
    if time.monotonic() - began >= 690:
        save()
        raise Checkpoint()


def gate():
    while True:
        budget()
        receipt = admission()
        if receipt['admitted']:
            return receipt
        save()
        time.sleep(2)


try:
    for side in (['rust', 'cpp'] if kind == 'aa' else ['compare']):
        for api in ['allocating', 'caller-buffer']:
            for name, data in cases:
                key = (name, api, side)
                if key in done:
                    continue
                budget()
                if kind == 'aa' and platform == 'native' and side == 'cpp' and (name, api) in reusable:
                    old = reusable[name, api]
                    record['rows'].append({'case': name, 'api': api, 'side': side, 'interval': old['interval'], 'reused': True,
                        'source_artifact': str(A.parent / 'p07-aa/native-aa.json'), 'artifact_sha256': q.m.sha(A.parent / 'p07-aa/native-aa.json'),
                        'binary_sha256': q.m.sha(A / 'bin/cpp-simd')})
                    save()
                    continue
                gate()
                probe = bytearray(data)
                into = int(api == 'caller-buffer')
                if platform == 'native':
                    if side not in cache:
                        rust_binary = A / 'bin' / ('cpp-simd' if side == 'cpp' else 'rust-simd')
                        cpp_binary = A / 'bin' / ('rust-simd' if side == 'rust' else 'cpp-simd')
                        cache[side] = {'scalar': q.m.Driver(A / 'bin/rust-scalar'), 'rust': q.m.Driver(rust_binary),
                            'simd': q.m.Driver(cpp_binary), 'cpp-scalar': q.m.Driver(A / 'bin/cpp-scalar'),
                            'sse2': q.Rust('sse2'), 'old': q.m.Driver(A / 'control-bin/rust-simd')}
                    ds = {k: d for k, d in cache[side].items() if k != 'sse2' or q.family(name) in ['vertex', 'view-none', 'view-filtered']}
                    if into:
                        struct.pack_into('<I', probe, 16, struct.unpack_from('<I', probe, 16)[0] | 128)
                    struct.pack_into('<I', probe, 32, 1)
                    denominator = 'simd'
                    def call(pair):
                        order = list(ds)
                        shift = pair % len(order)
                        order = order[shift:] + order[:shift]
                        times, hashes = {}, {}
                        for k in order:
                            status, output, timings = ds[k].call(probe)
                            assert status == 0, (name, k, status)
                            times[k] = timings[0]
                            hashes[k] = hashlib.sha256(output).hexdigest()
                        assert hashes['rust'] == hashes['simd'] if kind == 'aa' else hashes['rust'] == hashes['scalar'] == hashes['old']
                        return {'times': times, 'order': order, 'output_sha256': hashes}
                    iterations = max(1, min(1000000, math.ceil(.004 / max(ds['simd'].call(probe)[2][0], 1e-9))))
                    struct.pack_into('<I', probe, 36, iterations)
                else:
                    if side not in nodes:
                        nodes[side] = subprocess.Popen(['taskset', '-c', str(os.environ['MESHOPT_BENCH_CPU']), 'node',
                            str(q.ROOT / 'parity/wasm-p07-fix10-bench.mjs'), str(A / 'node-bin/arithmetic-simd.wasm'),
                            str(A / 'node-bin/arithmetic-scalar.wasm'), str(q.REF / 'js/meshopt_decoder.mjs'),
                            str(A / 'control-node-bin/arithmetic-simd.wasm')], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
                    proc = nodes[side]
                    query = {'input': base64.b64encode(data).decode(), 'into': into, 'side': side, 'iterations': 1}
                    denominator = 'cpp'
                    def call(pair):
                        query['pair'] = pair
                        proc.stdin.write(json.dumps(query) + '\n')
                        proc.stdin.flush()
                        line = proc.stdout.readline()
                        assert line, 'Node driver closed'
                        return json.loads(line)
                    iterations = max(1, min(10000, math.ceil(.004 / max(call(0)['times']['cpp'], 1e-9))))
                    query['iterations'] = iterations
                pending = record.get('current_row')
                row = pending if pending and (pending['case'], pending['api'], pending['side']) == key else {
                    'case': name, 'api': api, 'side': side, 'family': q.family(name), 'iterations': iterations,
                    'stages': [[]], 'discarded': []}
                if platform == 'native':
                    struct.pack_into('<I', probe, 36, row['iterations'])
                else:
                    query['iterations'] = row['iterations']
                record['current_row'] = row
                for stage in [0, 1]:
                    if stage == 1 and not row['stage1_interval'][0] <= 1.5 < row['stage1_interval'][1]:
                        break
                    if len(row['stages']) <= stage:
                        row['stages'].append([])
                    samples = row['stages'][stage]
                    maximum = 20 if stage == 0 else 30
                    # On resume, a completed early-stopped first stage is not sampled again.
                    while len(samples) < maximum and not (stage == 0 and row.get('stage1_complete')):
                        before = gate()
                        result = call(len(samples))
                        after = admission()
                        if not after['admitted']:
                            row['discarded'].append({'stage': stage + 1, 'result': result, 'before': before, 'after': after})
                            save()
                            continue
                        samples.append({'result': result, 'before': before, 'after': after})
                        save()
                        if stage == 0 and len(samples) >= 5:
                            ci = q.interval([p['result']['times']['rust'] / p['result']['times'][denominator] for p in samples])
                            if ci[1] <= 1.5 or ci[0] > 1.5:
                                break
                    ci = q.interval([p['result']['times']['rust'] / p['result']['times'][denominator] for p in samples])
                    if stage == 0:
                        row['stage1_complete'] = True
                        row['stage1_interval'] = ci
                    row['interval'] = ci
                raw = {k: [p['result']['times'][k] for p in row['stages'][0]] for k in row['stages'][0][0]['result']['times']}
                med = {k: statistics.median(v) for k, v in raw.items()}
                row['median_seconds'] = med
                row['time_ratio'] = med['rust'] / med[denominator]
                last = row['stages'][-1]
                row['paired_new_old_interval'] = q.interval([p['result']['times']['rust'] / p['result']['times']['old'] for p in last])
                row['s3'] = {k: q.interval([p['result']['times'][k] / p['result']['times']['scalar'] for p in last]) for k in ['rust', 'sse2'] if k in med}
                record.pop('current_row', None)
                record['rows'].append(row)
                save()
                print(platform, kind, side, api, name, round(row['time_ratio'], 3), row['interval'], flush=True)
    record['complete'] = True
except Checkpoint:
    print('pair-boundary checkpoint; release and requeue', flush=True)
finally:
    for ds in cache.values():
        for driver in ds.values():
            driver.close()
    for proc in nodes.values():
        proc.stdin.close()
        assert proc.wait() == 0
    record['bursts'].append({'seconds': time.monotonic() - began, 'admission': admission(), 'completed_rows': len(record['rows'])})
    assert record['sources'] == q.sources()
    assert all(q.m.sha(A / p) == digest for p, digest in record['binaries'].items())
    assert time.monotonic() - began < 840
    save()
