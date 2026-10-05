"""Visible shared admission plus GPU lease, separate from the frozen harness."""
import hashlib
import os
import time
from pathlib import Path

POLICY = 'owner-visible-heavy-4GB-2026-10-05'


def admission():
    assert os.environ.get('MOSS_GPU_LEASE') == '1'
    assert os.environ.get('MOSS_GPU_LEASE_LABEL') == 'heavy:timeout'
    holder = (Path(os.environ.get('MOSS_GPU_LEASE_DIR', '/tmp/moss-gpu-lease')) / 'holder').read_text().strip()
    label, pid, since = holder.split('|')
    assert label == 'heavy:timeout'
    active = int(os.environ['MOSS_HEAVY_ACTIVE'])
    assert int(os.environ['MOSS_HEAVY_RESERVED_GB']) >= 4
    ancestor = os.getpid()
    chain = []
    while ancestor > 1:
        proc = Path('/proc') / str(ancestor)
        chain.append({'pid': ancestor, 'command': (proc / 'cmdline').read_bytes().replace(b'\0', b' ').decode()})
        status = (proc / 'status').read_text()
        ancestor = int(next(line.split()[1] for line in status.splitlines() if line.startswith('PPid:')))
    assert int(pid) in {r['pid'] for r in chain}, 'lease holder must be an ancestor'
    wrapper = next(r for r in chain if r['pid'] == active)
    assert '/mnt/linux-extra/moss-coord/bin/moss-heavy.sh 4 timeout 840 ' in wrapper['command']
    admitted = Path(os.environ['MOSS_HEAVY_ADMITTED'])
    assert admitted.resolve() == Path('/usr/bin/timeout').resolve()
    queue = Path(os.environ.get('MOSS_HEAVY_DIR', os.environ.get('MOSS_COORD_DIR', '/mnt/linux-extra/moss-coord')))
    receipt_path = queue / 'heavy.reservations'
    receipt = next(line for line in receipt_path.read_text().splitlines() if line.split()[0] == str(active))
    assert int(receipt.split()[1]) >= 4
    return {'admitted': True, 'policy': POLICY, 'lease_holder': holder,
            'heavy_pid': active, 'reserved_gb': int(os.environ['MOSS_HEAVY_RESERVED_GB']),
            'queue_receipt_path': str(receipt_path), 'queue_receipt': receipt,
            'admitted_command_path': str(admitted), 'admitted_command_sha256': hashlib.sha256(admitted.read_bytes()).hexdigest(),
            'ancestors': chain, 'load': os.getloadavg(), 'unix': time.time()}
