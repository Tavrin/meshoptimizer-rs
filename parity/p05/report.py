#!/usr/bin/env python3
"""Verify complete phase-0.5 summaries and retained case artifacts."""
import gzip
import hashlib
import json
import math
import os
from pathlib import Path
import statistics
from timing_gate import covered
from benchmark import REF, SHAPES, SCREEN_T, shape_group, early_screen, stage2_screen

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ.get('MESHOPT_ARTIFACTS', '/mnt/linux-extra/meshopt-artifacts/p05'))
REQUIRED = ['run', 'sweep', 'no-default', 'wasm', 'fuzz', 'benchmark-moss', 'benchmark-default']
FAMILIES = ['stripify', 'stripify_bound', 'unstripify', 'unstripify_bound', 'vertex_cache', 'vertex_fetch', 'overdraw', 'coverage', 'omm_measure', 'omm_rasterize', 'omm_entry_size', 'omm_compact', 'tangents', 'normals', 'remesh']


def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()


def admitted(record):
    return record.get('admitted') is True and not record.get('reasons') and covered(record)


def valid_utilization(usage, cpus):
    return (set(usage) == {str(cpu) for cpu in cpus}
            and all(v['total_ticks'] >= 0 and 0 <= v['busy_ticks'] <= v['total_ticks']
                    and v['busy_percent'] == (100 * v['busy_ticks'] / v['total_ticks'] if v['total_ticks'] else None)
                    for v in usage.values()))


def valid_pairs(samples, allowed_cpus):
    return all(s.get('pair_index') == index and s.get('order') == ('cpp-rust' if index & 1 else 'rust-cpp')
               and s['rust_ns'] > 0 and s['cpp_ns'] > 0
               and s['ratio'] == s['rust_ns'] / s['cpp_ns']
               and math.isfinite(s['ratio'])
               and s.get('cpu') in allowed_cpus
               and len(s.get('load_before', [])) == len(s.get('load_after', [])) == 3
               and all(math.isfinite(v) and v >= 0 for v in s['load_before'] + s['load_after'])
               and admitted(s.get('timing_admission', {}))
               and valid_utilization(s.get('cpu_utilization', {}), allowed_cpus)
               and s.get('elapsed_seconds', 0) > 0 for index, s in enumerate(samples))


def valid_case_maximum(row, detail):
    samples = row['samples']
    if len(samples) not in SCREEN_T:
        return False
    screens = [early_screen(samples[:look]) for look in SCREEN_T if look <= len(samples)]
    if row.get('early_screens') != screens or any(v['verdict'] != 'BORDERLINE' for v in screens[:-1]):
        return False
    last = screens[-1]['verdict']
    stage2 = row.get('stage2')
    if last != 'BORDERLINE':
        return stage2 is None and row.get('maximum_verdict') == last
    if len(samples) != 20 or not stage2 or len(stage2.get('samples', [])) != 30:
        return False
    fresh = stage2['samples']
    interval = stage2_screen(fresh)
    cpu = stage2['physical_core']['cpu']
    return (stage2['interval'] == interval and row.get('maximum_verdict') == interval['verdict']
            and stage2['input_hash'] == row['input_hash']
            and stage2['rust_sha256'] == detail['rust_sha256']
            and stage2['cpp_sha256'] == detail['cpp_sha256']
            and stage2['physical_core']['allowed_cpus'] == detail['physical_core']['allowed_cpus']
            and fresh[0]['timing_admission']['at'] > samples[-1]['timing_admission']['at']
            and all(s['cpu'] == cpu and s['timing_admission']['label'].endswith(f':D146-{index}')
                    for index, s in enumerate(fresh))
            and valid_pairs(fresh, detail['physical_core']['allowed_cpus']))


