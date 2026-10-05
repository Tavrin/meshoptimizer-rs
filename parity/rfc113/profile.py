#!/usr/bin/env python3
"""Collect small, reproducible perf call-graph summaries for two S2 cooks."""
import argparse
import json
import os
import struct
import subprocess

import run


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--core', type=int, default=21)
    parser.add_argument('--perf', default='/usr/lib/linux-tools/6.17.0-22-generic/perf')
    args = parser.parse_args()
    assert args.core in os.sched_getaffinity(0)
    binaries = {'cpp': run.TARGET / 'rfc113-cpp',
                'moss_cpp': run.TARGET / 'rfc113-moss-cpp',
                'rust': run.TARGET / 'consumer/rfc113-rust'}
    assert all(path.is_file() for path in binaries.values())
    run.ART.mkdir(parents=True, exist_ok=True)
    records = []
    for name in ('pyramid', 'riverforest_01_branches'):
        payload, meta = run.prepare(run.MESHES / f'{name}.mesh', 48)
        request = b'R11T' + payload[4:]
        framed = struct.pack('<I', len(request)) + request
        for side, binary in binaries.items():
            data = run.TARGET / f'profile-{name}-{side}.data'
            try:
                subprocess.run([args.perf, 'record', '-F', '99', '-g', '--call-graph', 'dwarf,8192', '-o', str(data),
                                '--', 'taskset', '-c', str(args.core), str(binary)],
                               input=framed, stdout=subprocess.DEVNULL, check=True)
                report = subprocess.run([args.perf, 'report', '-i', str(data), '--stdio',
                                         '--no-children', '--sort', 'symbol',
                                         '--percent-limit', '0.3'],
                                        capture_output=True, text=True, check=True).stdout
                path = run.ART / f'profile-{name}-{side}.txt'
                path.write_text(report)
                records.append({'case': name, 'side': side, 'mesh': meta['input_sha256'],
                                'binary': run.sha(binary), 'report': str(path)})
                print(name, side, path, flush=True)
            finally:
                data.unlink(missing_ok=True)
    (run.ART / 'profile-identity.json').write_text(json.dumps(records, indent=2) + '\n')


if __name__ == '__main__':
    main()
