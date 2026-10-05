#!/usr/bin/env python3
"""Run pinned 0.5 fixture tests and retain a source-identified record."""
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ.get('MESHOPT_ARTIFACTS', '/mnt/linux-extra/meshopt-artifacts/p05'))
ART.mkdir(parents=True, exist_ok=True)
if 'CARGO_TARGET_DIR' not in os.environ:
    raise SystemExit('set CARGO_TARGET_DIR')


def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()


sources = {str(path.relative_to(ROOT)): sha(path) for path in sorted(ROOT.glob('src/**/*.rs'))}
sources['tests/p05.rs'] = sha(ROOT / 'tests/p05.rs')
log = ART / 'run-0.5.log'
command = ['cargo', 'test', '--offline', '--test', 'p05', '--all-features']
with log.open('w') as output:
    result = subprocess.run(command, cwd=ROOT, stdout=output, stderr=subprocess.STDOUT)
text = log.read_text()
fixtures = ['pinned_strip_and_cache_vectors', 'pinned_raster_vector', 'pinned_opacity_vectors',
            'upstream_tangents_basic', 'upstream_normals_basic', 'pinned_remesh_tetrahedron']
budgets = ['batched_strip_and_fetch_work_respects_exact_boundary',
           'batched_normal_tangent_and_remesh_work_respects_exact_boundary',
           'opacity_bulk_probe_work_preserves_numerical_failure_prefix']
contracts = ['raster_into_uses_caller_storage_and_preserves_atomic_failure',
             'normal_tangent_into_uses_caller_storage_and_preserves_failures',
             'normal_tangent_peak_storage_excludes_retired_remap_table',
             'strip_append_output_preserves_caller_prefixes_and_tails',
             'opacity_measure_ranges_keep_global_sources_and_work_prefixes']
passed = result.returncode == 0 and f'{len(fixtures) + len(budgets) + len(contracts)} passed; 0 failed' in text and all(
    f'test {name} ... ok' in text for name in fixtures + budgets + contracts)
summary = {'schema': 'meshopt-p05-run/1', 'phase': '0.5', 'passed': passed,
           'fixture_tests': len(fixtures), 'work_budget_tests': len(budgets),
           'contract_tests': len(contracts),
           'source_sha256': sources, 'detail_artifact': log.name, 'detail_sha256': sha(log)}
(ROOT / 'parity/results/run-0.5.json').write_text(json.dumps(summary, indent=2) + '\n')
raise SystemExit(0 if summary['passed'] else 1)
