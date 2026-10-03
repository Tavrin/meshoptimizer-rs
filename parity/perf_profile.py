#!/usr/bin/env python3
"""Paired hardware profiles and disassembly; never establish a timing ceiling."""
import argparse
import json
import os
from pathlib import Path
import struct
import subprocess
import time
import zipfile

import quiet
import runner as r


def counters(text):
    result = {}
    for line in text.splitlines():
        fields = line.split(',')
        if len(fields) > 5 and fields[0].isdigit():
            result[fields[2].split(':')[0]] = {'count': int(fields[0]),
                                             'scheduled_percent': float(fields[4])}
    required = {'cycles', 'instructions', 'branch-misses', 'cache-misses'}
    if set(result) != required or any(v['count'] <= 0 or v['scheduled_percent'] <= 0 for v in result.values()):
        raise ValueError('missing, unsupported or uncounted hardware events')
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--benchmark', type=Path, required=True)
    parser.add_argument('--archive', type=Path, required=True)
    parser.add_argument('--perf', type=Path, default=Path('/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf'))
    parser.add_argument('--family', action='append', choices=list(r.FAMILIES.values()),
                        help='profile the measured maximum in this family even when its bar passes; repeatable')
    args = parser.parse_args()
    reference, target, results = r.paths()
    benchmark = json.loads(args.benchmark.read_text())
    before = r.snapshot(reference)
    # Historical selection is useful diagnostics if algorithm sources are exact.
    for name, expected in benchmark['identities']['source_sha256']['rust_and_harness'].items():
        if name.startswith('src/') or name in {'Cargo.toml', 'Cargo.lock'}:
            if before['rust_and_harness'][name] != expected:
                raise ValueError('stale profile case selection: '+name)
    if r.sha(args.archive) != benchmark['artifacts']['benchmark-buffers.zip']:
        raise ValueError('changed benchmark archive')
    binaries = {'rust': target/'release/meshopt-driver',
                'cpp': target/'qualification-build/reference'}
    hashes = {name: r.sha(binary) for name, binary in binaries.items()}
    directory = r.artifacts_path()/'perf-profiles'
    directory.mkdir(exist_ok=True)
    core = benchmark.get('physical_core') or quiet.physical_core()
    selected = args.family or [f for f, summary in benchmark['families'].items() if not summary['passed']]
    record = {'schema': 1, 'selected_families': selected, 'started_unix': time.time(), 'source_sha256': before,
              'executable_sha256': hashes, 'physical_core': core,
              'selection_benchmark_sha256': r.sha(args.benchmark),
              'selection_quiet_qualified': benchmark.get('quiet_qualified', False),
              'perf': str(args.perf), 'perf_version': r.text_command([args.perf, '--version']),
              'method': 'maximum measured case per selected family; 100 samples, 64 calls per tiny sample; hardware stat and cycles sampling in separate processes; parsing, one warmup and serialization included in counters; exact final output comparison',
              'limits': 'diagnostic profiles under recorded load; no quiet timing or universal safe-Rust ceiling claim',
              'runs': [], 'disassembly': {}, 'passed': False}
    for backend, binary in binaries.items():
        path = directory/(backend+'-disassembly.txt')
        with path.open('wb') as output:
            r.command(['objdump', '-Cd', '--no-show-raw-insn', binary], stdout=output)
        record['disassembly'][backend] = {'path': str(path), 'sha256': r.sha(path)}
    with zipfile.ZipFile(args.archive) as archive:
        for family, summary in benchmark['families'].items():
            if family not in selected:
                continue
            name, case = max(((n, w) for n, w in benchmark['workloads'].items() if w['family'] == family),
                             key=lambda nw: nw[1]['rust_cpp_ratio'])
            data = bytearray(archive.read(case['input']['member']))
            struct.pack_into('<I', data, 24, 100)
            expected_frame = archive.read(case['outputs'][0]['member'])
            expected = r.response(expected_frame[:-8], samples=struct.unpack_from('<I', expected_frame, 12)[0])[0]
            inp = directory/(family+'.input')
            inp.write_bytes(data)
            for backend, binary in binaries.items():
                stem = family+'-'+backend
                period = max(10000, int(case['stats'][backend]['median_seconds']*100*(64 if case['size']=='tiny' else 1)*3e9/1000))
                commands = {
                    'stat': [str(args.perf), 'stat', '-x,', '-e', 'cycles,instructions,branch-misses,cache-misses', '--',
                             'taskset', '-c', str(core['cpu']), str(binary)],
                    'record': [str(args.perf), 'record', '-e', 'cycles:u', '-c', str(period),
                               '--call-graph', 'dwarf,32768', '-o', str(directory/(stem+'.data')), '--',
                               'taskset', '-c', str(core['cpu']), str(binary)]}
                for kind, command in commands.items():
                    start = time.time()
                    load_start = quiet.load()
                    result = subprocess.run(command, input=bytes(data), capture_output=True, env=r.ENV, timeout=300)
                    output = directory/(stem+'-'+kind+'.output')
                    diagnostics = directory/(stem+'-'+kind+'.stderr')
                    output.write_bytes(result.stdout)
                    diagnostics.write_bytes(result.stderr)
                    run = {'case': name, 'backend': backend, 'kind': kind, 'command': command,
                           'exit': result.returncode, 'elapsed_seconds': time.time()-start,
                           'load_start': load_start, 'load_end': quiet.load(),
                           'input_sha256': r.sha(inp), 'output_sha256': r.sha(output),
                           'diagnostics_sha256': r.sha(diagnostics), 'match': False}
                    if result.returncode == 0:
                        values, times = r.response(result.stdout[:-8], samples=100)
                        run.update(match=values == expected, timed_seconds=times)
                        if kind == 'stat':
                            run['counters'] = counters(result.stderr.decode())
                    record['runs'].append(run)
                    if result.returncode or not run['match']:
                        (results/'perf-profiles.json').write_text(json.dumps(record, indent=2)+'\n')
                        raise ValueError(f'profile failed: {stem}/{kind}; see {diagnostics}')
                for kind, command in {
                    'report': [str(args.perf), 'report', '--stdio', '--no-children', '--percent-limit', '0.5', '-i', str(directory/(stem+'.data'))],
                    'annotate': [str(args.perf), 'annotate', '--stdio', '-l', '--percent-limit', '5', '-i', str(directory/(stem+'.data'))]
                }.items():
                    path = directory/(stem+'-'+kind+'.txt')
                    diagnostics = directory/(stem+'-'+kind+'.stderr')
                    with path.open('wb') as output, diagnostics.open('wb') as errors:
                        result = subprocess.run(command, stdout=output, stderr=errors, env=r.ENV, timeout=300)
                    record['runs'][-1][kind] = {'path': str(path), 'sha256': r.sha(path),
                                               'exit': result.returncode, 'diagnostics_sha256': r.sha(diagnostics)}
                print(f'profiled {name}: {backend}; exact output; load {quiet.load()[0]:.2f}', flush=True)
    record.update(finished_unix=time.time(), identities_unchanged=before == r.snapshot(reference) and
                  hashes == {name: r.sha(binary) for name, binary in binaries.items()})
    record['passed'] = record['identities_unchanged'] and all(run['match'] and run['exit']==0 and
        (run['kind'] != 'record' or all(run[k]['exit']==0 for k in ['report', 'annotate'])) for run in record['runs'])
    (results/'perf-profiles.json').write_text(json.dumps(record, indent=2)+'\n')
    if not record['passed']:
        raise SystemExit('profile identity check failed')


if __name__ == '__main__':
    main()
