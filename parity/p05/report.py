#!/usr/bin/env python3
"""Verify complete phase-0.5 summaries and retained case artifacts."""
import gzip
import hashlib
import json
import math
import os
from pathlib import Path
import statistics

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ.get('MESHOPT_ARTIFACTS', '/mnt/linux-extra/meshopt-artifacts/p05'))
REQUIRED = ['run', 'sweep', 'wasm', 'fuzz', 'benchmark-moss', 'benchmark-default']
FAMILIES = ['stripify', 'stripify_bound', 'unstripify', 'unstripify_bound', 'vertex_cache', 'vertex_fetch', 'overdraw', 'coverage', 'omm_measure', 'omm_rasterize', 'omm_entry_size', 'omm_compact', 'tangents', 'normals', 'remesh']


def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()


def verify():
    errors = []
    for kind in REQUIRED:
        path = ROOT / f'parity/results/{kind}-0.5.json'
        if not path.exists(): errors.append('missing ' + kind); continue
        record = json.loads(path.read_text())
        if not record.get('passed'): errors.append('failed ' + kind)
        artifact = ART / record['detail_artifact']
        if not artifact.is_file() or sha(artifact) != record['detail_sha256']: errors.append('artifact identity ' + kind)
        if kind in ('sweep', 'wasm'):
            if record['counts'] != dict.fromkeys(FAMILIES, 2000): errors.append('incomplete ' + kind)
            if artifact.exists():
                with gzip.open(artifact, 'rt') as source:
                    cases = [json.loads(line) for line in source]
                if len(cases) != 30000 or any(not c['equal'] for c in cases): errors.append('case mismatch ' + kind)
            binaries = ({'sweep-0.5-rust': record['rust_sha256'], 'sweep-0.5-cpp.so': record['cpp_sha256']}
                        if kind == 'sweep' else {'wasm-0.5.wasm': record['wasm_sha256']})
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
            for family, verdict in detail['families'].items():
                name, api = family.split(':')
                rows = [row for row in detail['rows'] if row['family'] == name and row['api'] == api]
                if len(rows) != verdict['cases'] or len(rows) != 3 or any(len(row['samples']) < 10 for row in rows):
                    errors.append('benchmark samples ' + family)
                    continue
                ratios = [statistics.median(s['ratio'] for s in row['samples']) for row in rows]
                gm = math.exp(statistics.mean(math.log(value) for value in ratios))
                if abs(gm - verdict['geometric_mean']) > 1e-8 or abs(max(ratios) - verdict['maximum']) > 1e-8:
                    errors.append('benchmark statistics ' + family)
            if record['time_passed'] != all(v['passed'] for v in detail['families'].values()) or record['memory_passed'] is not None or record['corpus_complete']:
                errors.append('benchmark gate scope ' + profile)
        if kind == 'run':
            for name, expected in record['source_sha256'].items():
                if sha(ROOT / name) != expected: errors.append('stale fixture source ' + name)
    print(json.dumps({'phase': '0.5', 'verified': not errors, 'errors': errors}, indent=2))
    raise SystemExit(0 if not errors else 1)


if __name__ == '__main__': verify()
