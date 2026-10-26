"""Attribution resident-loop protocol; instructions only, three fresh arms."""
from pathlib import Path
import hashlib
import json
import math
import os
import struct
import subprocess

ART = Path('/mnt/linux-extra/moss-scratch/meshopt-0212-simplify')
PERF = '/usr/lib/linux-tools/6.17.0-22-generic/perf'
CPU = 9
ARMS = ['base', 'candidate', 'M7']
OPS = ['simplify', 'simplify-attributes', 'simplify-update', 'simplify-sloppy', 'simplify-prune', 'simplify-points']
CASES = ['tiny-smooth', 'medium-smooth', 'medium-seams', 'medium-sparse', 'million-smooth']

def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def valid(cell, data):
    op = cell['operation']
    if Path(cell['input']).suffix == '.tsv':
        vertices = cell['vertices']
    else:
        with Path(cell['input']).open('rb') as stream:
            vertices, _ = struct.unpack('<II', stream.read(8))
    assert len(data) % 4 == 0
    if op == 'simplify-update':
        count = struct.unpack_from('<I', data)[0]
        assert len(data) == 4 * (count + 2 + 3 * vertices)
        indices = memoryview(data)[4:4 * (count + 1)]
        assert all(math.isfinite(v[0]) for v in struct.iter_unpack('<f', data[4 * (count + 1):]))
    elif op in ['simplify', 'simplify-attributes', 'simplify-sloppy']:
        assert len(data) >= 4 and math.isfinite(struct.unpack_from('<f', data, len(data) - 4)[0])
        indices = memoryview(data)[:-4]
    else:
        indices = memoryview(data)
    assert all(v[0] < vertices for v in struct.iter_unpack('<I', indices))
    if op != 'simplify-points':
        assert len(indices) % 12 == 0

def run(cell, arm, rep, rows):
    case, op = cell['case'], cell['operation']
    prefix = ART / 'samples' / f'{case}-{op}-{arm}-{rep}'
    dst = ART / 'outputs' / f'{case}-{op}-{arm}-{rep}.bin'
    assert not prefix.with_suffix('.perf').exists() and not dst.exists()
    n = 10000 if case == 'tiny-smooth' else 10 if case.startswith('medium') else 1
    cr, cw = os.pipe()
    ar, aw = os.pipe()
    command = [PERF, 'stat', '-e', 'instructions:u', '-x', ';', '-o', str(prefix.with_suffix('.perf')),
               '-D', '-1', '--control', f'fd:{cr},{aw}', '--', 'taskset', '-c', str(CPU),
               str(ART / f'driver-{arm}'), op, cell['input'], str(dst)]
    row = dict(cell, arm=arm, rep=rep, repetitions=n, command=command,
               admission=os.environ['MOSS_HEAVY_ACTIVE'], input_sha256=sha(cell['input']),
               binary_sha256=sha(ART / f'driver-{arm}'))
    try:
        result = subprocess.run(command, env=dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar)),
                                input=f'{n}\nstop\n', pass_fds=(cr, cw, ar, aw),
                                capture_output=True, text=True, timeout=240)
    finally:
        for fd in [cr, cw, ar, aw]:
            os.close(fd)
    row['exit'] = result.returncode
    prefix.with_suffix('.json').write_text(json.dumps(row, indent=2) + '\n')
    prefix.with_suffix('.stderr').write_text(result.stderr)
    prefix.with_suffix('.stdout').write_text(result.stdout)
    assert result.returncode == 0 and result.stdout == 'READY\nCOUNTED\n', row
    data = dst.read_bytes()
    valid(cell, data)
    row.update(output_sha256=hashlib.sha256(data).hexdigest(), output_bytes=len(data), validation=True)
    if arm == 'candidate':
        assert data == (ART / 'outputs' / f'{case}-{op}-base-0.bin').read_bytes(), row
        row['base_byte_identical'] = True
    line = next(s for s in prefix.with_suffix('.perf').read_text().splitlines() if ';instructions:u;' in s)
    fields = line.split(';')
    count, running = int(fields[0]), float(fields[4])
    assert count > 0 and running == 100.0, line
    old = [r for r in rows if (r['case'], r['operation'], r['arm']) == (case, op, arm)]
    assert not old or old[0]['output_sha256'] == row['output_sha256'], row
    row.update(instructions=count, per_call=count / n, running_percent=running)
    prefix.with_suffix('.json').write_text(json.dumps(row, indent=2) + '\n')
    with (ART / 'counts.jsonl').open('a') as stream:
        stream.write(json.dumps(row) + '\n')
    rows.append(row)

if __name__ == '__main__':
    assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU'] == '0'
    for filename, expected in json.loads((ART / 'candidate-source.json').read_text()).items():
        assert sha(filename) == expected, ('source changed after build', filename)
    for filename, expected in json.loads((ART / 'ENTRY.json').read_text())['inputs'].items():
        assert sha(filename) == expected, ('input changed after registration', filename)
    for stage in json.loads((ART / 'stages.json').read_text()):
        for field in ['vertex_blob', 'index_blob']:
            assert sha(stage[field]) == Path(stage[field]).stem
    assert CPU in os.sched_getaffinity(0)
    assert subprocess.check_output([PERF, '--version'], text=True).startswith('perf version 6.17')
    assert not (ART / 'counts.jsonl').exists(), 'retain rejected campaign; register any rerun separately'
    cells = [dict(case=case, family=op, operation=op, input=str(ART / 'inputs' / f'{case}.bin')) for op in OPS for case in CASES]
    cells += [dict(s, operation='simplify') for s in json.loads((ART / 'stages.json').read_text())]
    assert len(cells) == 48
    for directory in ['samples', 'outputs']:
        (ART / directory).mkdir()
    rows = []
    for cell in cells:
        for rep in range(3):
            for arm in ARMS if rep % 2 == 0 else ['base', 'M7', 'candidate']:
                run(cell, arm, rep, rows)
        print(cell['case'], cell['operation'], 'complete', flush=True)
    (ART / 'measurement-exit.json').write_text(json.dumps({'exit': 0, 'cells': 48, 'arms': 3, 'samples': len(rows)}, indent=2) + '\n')
