#!/usr/bin/env python3
"""Run one ASan cargo-fuzz smoke per 0.5 public upstream entry point."""
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import random
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ.get('MESHOPT_ARTIFACTS', '/mnt/linux-extra/meshopt-artifacts/p05'))
TARGET = Path(os.environ.get('CARGO_TARGET_DIR', '/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p05'))
NAMES = ['stripify', 'stripify_bound', 'unstripify', 'unstripify_bound', 'vertex_cache', 'vertex_fetch', 'overdraw', 'coverage', 'omm_measure', 'omm_rasterize', 'omm_entry_size', 'omm_compact', 'tangents', 'normals', 'remesh']


def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    seconds = int(sys.argv[1]) if len(sys.argv) > 1 else 300
    if seconds < 1: raise SystemExit('positive seconds required')
    binary_dir = TARGET / 'x86_64-unknown-linux-gnu/release'
    base = ART / ('fuzz-0.5' if seconds == 300 else f'fuzz-0.5-preflight-{seconds}')
    base.mkdir(parents=True, exist_ok=True)
    cpus = sorted(os.sched_getaffinity(0))

    def one(pair):
        index, name = pair
        corpus = base / name / 'corpus'; crashes = base / name / 'crashes'
        corpus.mkdir(parents=True, exist_ok=True); crashes.mkdir(exist_ok=True)
        randomizer = random.Random('p05-' + name)
        for i in range(8): (corpus / f'seed-{i}').write_bytes(randomizer.randbytes(4 + i * 24))
        binary = binary_dir / name
        identity = sha(binary)
        log = base / name / 'smoke.log'
        command = ['taskset', '-c', str(cpus[index % len(cpus)]), str(binary), str(corpus), f'-max_total_time={seconds}', '-seed=20261004', '-max_len=256', '-timeout=10', '-rss_limit_mb=512', f'-artifact_prefix={crashes}/']
        start = time.monotonic()
        with log.open('w') as output: result = subprocess.run(command, stdout=output, stderr=subprocess.STDOUT, env={**os.environ, 'ASAN_OPTIONS': 'detect_leaks=1'})
        elapsed = time.monotonic() - start
        text = log.read_text()
        executions = re.findall(r'Done (\d+) runs', text)
        success = result.returncode == 0 and elapsed >= seconds and bool(executions) and not list(crashes.iterdir()) and sha(binary) == identity
        record = {'target': name, 'elapsed_seconds': elapsed, 'executions': int(executions[-1]) if executions else 0, 'exit_code': result.returncode, 'passed': success, 'binary_sha256': identity, 'log': str(log.relative_to(ART)), 'log_sha256': sha(log), 'corpus_files': len(list(corpus.iterdir())), 'cpu': cpus[index % len(cpus)]}
        print(name, 'PASS' if success else 'FAIL', record['executions'], flush=True)
        return record

    with concurrent.futures.ThreadPoolExecutor(max_workers=len(NAMES)) as pool: records = list(pool.map(one, enumerate(NAMES)))
    if seconds == 300:
        detail = base / 'record.json'
        full = {'schema': 'meshopt-p05-fuzz/1', 'seconds_per_target': seconds, 'targets': records, 'passed': all(r['passed'] for r in records)}
        detail.write_text(json.dumps(full, indent=2) + '\n')
        summary = {'schema': 'meshopt-p05-fuzz-summary/1', 'phase': '0.5', 'passed': full['passed'], 'detail_artifact': str(detail.relative_to(ART)), 'detail_sha256': sha(detail), 'targets': {r['target']: {'elapsed_seconds': r['elapsed_seconds'], 'executions': r['executions'], 'passed': r['passed']} for r in records}}
        (ROOT / 'parity/results/fuzz-0.5.json').write_text(json.dumps(summary, indent=2) + '\n')
    raise SystemExit(0 if all(r['passed'] for r in records) else 1)


if __name__ == '__main__': main()
