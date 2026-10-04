#!/usr/bin/env python3
"""Interleaved scalar C++/safe Rust 0.5 time comparisons under both consumer profiles."""
import ctypes as c
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time
from sweep import ART, FAMILIES, REF, ROOT, RUST, fixture, expected, api, U, F, B, Z

CPP = ART / 'libmeshopt-p05-bench.so'
SHAPES = {
    'geometry': [(8, 4), (128, 256), (1024, 2048)],
    'raster': [(8, 4), (16, 16), (32, 64)],
    'mesh': [(8, 4), (16, 32), (64, 128)],
    'remesh': [(8, 4), (16, 16), (32, 64)],
    'omm': [(8, 4), (64, 128), (256, 512)],
}


def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def shape_group(family):
    if family in ('overdraw', 'coverage'): return 'raster'
    if family == 'remesh': return 'remesh'
    if family in ('tangents', 'normals'): return 'mesh'
    if family.startswith('omm_'): return 'omm'
    return 'geometry'


def main():
    if len(sys.argv) != 3 or sys.argv[1] != '--consumer-profile' or sys.argv[2] not in ('moss', 'default'):
        raise SystemExit('usage: benchmark.py --consumer-profile moss|default')
    profile = sys.argv[2]
    target = Path(os.environ['CARGO_TARGET_DIR'])
    env = {**os.environ, 'CARGO_PROFILE_RELEASE_OPT_LEVEL': '3', 'CARGO_PROFILE_RELEASE_DEBUG': '0',
           'CARGO_PROFILE_RELEASE_LTO': 'thin' if profile == 'moss' else 'false',
           'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '1' if profile == 'moss' else '16'}
    subprocess.run(['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'parity/p05/Cargo.toml'], cwd=ROOT, env=env, check=True)
    rust = ART / f'benchmark-0.5-{profile}-rust'
    shutil.copyfile(RUST, rust)
    rust.chmod(0o755)
    cpp_cmd = ['g++', '-std=c++11', '-shared', '-fPIC', '-O3', '-fno-fast-math', '-ffp-contract=off', '-DMESHOPTIMIZER_NO_SIMD', '-I', str(REF / 'src'), str(ROOT / 'parity/p05/bench.cpp'), *map(str, sorted((REF / 'src').glob('*.cpp'))), '-o', str(CPP)]
    subprocess.run(cpp_cmd, check=True)
    lib = c.CDLL(str(CPP))
    bench = lib.p05_bench
    bench.restype = c.c_double
    bench.argtypes = [c.c_int, c.c_int, c.c_int, U, Z, Z, c.POINTER(U), c.POINTER(F), c.POINTER(F), c.POINTER(F), c.POINTER(B)]
    reference = api(lib)
    process = subprocess.Popen([str(rust)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)

    def rust_call(family, seed, n, triangles, iterations, caller):
        process.stdin.write(f'BENCH {family} {seed} {n} {triangles} {iterations} {"caller" if caller else "allocating"}\n'); process.stdin.flush()
        line = process.stdout.readline()
        if not line: raise RuntimeError('Rust benchmark runner exited')
        digest, nanoseconds = line.split()
        return digest, float(nanoseconds)

    rows = []
    for op, family in enumerate(FAMILIES):
        modes = [False, True] if family in ('stripify', 'unstripify', 'omm_rasterize', 'tangents', 'normals', 'remesh') else [False]
        for caller in modes:
            for size, (n, triangles) in enumerate(SHAPES[shape_group(family)]):
                seed = [0, 1, 2][size]
                data = fixture(seed, n, triangles)
                # Compare the full, untimed result before any time sample.
                process.stdin.write(f'{family} {seed} {n} {triangles}\n'); process.stdin.flush()
                digest, actual = process.stdout.readline().rstrip('\n').split(' ', 1)
                golden = expected(reference, family, seed, data).hex()
                if digest != f'{data[-1]:016x}' or actual != golden: raise RuntimeError(f'benchmark input/output parity failed: {family} {size}')
                count, positions, normals, uvs, indices, texture, _ = data
                ind = (U * len(indices))(*indices)
                pos = (F * len(positions))(*positions)
                norm = (F * len(normals))(*normals)
                uv = (F * len(uvs))(*uvs)
                tex = (B * 64).from_buffer_copy(texture)
                def cpp_call(iters): return bench(op, int(caller), iters, seed, count, len(indices), ind, pos, norm, uv, tex)
                # Calibrate to roughly 2 ms per sample; all timing remains in the drivers.
                _, rust_probe = rust_call(family, seed, n, triangles, 1, caller)
                cpp_probe = cpp_call(1)
                iterations = max(1, min(10000, round(2_000_000 / max(rust_probe, cpp_probe, 100))))
                rust_call(family, seed, n, triangles, iterations, caller)
                cpp_call(iterations)
                samples = []
                for sample in range(10):
                    if sample & 1:
                        cpp_ns = cpp_call(iterations)
                        input_hash, rust_ns = rust_call(family, seed, n, triangles, iterations, caller)
                    else:
                        input_hash, rust_ns = rust_call(family, seed, n, triangles, iterations, caller)
                        cpp_ns = cpp_call(iterations)
                    if input_hash != digest or rust_ns <= 0 or cpp_ns <= 0: raise RuntimeError('invalid sample')
                    samples.append({'rust_ns': rust_ns, 'cpp_ns': cpp_ns, 'ratio': rust_ns / cpp_ns})
                ratio = statistics.median(s['ratio'] for s in samples)
                row = {'family': family, 'api': 'caller' if caller else 'allocating', 'shape': size, 'vertices': n, 'triangles': triangles, 'seed': seed, 'input_hash': digest, 'iterations': iterations, 'samples': samples, 'median_ratio': ratio, 'rust_median_ns': statistics.median(s['rust_ns'] for s in samples), 'cpp_median_ns': statistics.median(s['cpp_ns'] for s in samples)}
                rows.append(row)
                print(profile, family, row['api'], size, f'{ratio:.3f}', flush=True)
    process.stdin.close(); process.wait()
    if process.returncode: raise RuntimeError('Rust benchmark runner failed')
    families = {}
    for family in FAMILIES:
        for caller in [False, True]:
            selected = [r for r in rows if r['family'] == family and r['api'] == ('caller' if caller else 'allocating')]
            if not selected: continue
            gm = math.exp(statistics.mean(math.log(r['median_ratio']) for r in selected))
            maximum = max(r['median_ratio'] for r in selected)
            families[f'{family}:{"caller" if caller else "allocating"}'] = {'cases': len(selected), 'geometric_mean': gm, 'maximum': maximum, 'passed': gm <= 1.25 and maximum <= 1.50}
    detail = ART / f'benchmark-0.5-{profile}.json'
    time_passed = all(v['passed'] for v in families.values())
    # The RFC also requires a comparable peak-storage ratio and a broader
    # corpus. Keep the overall acceptance bit false until both are supplied.
    full = {'schema': 'meshopt-p05-benchmark/1', 'phase': '0.5', 'profile': profile, 'bar': {'geometric_mean': 1.25, 'maximum': 1.5, 'maximum_memory_ratio': 1.25}, 'rows': rows, 'families': families, 'time_passed': time_passed, 'memory_passed': None, 'corpus_complete': False, 'passed': False, 'rust_sha256': sha(rust), 'cpp_sha256': sha(CPP), 'effective_profile_overrides': {k: env[k] for k in env if k.startswith('CARGO_PROFILE_RELEASE_')}}
    detail.write_text(json.dumps(full, indent=2) + '\n')
    summary = {k: full[k] for k in ['schema', 'phase', 'profile', 'bar', 'families', 'time_passed', 'memory_passed', 'corpus_complete', 'passed', 'rust_sha256', 'cpp_sha256', 'effective_profile_overrides']}
    summary.update({'detail_artifact': detail.name, 'detail_sha256': sha(detail)})
    (ROOT / f'parity/results/benchmark-{profile}-0.5.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps({'profile': profile, 'passed': full['passed'], 'time_passed': time_passed, 'memory_passed': None, 'corpus_complete': False, 'failures': {k: v for k, v in families.items() if not v['passed']}}, indent=2))
    raise SystemExit(0 if full['passed'] else 1)


if __name__ == '__main__': main()
