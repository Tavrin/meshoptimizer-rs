#!/usr/bin/env python3
"""Verify phase-0.5 timing coverage by visible shared heavy admission."""
from datetime import datetime
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import time
from zoneinfo import ZoneInfo

ART = Path(os.environ.get('MESHOPT_ARTIFACTS', '/mnt/linux-extra/meshopt-artifacts/p05'))
LEASE_DIR = Path(os.environ.get('MOSS_GPU_LEASE_DIR', '/tmp/moss-gpu-lease'))


def cpu_ticks():
    result = {}
    for line in Path('/proc/stat').read_text().splitlines():
        fields = line.split()
        if fields and fields[0].startswith('cpu') and fields[0][3:].isdigit():
            values = [int(value) for value in fields[1:9]]
            result[int(fields[0][3:])] = (sum(values), values[3] + values[4])
    return result


def cpu_utilization(before, after, cpus):
    # Requested counters for every allowed logical CPU; guest ticks are not
    # counted twice. Very short intervals can legitimately have zero ticks.
    result = {}
    for cpu in cpus:
        total = after[cpu][0] - before[cpu][0]
        idle = after[cpu][1] - before[cpu][1]
        if total < 0 or not 0 <= idle <= total:
            raise ValueError('invalid /proc/stat interval')
        result[cpu] = {'total_ticks': total, 'busy_ticks': total - idle,
                       'busy_percent': 100 * (total - idle) / total if total else None}
    return result


def command(args):
    try:
        result = subprocess.run(args, capture_output=True, text=True, timeout=30)
        return result.returncode, result.stdout.strip(), result.stderr.strip()
    except (OSError, subprocess.TimeoutExpired) as error:
        return -1, '', str(error)


def covered(record):
    match = re.search(r"^GPU lease: HELD by '([^']+)' \(pid (\d+)\)",
                      record.get('lease', ''), re.MULTILINE)
    lock = re.search(r'FLOCK\s+ADVISORY\s+WRITE\s+\d+\s+(\S+)\s+0\s+EOF',
                     record.get('lease_fd_lock', ''))
    return bool(match and lock and lock.group(1) == record.get('lease_key')
                and record.get('gate') == 'shared-admission-owned-lease'
                and record.get('lease_exit') == 0
                and record.get('lease_env') == '1'
                and record.get('lease_label') == 'heavy:timeout'
                and match.group(1) == record['lease_label']
                and int(match.group(2)) == record.get('holder_pid')
                and record['holder_pid'] in record.get('process_ancestors', [])
                and record.get('heavy_pid', 0) in record.get('process_ancestors', [])
                and 1 <= record.get('heavy_reserved_gb', 0) <= 64
                and record.get('heavy_admitted') == '/usr/bin/timeout'
                and record.get('timing_lane', '').startswith('p05')
                and record.get('dashboard_lane') == 'meshopt-timing:' + record['timing_lane']
                and 0 <= record.get('burst_elapsed_seconds', -1) < 840)


def lease_receipt():
    # Read the shared holder and its kernel lock receipt; never invoke the
    # acquisition script, including its reentrant path or status subprocess.
    try:
        label, pid, started = (LEASE_DIR / 'holder').read_text().strip().split('|')
        pid = int(pid)
        stat = (LEASE_DIR / 'gpu.lock').stat()
        key = f'{os.major(stat.st_dev):02x}:{os.minor(stat.st_dev):02x}:{stat.st_ino}'
        locks = Path(f'/proc/{pid}/fdinfo/200').read_text()
        elapsed = max(0, int(time.time()) - int(started))
        return 0, f"GPU lease: HELD by '{label}' (pid {pid}) for {elapsed}s", '', key, locks
    except (OSError, ValueError) as error:
        return -1, '', str(error), '', ''


def ancestors():
    result = []
    pid = os.getpid()
    while pid > 1 and pid not in result:
        result.append(pid)
        status = Path(f'/proc/{pid}/status').read_text()
        pid = int(next(line.split()[1] for line in status.splitlines()
                       if line.startswith('PPid:')))
    return result


def snapshot(label):
    now = datetime.now(ZoneInfo('Europe/Paris'))
    lease_rc, lease, lease_error, lease_key, lease_lock = lease_receipt()
    match = re.search(r"^GPU lease: HELD by '([^']+)' \(pid (\d+)\)", lease, re.MULTILINE)
    started = os.environ.get('MESHOPT_TIMING_BURST_START')
    record = {'at': now.isoformat(), 'label': label, 'gate': 'shared-admission-owned-lease',
              'load_1m': os.getloadavg()[0], 'load_average': os.getloadavg(),
              'lease_exit': lease_rc, 'lease': lease, 'lease_error': lease_error,
              'lease_key': lease_key, 'lease_fd_lock': lease_lock,
              'heavy_pid': int(os.environ.get('MOSS_HEAVY_ACTIVE', '0')),
              'heavy_reserved_gb': int(os.environ.get('MOSS_HEAVY_RESERVED_GB', '0')),
              'heavy_admitted': os.environ.get('MOSS_HEAVY_ADMITTED'),
              'dashboard_lane': os.environ.get('MOSS_LANE'),
              'timing_lane': os.environ.get('MESHOPT_TIMING_LANE', ''),
              'lease_env': os.environ.get('MOSS_GPU_LEASE'),
              'lease_label': os.environ.get('MOSS_GPU_LEASE_LABEL', ''),
              'holder_pid': int(match.group(2)) if match else None,
              'process_ancestors': ancestors(),
              'burst_elapsed_seconds': time.monotonic() - float(started) if started else -1}
    record['reasons'] = [] if covered(record) else ['not covered by shared admission and its timing lease']
    record['admitted'] = not record['reasons']
    ART.mkdir(parents=True, exist_ok=True)
    with (ART / 'timing-gate.jsonl').open('a') as output:
        output.write(json.dumps(record, separators=(',', ':')) + '\n')
    return record


def wait(label):
    record = snapshot(label)
    if not record['admitted']:
        raise RuntimeError('timing requires lease_run.py: ' + ', '.join(record['reasons']))
    return record


if __name__ == '__main__':
    if sys.argv[1:] == ['--monitor']:
        while True:
            record = snapshot('monitor')
            print(json.dumps({k: record[k] for k in ('at', 'load_1m', 'admitted', 'reasons')}), flush=True)
            time.sleep(15)
    elif sys.argv[1:] == ['--wait']:
        print(json.dumps(wait('manual')), flush=True)
    else:
        raise SystemExit('usage: timing_gate.py --monitor|--wait')
