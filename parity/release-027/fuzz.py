#!/usr/bin/env python3
"""Release-only CPU-accounted chunks, retained inputs/binaries and fail-fast findings."""
from concurrent.futures import FIRST_COMPLETED, ThreadPoolExecutor, wait
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_ARTIFACTS']) / 'fuzz'
TARGET = Path(os.environ['CARGO_TARGET_DIR'])
ART.mkdir(parents=True, exist_ok=False)
BUDGETS = dict(retention=900, vertex_fetch=300, overdraw=300, optimize_meshlet=300)

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def snapshot():
    paths = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', ROOT / 'fuzz/Cargo.toml',
             ROOT / 'fuzz/Cargo.lock', ROOT / 'fuzz/p03/Cargo.toml', ROOT / 'fuzz/p03/Cargo.lock',
             *ROOT.glob('src/**/*.rs'), *ROOT.glob('fuzz/src/**/*.rs'),
             *ROOT.glob('fuzz/p03/**/*.rs'), ROOT / 'fuzz/p03/src.rs', Path(__file__)]
    return {str(p.relative_to(ROOT)): sha(p) for p in sorted(set(paths))}

sources = snapshot()
with tarfile.open(ART / 'sources.tar.gz', 'w:gz') as archive:
    for name in sources:
        archive.add(ROOT / name, arcname=name)
binaries = {}
for name in BUDGETS:
    source = TARGET / ('x86_64-unknown-linux-gnu/release' if name == 'optimize_meshlet' else 'release') / name
    dest = ART / name
    shutil.copy2(source, dest)
    binaries[name] = dest
identities = {n: sha(p) for n, p in binaries.items()}
cpus = []
seen = set()
for cpu in sorted(os.sched_getaffinity(0)):
    topology = Path(f'/sys/devices/system/cpu/cpu{cpu}/topology')
    key = tuple((topology / f).read_text() for f in ['physical_package_id', 'core_id'])
    if key not in seen:
        cpus.append(cpu)
        seen.add(key)
    if len(cpus) == 4:
        break
record = dict(source_sha256=sources, binary_sha256=identities, required_cpu_seconds=BUDGETS,
              cpus=cpus, instrumentation='stable seeded mutation; meshlet: libFuzzer coverage, sanitizer none', shards=[])
totals = {n: dict(cpu_seconds=0., executions=0, crashes=0) for n in BUDGETS}

def persist():
    record['totals'] = totals
    (ART / 'record.json').write_text(json.dumps(record, indent=2) + '\n')

def shard(name, serial, cpu):
    folder = ART / f'{serial:04d}-{name}'
    folder.mkdir()
    seed = 20261026 + serial * 104729
    seconds = 30
    if name == 'optimize_meshlet':
        corpus = folder / 'corpus'
        corpus.mkdir()
        for i, triangle in enumerate([[0, 1, 2], [2, 0, 2], [2, 2, 2]]):
            data = bytearray(32)
            data[0:2] = bytes([3, 1])
            data[20:23] = bytes(triangle)
            (corpus / f'one-triangle-{i}').write_bytes(data)
        command = [str(binaries[name]), str(corpus), f'-max_total_time={seconds}',
                   '-max_len=4096', '-timeout=10', '-rss_limit_mb=512', f'-seed={seed}',
                   '-print_final_stats=1', f'-artifact_prefix={folder}/']
    else:
        command = [str(binaries[name]), str(seconds), str(seed)]
    command = ['taskset', '-c', str(cpu), 'nice', '-n', '19', *command]
    (folder / 'input.json').write_text(json.dumps(dict(target=name, seed=seed, command=command,
        replay_contract='Stable: seconds, seed, optional execution count; meshlet: retained initial/final corpus and libFuzzer crash artifact.'), indent=2) + '\n')
    with (folder / 'stdout').open('w') as out, (folder / 'stderr').open('w') as err:
        proc = subprocess.run(['/usr/bin/time', '-o', str(folder / 'usage.json'), '-f',
            '{"user":%U,"system":%S,"elapsed":%e}', *command], stdout=out, stderr=err, timeout=seconds + 60)
    usage = json.loads((folder / 'usage.json').read_text().splitlines()[-1])
    if name == 'optimize_meshlet':
        stderr = (folder / 'stderr').read_text()
        match = re.search(r'stat::number_of_executed_units:\s*(\d+)', stderr)
        executions = int(match[1]) if match else 0
        instrumented = bool(re.search(r'Loaded \d+ modules?.*inline 8-bit counters', stderr))
    else:
        stats = json.loads((folder / 'stdout').read_text()) if proc.returncode == 0 else {}
        executions = stats.get('executions', 0)
        instrumented = True
    result = dict(target=name, serial=serial, seed=seed, cpu=cpu, usage=usage,
        cpu_seconds=usage['user'] + usage['system'], executions=executions,
        crashes=int(proc.returncode != 0), exit=proc.returncode, instrumented=instrumented)
    if not executions or not instrumented:
        result['crashes'] = 1
    (folder / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    return result

persist()
serial = 0
active = {}
free = cpus.copy()
failed = False
with ThreadPoolExecutor(max_workers=len(cpus)) as pool:
    while True:
        while free and not failed:
            candidates = [n for n in BUDGETS if totals[n]['cpu_seconds'] +
                30 * sum(x[0] == n for x in active.values()) < BUDGETS[n]]
            if not candidates:
                break
            name = max(candidates, key=lambda n: BUDGETS[n] - totals[n]['cpu_seconds'] -
                30 * sum(x[0] == n for x in active.values()))
            cpu = free.pop(0)
            active[pool.submit(shard, name, serial, cpu)] = (name, cpu)
            serial += 1
        if not active:
            break
        completed, _ = wait(active, return_when=FIRST_COMPLETED)
        for future in completed:
            name, cpu = active.pop(future)
            free.append(cpu)
            result = future.result()
            record['shards'].append(result)
            for key in ['cpu_seconds', 'executions', 'crashes']:
                totals[name][key] += result[key]
            failed |= bool(result['crashes'])
            persist()
            print(json.dumps(dict(target=name, **totals[name])), flush=True)
        if failed:
            # Finish already-running bounded chunks, retaining every receipt;
            # never schedule another input after a finding.
            for future, (name, _) in active.items():
                result = future.result()
                record['shards'].append(result)
                for key in ['cpu_seconds', 'executions', 'crashes']:
                    totals[name][key] += result[key]
            break
record['identities_unchanged'] = sources == snapshot() and identities == {n: sha(p) for n, p in binaries.items()}
record['passed'] = not failed and record['identities_unchanged'] and all(
    totals[n]['cpu_seconds'] >= seconds and totals[n]['executions'] > 0 for n, seconds in BUDGETS.items())
persist()
if not record['passed']:
    raise SystemExit('release fuzz failed; stop and inspect retained input and stderr')
