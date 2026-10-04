#!/usr/bin/env python3
"""D146 fresh paired maximum checks on archived full-matrix inputs/binaries."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import statistics
import time
import zipfile

import performance
import quiet
import runner as r

ROOT = Path(__file__).resolve().parent.parent
T_CRITICAL_29 = 2.045229642132703
BAR = 1.5
STAGE1 = ('p01x-benchmark-moss', 'p01x-benchmark-defaults',
          'benchmark-moss', 'benchmark-default')


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write(path, value):
    Path(path).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def rule_hash():
    text = (ROOT / 'parity/DECISIONS.md').read_text()
    rule = text.split('## D146:', 1)[1].split('\n## D', 1)[0]
    return hashlib.sha256(rule.encode()).hexdigest()


def stage1(artifacts):
    selected = {}
    records = {}
    for name in STAGE1:
        summary_path = ROOT / 'parity/results' / (name + '.json')
        summary = json.loads(summary_path.read_text())
        detail_path = artifacts / summary['detail_artifact']
        if sha(detail_path) != summary['artifacts'][summary['detail_artifact']]:
            raise ValueError('changed stage-1 detailed record: ' + name)
        full = json.loads(detail_path.read_text())
        if len(full['workloads']) != (912 if name.startswith('p01x-') else 204):
            raise ValueError('incomplete stage-1 matrix: ' + name)
        records[name] = {'summary_sha256': sha(summary_path), 'detail_sha256': sha(detail_path),
                         'detail_artifact': summary['detail_artifact'],
                         'source_sha256': full['identities']['source_sha256'],
                         'executable_sha256': full['identities']['executable_sha256']}
        for key, work in full['workloads'].items():
            ratio = work['paired_ratio_stats']['median'] if name.startswith('p01x-') else work['rust_cpp_ratio']
            if ratio > BAR:
                selected[name + '/' + key] = (name, key, ratio, work, full, detail_path)
    return records, selected


def first_output(work, archive, p01x):
    item = work['outputs']['rust'] if p01x else work['outputs'][0]
    data = archive.read(item['member'])
    if hashlib.sha256(data).hexdigest() != item['sha256'] or len(data) != item['bytes']:
        raise ValueError('changed stage-1 Rust output')
    return r.response(data[:-8], samples=len(work['stats']['rust']['raw_seconds']))[0]


def decision(samples):
    if len(samples['rust']) != 30 or len(samples['cpp']) != 30:
        raise ValueError('stage 2 requires exactly 30 fresh pairs')
    logs = [math.log(a / b) for a, b in zip(samples['rust'], samples['cpp'], strict=True)]
    if any(not math.isfinite(x) for x in logs):
        raise ValueError('nonfinite paired log ratio')
    center = statistics.mean(logs)
    spread = statistics.stdev(logs)
    margin = T_CRITICAL_29 * spread / math.sqrt(30)
    lower, upper = math.exp(center - margin), math.exp(center + margin)
    verdict = 'PASS' if upper <= BAR else 'FAIL' if lower > BAR else 'INCONCLUSIVE'
    return {'samples': 30, 'mean_log_ratio': center, 'sample_stdev_log_ratio': spread,
            't_critical_29': T_CRITICAL_29, 'lower_ratio': lower, 'upper_ratio': upper,
            'verdict': verdict, 'counts_as_failure': verdict != 'PASS'}


def measure(artifacts, cpu):
    outdir = artifacts / 'stage2-0.1'
    if outdir.exists():
        raise ValueError('immutable stage-2 directory exists')
    outdir.mkdir()
    records, selected = stage1(artifacts)
    selection = quiet.physical_core()
    if selection['cpu'] != cpu or set(os.sched_getaffinity(0)) != {cpu}:
        raise ValueError('stage-2 process must be pinned to exactly the measured core')
    r.benchmark_cpu = cpu
    source_hash = sha(Path(__file__))
    detail = {'schema': 'meshopt-stage2/1', 'phase': '0.1', 'rule': 'D146',
              'rule_sha256': rule_hash(), 'driver_sha256': source_hash,
              'started_unix': time.time(), 'core_selection': selection,
              'stage1': records, 'cases': {}, 'bar': BAR,
              'confidence': 'two-sided 95% Student t interval on mean log paired ratio; df=29'}
    output_zip = outdir / 'stage2-outputs.zip'
    with zipfile.ZipFile(output_zip, 'w', zipfile.ZIP_DEFLATED, compresslevel=1) as retained:
        for index, (identifier, (name, key, ratio, work, full, path)) in enumerate(sorted(selected.items())):
            original = path.parent / ('p01x-benchmark-' + full['consumer_profile'] + '-buffers.zip'
                                      if name.startswith('p01x-') else 'benchmark-buffers.zip')
            with zipfile.ZipFile(original) as archive:
                data = archive.read(work['input']['member'])
                expected_output = first_output(work, archive, name.startswith('p01x-'))
            if len(data) != work['input']['bytes'] or hashlib.sha256(data).hexdigest() != work['input']['sha256']:
                raise ValueError('changed stage-1 input: ' + identifier)
            binaries = {backend: path.parent / 'binaries' / backend for backend in ('rust', 'cpp')}
            for backend, binary in binaries.items():
                if sha(binary) != full['identities']['executable_sha256'][backend]:
                    raise ValueError('changed stage-1 executable: ' + identifier + '/' + backend)
            input_path = outdir / f'case-{index:02d}.input'
            input_path.write_bytes(data)
            samples = {'rust': [], 'cpp': []}
            loads = []
            begun = time.time()
            backends = {backend: performance.Paired(r, binary, input_path)
                        for backend, binary in binaries.items()}
            try:
                for pair in range(30):
                    before = quiet.load()
                    for backend in (('rust', 'cpp') if pair % 2 == 0 else ('cpp', 'rust')):
                        value = backends[backend].sample()
                        if not math.isfinite(value) or value <= 0:
                            raise ValueError('invalid stage-2 timing: ' + identifier)
                        samples[backend].append(value)
                    loads.append({'before': before, 'after': quiet.load()})
                outputs = {backend: process.finish() for backend, process in backends.items()}
            finally:
                for process in backends.values():
                    process.close()
            retained_outputs = {}
            for backend, output in outputs.items():
                values, times = r.response(output[:-8], samples=30)
                if values != expected_output or times != samples[backend]:
                    raise ValueError('stage-2 exact output or timing mismatch: ' + identifier)
                member = f'case-{index:02d}-{backend}.output'
                retained.writestr(member, output)
                retained_outputs[backend] = {'member': member, 'bytes': len(output),
                                             'sha256': hashlib.sha256(output).hexdigest()}
            result = {'stage1_record': name, 'case': key, 'stage1_ratio': ratio,
                      'input': work['input'], 'stage1_rust_output_sha256':
                      (work['outputs']['rust'] if name.startswith('p01x-') else work['outputs'][0])['sha256'],
                      'binaries': full['identities']['executable_sha256'],
                      'started_unix': begun, 'finished_unix': time.time(), 'core': cpu,
                      'loads': loads, 'raw_seconds': samples, 'outputs': retained_outputs,
                      'interval': decision(samples)}
            detail['cases'][identifier] = result
            input_path.unlink()
            print(identifier, result['interval']['verdict'],
                  f"[{result['interval']['lower_ratio']:.6f}, {result['interval']['upper_ratio']:.6f}]",
                  flush=True)
    detail['finished_unix'] = time.time()
    detail['output_archive_sha256'] = sha(output_zip)
    detail_path = outdir / 'record.json'
    write(detail_path, detail)
    summary = {'schema': 'meshopt-stage2-summary/1', 'phase': '0.1',
               'detail_artifact': str(detail_path.relative_to(artifacts)),
               'artifacts': {str(detail_path.relative_to(artifacts)): sha(detail_path),
                             str(output_zip.relative_to(artifacts)): sha(output_zip)},
               'case_count': len(detail['cases']),
               'verdicts': {key: value['interval']['verdict'] for key, value in detail['cases'].items()},
               'residuals': [key for key, value in detail['cases'].items()
                             if value['interval']['counts_as_failure']]}
    write(ROOT / 'parity/results/stage2-0.1.json', summary)
    return summary


def verify(artifacts):
    summary_path = ROOT / 'parity/results/stage2-0.1.json'
    summary = json.loads(summary_path.read_text())
    detail_path = artifacts / summary['detail_artifact']
    for name, digest in summary['artifacts'].items():
        if sha(artifacts / name) != digest:
            raise ValueError('stage-2 artifact hash mismatch: ' + name)
    detail = json.loads(detail_path.read_text())
    if detail['driver_sha256'] != sha(Path(__file__)) or detail['rule_sha256'] != rule_hash():
        raise ValueError('stage-2 driver or registered rule changed')
    records, selected = stage1(artifacts)
    if detail['stage1'] != records or set(detail['cases']) != set(selected):
        raise ValueError('stage-2 case selection or stage-1 identity mismatch')
    if summary['case_count'] != len(selected) or detail['bar'] != BAR:
        raise ValueError('stage-2 count or bar changed')
    if detail['core_selection']['cpu'] not in {case['core'] for case in detail['cases'].values()}:
        raise ValueError('stage-2 core mismatch')
    archive = artifacts / 'stage2-0.1/stage2-outputs.zip'
    if sha(archive) != detail['output_archive_sha256']:
        raise ValueError('stage-2 output archive changed')
    with zipfile.ZipFile(archive) as outputs:
        for identifier, case in detail['cases'].items():
            name, key, ratio, work, full, path = selected[identifier]
            if case['stage1_record'] != name or case['case'] != key or case['stage1_ratio'] != ratio:
                raise ValueError('stage-2 source case changed: ' + identifier)
            if case['input'] != work['input'] or case['binaries'] != full['identities']['executable_sha256']:
                raise ValueError('stage-2 input or binary identity changed: ' + identifier)
            expected = decision(case['raw_seconds'])
            if case['interval'] != expected or len(case['loads']) != 30:
                raise ValueError('stage-2 interval or load telemetry differs: ' + identifier)
            for backend in ('rust', 'cpp'):
                item = case['outputs'][backend]
                data = outputs.read(item['member'])
                if len(data) != item['bytes'] or hashlib.sha256(data).hexdigest() != item['sha256']:
                    raise ValueError('stage-2 output archive member changed: ' + identifier)
                values, times = r.response(data[:-8], samples=30)
                if times != case['raw_seconds'][backend]:
                    raise ValueError('stage-2 paired timing differs: ' + identifier)
                original = path.parent / ('p01x-benchmark-' + full['consumer_profile'] + '-buffers.zip'
                                          if name.startswith('p01x-') else 'benchmark-buffers.zip')
                with zipfile.ZipFile(original) as original_zip:
                    if values != first_output(work, original_zip, name.startswith('p01x-')):
                        raise ValueError('stage-2 output differs from stage 1: ' + identifier)
    verdicts = {key: value['interval']['verdict'] for key, value in detail['cases'].items()}
    residuals = [key for key, value in detail['cases'].items() if value['interval']['counts_as_failure']]
    if summary['verdicts'] != verdicts or summary['residuals'] != residuals:
        raise ValueError('stage-2 compact summary differs')
    return summary


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--execute', action='store_true')
    parser.add_argument('--verify', action='store_true')
    parser.add_argument('--cpu', type=int)
    args = parser.parse_args()
    if args.execute == args.verify:
        parser.error('choose exactly one of --execute or --verify')
    artifacts = Path(os.environ['MESHOPT_ARTIFACTS']).resolve()
    if args.execute:
        if args.cpu is None:
            parser.error('--execute requires --cpu')
        measure(artifacts, args.cpu)
    else:
        result = verify(artifacts)
        print('stage 2 verified', result['case_count'], 'fresh cases; residuals:', result['residuals'])


if __name__ == '__main__':
    main()
