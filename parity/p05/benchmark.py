#!/usr/bin/env python3
"""Interleaved scalar C++/safe Rust 0.5 time comparisons under both consumer profiles."""
import argparse
import ctypes as c
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import statistics
import select
import struct
import subprocess
import sys
import time
from sweep import ART, FAMILIES, REF, ROOT, RUST, fixture, expected, api, U, F, B, Z
sys.path.insert(0, str(ROOT / 'parity'))
import quiet
from timing_gate import wait as wait_gate, cpu_ticks, cpu_utilization

CPP = ART / 'libmeshopt-p05-bench.so'
CALLER_FAMILIES = {'stripify', 'unstripify', 'omm_rasterize', 'tangents', 'normals', 'remesh'}
SCREEN_T = {5: 3.960786482770179, 10: 2.933324088373988, 20: 2.625105913222785}


def early_screen(samples):
    logs = [math.log(sample['ratio']) for sample in samples]
    center = statistics.mean(logs)
    margin = SCREEN_T[len(samples)] * statistics.stdev(logs) / math.sqrt(len(samples))
    lower, upper = math.exp(center - margin), math.exp(center + margin)
    return {'pairs': len(samples), 'lower': lower, 'upper': upper,
            'verdict': 'PASS' if upper <= 1.5 else 'FAIL' if lower > 1.5 else 'BORDERLINE'}


def stage2_screen(samples):
    if len(samples) != 30:
        raise ValueError('D146 requires exactly 30 fresh pairs')
    logs = [math.log(sample['ratio']) for sample in samples]
    center = statistics.mean(logs)
    margin = 2.045229642132703 * statistics.stdev(logs) / math.sqrt(30)
    lower, upper = math.exp(center - margin), math.exp(center + margin)
    return {'pairs': 30, 'lower': lower, 'upper': upper,
            'verdict': 'PASS' if upper <= 1.5 else 'FAIL' if lower > 1.5 else 'INCONCLUSIVE'}


SHAPES = {
    'geometry': [(8, 4, 0), (128, 256, 0), (1024, 2048, 0),
                 (4096, 8192, 1), (4096, 8192, 2), (8192, 8192, 3), (24576, 8192, 4),
                 (600000, 1000000, 1)],
    'raster': [(8, 4, 0), (16, 16, 0), (32, 64, 0),
               (256, 512, 1), (256, 512, 2), (512, 512, 3), (1536, 512, 4),
               (600000, 1000000, 1)],
    'mesh': [(8, 4, 0), (16, 32, 0), (64, 128, 0),
             (512, 1024, 1), (512, 1024, 2), (1024, 1024, 3), (3072, 1024, 4),
             (600000, 1000000, 1)],
    'remesh': [(8, 4, 0), (16, 16, 0), (32, 64, 0),
               (256, 512, 1), (256, 512, 2), (512, 512, 3), (1536, 512, 4),
               (600000, 1000000, 1)],
    'omm': [(8, 4, 0), (64, 128, 0), (256, 512, 0),
            (256, 512, 1), (256, 512, 2), (512, 512, 3)],
}


def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def shape_group(family):
    if family in ('overdraw', 'coverage'): return 'raster'
    if family == 'remesh': return 'remesh'
    if family in ('tangents', 'normals'): return 'mesh'
    if family == 'omm_measure': return 'geometry'
    if family.startswith('omm_'): return 'omm'
    return 'geometry'


def source_identity():
    paths = [*ROOT.glob('src/**/*.rs'), *(ROOT / 'parity/p05/src').glob('*.rs'),
             *(ROOT / 'parity/p05').glob('*.py'), *(ROOT / 'parity/p05').glob('*.cpp'),
             ROOT / 'parity/quiet.py', ROOT / 'Cargo.toml', ROOT / 'Cargo.lock',
             ROOT / 'parity/p05/Cargo.toml', ROOT / 'parity/p05/Cargo.lock']
    return {str(path.relative_to(ROOT)): sha(path) for path in sorted(paths)}


def reference_identity():
    paths = [*REF.glob('src/*.cpp'), *REF.glob('src/*.h')]
    return {str(path.relative_to(REF)): sha(path) for path in sorted(paths)}


