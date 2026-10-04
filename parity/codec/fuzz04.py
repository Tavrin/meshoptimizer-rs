#!/usr/bin/env python3
"""Run the nineteen 0.4 cargo-fuzz targets as 300-second ASan/libFuzzer smokes.

Encoder targets assert their round trips, so a crash is a parity defect as
well as a robustness one. Corpora, logs and binary identities are retained.
This smoke does not establish the RFC's four-CPU-hour release fuzz gate.
"""
import concurrent.futures
import json
import os
from pathlib import Path
import random
import re
import struct
import subprocess
import time
from measure import ROOT, TARGET, ART, sha

NAMES = ['vertex_encode', 'vertex_encode_into', 'index_encode', 'index_encode_into', 'sequence_encode',
         'sequence_encode_into', 'oct_encode', 'quat_encode', 'exp_encode', 'color_encode', 'filter_encode_into',
         'color', 'meshlet_encode', 'meshlet_encode_into', 'meshlet_decode', 'meshlet_decode_into', 'meshlet_raw',
         'meshlet_raw_into', 'bounds']
TOOLS = Path(os.environ.get('MESHOPT_FUZZ_TOOLS', ART.parent))
FUZZ = TOOLS / 'tools/bin/cargo-fuzz'
ENV = dict(os.environ, CARGO_HOME=str(TOOLS / 'cargo-home'), CARGO_TARGET_DIR=str(TARGET), CARGO_NET_OFFLINE='true',
           RUSTC_BOOTSTRAP='1', RUSTC_WRAPPER='', TMPDIR=str(TARGET / 'tmp'))


def header(count, stride, a, b, out=255):
    return struct.pack('<IHBBB', count, stride, a, b, out)


def seeds(name, rng):
    """Small valid-looking inputs for each entry point (count, stride, a, b, out-size, payload)."""
    out = []
    for _ in range(8):
        if name.startswith('vertex_encode'):
            stride = rng.choice([4, 12, 16, 32])
            out.append(header(0, stride, rng.randrange(20), 0) + rng.randbytes(stride * rng.randrange(1, 40)))
        elif name.startswith(('index_encode', 'sequence_encode')):
            values = [rng.randrange(64) for _ in range(rng.randrange(0, 60))]
            out.append(header(0, 4, rng.randrange(2), 0) + struct.pack(f'<{len(values)}I', *values))
        elif name in ('oct_encode', 'quat_encode', 'exp_encode', 'color_encode', 'filter_encode_into'):
            values = [rng.uniform(-1, 1) for _ in range(4 * rng.randrange(1, 16))]
            out.append(header(0, rng.choice([4, 8, 12]), rng.randrange(4), rng.randrange(2, 17)) + struct.pack(f'<{len(values)}f', *values))
        elif name == 'color':
            count = rng.randrange(1, 16)
            out.append(header(count, rng.choice([4, 8]), 0, 0) + rng.randbytes(count * 8))
        elif name.startswith('meshlet_encode'):
            vc = rng.randrange(1, 64)
            tri = bytes(rng.randrange(vc) for _ in range(3 * rng.randrange(0, 40)))
            out.append(header(0, 0, vc, 0) + rng.randbytes(vc * 4) + tri)
        elif name.startswith('meshlet'):
            out.append(header(rng.randrange(8), 0, rng.randrange(64), rng.randrange(64)) + rng.randbytes(rng.randrange(16, 300)))
        else:
            out.append(header(rng.randrange(1 << 20), rng.choice([4, 16, 6, 0]), 0, 0))
    return out


def main():
    subprocess.run([FUZZ, 'fuzz', 'build', '--fuzz-dir', ROOT / 'fuzz/codec', '--target-dir', TARGET], env=ENV, check=True)
    before = {str(p.relative_to(ROOT)): sha(p) for p in ROOT.glob('src/**/*.rs')}
    cpus = sorted(os.sched_getaffinity(0))
    reserved = int(os.environ.get('MESHOPT_BENCH_CPU', cpus[0]))
    siblings = Path(f'/sys/devices/system/cpu/cpu{reserved}/topology/thread_siblings_list').read_text().strip()
    excluded = set()
    for group in siblings.split(','):
        a, _, b = group.partition('-')
        excluded.update(range(int(a), int(b or a) + 1))
    cpus = [c for c in cpus if c not in excluded]
    binary_dir = TARGET / 'x86_64-unknown-linux-gnu/release'

    def run(index, name):
        corpus = ART / 'fuzz' / name / 'corpus'
        crashes = ART / 'fuzz' / name / 'crashes'
        corpus.mkdir(parents=True, exist_ok=True)
        crashes.mkdir(exist_ok=True)
        for i, seed in enumerate(seeds(name, random.Random(f'{name}-20261004'))):
            (corpus / f'seed-{i}').write_bytes(seed)
        executable = binary_dir / name
        identity = sha(executable)
        cpu = cpus[index % len(cpus)]
        command = ['taskset', '-c', str(cpu), str(executable), str(corpus), '-max_total_time=300', '-seed=20261004',
                   '-max_len=8192', '-timeout=10', '-rss_limit_mb=512', f'-artifact_prefix={crashes}/']
        log = ART / 'fuzz' / name / 'smoke.log'
        start = time.monotonic()
        with log.open('w') as f:
            result = subprocess.run(command, env=ENV, stdout=f, stderr=subprocess.STDOUT)
        elapsed = time.monotonic() - start
        matches = re.findall(r'Done (\d+) runs', log.read_text())
        if result.returncode != 0 or elapsed < 300 or not matches or list(crashes.iterdir()) or identity != sha(executable):
            raise AssertionError((name, result.returncode, elapsed))
        record = {'target': name, 'elapsed_seconds': elapsed, 'executions': int(matches[-1]), 'exit_code': result.returncode,
                  'binary_sha256': identity, 'log': str(log), 'log_sha256': sha(log), 'command': command, 'cpu': cpu,
                  'sanitizer': 'address', 'compiler': 'stable with RUSTC_BOOTSTRAP=1, instrumented by cargo-fuzz 0.13.2',
                  'libfuzzer_sys': '0.4.13'}
        print(name, 'PASS', record['executions'], flush=True)
        return record
    with concurrent.futures.ThreadPoolExecutor(max_workers=len(NAMES)) as pool:
        rows = list(pool.map(lambda pair: run(*pair), enumerate(NAMES)))
    if before != {str(p.relative_to(ROOT)): sha(p) for p in ROOT.glob('src/**/*.rs')}:
        raise ValueError('sources changed during fuzzing')
    (ART / 'fuzz.json').write_text(json.dumps({'phase': '0.4', 'profile': 'asan-libfuzzer-300s', 'sources': before, 'targets': rows,
                                               'release_four_cpu_hours_per_target': 'not established by this 300-second smoke'}, indent=2))


if __name__ == '__main__':
    main()
