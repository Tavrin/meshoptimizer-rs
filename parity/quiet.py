"""Linux load telemetry and measured physical-core selection for lane 3b."""
import json
import os
from pathlib import Path
import time


LIMIT = 1.5
INTERVAL_SECONDS = 30
TIMEOUT_SECONDS = 3600


class QuietTimeout(RuntimeError):
    pass


def load():
    # Read the requested source, retaining all three averages.
    return [float(value) for value in Path('/proc/loadavg').read_text().split()[:3]]


def physical_core():
    allowed = sorted(os.sched_getaffinity(0))
    def ticks():
        result = {}
        for line in Path('/proc/stat').read_text().splitlines():
            fields = line.split()
            if fields and fields[0].startswith('cpu') and fields[0][3:].isdigit():
                # guest time is already included in user/nice; exclude duplicates.
                values = [int(v) for v in fields[1:9]]
                result[int(fields[0][3:])] = (sum(values), values[3]+values[4])
        return result
    samples = []
    before = ticks()
    for _ in range(3):
        started = time.time()
        time.sleep(1)
        after = ticks()
        usage = {}
        for cpu in allowed:
            total = after[cpu][0]-before[cpu][0]
            idle = after[cpu][1]-before[cpu][1]
            if total <= 0 or not 0 <= idle <= total:
                raise ValueError('invalid /proc/stat interval')
            usage[cpu] = {'total_ticks': total, 'busy_ticks': total-idle,
                          'busy_percent': 100*(total-idle)/total}
        samples.append({'started_unix': started, 'finished_unix': time.time(), 'cpus': usage})
        before = after
    cpus = {}
    cores = {}
    for cpu in allowed:
        topology = Path(f'/sys/devices/system/cpu/cpu{cpu}/topology')
        package = int((topology/'physical_package_id').read_text())
        core = int((topology/'core_id').read_text())
        busy = sum(s['cpus'][cpu]['busy_ticks'] for s in samples)
        total = sum(s['cpus'][cpu]['total_ticks'] for s in samples)
        cpus[cpu] = {'package_id': package, 'core_id': core, 'busy_percent': 100*busy/total,
                     'thread_siblings': (topology/'thread_siblings_list').read_text().strip()}
        group = cores.setdefault((package, core), {'cpus': [], 'busy': 0, 'total': 0})
        group['cpus'].append(cpu); group['busy'] += busy; group['total'] += total
    winning = min(cores, key=lambda key: (cores[key]['busy']/cores[key]['total'], key))
    cpu = min(cores[winning]['cpus'], key=lambda n: (cpus[n]['busy_percent'], n))
    return {'cpu': cpu, **cpus[cpu], 'allowed_cpus': allowed,
            'method': 'three one-second /proc/stat intervals; least aggregate-busy physical core, then least-busy allowed sibling; taskset before driver startup',
            'selection_samples': samples,
            'physical_core_usage': [{'package_id': k[0], 'core_id': k[1], 'cpus': v['cpus'],
                                     'busy_percent': 100*v['busy']/v['total']} for k,v in sorted(cores.items())]}


def wait(record, destination, context):
    started = time.monotonic()
    admission = {'context': context, 'started_unix': time.time(),
                 'threshold_exclusive': LIMIT, 'interval_seconds': INTERVAL_SECONDS,
                 'timeout_seconds': TIMEOUT_SECONDS, 'checks': [], 'passed': False}
    record.setdefault('quiet_waits', []).append(admission)
    while True:
        averages = load()
        elapsed = time.monotonic() - started
        admission['checks'].append({'unix': time.time(), 'elapsed_seconds': elapsed,
                                    'load_average': averages})
        admission['waited_seconds'] = elapsed
        admission['passed'] = averages[0] < LIMIT
        destination.write_text(json.dumps(record, indent=2) + '\n')
        if admission['passed']:
            print(f'quiet admission {context}: load {averages[0]:.2f}, waited {elapsed:.1f}s', flush=True)
            return admission
        if elapsed >= TIMEOUT_SECONDS:
            raise QuietTimeout(f'quiet admission {context}: load {averages[0]:.2f} never below {LIMIT} within {TIMEOUT_SECONDS}s')
        print(f'quiet wait {context}: load {averages[0]:.2f}, elapsed {elapsed:.1f}s', flush=True)
        time.sleep(min(INTERVAL_SECONDS, TIMEOUT_SECONDS - elapsed))