def scoped_inventory(requested):
    return {(family, mode, shape)
            for family in requested
            for mode in (['allocating', 'caller'] if family in CALLER_FAMILIES else ['allocating'])
            for shape in range(len(SHAPES[shape_group(family)]))}


def scope_complete(rows, requested):
    inventory = scoped_inventory(requested)
    return bool(inventory) and len(rows) == len(inventory) and {
        (row['family'], row['api'], row['shape']) for row in rows} == inventory


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--consumer-profile', choices=('moss', 'default'), required=True)
    parser.add_argument('--families', required=True, help='comma-separated touched families; all only for final')
    parser.add_argument('--mode', choices=('diagnostic', 'lean', 'final'), default='diagnostic')
    parser.add_argument('--cases', help='diagnostic family:shape filters; other selected families remain complete')
    args = parser.parse_args()
    profile = args.consumer_profile
    requested = set(FAMILIES if args.families == 'all' else args.families.split(','))
    if not requested or not requested <= set(FAMILIES):
        raise SystemExit('unknown family selection')
    if args.mode in ('diagnostic', 'lean') and requested == set(FAMILIES):
        raise SystemExit('diagnostic/lean require an explicit touched-family subset')
    if args.mode == 'final' and requested != set(FAMILIES):
        raise SystemExit('final requires every family')
    case_filters = {}
    if args.cases:
        if args.mode != 'diagnostic':
            raise SystemExit('lean/final do not permit case filtering')
        try:
            for token in args.cases.split(','):
                family, shape = token.rsplit(':', 1)
                shape = int(shape)
                if family not in requested or not 0 <= shape < len(SHAPES[shape_group(family)]):
                    raise ValueError(token)
                case_filters.setdefault(family, set()).add(shape)
        except ValueError as error:
            raise SystemExit(f'invalid case filter: {error}') from error
    case_filters = {family: sorted(shapes) for family, shapes in case_filters.items()}
    purpose = {'diagnostic': 'diagnostic', 'lean': 'lean', 'final': 'benchmark'}[args.mode]
    pair_limit = 5 if args.mode == 'diagnostic' else 20
    source_sha256 = source_identity()
    reference_sha256 = reference_identity()
    env = {**os.environ, 'CARGO_PROFILE_RELEASE_OPT_LEVEL': '3', 'CARGO_PROFILE_RELEASE_DEBUG': '0',
           'CARGO_PROFILE_RELEASE_LTO': 'thin' if profile == 'moss' else 'false',
           'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '1' if profile == 'moss' else '16'}
    subprocess.run(['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'parity/p05/Cargo.toml'], cwd=ROOT, env=env, check=True)
    rust = ART / f'{purpose}-0.5-{profile}-rust'
    shutil.copyfile(RUST, rust)
    rust.chmod(0o755)
    cpp_cmd = ['g++', '-std=c++11', '-shared', '-fPIC', '-O3', '-fno-fast-math', '-ffp-contract=off', '-DMESHOPTIMIZER_NO_SIMD', '-I', str(REF / 'src'), str(ROOT / 'parity/p05/bench.cpp'), *map(str, sorted((REF / 'src').glob('*.cpp'))), '-o', str(CPP)]
    cpp_identity = {'command': cpp_cmd, 'reference': reference_sha256,
                    'driver': sha(ROOT / 'parity/p05/bench.cpp'),
                    'compiler': sha(Path(shutil.which('g++')).resolve())}
    cpp_manifest = ART / 'libmeshopt-p05-bench.build.json'
    cached = json.loads(cpp_manifest.read_text()) if cpp_manifest.is_file() else {}
    if not CPP.is_file() or cached.get('identity') != cpp_identity or cached.get('sha256') != sha(CPP):
        subprocess.run(cpp_cmd, check=True)
        cpp_manifest.write_text(json.dumps({'identity': cpp_identity, 'sha256': sha(CPP)}, indent=2) + '\n')
    identity = {'profile': profile, 'mode': args.mode, 'families': sorted(requested), 'case_filters': case_filters, 'source_sha256': source_sha256,
                'reference_sha256': reference_sha256,
                'rust_sha256': sha(rust), 'cpp_sha256': sha(CPP),
                'effective_profile_overrides': {k: env[k] for k in env if k.startswith('CARGO_PROFILE_RELEASE_')}}
    checkpoint = ART / f'{purpose}-0.5-{profile}.partial.json'
    previous = json.loads(checkpoint.read_text()) if checkpoint.is_file() else None
    wait_gate(f'benchmark-{profile}:core-selection')
    if previous and previous['identity'] == identity and previous['physical_core']['allowed_cpus'] == sorted(os.sched_getaffinity(0)):
        core, rows = quiet.physical_core(), previous['rows']
        print(f'resuming {profile}: {len(rows)} completed cases', flush=True)
    else:
        core, rows = quiet.physical_core(), []
        if previous:
            print('discarding checkpoint with different source, binary, profile or CPU availability', flush=True)
    os.sched_setaffinity(0, {core['cpu']})
    load_start = os.getloadavg()
    lib = c.CDLL(str(CPP))
    bench = lib.p05_bench
    bench.restype = c.c_double
    bench.argtypes = [c.c_int, c.c_int, c.c_int, U, Z, Z, c.POINTER(U), c.POINTER(F), c.POINTER(F), c.POINTER(F), c.POINTER(B)]
    lib.p05_install_memory()
    lib.p05_track_memory.argtypes = [c.c_int]
    lib.p05_peak_memory.restype = Z
    reference = api(lib)
    process = subprocess.Popen([str(rust)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)

    burst_started = float(os.environ['MESHOPT_TIMING_BURST_START'])
    core_selections = (previous.get('core_selections', [previous['physical_core']]) + [core]
                       if previous and previous['identity'] == identity
                       and previous['physical_core']['allowed_cpus'] == core['allowed_cpus'] else [core])

    def release():
        os.sched_setaffinity(0, set(core['allowed_cpus']))
        os.sched_setaffinity(process.pid, set(core['allowed_cpus']))

    def admit(label, fixed_core=None):
        nonlocal core
        release()
        record = wait_gate(label)
        if fixed_core is not None:
            core = fixed_core
        os.sched_setaffinity(0, {core['cpu']})
        os.sched_setaffinity(process.pid, {core['cpu']})
        return record

    def rust_call(family, seed, n, triangles, iterations, caller):
        process.stdin.write(f'BENCH {family} {seed} {n} {triangles} {style} {iterations} {"caller" if caller else "allocating"}\n'); process.stdin.flush()
        if not select.select([process.stdout], [], [], 30)[0]:
            process.kill()
            raise RuntimeError('Rust benchmark request exceeded 30 seconds')
        line = process.stdout.readline()
        if not line: raise RuntimeError('Rust benchmark runner exited')
        digest, nanoseconds, bytes_used = line.split()
        return digest, float(nanoseconds), int(bytes_used)

    completed = {(r['family'], r['api'], r['shape']) for r in rows}
    for op, family in enumerate(FAMILIES):
        if family not in requested: continue
        modes = [False, True] if family in CALLER_FAMILIES else [False]
        for caller in modes:
            for size, (n, triangles, style) in enumerate(SHAPES[shape_group(family)]):
                if family in case_filters and size not in case_filters[family]:
                    continue
                if (family, 'caller' if caller else 'allocating', size) in completed:
                    continue
                # Exit at a completed-case boundary and requeue fairly. The
                # enclosing lease command has an independent 840-second cap.
                if time.monotonic() - burst_started >= 600:
                    release()
                    process.stdin.close()
                    process.wait(timeout=30)
                    print('completed ten-minute lease burst; checkpoint retained', flush=True)
                    raise SystemExit(76)
                seed = size
                # Honor a lease that began during the preceding case's last
                # pair before preparing the next potentially large fixture.
                admit(f'benchmark-{profile}:{family}:{caller}:{size}:setup')
                data = fixture(seed, n, triangles, style)
                # Compare the full, untimed result before any time sample.
                process.stdin.write(f'{family} {seed} {n} {triangles} {style}\n'); process.stdin.flush()
                digest, actual = process.stdout.readline().rstrip('\n').split(' ', 1)
                golden_bytes = expected(reference, family, seed, data)
                if digest != f'{data[-1]:016x}' or actual != golden_bytes.hex(): raise RuntimeError(f'benchmark input/output parity failed: {family} {size}')
                count, positions, normals, uvs, indices, texture, _ = data
                ind = (U * len(indices))(*indices)
                pos = (F * len(positions))(*positions)
                norm = (F * len(normals))(*normals)
                uv = (F * len(uvs))(*uvs)
                tex = (B * 64).from_buffer_copy(texture)
                def cpp_call(iters): return bench(op, int(caller), iters, seed, count, len(indices), ind, pos, norm, uv, tex)
                admission = admit(f'benchmark-{profile}:{family}:{caller}:{size}:calibration')
                # Tiny bounds need enough calls to exceed clock quantization and
                # loop/control overhead by several orders of magnitude.
                _, rust_probe, rust_memory = rust_call(family, seed, n, triangles, 1, caller)
                cpp_probe = cpp_call(1)
                admit(f'benchmark-{profile}:{family}:{caller}:{size}:memory')
                lib.p05_track_memory(1)
                cpp_call(1)
                cpp_scratch = int(lib.p05_peak_memory())
                lib.p05_track_memory(0)
                cpp_output = 0
                if not caller:
                    if family == 'stripify': cpp_output = 4 * reference['strip_bound'](len(indices))
                    elif family == 'omm_measure': cpp_output = 9 * triangles
                    elif family == 'omm_rasterize': cpp_output = reference['entry'](seed % 4, 2 if seed & 1 == 0 else 4)
                    elif family == 'tangents': cpp_output = 16 * len(indices)
                    elif family == 'normals': cpp_output = 12 * len(indices)
                    elif family == 'remesh': cpp_output = 36 * struct.unpack_from('<Q', golden_bytes)[0]
                if family == 'unstripify' and not caller:
                    prepared = (U * reference['strip_bound'](len(indices)))()
                    strip_len = reference['strip'](prepared, ind, len(indices), count, 0 if seed & 1 == 0 else 65535)
                    cpp_output = 4 * reference['unstrip_bound'](strip_len)
                cpp_memory = cpp_scratch + cpp_output
                memory_ratio = (rust_memory / cpp_memory if cpp_memory else (1.0 if rust_memory == 0 else None))
                iterations = max(100_000 if family in ('stripify_bound', 'unstripify_bound', 'omm_entry_size') else 1,
                                 min(10_000_000, round(20_000_000 / max(rust_probe, cpp_probe, 1))))
                admit(f'benchmark-{profile}:{family}:{caller}:{size}:warmup')
                _, rust_warm, _ = rust_call(family, seed, n, triangles, iterations, caller)
                cpp_warm = cpp_call(iterations)
                initial_iterations = iterations
                # Cold one-call allocation/dispatch can greatly overestimate
                # resident per-call cost. Recalibrate both languages from the
                # same warmed batch, preserving the 20ms target and caps.
                iterations = max(100_000 if family in ('stripify_bound', 'unstripify_bound', 'omm_entry_size') else 1,
                                 min(10_000_000, round(20_000_000 / max(rust_warm, cpp_warm, 1))))
                calibration = {'initial_iterations': initial_iterations,
                               'cold_rust_ns': rust_probe, 'cold_cpp_ns': cpp_probe,
                               'warm_rust_ns': rust_warm, 'warm_cpp_ns': cpp_warm,
                               'target_backend_ns': 20_000_000,
                               'final_iterations': iterations}
                if iterations != initial_iterations:
                    admit(f'benchmark-{profile}:{family}:{caller}:{size}:refined-warmup')
                    rust_call(family, seed, n, triangles, iterations, caller)
                    cpp_call(iterations)
                samples = []
                screens = []
                for sample in range(pair_limit):
                    admission = admit(f'benchmark-{profile}:{family}:{caller}:{size}:pair-{sample}')
                    load_before = os.getloadavg()
                    cpu_before = cpu_ticks()
                    started = time.monotonic()
                    if sample & 1:
                        cpp_ns = cpp_call(iterations)
                        input_hash, rust_ns, _ = rust_call(family, seed, n, triangles, iterations, caller)
                    else:
                        input_hash, rust_ns, _ = rust_call(family, seed, n, triangles, iterations, caller)
                        cpp_ns = cpp_call(iterations)
                    if input_hash != digest or rust_ns <= 0 or cpp_ns <= 0: raise RuntimeError('invalid sample')
                    samples.append({'pair_index': sample, 'order': 'cpp-rust' if sample & 1 else 'rust-cpp', 'rust_ns': rust_ns, 'cpp_ns': cpp_ns, 'ratio': rust_ns / cpp_ns,
                                    'load_before': load_before, 'load_after': os.getloadavg(),
                                    'elapsed_seconds': time.monotonic() - started, 'cpu': core['cpu'],
                                    'cpu_utilization': cpu_utilization(cpu_before, cpu_ticks(), core['allowed_cpus']),
                                    'timing_admission': admission})
                    release()
                    if args.mode != 'diagnostic' and len(samples) in SCREEN_T:
                        screens.append(early_screen(samples))
                        if screens[-1]['verdict'] != 'BORDERLINE':
                            break
                stage2 = None
                maximum_verdict = (screens[-1]['verdict'] if screens else
                                   ('PASS' if statistics.median(s['ratio'] for s in samples) <= 1.5 else 'FAIL'))
                if args.mode != 'diagnostic' and maximum_verdict == 'BORDERLINE':
                    # A new sample stream, never pooled with stage-one screening.
                    # Keep this core fixed through any admission/burst pause.
                    release()
                    wait_gate(f'benchmark-{profile}:{family}:{caller}:{size}:D146-core')
                    fixed_core = quiet.physical_core()
                    core = fixed_core
                    core_selections.append(core)
                    fresh = []
                    for pair in range(30):
                        admission = admit(f'benchmark-{profile}:{family}:{caller}:{size}:D146-{pair}', fixed_core)
                        load_before, cpu_before = os.getloadavg(), cpu_ticks()
                        started = time.monotonic()
                        if pair & 1:
                            cpp_ns = cpp_call(iterations)
                            input_hash, rust_ns, _ = rust_call(family, seed, n, triangles, iterations, caller)
                        else:
                            input_hash, rust_ns, _ = rust_call(family, seed, n, triangles, iterations, caller)
                            cpp_ns = cpp_call(iterations)
                        if input_hash != digest or rust_ns <= 0 or cpp_ns <= 0:
                            raise RuntimeError('invalid D146 sample')
                        fresh.append({'pair_index': pair, 'order': 'cpp-rust' if pair & 1 else 'rust-cpp', 'rust_ns': rust_ns, 'cpp_ns': cpp_ns, 'ratio': rust_ns / cpp_ns,
                                      'load_before': load_before, 'load_after': os.getloadavg(),
                                      'elapsed_seconds': time.monotonic() - started, 'cpu': core['cpu'],
                                      'cpu_utilization': cpu_utilization(cpu_before, cpu_ticks(), core['allowed_cpus']),
                                      'timing_admission': admission})
                        release()
                    stage2 = {'samples': fresh, 'interval': stage2_screen(fresh),
                              'physical_core': fixed_core, 'input_hash': digest,
                              'rust_sha256': identity['rust_sha256'], 'cpp_sha256': identity['cpp_sha256']}
                    maximum_verdict = stage2['interval']['verdict']
                ratio = statistics.median(s['ratio'] for s in samples)
                row = {'family': family, 'api': 'caller' if caller else 'allocating', 'shape': size, 'style': style, 'vertices': n, 'triangles': triangles, 'seed': seed, 'input_hash': digest, 'iterations': iterations, 'calibration': calibration, 'samples': samples, 'early_screens': screens, 'maximum_verdict': maximum_verdict, 'stage2': stage2, 'cpus': sorted({s['cpu'] for s in samples}), 'median_ratio': ratio, 'rust_median_ns': statistics.median(s['rust_ns'] for s in samples), 'cpp_median_ns': statistics.median(s['cpp_ns'] for s in samples), 'rust_memory_bytes': rust_memory, 'cpp_memory_bytes': cpp_memory, 'cpp_scratch_bytes': cpp_scratch, 'cpp_output_bytes': cpp_output, 'memory_ratio': memory_ratio}
                rows.append(row)
                pending = checkpoint.with_suffix('.pending')
                pending.write_text(json.dumps({'identity': identity, 'physical_core': core, 'core_selections': core_selections, 'rows': rows}, separators=(',', ':'), allow_nan=False) + '\n')
                pending.replace(checkpoint)
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
            memory_max = max((r['memory_ratio'] if r['memory_ratio'] is not None else math.inf) for r in selected)
            maximum_passed = all(r['maximum_verdict'] == 'PASS' for r in selected)
            families[f'{family}:{"caller" if caller else "allocating"}'] = {'cases': len(selected), 'geometric_mean': gm, 'maximum': maximum, 'maximum_memory_ratio': None if math.isinf(memory_max) else memory_max, 'maximum_passed': maximum_passed, 'passed': gm <= 1.25 and maximum_passed and memory_max <= 1.25}
    detail = ART / f'{purpose}-0.5-{profile}.json'
    time_passed = all(v['geometric_mean'] <= 1.25 and v['maximum_passed'] for v in families.values())
    memory_passed = all(r['memory_ratio'] is not None and r['memory_ratio'] <= 1.25 for r in rows)
    corpus_complete = all({r['style'] for r in rows if r['family'] == f} == {0, 1, 2, 3, 4} and
                          any(r['triangles'] >= 1_000_000 for r in rows if r['family'] == f)
                          for f in FAMILIES if shape_group(f) != 'omm')
    if source_identity() != source_sha256 or reference_identity() != reference_sha256:
        raise RuntimeError('source changed during benchmark; records cannot be qualified')
    selected_complete = scope_complete(rows, requested)
    full = {'scope_complete': selected_complete, 'scope_passed': time_passed and memory_passed and selected_complete, 'schema': 'meshopt-p05-benchmark/2', 'phase': '0.5', 'profile': profile, 'bar': {'geometric_mean': 1.25, 'maximum': 1.5, 'maximum_memory_ratio': 1.25}, 'rows': rows, 'families': families, 'time_passed': time_passed, 'memory_passed': memory_passed, 'corpus_complete': corpus_complete, 'passed': time_passed and memory_passed and corpus_complete, 'physical_core': core, 'core_selections': core_selections, 'mode': args.mode, 'selected_families': sorted(requested), 'case_filters': case_filters, 'load_start': load_start, 'rust_sha256': sha(rust), 'cpp_sha256': sha(CPP), 'effective_profile_overrides': {k: env[k] for k in env if k.startswith('CARGO_PROFILE_RELEASE_')}, 'source_sha256': source_sha256, 'reference_sha256': reference_sha256}
    detail.write_text(json.dumps(full, indent=2, allow_nan=False) + '\n')
    summary = {k: full[k] for k in ['schema', 'phase', 'profile', 'bar', 'families', 'time_passed', 'memory_passed', 'corpus_complete', 'passed', 'rust_sha256', 'cpp_sha256', 'effective_profile_overrides', 'source_sha256', 'reference_sha256']}
    summary.update({'detail_artifact': detail.name, 'detail_sha256': sha(detail)})
    if args.mode == 'final':
        (ROOT / f'parity/results/benchmark-{profile}-0.5.json').write_text(json.dumps(summary, indent=2, allow_nan=False) + '\n')
    checkpoint.unlink()
    print(json.dumps({'profile': profile, 'passed': full['passed'], 'time_passed': time_passed, 'memory_passed': memory_passed, 'corpus_complete': corpus_complete, 'scope_complete': selected_complete, 'scope_passed': full['scope_passed'], 'failures': {k: v for k, v in families.items() if not v['passed']}}, indent=2))
    raise SystemExit(0 if (full['scope_passed'] if args.mode == 'lean' else full['passed']) else 1)


if __name__ == '__main__': main()