def verify():
    errors = []
    for kind in REQUIRED:
        path = ROOT / f'parity/results/{kind}-0.5.json'
        if not path.exists(): errors.append('missing ' + kind); continue
        record = json.loads(path.read_text())
        if not record.get('passed') and not kind.startswith('benchmark-'):
            errors.append('failed ' + kind)
        artifact = ART / record['detail_artifact']
        if not artifact.is_file() or sha(artifact) != record['detail_sha256']: errors.append('artifact identity ' + kind)
        if kind in ('sweep', 'no-default', 'wasm'):
            if not record.get('source_sha256') or any(sha(ROOT / name) != digest for name, digest in record.get('source_sha256', {}).items()):
                errors.append('differential source identity ' + kind)
            if record['counts'] != dict.fromkeys(FAMILIES, 2000): errors.append('incomplete ' + kind)
            if artifact.exists():
                with gzip.open(artifact, 'rt') as source:
                    cases = [json.loads(line) for line in source]
                if len(cases) != 30000 or any(not c['equal'] for c in cases): errors.append('case mismatch ' + kind)
            prefix = 'no-default' if kind == 'no-default' else 'sweep'
            binaries = ({f'{prefix}-0.5-rust': record['rust_sha256'], f'{prefix}-0.5-cpp.so': record['cpp_sha256']}
                        if kind != 'wasm' else {'wasm-0.5.wasm': record['wasm_sha256']})
            for name, digest in binaries.items():
                binary = ART / name
                if not binary.is_file() or sha(binary) != digest:
                    errors.append('binary identity ' + name)
        if kind == 'fuzz':
            if set(record['targets']) != set(FAMILIES): errors.append('fuzz target inventory')
            if any(not t['passed'] or t['elapsed_seconds'] < 300 or not t['executions'] for t in record['targets'].values()): errors.append('fuzz duration/execution')
            if artifact.is_file():
                detail = json.loads(artifact.read_text())
                if not detail['passed'] or {t['target'] for t in detail['targets']} != set(FAMILIES): errors.append('fuzz detail inventory')
                for target in detail['targets']:
                    log = ART / target['log']
                    binary = ART / 'fuzz-0.5' / 'binaries' / target['target']
                    if not log.is_file() or sha(log) != target['log_sha256']:
                        errors.append('fuzz log identity ' + target['target'])
                    if not binary.is_file() or sha(binary) != target['binary_sha256']:
                        errors.append('fuzz binary identity ' + target['target'])
                    if target['exit_code'] or not target['passed'] or target['elapsed_seconds'] < 300 or not target['executions']:
                        errors.append('fuzz target failure ' + target['target'])
        if kind.startswith('benchmark-') and artifact.is_file():
            detail = json.loads(artifact.read_text())
            profile = kind.removeprefix('benchmark-')
            binary = ART / f'benchmark-0.5-{profile}-rust'
            if not binary.is_file() or sha(binary) != detail['rust_sha256'] or detail['rust_sha256'] != record['rust_sha256']:
                errors.append('benchmark Rust identity ' + profile)
            if record['families'] != detail['families'] or detail['profile'] != profile:
                errors.append('benchmark summary ' + profile)
            if (record.get('source_sha256') != detail.get('source_sha256')
                or not detail.get('source_sha256')
                or any(not (ROOT / name).is_file() or sha(ROOT / name) != digest
                       for name, digest in detail['source_sha256'].items())):
                errors.append('benchmark source identity ' + profile)
            if (record.get('reference_sha256') != detail.get('reference_sha256')
                or not detail.get('reference_sha256')
                or any(not (REF / name).is_file() or sha(REF / name) != digest
                       for name, digest in detail['reference_sha256'].items())):
                errors.append('benchmark C++ source identity ' + profile)
            caller_families = {'stripify', 'unstripify', 'omm_rasterize', 'tangents', 'normals', 'remesh'}
            expected_groups = {f'{name}:{api}' for name in FAMILIES
                               for api in (['allocating', 'caller'] if name in caller_families else ['allocating'])}
            if (detail.get('schema') != 'meshopt-p05-benchmark/2' or detail.get('mode') != 'final'
                or set(detail['families']) != expected_groups or len(detail['rows']) != 160
                or set(detail.get('selected_families', [])) != set(FAMILIES) or detail.get('case_filters') != {}):
                errors.append('benchmark final inventory/protocol ' + profile)
            expected_overrides = {'CARGO_PROFILE_RELEASE_OPT_LEVEL': '3', 'CARGO_PROFILE_RELEASE_DEBUG': '0',
                                  'CARGO_PROFILE_RELEASE_LTO': 'thin' if profile == 'moss' else 'false',
                                  'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '1' if profile == 'moss' else '16'}
            if detail['effective_profile_overrides'] != expected_overrides or record['effective_profile_overrides'] != expected_overrides:
                errors.append('benchmark consumer profile ' + profile)
            for family, verdict in detail['families'].items():
                name, api = family.split(':')
                rows = [row for row in detail['rows'] if row['family'] == name and row['api'] == api]
                shapes = SHAPES[shape_group(name)]
                if len(rows) != verdict['cases'] or len(rows) != len(shapes) or any(not valid_case_maximum(row, detail) for row in rows):
                    errors.append('benchmark samples ' + family)
                    continue
                if [(r['vertices'], r['triangles'], r['style']) for r in rows] != shapes:
                    errors.append('benchmark corpus ' + family)
                ratios = [statistics.median(s['ratio'] for s in row['samples']) for row in rows]
                gm = math.exp(statistics.mean(math.log(value) for value in ratios))
                if abs(gm - verdict['geometric_mean']) > 1e-8 or abs(max(ratios) - verdict['maximum']) > 1e-8:
                    errors.append('benchmark statistics ' + family)
                memory = max((r['memory_ratio'] if r['memory_ratio'] is not None else math.inf) for r in rows)
                expected_memory = None if math.isinf(memory) else memory
                maximum_passed = all(row['maximum_verdict'] == 'PASS' for row in rows)
                if (expected_memory != verdict['maximum_memory_ratio'] or verdict.get('maximum_passed') != maximum_passed
                    or verdict['passed'] != (gm <= 1.25 and maximum_passed and memory <= 1.25)):
                    errors.append('benchmark memory/statistics ' + family)
                if any(not valid_pairs(r['samples'], detail['physical_core']['allowed_cpus']) for r in rows):
                    errors.append('benchmark load ' + family)
                for row in rows:
                    rust_bytes, cpp_bytes = row['rust_memory_bytes'], row['cpp_memory_bytes']
                    ratio = rust_bytes / cpp_bytes if cpp_bytes else (1.0 if rust_bytes == 0 else None)
                    if (min(rust_bytes, cpp_bytes, row['cpp_scratch_bytes'], row['cpp_output_bytes']) < 0
                        or cpp_bytes != row['cpp_scratch_bytes'] + row['cpp_output_bytes']
                        or row['memory_ratio'] != ratio):
                        errors.append('benchmark memory accounting ' + family)
                        break
                if any(any(not admitted(s.get('timing_admission', {})) for s in r['samples']) for r in rows):
                    errors.append('benchmark timing admission ' + family)
                if any(any(not valid_utilization(s.get('cpu_utilization', {}), detail['physical_core']['allowed_cpus'])
                           or s.get('elapsed_seconds', 0) <= 0 for s in r['samples']) for r in rows):
                    errors.append('benchmark CPU utilization ' + family)
            time_passed = all(v['geometric_mean'] <= 1.25 and v.get('maximum_passed') is True for v in detail['families'].values())
            memory_passed = all(r['memory_ratio'] is not None and r['memory_ratio'] <= 1.25 for r in detail['rows'])
            corpus_complete = all({r['style'] for r in detail['rows'] if r['family'] == f} == {0, 1, 2, 3, 4} and
                                  any(r['triangles'] >= 1_000_000 for r in detail['rows'] if r['family'] == f)
                                  for f in FAMILIES if shape_group(f) != 'omm')
            if record['time_passed'] != time_passed or record['memory_passed'] != memory_passed or record['corpus_complete'] != corpus_complete or record['passed'] != (time_passed and memory_passed and corpus_complete):
                errors.append('benchmark gate scope ' + profile)
            cpp = ART / 'libmeshopt-p05-bench.so'
            if not cpp.is_file() or sha(cpp) != record['cpp_sha256'] or detail['cpp_sha256'] != record['cpp_sha256']:
                errors.append('benchmark C++ identity ' + profile)
            if not detail.get('physical_core') or any(r['iterations'] < 1 for r in detail['rows']):
                errors.append('benchmark protocol ' + profile)
            if not record['passed']:
                profile_path = ROOT / f'parity/results/profile-{profile}-0.5.json'
                if not profile_path.is_file():
                    errors.append('missing perf profile ' + profile)
                else:
                    profile_record = json.loads(profile_path.read_text())
                    profile_artifact = ART / profile_record['detail_artifact']
                    failing = {key.split(':')[0] for key, value in detail['families'].items() if not value['passed']}
                    if (not profile_record['profile_complete'] or profile_record['benchmark_sha256'] != sha(artifact)
                        or set(profile_record['families']) != failing or not profile_artifact.is_file()
                        or sha(profile_artifact) != profile_record['detail_sha256']):
                        errors.append('incomplete perf profile ' + profile)
                    else:
                        raw = json.loads(profile_artifact.read_text())
                        profile_cpp = profile_artifact.parent / 'cpp-runner'
                        if (raw.get('benchmark_sha256') != sha(artifact)
                            or raw.get('rust_sha256') != record['rust_sha256']
                            or raw.get('benchmark_cpu') != detail['physical_core']['cpu']
                            or not profile_cpp.is_file()
                            or raw.get('cpp_sha256') != sha(profile_cpp)):
                            errors.append('perf binary/record identity ' + profile)
                        for name in failing:
                            entry = raw['families'].get(name)
                            if not entry or entry['row']['family'] != name:
                                errors.append('perf case ' + profile + ' ' + name)
                                continue
                            candidates = [row for row in detail['rows'] if row['family'] == name
                                          and not detail['families'][name + ':' + row['api']]['passed']]
                            worst = max(candidates, key=lambda row: row['median_ratio'])
                            expected_case = {key: worst[key] for key in
                                             ('family', 'api', 'seed', 'vertices', 'triangles', 'style', 'median_ratio')}
                            if entry['row'] != expected_case:
                                errors.append('perf worst case ' + profile + ' ' + name)
                            core = entry.get('physical_core', {})
                            if (core.get('allowed_cpus') != detail['physical_core']['allowed_cpus']
                                or core.get('cpu') not in detail['physical_core']['allowed_cpus']
                                or len(core.get('selection_samples', [])) != 3):
                                errors.append('perf core selection ' + profile + ' ' + name)
                            compressed = profile_artifact.parent / entry.get('input_artifact', 'missing')
                            if not compressed.is_file() or sha(compressed) != entry.get('input_artifact_sha256'):
                                errors.append('perf input identity ' + profile + ' ' + name)
                            else:
                                digest = hashlib.sha256()
                                with gzip.open(compressed, 'rb') as source:
                                    for chunk in iter(lambda: source.read(1 << 20), b''):
                                        digest.update(chunk)
                                if digest.hexdigest() != entry['input_sha256']:
                                    errors.append('perf decompressed input identity ' + profile + ' ' + name)
                            for backend in ('rust', 'cpp'):
                                item = entry['backends'][backend]
                                perf_data = profile_artifact.parent / f'{name}-{backend}.data'
                                if (item['stat_exit'] != 0 or item['record_exit'] != 0 or not item['instructions']
                                    or not perf_data.is_file() or sha(perf_data) != item['perf_data_sha256']):
                                    errors.append('perf evidence ' + profile + ' ' + name + ' ' + backend)
                                if not admitted(item.get('stat_admission', {})) or not admitted(item.get('record_admission', {})):
                                    errors.append('perf timing admission ' + profile + ' ' + name + ' ' + backend)
                                if any(not valid_utilization(item.get(kind + '_cpu_utilization', {}), detail['physical_core']['allowed_cpus'])
                                       or len(item.get(kind + '_load_after', [])) != 3 for kind in ('stat', 'record')):
                                    errors.append('perf CPU utilization/load ' + profile + ' ' + name + ' ' + backend)
                            cpp_instructions = entry['backends']['cpp']['instructions']
                            rust_instructions = entry['backends']['rust']['instructions']
                            if not cpp_instructions or not rust_instructions:
                                continue
                            ratio = rust_instructions / cpp_instructions
                            if profile_record['families'].get(name) != {
                                'cpp_instructions': cpp_instructions,
                                'rust_instructions': rust_instructions,
                                'ratio': ratio,
                            }:
                                errors.append('perf summary ' + profile + ' ' + name)
        if kind == 'run':
            for name, expected in record['source_sha256'].items():
                if sha(ROOT / name) != expected: errors.append('stale fixture source ' + name)
    benchmarks = [json.loads((ROOT / f'parity/results/benchmark-{profile}-0.5.json').read_text())
                  for profile in ('moss', 'default')]
    print(json.dumps({'phase': '0.5', 'verified': not errors,
                      'performance_qualified': all(record['passed'] for record in benchmarks),
                      'errors': errors}, indent=2))
    raise SystemExit(0 if not errors else 1)


if __name__ == '__main__': verify()
