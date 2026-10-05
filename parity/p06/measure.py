#!/usr/bin/env python3
"""One fast P06 curve, five pairs/case, resumable pauses with no worker alive."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import statistics
import struct
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_ARTIFACTS']) / 'parallel'
TARGET = Path(os.environ['CARGO_TARGET_DIR'])
REF = Path(os.environ['MESHOPT_REFERENCE'])
LEASE = Path.home() / 'Documents/automation_game/assets_toolings/Moss/scripts/gpu-lease.sh'
FAMILIES = ['lod', 'encode', 'decode', 'meshlets', 'cluster_lod', 'hierarchy']
THREADS = [1, 2, 4, 8, 16]

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def command(args):
    return subprocess.check_output(args, text=True).strip()

def sources():
    files = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', *sorted((ROOT / 'src').rglob('*.rs')),
             *sorted((ROOT / 'tests').glob('*.rs')),
             *sorted((ROOT / 'parity/p06').rglob('*.rs')),
             ROOT / 'parity/p06/Cargo.toml', ROOT / 'parity/p06/Cargo.lock', Path(__file__)]
    return {str(p.relative_to(ROOT)): sha(p) for p in files}

def admission():
    status = command([str(LEASE), 'status'])
    units = command(['systemctl', '--user', 'list-units', 'moss-scoreboard-*', '--state=running',
                     '--plain', '--no-pager', '--no-legend'])
    observations = []
    for line in units.splitlines():
        unit = line.split()[0]
        properties = dict(x.split('=', 1) for x in command(['systemctl', '--user', 'show', unit,
                              '-p', 'MainPID', '-p', 'ControlGroup']).splitlines())
        main = int(properties['MainPID'])
        cgroup = Path('/sys/fs/cgroup') / properties['ControlGroup'].lstrip('/')
        processes = []
        for file in cgroup.rglob('cgroup.procs'):
            for pid in file.read_text().split():
                try:
                    comm = (Path('/proc') / pid / 'comm').read_text().strip()
                except FileNotFoundError:
                    continue
                processes.append({'pid': int(pid), 'comm': comm})
        children = [p for p in processes if p['pid'] != main]
        # The owner explicitly permits a wrapper whose only child is sleep.
        idle = all(p['comm'] == 'sleep' for p in children)
        observations.append({'unit': unit, 'main_pid': main, 'processes': processes, 'idle': idle})
    return {'unix': time.time(), 'load': Path('/proc/loadavg').read_text().strip(),
            'gpu_status': status, 'scoreboards': observations,
            'admitted': status.startswith('GPU lease: FREE') and all(o['idle'] for o in observations)}

def select_cores():
    allowed = os.sched_getaffinity(0)
    def ticks():
        out = {}
        for line in Path('/proc/stat').read_text().splitlines():
            f = line.split()
            if f[0].startswith('cpu') and f[0][3:].isdigit():
                values = list(map(int, f[1:9])); out[int(f[0][3:])] = (sum(values), values[3] + values[4])
        return out
    before = ticks(); time.sleep(1); after = ticks()
    groups = {}
    for cpu in allowed:
        topology = Path(f'/sys/devices/system/cpu/cpu{cpu}/topology')
        key = (int((topology / 'physical_package_id').read_text()), int((topology / 'core_id').read_text()))
        total = after[cpu][0] - before[cpu][0]; idle = after[cpu][1] - before[cpu][1]
        if total <= 0 or not 0 <= idle <= total:
            raise ValueError('invalid CPU tick interval')
        groups.setdefault(key, []).append({'cpu': cpu, 'busy': (total - idle) / total,
                                          'total_ticks': total, 'busy_ticks': total - idle})
    ranked = sorted(groups.items(), key=lambda g: (statistics.mean(p['busy'] for p in g[1]), g[0]))
    cores = [{'package': k[0], 'core': k[1], **min(v, key=lambda p: (p['busy'], p['cpu']))} for k, v in ranked]
    if len(cores) < 16:
        raise ValueError('the registered curve needs sixteen allowed physical cores')
    return cores

def read_exact(pipe, n):
    out = bytearray()
    while len(out) < n:
        data = pipe.read(n - len(out))
        if not data:
            raise ValueError('timing driver exited or truncated a response')
        out.extend(data)
    return bytes(out)

def sample(driver, kind):
    driver.stdin.write(kind.encode() + b'\n'); driver.stdin.flush()
    if read_exact(driver.stdout, 1) != b'T':
        raise ValueError('invalid sample frame')
    seconds, length = struct.unpack('<dQ', read_exact(driver.stdout, 16))
    if not math.isfinite(seconds) or seconds <= 0 or length > 128 * 1024 * 1024:
        raise ValueError('invalid sample time or bounded output length')
    return seconds, read_exact(driver.stdout, length)

def save(record):
    ART.mkdir(parents=True, exist_ok=True)
    (ART / 'curve.json').write_text(json.dumps(record, indent=2) + '\n')

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--resume', action='store_true')
    args = parser.parse_args()
    ART.mkdir(parents=True, exist_ok=True)
    binary = TARGET / 'release/meshopt-p06-parity'
    obj = REF / 'demo/pirate.obj'
    identity = {'sources': sources(), 'binary_sha256': sha(binary), 'obj_sha256': sha(obj),
                'reference': command(['git', '-C', str(REF), 'rev-parse', 'HEAD']),
                'rustc': command(['rustc', '-Vv']), 'cargo': command(['cargo', '-V']),
                'machine': command(['lscpu', '-J']),
                'profile': {'opt_level': 3, 'codegen_units': 16, 'lto': False},
                'dependencies': json.loads(command(['cargo', 'metadata', '--offline', '--locked',
                                       '--format-version', '1', '--manifest-path', str(ROOT / 'parity/p06/Cargo.toml')]))['packages']}
    if args.resume and (ART / 'curve.json').exists():
        record = json.loads((ART / 'curve.json').read_text())
        if identity != record['identity']:
            raise ValueError('resume identity differs; do not combine source epochs')
        for case in record['cases'].values():
            if sha(ART / case['output_file']) != case['output_sha256']:
                raise ValueError('retained output changed')
    else:
        if (ART / 'curve.json').exists():
            raise ValueError('existing curve: use --resume rather than overwrite samples')
        record = {'schema': 'meshopt-p06-curve/1', 'identity': identity, 'meshes': 16,
                  'corpus': 'sixteen translated/scaled variants of pinned demo/pirate.obj; positions only',
                  'source_credit': 'Pirate by Clint Bellanger, CC-BY-SA 3.0, https://opengameart.org/content/pirate',
                  'pairs_per_case': 5, 'bar': None, 'cases': {}, 'admissions': [], 'bursts': [], 'complete': False}
    admitted = admission(); record['admissions'].append(admitted); save(record)
    if not admitted['admitted']:
        print('Paused before timing: GPU lease or active scoreboard.', flush=True); return 75
    cores = select_cores()
    started = time.monotonic()
    burst = {'started_unix': time.time(), 'cores': cores, 'ended_unix': None}
    record['bursts'].append(burst); save(record)
    try:
        for family in FAMILIES:
            for threads in THREADS:
                key = f'{family}/{threads}'
                if key in record['cases'] and len(record['cases'][key]['pairs']) == 5:
                    continue
                observation = admission(); record['admissions'].append(observation); save(record)
                if not observation['admitted'] or time.monotonic() - started > 600:
                    print('Paused: admission or ten-minute burst limit.', flush=True); return 75
                selected = [c['cpu'] for c in cores[:threads]]
                driver = subprocess.Popen(['taskset', '-c', ','.join(map(str, selected)), str(binary), family,
                                          str(threads), str(obj), '16'], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
                try:
                    if read_exact(driver.stdout, 1) != b'R':
                        raise ValueError('driver not ready')
                    # Pin each pool worker to one physical core and serial caller to core zero.
                    affinities = {}
                    for task in Path(f'/proc/{driver.pid}/task').iterdir():
                        name = (task / 'comm').read_text().strip()
                        if name.startswith('p06-worker-'):
                            cpu = selected[int(name.rsplit('-', 1)[1])]
                            os.sched_setaffinity(int(task.name), {cpu})
                            affinities[task.name] = {'name': name, 'cpus': sorted(os.sched_getaffinity(int(task.name)))}
                    if len(affinities) != threads:
                        raise ValueError('missing pool thread affinity')
                    os.sched_setaffinity(driver.pid, {selected[0]})
                    admitted = admission(); record['admissions'].append(admitted); save(record)
                    if not admitted['admitted']:
                        return 75
                    _, expected = sample(driver, 'S')
                    _, actual = sample(driver, 'P')
                    if expected != actual:
                        raise ValueError('warm-up byte mismatch')
                    if key not in record['cases']:
                        name = key.replace('/', '-') + '.output'
                        (ART / name).write_bytes(expected)
                        record['cases'][key] = {'family': family, 'threads': threads, 'pairs': [],
                            'output_file': name, 'output_sha256': hashlib.sha256(expected).hexdigest(),
                            'output_bytes': len(expected), 'affinities': [], 'warmups': 0}
                    case = record['cases'][key]
                    if hashlib.sha256(expected).hexdigest() != case['output_sha256']:
                        raise ValueError('resumed output differs')
                    case['affinities'].append(affinities); case['warmups'] += 1
                    while len(case['pairs']) < 5:
                        before = admission(); record['admissions'].append(before); save(record)
                        if not before['admitted'] or time.monotonic() - started > 600:
                            return 75
                        order = ['S', 'P'] if len(case['pairs']) % 2 == 0 else ['P', 'S']
                        times = {}
                        for kind in order:
                            times[kind], data = sample(driver, kind)
                            if data != expected:
                                raise ValueError('timed sample byte mismatch')
                        after = admission()
                        case['pairs'].append({'sequential_seconds': times['S'], 'parallel_seconds': times['P'],
                                              'speedup': times['S'] / times['P'], 'order': order,
                                              'before': before, 'after': after})
                        case['speedup_median'] = statistics.median(p['speedup'] for p in case['pairs'])
                        case['speedup_range'] = [min(p['speedup'] for p in case['pairs']), max(p['speedup'] for p in case['pairs'])]
                        save(record)
                        if not after['admitted']:
                            print('Paused after the current pair: lease/scoreboard started.', flush=True); return 75
                    print(f'{key}: {case["speedup_median"]:.3f}x', flush=True)
                finally:
                    if driver.poll() is None:
                        driver.stdin.write(b'Q\n'); driver.stdin.flush(); driver.stdin.close()
                    if driver.wait(timeout=10) != 0:
                        raise ValueError('driver exited unsuccessfully')
                # The process exits here; no cores remain held between cases.
        if identity['sources'] != sources() or sha(binary) != identity['binary_sha256'] or sha(obj) != identity['obj_sha256']:
            raise ValueError('timing identity changed')
        record['complete'] = True; save(record)
    finally:
        burst['ended_unix'] = time.time(); burst['elapsed_seconds'] = time.monotonic() - started; save(record)
    return 0

if __name__ == '__main__':
    sys.exit(main())
