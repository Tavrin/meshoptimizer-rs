#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
export PYTHONDONTWRITEBYTECODE=1
exec "${MESHOPT_PYTHON:-python3}" - "$@" <<'PY'
"""Small repository summaries; immutable detailed records outside git."""
import argparse
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import re
import statistics
import shutil
import struct
import subprocess
import sys
import tarfile
import time
import zipfile
import types

ROOT = Path.cwd()
sys.path.insert(0, str(ROOT / 'parity'))
import runner as r

def sha(p):
    h = hashlib.sha256()
    with p.open('rb') as f:
        for data in iter(lambda: f.read(1024 * 1024), b''):
            h.update(data)
    return h.hexdigest()

def artifacts():
    return Path(os.environ.get('MESHOPT_ARTIFACTS', Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target')) / 'parity-artifacts')).resolve()

def write(p, record):
    p.parent.mkdir(parents=True, exist_ok=True)
    temporary = p.with_suffix(p.suffix + '.partial')
    temporary.write_text(json.dumps(record, indent=2) + '\n')
    temporary.replace(p)

def coverage(case_id):
    if case_id.startswith('seed-'):
        return re.sub(r'^seed-\d+-case-\d+-', '', case_id)
    return re.sub(r'-\d+$', '', case_id)

def compact(path, destination, historical=False, profile=None):
    full = json.loads(path.read_text())
    relative = str(path.relative_to(artifacts()))
    manifest = {relative: sha(path)}
    linked = full.get('artifacts', {})
    for name, value in (linked.items() if isinstance(linked, dict) else []):
        if not isinstance(value, str) or not re.fullmatch('[0-9a-f]{64}', value):
            continue
        candidate = path.parent / name
        if not candidate.is_file():
            candidate = artifacts() / name
        if candidate.is_file() and sha(candidate) == value:
            manifest[str(candidate.relative_to(artifacts()))] = value
        else:
            manifest[str(Path('unavailable-historical') / path.stem / name)] = value
    if isinstance(full.get('package_artifact'), dict):
        item = full['package_artifact']
        package = path.parent / item['name']
        manifest[str(package.relative_to(artifacts()))] = item['sha256']
    summary = {'schema': 'meshopt-summary/1', 'historical': historical, 'command': full.get('command', path.stem),
               'passed': full.get('passed'), 'phase': full.get('phase', '0.1') if historical else '0.1', 'seed': full.get('seed'),
               'profile': profile or full.get('profile'), 'detail_artifact': relative, 'artifacts': manifest}
    if 'identities' in full:
        summary['identities'] = full['identities']
    for key in ['source_sha256', 'required_cases_per_family', 'mismatches', 'started_unix', 'finished_unix', 'identities_unchanged']:
        if key in full:
            summary[key] = full[key]
    if isinstance(full.get('cases'), list) and full['cases'] and all(isinstance(c, dict) and {'family', 'id', 'match', 'indices'} <= c.keys() for c in full['cases']):
        summary['functions'] = {}
        for family in r.FAMILIES.values():
            cases = [c for c in full['cases'] if c['family'] == family]
            summary['functions'][family] = {'count': len(cases), 'mismatches': sum(not c['match'] for c in cases),
                'seeds': [full['seed']] if full.get('seed') else [],
                'input_families': dict(sorted(Counter(coverage(c['id']) for c in cases).items())),
                'maximum_indices': max((c['indices'] for c in cases), default=0)}
        summary['counts'] = full['counts']
        if 'upstream_fixture_inventory' in full:
            summary['upstream_fixture_counts'] = dict(Counter(c['family'] for c in full['upstream_fixture_inventory']))
        summary['math_probe_match'] = full.get('math_probe', {}).get('match')
    if isinstance(full.get('families'), dict) and 'workloads' in full:
        summary['functions'] = full['families']
        summary['performance_bars'] = {'geometric_mean': 1.25, 'maximum': 1.5, 'maximum_memory_ratio': 1.25}
        summary['input_families'] = sorted({w['shape'] for w in full.get('workloads', {}).values()})
        summary['sizes'] = sorted({w['size'] for w in full.get('workloads', {}).values()})
        summary['apis'] = sorted({w['api'] for w in full.get('workloads', {}).values()})
    write(destination, summary)
    return summary

CODEC_RECORDS = {'run': ['fixtures', 'malformed'], 'sweep': ['sweep']}

CODEC_PHASES = ['0.2', '0.4']
# Phase 0.4 adds codec completion and re-runs every 0.2 decoder fixture and sweep family.
CODEC_RUNNERS = {'0.2': 'runner.py', '0.4': 'runner04.py'}

def phase03(argv):
    for i, argument in enumerate(argv):
        if argument == '--phase' and argv[i + 1:i + 2] == ['0.3']:
            return argv[:i] + argv[i + 2:]
        if argument == '--phase=0.3':
            return argv[:i] + argv[i + 1:]
    return None

def execute03(action, argv):
    if action not in ('run', 'sweep'):
        raise SystemExit('phase 0.3 supports run and sweep')
    root = artifacts()
    env = {**os.environ, 'MESHOPT_ARTIFACTS': str(root)}
    code = subprocess.run([sys.executable, str(ROOT / 'parity/p03/runner.py'), action,
                           '--phase', '0.3', *argv], env=env).returncode
    detail = root / action / 'record.json'
    corpus = root / action / 'corpus.bin'
    if code or not detail.is_file() or not corpus.is_file():
        raise SystemExit(code or 1)
    full = json.loads(detail.read_text())
    if full.get('schema') != 'meshopt-p03/1' or full.get('action') != action or full.get('mismatches') != 0 or sha(corpus) != full.get('corpus_sha256'):
        raise SystemExit('phase 0.3 record verification failed')
    summary = {'schema': 'meshopt-summary/1', 'historical': False, 'command': action,
               'phase': '0.3', 'passed': True, 'mismatches': 0, 'seed': full['seed'],
               'cases': full['cases'], 'fixture_count': len(full['fixtures']),
               'detail_artifact': f'{action}/record.json',
               'artifacts': {f'{action}/record.json': sha(detail), f'{action}/corpus.bin': sha(corpus)}}
    write(ROOT / 'parity/results' / f'{action}-0.3.json', summary)

def report03(verify):
    for action in ('run', 'sweep'):
        path = ROOT / 'parity/results' / f'{action}-0.3.json'
        summary = json.loads(path.read_text())
        if summary.get('schema') != 'meshopt-summary/1' or summary.get('phase') != '0.3' or summary.get('command') != action or summary.get('passed') is not True or summary.get('mismatches') != 0:
            raise ValueError('invalid phase 0.3 summary: ' + path.name)
        if summary['cases'] != {name: 2000 if action == 'sweep' else 25 for name in p03_names()} | ({'upstream-fixtures': 47} if action == 'run' else {}):
            raise ValueError('phase 0.3 case inventory changed: ' + path.name)
        if not verify:
            continue
        for relative, digest in summary['artifacts'].items():
            if sha(artifacts() / relative) != digest:
                raise ValueError('phase 0.3 artifact hash changed: ' + relative)
        full = json.loads((artifacts() / summary['detail_artifact']).read_text())
        if full['cases'] != summary['cases'] or full['mismatches'] != 0 or full['corpus_sha256'] != summary['artifacts'][f'{action}/corpus.bin'] or len(full['fixtures']) != summary['fixture_count']:
            raise ValueError('phase 0.3 detailed record differs: ' + action)
        corpus = artifacts() / action / 'corpus.bin'
        frames = 0
        with corpus.open('rb') as stream:
            while header := stream.read(8):
                if len(header) != 8:
                    raise ValueError('truncated phase 0.3 corpus: ' + action)
                input_length, output_length = struct.unpack('<II', header)
                stream.seek(input_length + output_length, 1)
                frames += 1
            if stream.tell() != corpus.stat().st_size:
                raise ValueError('truncated phase 0.3 frame: ' + action)
        expected = sum(summary['cases'].values()) + sum(summary['cases'][name] for name in p03_names() if name not in ('build_meshlets_bound', 'compute_cluster_bounds', 'compute_meshlet_bounds', 'compute_sphere_bounds'))
        if frames != expected:
            raise ValueError('phase 0.3 message inventory differs: ' + action)
        import importlib.util
        spec = importlib.util.spec_from_file_location('p03_runner', ROOT / 'parity/p03/runner.py')
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        if full['identity']['sources'] != module.snapshot():
            raise ValueError('stale phase 0.3 source identity: ' + action)
    print('0.3 exact parity records pass: 47 fixtures, 25 generated and 2000 seeded cases per family; zero mismatches.')

def p03_names():
    return ('build_meshlets', 'build_meshlets_scan', 'build_meshlets_flex', 'build_meshlets_spatial',
            'build_meshlets_bound', 'compute_cluster_bounds', 'compute_meshlet_bounds',
            'compute_sphere_bounds', 'optimize_meshlet', 'optimize_meshlet_level',
            'extract_meshlet_indices', 'partition_clusters', 'spatial_sort_remap',
            'spatial_sort_triangles', 'spatial_cluster_points')

def codec_phase(argv):
    """(phase, arguments without the selection) for a codec phase, or None for phase 0.1."""
    for i, argument in enumerate(argv):
        for phase in CODEC_PHASES:
            if argument == '--phase' and argv[i + 1:i + 2] == [phase]:
                return phase, argv[:i] + argv[i + 2:]
            if argument == '--phase=' + phase or (phase == '0.2' and argument == '0.2'):
                return phase, argv[:i] + argv[i + 1:]
    return None

def codec_module(phase='0.2'):
    os.environ.setdefault('MESHOPT_ARTIFACTS', str(artifacts()))
    sys.path.insert(0, str(ROOT / 'parity/codec'))
    import importlib.util
    # The 0.1 report imports parity/runner.py as "runner" above. runner04.py
    # imports its 0.2 sibling under that same name, so temporarily let that
    # import resolve in the codec directory and restore the report's module.
    previous = sys.modules.pop('runner', None) if phase == '0.4' else None
    try:
        spec = importlib.util.spec_from_file_location('codec_runner_' + phase.replace('.', ''), ROOT / 'parity/codec' / CODEC_RUNNERS[phase])
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module
    finally:
        if phase == '0.4':
            sys.modules.pop('runner', None)
            if previous is not None:
                sys.modules['runner'] = previous

def execute_codec04_extra(action, argv):
    """Phase 0.4 benchmark and fuzz smokes: detailed records in a fresh artifact
    subdirectory, compact summaries under parity/results."""
    base = artifacts()
    raw = base / (action + '-0.4')
    raw.mkdir(parents=True, exist_ok=True)
    env = {**os.environ, 'MESHOPT_ARTIFACTS': str(raw)}
    if action == 'benchmark':
        if '--consumer-profile' not in argv:
            raise SystemExit('phase 0.4 benchmark requires --consumer-profile moss|default')
        profile = argv[argv.index('--consumer-profile') + 1]
        script = 'measure04.py'
        detail = raw / f'benchmark-0.4-{profile}.json'
        destination = ROOT / 'parity/results' / f'benchmark-0.4-{profile}.json'
        command = [sys.executable, str(ROOT / 'parity/codec' / script), '--phase', '0.4', *argv]
    else:
        detail = raw / 'fuzz.json'
        destination = ROOT / 'parity/results/fuzz-0.4.json'
        command = [sys.executable, str(ROOT / 'parity/codec/fuzz04.py'), *argv]
    code = subprocess.run(command, env=env).returncode
    if not detail.is_file():
        raise SystemExit(code or 1)
    full = json.loads(detail.read_text())
    relative = str(detail.relative_to(base))
    summary = {'schema': 'meshopt-summary/1', 'historical': False, 'command': action, 'phase': '0.4',
               'detail_artifact': relative, 'artifacts': {relative: sha(detail)}, 'mismatches': 0}
    if action == 'benchmark':
        manifest = raw / 'benchmark04-inputs.json'
        summary['artifacts'][str(manifest.relative_to(base))] = sha(manifest)
        summary.update({'profile': profile, 'effective_profile_overrides': full['effective_profile_overrides'],
                        'bar': full['bar'], 'executable_sha256': full['executable_sha256'],
                        'families': {api: {f: {k: v[k] for k in ['cases', 'geometric_mean_rust_scalar_time_ratio', 'maximum_rust_scalar_time_ratio', 'pass']}
                                           for f, v in families.items()} for api, families in full['verdicts'].items()},
                        'family_pass': full['family_pass'], 'passed': all(full['family_pass'].values())})
    else:
        summary.update({'profile': full['profile'], 'targets': {t['target']: {'executions': t['executions'], 'elapsed_seconds': t['elapsed_seconds'], 'exit_code': t['exit_code']} for t in full['targets']},
                        'passed': all(t['exit_code'] == 0 and t['elapsed_seconds'] >= 300 and t['executions'] > 0 for t in full['targets'])})
        for t in full['targets']:
            log = Path(t['log'])
            summary['artifacts'][str(log.relative_to(base))] = t['log_sha256']
    write(destination, summary)
    raise SystemExit(code)

def execute_codec(action, argv, phase):
    """Phases 0.2 and 0.4: detailed codec records under MESHOPT_ARTIFACTS, slim summaries in git."""
    if phase == '0.4' and action in ('benchmark', 'fuzz'):
        return execute_codec04_extra(action, argv)
    if action == 'benchmark':
        # The pre-registered decoder measurement protocol owns its records.
        command = [sys.executable, str(ROOT / 'parity/codec/measure.py'), '--phase', '0.2', *argv]
        raise SystemExit(subprocess.run(command).returncode)
    if action not in CODEC_RECORDS:
        raise SystemExit(f'phase {phase} supports run, sweep and benchmark')
    directory = action + '-' + phase
    if '--record-directory' in argv:
        i = argv.index('--record-directory')
        directory = argv[i + 1]
        del argv[i:i + 2]
        if not re.fullmatch(r'[a-zA-Z0-9_-]+', directory):
            raise SystemExit('record directory must be one plain directory name')
    base = artifacts()
    raw = base / directory
    names = CODEC_RECORDS[action]
    if any((raw / (name + '.json')).exists() for name in names):
        raise SystemExit('immutable record exists; select a fresh MESHOPT_ARTIFACTS')
    raw.mkdir(parents=True, exist_ok=True)
    command = [sys.executable, str(ROOT / 'parity/codec' / CODEC_RUNNERS[phase]), action, '--phase', phase, *argv]
    code = subprocess.run(command, env={**os.environ, 'MESHOPT_ARTIFACTS': str(raw)}).returncode
    if code or not all((raw / (name + '.json')).is_file() for name in names):
        raise SystemExit(code or 1)
    summary = {'schema': 'meshopt-summary/1', 'historical': False, 'command': action, 'phase': phase,
               'profile': 'scalar-strict', 'records': {}, 'artifacts': {}}
    for name in names:
        detail = raw / (name + '.json')
        full = json.loads(detail.read_text())
        archive = raw / (name + '.zip')
        if Path(full['archive']).resolve() != archive.resolve() or sha(archive) != full['archive_sha256']:
            raise ValueError('codec archive identity differs: ' + name)
        relative = str(detail.relative_to(base))
        summary.setdefault('detail_artifact', relative)
        summary['seed'] = full.get('seed')
        summary['artifacts'][relative] = sha(detail)
        summary['artifacts'][str(archive.relative_to(base))] = full['archive_sha256']
        summary['records'][name] = {'detail_artifact': relative, 'archive_artifact': str(archive.relative_to(base)),
                                    'cases': len(full['cases']), 'counts': full['counts'], 'mismatches': full['mismatches'],
                                    'simd_filter_conformance_cases': full['simd_filter_conformance_cases'],
                                    'executable_sha256': full['executable_sha256'],
                                    'reference_revision': full['reference_revision']}
        for key in ['cpp_undefined_cases', 'cross_decoded_cases', 'color_simd_max_unit_difference']:
            if key in full:
                summary['records'][name][key] = full[key]
    summary['mismatches'] = sum(v['mismatches'] for v in summary['records'].values())
    summary['passed'] = summary['mismatches'] == 0
    write(ROOT / 'parity/results' / (action + '-' + phase + '.json'), summary)

def report_codec(verify, phase='0.2'):
    records = {}
    missing = []
    for path in sorted((ROOT / 'parity/results').glob('*.json')):
        summary = json.loads(path.read_text())
        if summary.get('schema') != 'meshopt-summary/1' or summary['historical'] or summary.get('phase') != phase:
            continue
        key = summary['command'] + ('-' + summary['profile'] if summary['command'] == 'benchmark' and phase == '0.4' else '')
        records[key] = summary
        if summary['passed'] is not True or summary['mismatches'] != 0:
            raise SystemExit(f'{phase} gate failed: ' + path.name)
        if not verify:
            continue
        absent = []
        full = full_record(summary, absent)
        missing += absent
        if absent:
            continue
        current = codec_module(phase).sources()
        if summary['command'] in ('benchmark', 'fuzz'):
            if summary['command'] == 'benchmark' and (full['sources'] != current or not all(full['family_pass'].values())):
                raise ValueError('stale or failing 0.4 benchmark record: ' + path.name)
            if summary['command'] == 'fuzz':
                core = {str(p.relative_to(ROOT)): sha(p) for p in ROOT.glob('src/**/*.rs')}
                if full['sources'] != core:
                    raise ValueError('stale 0.4 fuzz record')
            continue
        for name, item in summary['records'].items():
            full = json.loads((artifacts() / item['detail_artifact']).read_text())
            if full['mismatches'] != 0 or full['counts'] != item['counts'] or len(full['cases']) != item['cases']:
                raise ValueError(f'{phase} summary differs from detailed record: ' + name)
            if full['sources'] != current:
                raise ValueError(f'stale {phase} record: ' + name)
            with zipfile.ZipFile(artifacts() / item['archive_artifact']) as archive:
                for case in full['cases']:
                    for value in case['files'].values():
                        if hashlib.sha256(archive.read(value['member'])).hexdigest() != value['sha256']:
                            raise ValueError(f'{phase} case buffer identity mismatch: ' + case['case'])
    incomplete = set(CODEC_RECORDS) - records.keys()
    fixtures = 287 if phase == '0.2' else 287 + 563 + 19
    operations = list(range(1, 8)) + (list(range(11, 26)) if phase == '0.4' else [])
    if 'run' in records and records['run']['records']['fixtures']['cases'] != fixtures:
        incomplete.add(f'{fixtures} pinned fixtures')
    if 'sweep' in records and any(int(records['sweep']['records']['sweep']['counts'].get(str(op), 0)) < 2000 for op in operations):
        incomplete.add('2000 seeded cases per codec operation')
    if phase == '0.4':
        incomplete |= {'benchmark-moss', 'benchmark-default', 'fuzz'} - records.keys()
    if missing:
        print('ABSENT artifacts (hashes cannot be verified):\n' + '\n'.join(sorted(set(missing))))
    else:
        print(f'All recorded {phase} artifact SHA-256 hashes and case buffers verified.' if verify else 'Artifact hashes not requested.')
    if incomplete:
        print(f'{phase} qualification incomplete:', ', '.join(sorted(incomplete)))
        raise SystemExit(1)
    if missing:
        raise SystemExit(1)
    print(f'{phase} exact codec parity records pass:', {name: item['cases'] for v in records.values() for name, item in v.get('records', {}).items()})
    if phase == '0.4':
        for key in ['benchmark-moss', 'benchmark-default']:
            print(key, 'family verdicts:', records[key]['family_pass'])
        print('fuzz smokes:', {k: v['executions'] for k, v in records['fuzz']['targets'].items()})

def execute(argv):
    action = argv.pop(0)
    p03 = phase03(argv)
    if p03 is not None:
        return execute03(action, p03)
    codec = codec_phase(argv)
    if codec is not None:
        return execute_codec(action, codec[1], codec[0])
    resume = '--resume' in argv
    if resume:
        argv.remove('--resume')
    if resume and action != 'benchmark':
        raise SystemExit('--resume is only supported for interrupted benchmark matrices')
    profile = 'crate'
    if '--consumer-profile' in argv:
        i = argv.index('--consumer-profile')
        profile = argv[i + 1]
        del argv[i:i + 2]
    if profile not in ['crate', 'moss', 'default']:
        raise SystemExit('consumer profile must be crate, moss or default')
    base = artifacts()
    label = action + ('-' + profile if action == 'benchmark' else '')
    directory = label
    if '--record-directory' in argv:
        i = argv.index('--record-directory')
        directory = argv[i + 1]
        del argv[i:i + 2]
        if not re.fullmatch(r'[a-zA-Z0-9_-]+', directory):
            raise SystemExit('record directory must be one plain directory name')
    raw = base / directory
    raw.mkdir(parents=True, exist_ok=True)
    if (raw / (action + '.json')).exists():
        raise SystemExit('immutable record exists; select a fresh MESHOPT_ARTIFACTS')
    os.environ['MESHOPT_RESULTS'] = str(raw)
    os.environ['MESHOPT_ARTIFACTS'] = str(raw)
    r.ENV.update(MESHOPT_RESULTS=str(raw), MESHOPT_ARTIFACTS=str(raw))
    # Environment overrides apply to the harness workspace, which is the root
    # of the dependent build. The library's own release profile is not inherited.
    configuration = {'CARGO_PROFILE_RELEASE_OPT_LEVEL': '3', 'CARGO_PROFILE_RELEASE_DEBUG': '0'}
    configuration.update({'CARGO_PROFILE_RELEASE_LTO': 'thin', 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '1'} if profile == 'moss'
                         else {'CARGO_PROFILE_RELEASE_LTO': 'false', 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '16'} if profile == 'default' else {})
    if action == 'benchmark':
        r.ENV.update(configuration)
    r.artifacts_path = lambda: raw
    original_build = r.build
    def build(*args, **kwargs):
        binaries, identities = original_build(*args, **kwargs)
        identities['effective_profile_overrides'] = configuration if action == 'benchmark' else {}
        archived = {}
        for name, binary in binaries.items():
            copy = raw / 'binaries' / name
            copy.parent.mkdir(exist_ok=True)
            if copy.exists() and sha(copy) != identities['executable_sha256'][name]:
                raise ValueError('archived executable would be overwritten: ' + name)
            if not copy.exists():
                shutil.copyfile(binary, copy)
                copy.chmod(0o755)
            if sha(copy) != identities['executable_sha256'][name]:
                raise ValueError('executable changed during archival: ' + name)
            archived[name] = copy
        with tarfile.open(raw / 'sources.tar.gz', 'w:gz') as archive:
            for name, expected in identities['source_sha256']['rust_and_harness'].items():
                if sha(ROOT / name) != expected:
                    raise ValueError('source changed before archival: ' + name)
                archive.add(ROOT / name, arcname=name)
        # Resident drivers run immutable copies. A surviving old run or another
        # profile build cannot replace an executable during timing.
        binaries = archived
        return binaries, identities
    r.build = build
    if action == 'benchmark':
        import performance
        saved = json.loads((raw / 'benchmark.partial.json').read_text()) if resume else None
        if saved:
            previous = raw / 'benchmark.resumed-from.json'
            if previous.exists():
                raise ValueError('resume evidence already exists; inspect the previous continuation')
            with zipfile.ZipFile(raw / 'benchmark-buffers.zip') as buffers:
                for work in saved['workloads'].values():
                    for item in [work['input'], *work['outputs']]:
                        data = buffers.read(item['member'])
                        if hashlib.sha256(data).hexdigest() != item['sha256'] or len(data) != item['bytes']:
                            raise ValueError('interrupted benchmark buffer identity differs')
                    work['cpu_affinity'] = saved['cpu_affinity']
            write(previous, saved)
            old_build = r.build
            def resumed_build(*args, **kwargs):
                binaries, identities = old_build(*args, **kwargs)
                old = saved['identities']
                for key in ['executable_sha256', 'effective_profile_overrides', 'rustc', 'cxx']:
                    if old[key] != identities[key]:
                        raise ValueError('different benchmark build identity: ' + key)
                old_sources = old['source_sha256']
                current_sources = identities['source_sha256']
                if any(old_sources[k] != current_sources[k] for k in ['upstream', 'dependencies']):
                    raise ValueError('benchmark oracle or dependencies changed')
                old_rust = old_sources['rust_and_harness']
                current_rust = current_sources['rust_and_harness']
                changed = {n for n in old_rust.keys() | current_rust.keys() if old_rust.get(n) != current_rust.get(n)}
                if not changed <= {'parity/report.sh', 'parity/fuzz.sh', 'parity/fuzz-continuous.sh'}:
                    raise ValueError('benchmark algorithm or measurement source changed: ' + str(changed))
                saved['resume'] = {'previous_record_sha256': sha(previous), 'previous_workloads': len(saved['workloads']),
                                   'previous_identities': old, 'changed_orchestration_files': sorted(changed),
                                   'verified_identical_executables': True}
                saved['identities'] = identities
                return binaries, identities
            r.build = resumed_build
        # Keep the measured protocol in its lane-owned module unchanged. This
        # shell adapter adds resumability and per-session CPU metadata only.
        source = (ROOT / 'parity/performance.py').read_text()
        replacements = {
            "    artifact = r.artifacts_path() / 'benchmark-buffers.zip'":
                "    if saved is not None:\n        record = saved\n        record['resume']['physical_core'] = core\n    artifact = r.artifacts_path() / 'benchmark-buffers.zip'",
            "zipfile.ZipFile(artifact, 'w', zipfile.ZIP_DEFLATED, compresslevel=1)":
                "zipfile.ZipFile(artifact, 'a' if saved is not None else 'w', zipfile.ZIP_DEFLATED, compresslevel=1)",
            "                    data = bytearray(r.message(op, p, topology,":
                "                    if name in record['workloads']:\n                        continue\n                    data = bytearray(r.message(op, p, topology,",
            "                    work = {'family': r.FAMILIES[op],":
                "                    work = {'cpu_affinity': r.benchmark_cpu, 'family': r.FAMILIES[op],",
            "                    path = target/'paired-benchmark.input'":
                "                    path = target/('paired-' + record_directory + '.input')"}
        for old, new in replacements.items():
            if source.count(old) != 1:
                raise ValueError('measurement adapter no longer matches the pinned protocol')
            source = source.replace(old, new)
        module = types.ModuleType('performance')
        module.__dict__['saved'] = saved
        module.__dict__['record_directory'] = directory
        exec(compile(source, str(ROOT / 'parity/performance.py'), 'exec'), module.__dict__)
        sys.modules['performance'] = module
    sys.argv = ['runner.py', action, *argv]
    if action == 'sweep' and '--cases-per-family' not in argv:
        sys.argv += ['--cases-per-family', '10000']
    code = 0
    try:
        if action == 'gates':
            subprocess.run([str(ROOT / 'parity/gates.sh')], env=r.ENV, check=True)
        else:
            r.main()
    except SystemExit as e:
        code = e.code or 0
    finally:
        os.environ['MESHOPT_ARTIFACTS'] = str(base)
        if (raw / (action + '.json')).exists():
            path = raw / (action + '.json')
            detail = json.loads(path.read_text())
            detail.setdefault('artifacts', {}).update({str(p.relative_to(raw)): sha(p)
                for p in [raw / 'sources.tar.gz', *(raw / 'binaries').glob('*')] if p.is_file()})
            write(path, detail)
            compact(raw / (action + '.json'), ROOT / 'parity/results' / (label + '.json'), profile=profile if action == 'benchmark' else None)
    if code:
        raise SystemExit(code)

def archive_records():
    destination = artifacts() / 'historical'
    destination.mkdir(parents=True, exist_ok=True)
    for source in sorted((ROOT / 'parity/results').iterdir()):
        if source.suffix == '.json':
            value = json.loads(source.read_text())
            if value.get('schema') == 'meshopt-summary/1' or value.get('schema') == 2 and source.name == 'fuzz.json':
                continue
            archive = destination / source.name
            if archive.exists() and archive.read_bytes() != source.read_bytes():
                raise ValueError('historical artifact would be overwritten')
            archive.write_bytes(source.read_bytes())
            compact(archive, source, historical=True)
        elif source.suffix == '.crate':
            archive = destination / source.name
            archive.write_bytes(source.read_bytes())
            if sha(archive) != sha(source):
                raise ValueError('package archive copy differs')
            source.unlink()

def full_record(summary, missing):
    if summary['detail_artifact'] not in summary['artifacts']:
        raise ValueError('detailed record lacks a SHA-256 identity')
    for name, expected in summary['artifacts'].items():
        path = (artifacts() / name).resolve()
        if artifacts() not in path.parents:
            raise ValueError('artifact path leaves artifact directory')
        if not path.is_file():
            missing.append(name)
        elif sha(path) != expected:
            raise ValueError('artifact SHA-256 mismatch: ' + name)
    path = artifacts() / summary['detail_artifact']
    return json.loads(path.read_text()) if path.is_file() else None

def identity_corpus(destination):
    summary = json.loads((ROOT / 'parity/results/run.json').read_text())
    missing = []
    full = full_record(summary, missing)
    if missing or not full or not full['passed']:
        raise ValueError('qualified native corpus artifacts required: ' + ', '.join(missing))
    corpus = Path(destination)
    corpus.mkdir(parents=True, exist_ok=True)
    raw = artifacts() / summary['detail_artifact']
    archive = raw.parent / 'run-buffers.zip'
    records = [*full['cases'], {'family': 'math', 'id': 'math-probe', **full['math_probe']}]
    with zipfile.ZipFile(archive) as buffers, zipfile.ZipFile(corpus / 'native.zip', 'w', zipfile.ZIP_DEFLATED) as out:
        pairs = []
        for case in records:
            item = {'family': case['family'], 'id': case['id']}
            for role, value in [('input', case['input']), ('output', case['outputs']['rust'])]:
                data = buffers.read(value['member'])
                if hashlib.sha256(data).hexdigest() != value['sha256']:
                    raise ValueError('native corpus member hash mismatch')
                member = case['family'] + '/' + case['id'] + '.' + role
                out.writestr(zipfile.ZipInfo(member, (1980, 1, 1, 0, 0, 0)), data, compress_type=zipfile.ZIP_DEFLATED)
                item[role] = {'member': member, 'sha256': value['sha256']}
            pairs.append(item)
    manifest = {'schema': 1, 'origin': 'qualified Linux x86_64 Rust, exact against scalar-strict C++ and wasm32',
                'reference_revision': r.PIN, 'source_sha256': full['identities']['source_sha256']['rust_and_harness'],
                'archive_sha256': sha(corpus / 'native.zip'), 'counts': {**full['counts'], 'math': 1}, 'pairs': pairs}
    write(corpus / 'manifest.json', manifest)
    print('native corpus', len(pairs), 'pairs; SHA-256', manifest['archive_sha256'])

def identity_check(directory):
    corpus = Path(directory)
    manifest = json.loads((corpus / 'manifest.json').read_text())
    if manifest['reference_revision'] != r.PIN or sha(corpus / 'native.zip') != manifest['archive_sha256']:
        raise ValueError('unqualified native corpus identity')
    for name, expected in manifest['source_sha256'].items():
        if (name.startswith(('src/', 'parity/src/')) or name in ['Cargo.toml', 'Cargo.lock', 'parity/Cargo.toml', 'parity/Cargo.lock']) and sha(ROOT / name) != expected:
            raise ValueError('different Rust source: ' + name)
    target = Path(os.environ['CARGO_TARGET_DIR'])
    env = {**os.environ, 'RUSTFLAGS': '', 'CARGO_ENCODED_RUSTFLAGS': '', 'RUSTC_WRAPPER': '', 'RUSTC_WORKSPACE_WRAPPER': ''}
    subprocess.run(['cargo', 'build', '--locked', '--release', '--manifest-path', 'parity/Cargo.toml'], env=env, check=True)
    binary = target / 'release' / ('meshopt-driver.exe' if sys.platform == 'win32' else 'meshopt-driver')
    counts = Counter()
    ids = set()
    with zipfile.ZipFile(corpus / 'native.zip') as archive:
        for pair in manifest['pairs']:
            identifier = (pair['family'], pair['id'])
            if identifier in ids:
                raise ValueError('duplicate native corpus identifier')
            ids.add(identifier)
            data = archive.read(pair['input']['member'])
            expected = archive.read(pair['output']['member'])
            if any(hashlib.sha256(value).hexdigest() != pair[role]['sha256'] for role, value in [('input', data), ('output', expected)]):
                raise ValueError('native corpus member identity failure')
            result = subprocess.run([str(binary)], input=data, capture_output=True, env=env, check=True).stdout
            if result != expected:
                raise ValueError('cross-target bit mismatch: ' + pair['input']['member'])
            counts[pair['family']] += 1
    if set(counts) != set(r.FAMILIES.values()) | {'math'} or counts['math'] != 1 or any(counts[f] < 50 for f in r.FAMILIES.values()):
        raise ValueError('incomplete cross-target corpus')
    if dict(counts) != manifest['counts'] or not {(f, i) for i, f in r.UPSTREAM_FIXTURES.items()} <= ids:
        raise ValueError('missing recorded native outputs or upstream fixtures')
    write(corpus / ('identity-' + sys.platform + '.json'), {'passed': True, 'counts': dict(counts), 'archive_sha256': manifest['archive_sha256'],
          'executable_sha256': sha(binary), 'rustc': subprocess.check_output(['rustc', '-Vv'], text=True), 'target': sys.platform})
    print('cross-target exact identity:', dict(counts))

def render(records, fuzz, missing, incomplete):
    family_names = list(r.FAMILIES.values())
    passed = not missing and not incomplete and fuzz.get('passed', False) and all(
        n in records and records[n][0]['passed'] for n in ['run', 'sweep', 'benchmark-moss', 'gates', 'js'])
    evidence_complete = not missing and not incomplete and fuzz.get('passed', False) and all(
        n in records and records[n][0]['passed'] for n in ['run', 'sweep', 'gates', 'js'])
    summary = {'schema': 'meshopt-release-summary/1', 'phase': '0.1', 'passed': passed,
               'evidence_complete': evidence_complete,
               'moss_performance_accepted': 'benchmark-moss' in records and records['benchmark-moss'][0]['passed'],
               'report_exit_policy': 'complete verified evidence exits zero; failed Moss bars block release acceptance',
               'incomplete': sorted(incomplete), 'absent_current_artifacts': sorted(set(missing)),
               'local_execution_targets': ['linux-x86_64', 'wasm32 (qualified native Rust reference)'],
               'remote_execution': 'CI configured; local records do not establish remote execution',
               'records': {name: {'summary_sha256': sha(ROOT / 'parity/results' / (name + '.json')),
                                   'artifacts': value[0]['artifacts']} for name, value in records.items()},
               'counts': {name: records[name][0]['counts'] for name in ['run', 'sweep'] if name in records},
               'consumer_performance': {name: records[name][0]['functions'] for name in ['benchmark-moss', 'benchmark-default'] if name in records},
               'fuzz': {'passed': fuzz.get('passed', False), 'required_cpu_seconds_per_target': 14400,
                        'runs': fuzz.get('runs', []), 'artifacts': fuzz.get('artifacts', {})}}
    write(ROOT / 'parity/MEASURED_RESULTS.json', summary)
    lines = ['# Measured geometry parity', '', 'Exact meaningful output bits: scalar-strict C++ 1.3, native Rust and executed wasm32.', '',
             '| Function | Fixture corpus | Seeded release sweep | Mismatches |', '|---|---:|---:|---:|']
    for family in family_names:
        values = [records[n][0]['counts'][family] if n in records else 'Pending' for n in ['run', 'sweep']]
        mismatch = sum(records[n][0]['functions'][family]['mismatches'] for n in ['run', 'sweep'] if n in records)
        lines.append(f'| {family} | {values[0]} | {values[1]} | {mismatch} |')
    lines += ['', 'Seed: 20261002. Per-case records and buffers are external; their SHA-256 identities are in results/run.json and results/sweep.json.', '',
              'Long fuzzing requires four process CPU-hours for each of the five targets (owner amendment, RFC section 14, 2026-10-04). See results/fuzz.json for executions, crashes, elapsed and CPU time, and replay-corpus identities.', '',
              'Local checks do not establish remote macOS, Windows or Linux arm64 execution. CI compares a hashed native-output corpus exactly and keeps C++ on Linux.', '',
              'Overall release qualification: ' + ('passed' if passed else 'incomplete or blocked; see MEASURED_RESULTS.json') + '.']
    (ROOT / 'parity/MEASURED_PARITY.md').write_text('\n'.join(lines) + '\n')
    fat = json.loads((ROOT / 'parity/results/benchmark.json').read_text()).get('functions', {})
    lines = ['# Measured release performance', '', 'Rust/C++ time ratios; each value is the family geometric mean of case median paired ratios, followed by the maximum case in parentheses.', '',
             '| Function | Fat LTO (historical lane 3b) | Moss thin LTO | Cargo release defaults |', '|---|---:|---:|---:|']
    for family in family_names:
        values = [fat.get(family)] + [records[n][0]['functions'].get(family) if n in records else None for n in ['benchmark-moss', 'benchmark-default']]
        cells = [f"{v['geometric_mean']:.3f} ({v['maximum']:.3f})" if v else 'Pending' for v in values]
        lines.append('| ' + family + ' | ' + ' | '.join(cells) + ' |')
    lines += ['', 'RFC bars are unchanged: family geometric mean <= 1.25, maximum case <= 1.50, and requested output-plus-scratch memory <= 1.25 times C++.', '',
              'Moss uses thin LTO, one codegen unit and opt-level 3. Cargo defaults use LTO disabled, 16 codegen units and opt-level 3. Both consumer records disable release debug. The crate retains its own fat-LTO profile; dependents do not inherit it.', '',
              'The complete matrix has 204 allocating, caller-buffer and allocation-free cases, covering tiny, medium and million-triangle smooth, seam-heavy and sparse meshes. The single-thread resident drivers share one physical core, one warm-up each and 20 or 30 alternating pairs. Validation, required copies, allocation and scratch are timed.', '',
              'These are shared-load measurements. Per-case timings, dispersion, memory, load telemetry and source/executable identities are in the external records. This table does not claim quiet-host performance or a universal speedup.', '',
              'See results/benchmark.json, results/benchmark-moss.json and results/benchmark-default.json for per-function summaries and SHA-256 identities. report.sh --verify-artifacts verifies available data and identifies absent historical artifacts.']
    text = '\n'.join(lines) + '\n'
    (ROOT / 'parity/MEASURED_PERFORMANCE.md').write_text(text)
    (ROOT / 'parity/results/MEASURED_PERFORMANCE.md').write_text(text)

def report(verify):
    missing = []
    historical_missing = []
    records = {}
    for path in sorted((ROOT / 'parity/results').glob('*.json')):
        summary = json.loads(path.read_text())
        if summary.get('schema') != 'meshopt-summary/1' or summary.get('phase') in CODEC_PHASES:
            continue
        full = full_record(summary, historical_missing if summary['historical'] else missing) if verify else None
        if not summary['historical']:
            records[path.stem] = (summary, full)
        if full and not summary['historical'] and 'identities' in full:
            for name, expected in full['identities']['executable_sha256'].items():
                if full['artifacts'].get('binaries/' + name) != expected:
                    raise ValueError('archived executable differs from build identity: ' + name)
        if full and not summary['historical'] and summary['command'] in ['run', 'sweep']:
            raw = artifacts() / summary['detail_artifact']
            r.artifacts_path = lambda: raw.parent
            # full_record checked all attachments; the shared native verifier
            # owns the single ZIP of executable input/output buffers only.
            buffer = full['command'] + '-buffers.zip'
            r.verify_record({**full, 'artifacts': {buffer: full['artifacts'][buffer]}}, raw.parent)
            if full['identities']['source_sha256'] != r.snapshot(Path(os.environ['MESHOPT_REFERENCE'])):
                raise ValueError('stale ' + path.stem + ' record')
            if summary['counts'] != {f: sum(c['family'] == f for c in full['cases']) for f in r.FAMILIES.values()}:
                raise ValueError('summary counts differ')
            if any(len({c['id'] for c in full['cases'] if c['family'] == f}) != summary['counts'][f] for f in r.FAMILIES.values()):
                raise ValueError('duplicate parity case identifiers')
            if any(summary['functions'][f]['maximum_indices'] < 1000000 for f in r.FAMILIES.values()):
                raise ValueError('missing large geometry coverage')
            if path.stem == 'run' and ({c['id']: c['family'] for c in full['upstream_fixture_inventory']} != r.UPSTREAM_FIXTURES or not full['math_probe']['match']):
                raise ValueError('missing upstream fixtures or math identity')
            if path.stem == 'sweep' and min(summary['counts'].values()) < 10000:
                raise ValueError('release requires 10000 cases per family')
        if full and not summary['historical'] and summary['command'] == 'benchmark':
            if not full.get('completed') or not full.get('identities_unchanged'):
                raise ValueError('incomplete performance matrix')
            if full['identities']['source_sha256'] != r.snapshot(Path(os.environ['MESHOPT_REFERENCE'])):
                raise ValueError('stale benchmark')
            expected_profile = {'CARGO_PROFILE_RELEASE_OPT_LEVEL': '3', 'CARGO_PROFILE_RELEASE_DEBUG': '0',
                                'CARGO_PROFILE_RELEASE_LTO': 'thin' if path.stem == 'benchmark-moss' else 'false',
                                'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '1' if path.stem == 'benchmark-moss' else '16'}
            if full['identities']['effective_profile_overrides'] != expected_profile:
                raise ValueError('wrong consumer profile')
            if len(full['workloads']) != 204:
                raise ValueError('incomplete 204-case consumer matrix')
            for family in r.FAMILIES.values():
                cases = [w for w in full['workloads'].values() if w['family'] == family]
                expected_count = 12 if family == 'simplify_scale' else 72 if family.startswith('simplify') else 24
                if len(cases) != expected_count:
                    raise ValueError('incomplete benchmark family: ' + family)
                gm = math.exp(statistics.mean(math.log(w['rust_cpp_ratio']) for w in cases))
                maximum = max(w['rust_cpp_ratio'] for w in cases)
                memory = max(w['memory_ratio'] if w['memory_ratio'] is not None else math.inf for w in cases)
                expected = {'cases': len(cases), 'geometric_mean': gm, 'maximum': maximum,
                            'maximum_memory_ratio': memory, 'passed': gm <= 1.25 and maximum <= 1.5 and memory <= 1.25}
                if full['families'][family] != expected:
                    raise ValueError('benchmark family statistics or bars differ')
            if full['passed'] != all(f['passed'] for f in full['families'].values()):
                raise ValueError('benchmark verdict differs from unchanged RFC bars')
            archive = (artifacts() / summary['detail_artifact']).parent / 'benchmark-buffers.zip'
            with zipfile.ZipFile(archive) as buffers:
                for work in full['workloads'].values():
                    for item in [work['input'], *work['outputs']]:
                        data = buffers.read(item['member'])
                        if hashlib.sha256(data).hexdigest() != item['sha256'] or len(data) != item['bytes']:
                            raise ValueError('benchmark buffer identity mismatch')
                    if len(work.get('paired_ratio_stats', {}).get('raw_ratios', [])) < 20:
                        raise ValueError('missing benchmark paired samples')
                    outputs = []
                    for item in work['outputs']:
                        data = buffers.read(item['member'])
                        count = struct.unpack_from('<I', data, 12)[0]
                        outputs.append(r.response(data[:-8], samples=count)[0])
                    if len(outputs) != 2 or outputs[0] != outputs[1]:
                        raise ValueError('benchmark exact output mismatch')
                    for backend in ['rust', 'cpp']:
                        if len(work['stats'][backend]['raw_seconds']) < 20 or any(not math.isfinite(t) or t <= 0 for t in work['stats'][backend]['raw_seconds']):
                            raise ValueError('invalid benchmark sample')
            if summary['functions'] != full['families']:
                raise ValueError('benchmark summary differs from detailed record')
        if full and not summary['historical'] and path.stem == 'gates':
            if not full['passed'] or not full['identities_unchanged'] or any(c['exit'] for c in full['commands']):
                raise ValueError('build/package gates incomplete')
            for name, expected in full['source_sha256'].items():
                if sha(ROOT / name) != expected:
                    raise ValueError('stale build/package gates: ' + name)
        if full and not summary['historical'] and path.stem == 'js':
            if not full['passed'] or {s['suite'] for s in full['suites']} != {'encoder', 'decoder', 'simplifier', 'clusterizer', 'tangents'} or any(s['exit'] for s in full['suites']):
                raise ValueError('upstream JS sanity suite incomplete')
    if missing:
        print('ABSENT artifacts (hashes cannot be verified):\n' + '\n'.join(sorted(set(missing))))
    else:
        print('All recorded artifact SHA-256 hashes verified.' if verify else 'Artifact hashes not requested.')
    if historical_missing:
        print('ABSENT historical artifacts (hashes cannot be verified; historical evidence is not current qualification):\n' + '\n'.join(sorted(set(historical_missing))))
    required = {'run', 'sweep', 'benchmark-moss', 'benchmark-default', 'gates', 'js'}
    incomplete = required - records.keys()
    fuzz_path = ROOT / 'parity/results/fuzz.json'
    fuzz = json.loads(fuzz_path.read_text())
    if fuzz.get('schema') != 2 or not fuzz.get('passed') or not fuzz.get('identities_unchanged') or len(fuzz.get('runs', [])) != 5:
        incomplete.add('4 CPU-hours per fuzz target')
    elif (fuzz.get('required_cpu_seconds_per_target') != 14400
          or {x['target'] for x in fuzz['runs']} != set(r.FAMILIES.values())
          or any(x['cpu_seconds'] < 14400 or x['crashes'] or not x['executions'] for x in fuzz['runs'])):
        incomplete.add('4 CPU-hours per fuzz target')
    if verify and fuzz.get('schema') == 2:
        for name, expected in fuzz['artifacts'].items():
            path = artifacts() / name
            if not path.is_file():
                missing.append(name)
                print('ABSENT fuzz artifact:', name)
            elif sha(path) != expected:
                raise ValueError('fuzz artifact SHA-256 mismatch')
        detail_path = artifacts() / 'fuzz/record.json'
        if detail_path.is_file():
            detail = json.loads(detail_path.read_text())
            if detail['runs'] != fuzz['runs'] or detail['passed'] != fuzz['passed'] or detail['source_sha256'] != fuzz['source_sha256'] or detail['dependency_sha256'] != fuzz['dependency_sha256']:
                raise ValueError('fuzz summary differs from retained execution record')
            actual = {f: {'cpu_seconds': 0., 'executions': 0, 'crashes': 0, 'successes': 0} for f in r.FAMILIES.values()}
            for shard in detail['shards']:
                path = (artifacts() / shard['corpus']).resolve()
                if artifacts() not in path.parents or not path.is_file():
                    missing.append(shard['corpus'])
                    continue
                if sha(path) != shard['corpus_sha256']:
                    raise ValueError('fuzz corpus SHA-256 mismatch')
                corpus = json.loads(path.read_text())
                if corpus['seed'] != shard['seed'] or corpus['target'] != shard['target'] or corpus['executable_sha256'] != detail['executable_sha256'][shard['target']]:
                    raise ValueError('fuzz replay identity mismatch')
                if corpus['source_sha256'] != detail['source_sha256'] or corpus['executions'] != shard['executions']:
                    raise ValueError('fuzz replay source or execution count differs')
                for key in actual[shard['target']]:
                    actual[shard['target']][key] += shard[key]
            for run in detail['runs']:
                if any(actual[run['target']][key] != run[key] for key in actual[run['target']]):
                    raise ValueError('fuzz budget or executions differ from recorded shards')
            corpus_hashes = {n: hashlib.sha256(''.join(sorted(s['corpus_sha256'] for s in detail['shards'] if s['target'] == n)).encode()).hexdigest() for n in r.FAMILIES.values()}
            if corpus_hashes != fuzz['corpus_sha256']:
                raise ValueError('fuzz aggregate corpus SHA-256 differs')
            source_archive = artifacts() / 'fuzz/sources.tar.gz'
            if not source_archive.is_file():
                missing.append('fuzz/sources.tar.gz')
            else:
                with tarfile.open(source_archive) as archive:
                    for name, expected in detail['source_sha256'].items():
                        data = archive.extractfile(name).read()
                        if hashlib.sha256(data).hexdigest() != expected:
                            raise ValueError('archived fuzz source identity differs: ' + name)
            if fuzz['dependency_sha256'] != r.snapshot(Path(os.environ['MESHOPT_REFERENCE']))['dependencies']:
                raise ValueError('stale fuzz dependencies')
            for name, expected in detail['executable_sha256'].items():
                binary = artifacts() / 'fuzz/binaries' / name
                if not binary.is_file():
                    missing.append('fuzz/binaries/' + name)
                elif sha(binary) != expected:
                    raise ValueError('fuzz executable SHA-256 mismatch')
        for name, expected in fuzz['source_sha256'].items():
            if name == 'parity/fuzz.sh':
                continue  # Archived original orchestration; algorithm/binary identities remain exact.
            if sha(ROOT / name) != expected:
                raise ValueError('stale fuzz source: ' + name)
    if incomplete:
        print('Qualification incomplete:', ', '.join(sorted(incomplete)))
    render(records, fuzz, missing, incomplete)
    if incomplete:
        raise SystemExit(1)
    if any(not records[name][0]['passed'] for name in ['run', 'sweep']):
        raise SystemExit('exact parity gate failed')
    if missing:
        print('Summary available; release artifact verification incomplete.')
        raise SystemExit(1)
    if not records['benchmark-moss'][0]['passed']:
        print('Release evidence verified. Moss-profile RFC performance gate failed; release acceptance is blocked, bars unchanged.')
    else:
        print('Release parity, Moss-profile performance and long fuzz records pass.')

argv = sys.argv[1:]
if argv and argv[0] == '--execute':
    execute(argv[1:])
elif argv == ['--archive-records']:
    archive_records()
elif len(argv) == 2 and argv[0] == '--export-identity':
    identity_corpus(argv[1])
elif len(argv) == 2 and argv[0] == '--identity-check':
    identity_check(argv[1])
else:
    p = argparse.ArgumentParser()
    p.add_argument('--phase', choices=['0.1', '0.3', *CODEC_PHASES], default='0.1')
    p.add_argument('--verify-artifacts', action='store_true')
    args = p.parse_args(argv)
    if args.phase == '0.3':
        report03(args.verify_artifacts)
    elif args.phase in CODEC_PHASES:
        report_codec(args.verify_artifacts, args.phase)
    else:
        report(args.verify_artifacts)
PY
