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
summary = {'schema': 'meshopt-p05-run/1', 'phase': '0.5', 'passed': result.returncode == 0 and '6 passed; 0 failed' in text, 'fixture_tests': 6, 'source_sha256': sources, 'detail_artifact': log.name, 'detail_sha256': sha(log)}
(ROOT / 'parity/results/run-0.5.json').write_text(json.dumps(summary, indent=2) + '\n')
raise SystemExit(0 if summary['passed'] else 1)
