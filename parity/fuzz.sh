#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
: "${CARGO_TARGET_DIR:?set an isolated CARGO_TARGET_DIR}"
export PYTHONDONTWRITEBYTECODE=1
exec python3 - "$@" <<'PY'
"""CPU-accounted, replayable seeded mutation qualification (at most 8 workers)."""
import argparse
import fcntl
from concurrent.futures import ThreadPoolExecutor, wait, FIRST_COMPLETED
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import signal
import subprocess
import sys
import tarfile
import time

ROOT = Path.cwd()
sys.path.insert(0, str(ROOT / 'fuzz'))
import smoke

p = argparse.ArgumentParser()
p.add_argument('--phase', choices=['0.1'], default='0.1')
p.add_argument('--cpu-hours-per-target', type=float, default=4)
p.add_argument('--jobs', '--cores', type=int, default=8)
p.add_argument('--chunk-seconds', type=int, default=300)
p.add_argument('--seed', type=int, default=20261002)
p.add_argument('--resume', action='store_true')
p.add_argument('--continuous', action='store_true')
p.add_argument('--wall-seconds', type=float)
a = p.parse_args()
if (not 1 <= a.jobs <= 8 or not 0 < a.cpu_hours_per_target < float('inf')
        or a.chunk_seconds < 1 or a.seed < 1 or a.wall_seconds is not None
        and not 0 < a.wall_seconds < float('inf')):
    p.error('positive budget, chunk and seed; 1 through 8 workers required')
target = Path(os.environ['CARGO_TARGET_DIR']).resolve()
artifacts = Path(os.environ.get('MESHOPT_ARTIFACTS', target / 'parity-artifacts')).resolve()
if artifacts == target or target in artifacts.parents:
    p.error('long fuzz artifacts must survive build-target cleanup; set MESHOPT_ARTIFACTS')
if ROOT == artifacts or ROOT in artifacts.parents:
    p.error('release fuzz artifacts must be outside the repository')
base = artifacts / ('fuzz-continuous' if a.continuous else 'fuzz')
base.mkdir(parents=True, exist_ok=True)
lock = (base / 'run.lock').open('a')
try:
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
except BlockingIOError:
    raise SystemExit('another fuzz driver owns this artifact directory')
