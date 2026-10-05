#!/usr/bin/env python3
"""Queue bounded timing commands through visible shared heavy admission."""
import argparse
import os
from pathlib import Path
import subprocess
import sys
import time

from benchmark import source_identity, reference_identity
from timing_gate import wait

HEAVY = Path('/mnt/linux-extra/moss-coord/bin/moss-heavy.sh')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--lane', required=True)
    parser.add_argument('--memory-gb', type=int, default=4, choices=range(1, 65))
    parser.add_argument('--inside', action='store_true')
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command or not args.lane.startswith('p05'):
        parser.error('a p05 lane and command are required')
    if args.inside:
        os.environ['MESHOPT_TIMING_BURST_START'] = str(time.monotonic())
        wait('burst:' + args.lane)
        # The admission wrapper owns the timeout and descendant cleanup.
        # Keep its process group; do not acquire another GPU lease.
        raise SystemExit(subprocess.run(command).returncode)
    source, reference = source_identity(), reference_identity()
    env = {**os.environ, 'MOSS_HEAVY_GPU': '1',
           'MOSS_LANE': 'meshopt-timing:' + args.lane,
           'MESHOPT_TIMING_LANE': args.lane}
    # Force each burst into visible admission, never an inherited nested path.
    for key in ('MOSS_GPU_LEASE', 'MOSS_GPU_LEASE_LABEL', 'MOSS_HEAVY_ACTIVE',
                'MOSS_HEAVY_ADMITTED', 'MOSS_HEAVY_RESERVED_GB',
                'MESHOPT_TIMING_BURST_START'):
        env.pop(key, None)
    while True:
        if source_identity() != source or reference_identity() != reference:
            raise SystemExit('source changed between admitted timing bursts')
        result = subprocess.run([str(HEAVY), str(args.memory_gb), 'timeout',
                                 '--foreground', '--signal=TERM', '--kill-after=5',
                                 '840', sys.executable, str(Path(__file__).resolve()),
                                 '--inside', '--lane', args.lane, '--', *command], env=env)
        if result.returncode not in (75, 76):
            raise SystemExit(result.returncode)
        print('requeueing after ' + ('admission timeout' if result.returncode == 75
                                    else 'completed checkpointed burst'), flush=True)


if __name__ == '__main__':
    main()
