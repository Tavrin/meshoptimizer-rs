"""Owner-authorized queued timing admission, separate from the frozen harness."""
import os
import time
from pathlib import Path


def admission():
    assert os.environ.get('MOSS_GPU_LEASE') == '1'
    assert os.environ.get('MOSS_GPU_LEASE_LABEL') == 'meshopt-timing:p07'
    holder = (Path(os.environ.get('MOSS_GPU_LEASE_DIR', '/tmp/moss-gpu-lease')) / 'holder').read_text().strip()
    label, pid, since = holder.split('|')
    assert label == 'meshopt-timing:p07'
    ancestor = os.getpid()
    while ancestor != int(pid):
        status = (Path('/proc') / str(ancestor) / 'status').read_text()
        ancestor = int(next(line.split()[1] for line in status.splitlines() if line.startswith('PPid:')))
        assert ancestor > 1
    return {'admitted': True, 'policy': 'owner-queued-lease-2026-10-05',
            'lease_holder': holder, 'load': os.getloadavg(), 'unix': time.time()}