destination = Path(os.environ.get('MESHOPT_RESULTS', ROOT / 'parity/results'))
destination.mkdir(parents=True, exist_ok=True)
(target / 'tmp').mkdir(parents=True, exist_ok=True)
env = {**smoke.ENV, 'TMPDIR': str(target / 'tmp')}
before = smoke.snapshot()
driver_sha = smoke.sha(ROOT / 'parity/fuzz.sh')
subprocess.run(['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'fuzz/Cargo.toml'], env=env, check=True)
names = ['vertex_cache', 'overdraw', 'simplify', 'simplify_with_attributes', 'simplify_scale']
binaries = {}
for name in names:
    archive = base / 'binaries' / name
    archive.parent.mkdir(exist_ok=True)
    if archive.exists() and smoke.sha(archive) != smoke.sha(target / 'release' / name):
        raise SystemExit('saved fuzz executable differs: ' + name)
    if not archive.exists():
        shutil.copyfile(target / 'release' / name, archive)
    archive.chmod(0o755)
    binaries[name] = archive
identities = {n: smoke.sha(b) for n, b in binaries.items()}
dependencies = smoke.dependency_snapshot()
if before != smoke.snapshot():
    raise SystemExit('source changed during fuzz build')
if not (base / 'sources.tar.gz').exists():
    with tarfile.open(base / 'sources.tar.gz', 'w:gz') as archive:
        for name in before:
            archive.add(ROOT / name, arcname=name)
cpus = [int(c) for c in os.environ.get('MESHOPT_FUZZ_CPUS', '').split(',') if c]
if not cpus:
    # Prefer one logical processor per physical core, leaving other cores for
    # the consumer benchmarks. Affinity is fixed before each target starts.
    seen = set()
    for cpu in sorted(os.sched_getaffinity(0)):
        topology = Path(f'/sys/devices/system/cpu/cpu{cpu}/topology')
        key = tuple((topology / f).read_text().strip() for f in ['physical_package_id', 'core_id'])
        if key not in seen:
            cpus.append(cpu)
            seen.add(key)
        if len(cpus) == a.jobs:
            break
if not cpus or len(set(cpus)) != len(cpus) or not set(cpus) <= os.sched_getaffinity(0):
    p.error('distinct allowed CPUs required for every worker')
a.jobs = min(a.jobs, len(cpus))
record_path = base / 'record.json'
if a.continuous and not record_path.exists() and (artifacts / 'fuzz/record.json').exists():
    saved = json.loads((artifacts / 'fuzz/record.json').read_text())
    if ({n: h for n, h in saved['source_sha256'].items() if n != 'parity/fuzz.sh'} != before
            or saved['dependency_sha256'] != dependencies or saved['executable_sha256'] != identities):
        raise SystemExit('saved release corpus belongs to different sources or executables')
    saved['continuation_origin'] = {'record': 'fuzz/record.json',
                                  'sha256': smoke.sha(artifacts / 'fuzz/record.json'),
                                  'corpus_contract': 'Retain completed seeded input ranges; continue with unused seeds.'}
    shutil.copyfile(artifacts / 'fuzz/sources.tar.gz', base / 'sources.tar.gz')
    record_path.write_text(json.dumps(saved, indent=2) + '\n')
if a.resume or a.continuous and record_path.exists():
    record = json.loads(record_path.read_text())
    recorded_sources = {n: h for n, h in record['source_sha256'].items() if n != 'parity/fuzz.sh'}
    if (recorded_sources != before or record['dependency_sha256'] != dependencies
            or record['executable_sha256'] != identities or record['seed'] != a.seed):
        raise SystemExit('cannot resume a different algorithm source, dependency, binary or seed')
    if not a.continuous and record['required_cpu_seconds_per_target'] != a.cpu_hours_per_target * 3600:
        if record['required_cpu_seconds_per_target'] != 86400 or a.cpu_hours_per_target != 4:
            raise SystemExit('different release budget requires a recorded amendment')
        record['budget_amendment'] = {'date': '2026-10-04', 'authority': 'RFC section 14, owner decision',
                                     'previous_cpu_seconds_per_target': 86400, 'cpu_seconds_per_target': 14400}
        record['required_cpu_seconds_per_target'] = 14400
else:
    if record_path.exists():
        raise SystemExit('fuzz record already exists; use --resume or a new artifact directory')
    record = {'schema': 2, 'phase': a.phase, 'profile': 'stable seeded mutation; no sanitizer or coverage instrumentation',
              'source_sha256': before, 'dependency_sha256': dependencies, 'executable_sha256': identities,
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
              'hardware': {k: v for k, v in platform.uname()._asdict().items() if k != 'node'},
              'started_unix': time.time(), 'required_cpu_seconds_per_target': a.cpu_hours_per_target * 3600,
              'jobs': a.jobs, 'worker_cpus': cpus[:a.jobs], 'seed': a.seed, 'shards': [], 'passed': False}
record.setdefault('driver_sha256_history', [])
if driver_sha not in record['driver_sha256_history']:
    record['driver_sha256_history'].append(driver_sha)
record['coverage'] = 'unavailable: existing stable targets have no coverage instrumentation'
for s in record['shards']:
    corpus_path = artifacts / s['corpus']
    if not corpus_path.is_file() or smoke.sha(corpus_path) != s['corpus_sha256']:
        raise SystemExit('saved replay corpus missing or changed: ' + s['corpus'])
totals = {n: {'cpu_seconds': 0., 'elapsed_seconds': 0., 'executions': 0, 'successes': 0, 'crashes': 0, 'shards': 0} for n in names}
for s in record['shards']:
    t = totals[s['target']]
    for key in ['cpu_seconds', 'elapsed_seconds', 'executions', 'successes', 'crashes']:
        t[key] += s[key]
    t['shards'] += 1
if any(t['crashes'] for t in totals.values()):
    raise SystemExit('retained crash is a release blocker')

session_start = time.monotonic()
deadline = session_start + a.wall_seconds if a.wall_seconds else float('inf')
session_cpu_limit = {n: t['cpu_seconds'] + a.cpu_hours_per_target * 3600 for n, t in totals.items()}
stopping = False
def stop(signum, frame):
    global stopping
    stopping = True  # Finish the current bounded chunks and retain exact statistics.
signal.signal(signal.SIGTERM, stop)
signal.signal(signal.SIGINT, stop)

def persist():
    record['runs'] = [{'target': n, **t} for n, t in totals.items()]
    record['updated_unix'] = time.time()
    temp = record_path.with_suffix('.partial')
    temp.write_text(json.dumps(record, indent=2) + '\n')
    temp.replace(record_path)
    summary = {k: v for k, v in record.items() if k != 'shards'}
    summary['artifacts'] = {'fuzz/record.json': smoke.sha(record_path)}
    summary['corpus_sha256'] = {n: hashlib.sha256(''.join(sorted(s['corpus_sha256'] for s in record['shards'] if s['target'] == n)).encode()).hexdigest() for n in names}
    summary['corpus_contract'] = 'Replay descriptors: target binary, seed and execution count reproduce every generated input from the retained exact source.'
    if not a.continuous:
        (destination / 'fuzz.json').write_text(json.dumps(summary, indent=2) + '\n')

def shard(name, serial, cpu):
    seed = a.seed + serial * 104729
    directory = base / 'corpus' / name
    directory.mkdir(parents=True, exist_ok=True)
    prefix = directory / str(serial)
    seconds = a.chunk_seconds if a.wall_seconds is None else max(1, min(a.chunk_seconds, int(deadline - time.monotonic())))
    command = ['taskset', '-c', str(cpu), str(binaries[name]), str(seconds), str(seed)]
    usage = prefix.with_suffix('.usage.json')
    with prefix.with_suffix('.stdout').open('w') as out, prefix.with_suffix('.stderr').open('w') as err:
        proc = subprocess.Popen(['/usr/bin/time', '-o', str(usage), '-f', '{"user":%U,"system":%S,"elapsed":%e}', *command], stdout=out, stderr=err, env=env, start_new_session=True)
        # The target owns the elapsed chunk. CPU accounting, rather than elapsed
        # time, decides when a family's release budget is complete.
        try:
            proc.wait(timeout=seconds + 30)
        except subprocess.TimeoutExpired:
            # Kill the target, allowing GNU time to finish recording CPU usage.
            subprocess.run(['pkill', '-KILL', '-P', str(proc.pid)], check=False)
            proc.wait(timeout=10)
            err.write('\nhang: chunk wall budget exceeded\n')
    u = json.loads(usage.read_text().splitlines()[-1])
    stats = json.loads(prefix.with_suffix('.stdout').read_text()) if proc.returncode == 0 else {}
    failure = re.search(r'execution=(\d+)', prefix.with_suffix('.stderr').read_text())
    replay_count = stats.get('executions', int(failure.group(1)) + 1 if failure else (2**64 - 1))
    corpus = {'schema': 1, 'target': name, 'seed': seed, 'executions': replay_count,
              'execution_count_known': proc.returncode == 0 or failure is not None,
              'replay_contract': 'Known execution prefix, or the full deterministic input stream when a timeout has no counter.',
              'executable_sha256': identities[name], 'source_sha256': record['source_sha256'],
              'replay': [str(binaries[name]), '18446744073709551615', str(seed), str(replay_count)]}
    corpus_path = prefix.with_suffix('.json')
    corpus_path.write_text(json.dumps(corpus, indent=2) + '\n')
    result = {'target': name, 'serial': serial, 'seed': seed, 'cpu': cpu, 'exit': proc.returncode,
              'cpu_seconds': u['user'] + u['system'], 'elapsed_seconds': u['elapsed'],
              'executions': stats.get('executions', 0), 'successes': stats.get('successes', 0),
              'crashes': int(proc.returncode != 0), 'corpus': str(corpus_path.relative_to(artifacts)),
              'corpus_sha256': smoke.sha(corpus_path)}
    if proc.returncode:
        crashes = base / 'crashes' / name / str(serial)
        crashes.mkdir(parents=True, exist_ok=True)
        for f in directory.glob(str(serial) + '.*'):
            shutil.copyfile(f, crashes / f.name)
    return result

persist()
serial = max((s['serial'] for s in record['shards']), default=-1) + 1
active = {}
free = cpus[:a.jobs].copy()
failed = False
try:
    with ThreadPoolExecutor(max_workers=a.jobs) as pool:
        while True:
            while free and not failed and not stopping and time.monotonic() < deadline:
                candidates = [n for n in names if totals[n]['cpu_seconds'] <
                              (session_cpu_limit[n] if a.continuous else record['required_cpu_seconds_per_target'])]
                if not candidates:
                    break
                name = min(candidates, key=lambda n: totals[n]['cpu_seconds'] + sum(a.chunk_seconds for x in active.values() if x[0] == n))
                cpu = free.pop(0)
                active[pool.submit(shard, name, serial, cpu)] = (name, cpu)
                serial += 1
            if not active:
                break
            completed, _ = wait(active, return_when=FIRST_COMPLETED)
            for future in completed:
                name, cpu = active.pop(future)
                free.append(cpu)
                s = future.result()
                record['shards'].append(s)
                for key in ['cpu_seconds', 'elapsed_seconds', 'executions', 'successes', 'crashes']:
                    totals[name][key] += s[key]
                totals[name]['shards'] += 1
                failed |= bool(s['crashes'])
                persist()
                if a.continuous:
                    item = {'schema': 1, 'unix': time.time(), 'target': name,
                            'cumulative_cpu_hours': totals[name]['cpu_seconds'] / 3600,
                            'executions': totals[name]['executions'], 'new_coverage': None,
                            'coverage': record['coverage'], 'findings': totals[name]['crashes'],
                            'shard': s, 'corpus_sha256': smoke.sha(base / 'record.json'),
                            'executable_sha256': identities[name], 'source_sha256': before}
                    with (base / 'ledger.jsonl').open('a') as ledger:
                        ledger.write(json.dumps(item, sort_keys=True) + '\n')
                        ledger.flush()
                        os.fsync(ledger.fileno())
                print(json.dumps({'target': name, 'cpu_hours': round(totals[name]['cpu_seconds'] / 3600, 4), 'executions': totals[name]['executions'], 'crashes': totals[name]['crashes']}), flush=True)
finally:
    record['identities_unchanged'] = before == smoke.snapshot() and dependencies == smoke.dependency_snapshot() and identities == {n: smoke.sha(b) for n, b in binaries.items()}
    record['passed'] = not failed and record['identities_unchanged'] and all(t['cpu_seconds'] >= record['required_cpu_seconds_per_target'] and t['successes'] > 0 for t in totals.values())
    record['finished_unix'] = time.time()
    persist()
if a.continuous:
    if failed or not record['identities_unchanged']:
        raise SystemExit('background fuzz finding or identity change; see saved replay input')
    if a.cpu_hours_per_target < 1000000000 and not stopping and any(
            totals[n]['cpu_seconds'] < session_cpu_limit[n] for n in names):
        raise SystemExit('background CPU budget incomplete within the wall-clock limit')
elif not record['passed']:
    raise SystemExit('release fuzz gate failed or incomplete; see retained record and replay corpus')
PY
