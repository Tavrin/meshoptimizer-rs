#!/usr/bin/env python3
"""Record perf stat/record for the worst case of each failing 0.5 family."""
import hashlib
import gzip
import json
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import time

from sweep import ART, FAMILIES, ROOT, fixture
sys.path.insert(0, str(ROOT / "parity"))
import quiet
from timing_gate import wait as wait_gate, cpu_ticks, cpu_utilization

PERF = Path('/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf')


def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    profile = sys.argv[1] if len(sys.argv) > 1 else 'moss'
    diagnostic = '--diagnostic' in sys.argv[2:]
    purpose = 'diagnostic' if diagnostic else 'benchmark'
    detail = json.loads((ART / f'{purpose}-0.5-{profile}.json').read_text())
    out = ART / (f'profiles-0.5-{profile}' if not diagnostic else f'diagnostic-profiles-0.5-{profile}')
    out.mkdir(parents=True, exist_ok=True)
    cpp = out / 'cpp-runner'
    subprocess.run(['g++', '-std=c++11', '-O3', '-g', str(ROOT / 'parity/p05/profile.cpp'),
                    '-L', str(ART), '-l:libmeshopt-p05-bench.so', '-Wl,-rpath,' + str(ART), '-o', str(cpp)], check=True)
    rust = ART / f'{purpose}-0.5-{profile}-rust'
    benchmark_cpu = detail['physical_core']['cpu']
    cpu = benchmark_cpu
    if sha(rust) != detail['rust_sha256'] or sha(ART / 'libmeshopt-p05-bench.so') != detail['cpp_sha256']:
        raise RuntimeError('profiling binaries do not match benchmark')
    rows = detail['rows']
    summary_path = out / 'summary.json'
    previous = json.loads(summary_path.read_text()) if summary_path.is_file() else {}
    records = (previous.get('families', {})
               if previous.get('benchmark_sha256') == sha(ART / f'{purpose}-0.5-{profile}.json')
               and previous.get('rust_sha256') == sha(rust)
               and previous.get('cpp_sha256') == sha(cpp) else {})
    burst_started = float(os.environ['MESHOPT_TIMING_BURST_START'])
    for op, family in enumerate(FAMILIES):
        candidates = [r for r in rows if r['family'] == family and not detail['families'][family + ':' + r['api']]['passed']]
        if not candidates or family in records: continue
        if time.monotonic() - burst_started >= 600:
            print('completed ten-minute profile lease burst; checkpoint retained', flush=True)
            raise SystemExit(76)
        row = max(candidates, key=lambda r: r['median_ratio'])
        caller = int(row['api'] == 'caller')
        seed, n, triangles = row['seed'], row['vertices'], row['triangles']
        style = row['style']
        data = fixture(seed, n, triangles, style)
        path = out / f'{family}.input'
        with path.open('wb') as f:
            count, positions, normals, uvs, indices, texture, _ = data
            f.write(struct.pack('<II', count, len(indices)))
            for values, code in [(positions, 'f'), (normals, 'f'), (uvs, 'f'), (indices, 'I')]:
                f.write(struct.pack('<' + str(len(values)) + code, *values))
            f.write(texture)
        # Amortize fixture setup so whole-process instruction counts mostly
        # describe repeated algorithm calls, even for the million-face cases.
        iterations = max(10, min(1_000_000, round(1_000_000_000 / max(row['rust_median_ns'], row['cpp_median_ns'], 1))))
        commands = {
            'cpp': [str(cpp), str(op), str(caller), str(iterations), str(seed), str(path)],
            'rust': [str(rust)],
        }
        entry = {'row': {k: row[k] for k in ('family', 'api', 'seed', 'vertices', 'triangles', 'style', 'median_ratio')},
                 'iterations': iterations, 'input_sha256': sha(path), 'backends': {}}
        for backend, command in commands.items():
            stdin = (f'PROFILE {family} {seed} {n} {triangles} {style} {iterations} {row["api"]}\n' if backend == 'rust' else None)
            label = f'profile-{profile}:{family}:{backend}:stat'
            stat_admission = wait_gate(label)
            if backend == 'cpp':
                # Select near first admission, then preserve this case's core
                # across all four runs and any intervening lease pause.
                while True:
                    core = quiet.physical_core()
                    started_wait = time.monotonic()
                    stat_admission = wait_gate(label + ':after-core-selection')
                    if time.monotonic() - started_wait <= 30:
                        break
                if core['allowed_cpus'] != detail['physical_core']['allowed_cpus']:
                    raise RuntimeError('CPU inventory changed since benchmark')
                cpu = core['cpu']
                entry['physical_core'] = core
            command = ['taskset', '-c', str(cpu), *command]
            args = [str(PERF), 'stat', '-e', 'instructions:u,cycles:u,branch-misses:u', '--', *command]
            stat_before = cpu_ticks()
            stat = subprocess.run(args, input=stdin, text=True, capture_output=True, timeout=30)
            stat_usage = cpu_utilization(stat_before, cpu_ticks(), detail['physical_core']['allowed_cpus'])
            stat_load_after = os.getloadavg()
            data_file = out / f'{family}-{backend}.data'
            record_admission = wait_gate(f'profile-{profile}:{family}:{backend}:record')
            record_before = cpu_ticks()
            record = subprocess.run([str(PERF), 'record', '-g', '-o', str(data_file), '--', *command],
                                    input=stdin, text=True, capture_output=True, timeout=30)
            record_usage = cpu_utilization(record_before, cpu_ticks(), detail['physical_core']['allowed_cpus'])
            record_load_after = os.getloadavg()
            report = subprocess.run([str(PERF), 'report', '--stdio', '--no-children', '--percent-limit', '1', '-i', str(data_file)],
                                    text=True, capture_output=True) if record.returncode == 0 else None
            match = re.search(r'([\d,]+)\s+instructions:u', stat.stderr)
            instructions = int(match.group(1).replace(',', '')) if match else None
            entry['backends'][backend] = {'stat_exit': stat.returncode, 'stat': stat.stderr, 'output': stat.stdout,
                'record_exit': record.returncode, 'record': record.stderr,
                'perf_data_sha256': sha(data_file) if data_file.is_file() else None,
                'report': report.stdout if report else None, 'instructions': instructions,
                'stat_admission': stat_admission, 'record_admission': record_admission}
            entry['backends'][backend].update(stat_cpu_utilization=stat_usage, record_cpu_utilization=record_usage,
                                             stat_load_after=stat_load_after, record_load_after=record_load_after)
        compressed = path.with_suffix('.input.gz')
        with path.open('rb') as source, gzip.open(compressed, 'wb', compresslevel=6) as destination:
            shutil.copyfileobj(source, destination)
        entry['input_artifact'] = compressed.name
        entry['input_artifact_sha256'] = sha(compressed)
        path.unlink()
        records[family] = entry
        (out / 'summary.json').write_text(json.dumps({'profile': profile, 'benchmark_sha256': sha(ART / f'{purpose}-0.5-{profile}.json'),
            'rust_sha256': sha(rust), 'cpp_sha256': sha(cpp), 'benchmark_cpu': benchmark_cpu, 'families': records}, indent=2) + '\n')
        print(f'profiled {family}', flush=True)
    detail_path = out / 'summary.json'
    detail_path.write_text(json.dumps({'profile': profile, 'benchmark_sha256': sha(ART / f'{purpose}-0.5-{profile}.json'),
        'rust_sha256': sha(rust), 'cpp_sha256': sha(cpp), 'benchmark_cpu': benchmark_cpu, 'families': records}, indent=2) + '\n')
    failing = {f for f in FAMILIES if any(not detail['families'][k]['passed'] for k in detail['families'] if k.startswith(f + ':'))}
    complete = set(records) == failing and all(
        all(v['stat_exit'] == 0 and v['record_exit'] == 0 and v['instructions'] and v['perf_data_sha256']
            for v in item['backends'].values()) for item in records.values())
    result = {'schema': 'meshopt-p05-profile/1', 'phase': '0.5', 'profile': profile, 'profile_complete': complete,
              'benchmark_sha256': sha(ART / f'{purpose}-0.5-{profile}.json'),
              'detail_artifact': str(detail_path.relative_to(ART)), 'detail_sha256': sha(detail_path),
              'families': {f: {'cpp_instructions': item['backends']['cpp']['instructions'],
                              'rust_instructions': item['backends']['rust']['instructions'],
                              'ratio': item['backends']['rust']['instructions'] / item['backends']['cpp']['instructions']}
                           for f, item in records.items()}}
    if not diagnostic:
        (ROOT / f'parity/results/profile-{profile}-0.5.json').write_text(json.dumps(result, indent=2) + '\n')
    if not complete:
        raise SystemExit('incomplete perf evidence')


if __name__ == '__main__': main()
