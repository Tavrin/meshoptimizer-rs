"""Resumable, interleaved, resident-loop instructions:u campaign (SPEC 0217).

Derived from parity/0216-qualify/measure.py. Environment:
  PEERS_ART   directory holding driver-<arm>, inputs/ and inputs.json
  PEERS_PERF  perf executable used for `perf stat`
  PEERS_CPU   pinned CPU (default 9)
  MEASURE_ARMS, MEASURE_OPS, MEASURE_CASES  comma-separated selections
  MEASURE_REPS  repetitions per cell (default 3)
  MEASURE_NAME  campaign name; records go to <art>/<name>/
One record per finished sample is appended and fsynced; finished samples are
skipped on restart. Base/candidate output hashes must match; a cell whose
candidate/base median ratio exceeds 1.01 is recorded, not fatal, so the
campaign can finish and the regression rule can be applied afterwards.
"""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import tempfile

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
art = Path(os.environ['PEERS_ART'])
perf = os.environ['PEERS_PERF']
cpu = int(os.environ.get('PEERS_CPU', '9'))
assert cpu in os.sched_getaffinity(0)
arms = os.environ.get('MEASURE_ARMS', 'base,candidate,cpp,rust').split(',')
ops = os.environ.get('MEASURE_OPS', 'simplify-attributes').split(',')
reps = int(os.environ.get('MEASURE_REPS', '3'))
out_dir = art / os.environ['MEASURE_NAME']


def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


binaries = {arm: sha(art / f'driver-{arm}') for arm in arms}
inputs = json.loads((art / 'inputs.json').read_text())
cases = os.environ.get('MEASURE_CASES')
if cases:
    wanted = cases.split(',')
    inputs = [x for x in inputs if x['case'] in wanted]
    assert len(inputs) == len(wanted)
for x in inputs:
    assert sha(art / 'inputs' / f"{x['case']}.bin") == x['sha256']
out_dir.mkdir(exist_ok=True)
registration = dict(binaries=binaries, inputs=inputs, cpu=cpu, ops=ops, arms=arms, reps=reps,
                    runner_sha256=sha(Path(__file__)),
                    perf=subprocess.check_output([perf, '--version'], text=True).strip())
registration_path = out_dir / 'campaign.json'
if registration_path.exists():
    assert json.loads(registration_path.read_text()) == registration, 'campaign binding changed'
else:
    registration_path.write_text(json.dumps(registration, indent=2) + '\n')
for directory in ['samples', 'outputs', 'interrupted']:
    (out_dir / directory).mkdir(exist_ok=True)
counts = out_dir / 'counts.jsonl'
rows = [json.loads(line) for line in counts.read_text().splitlines()] if counts.exists() else []
keys = {(r['case'], r['op'], r['arm'], r['rep']) for r in rows}
assert len(keys) == len(rows)
flags = []
for x in inputs:
    for op in ops:
        for rep in range(reps):
            for arm in (arms if rep % 2 == 0 else list(reversed(arms))):
                key = (x['case'], op, arm, rep)
                if key in keys:
                    continue
                name = '-'.join(map(str, key))
                prefix = out_dir / 'samples' / name
                out = out_dir / 'outputs' / (name + '.bin')
                leftovers = [*prefix.parent.glob(name + '.*'), *out.parent.glob(out.name + '*')]
                if leftovers:
                    dest = Path(tempfile.mkdtemp(prefix=name + '-', dir=out_dir / 'interrupted'))
                    for file in leftovers:
                        file.rename(dest / file.name)
                n = 1000 if x['case'] == 'tiny-smooth' else 3 if x['case'].startswith('medium') else 1
                cr, cw = os.pipe()
                ar, aw = os.pipe()
                cmd = [perf, 'stat', '-e', 'instructions:u', '-x', ';', '-o', str(prefix) + '.perf',
                       '-D', '-1', '--control', f'fd:{cr},{aw}', '--', 'taskset', '-c', str(cpu),
                       str(art / f'driver-{arm}'), op, str(art / 'inputs' / f"{x['case']}.bin"), str(out)]
                try:
                    result = subprocess.run(cmd, input=f'{n}\nstop\n',
                                            env=dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar)),
                                            pass_fds=(cr, cw, ar, aw), capture_output=True, text=True,
                                            timeout=1200)
                finally:
                    for fd in [cr, cw, ar, aw]:
                        os.close(fd)
                assert result.returncode == 0, (key, result.stderr)
                lines = result.stdout.splitlines()
                assert lines == ['READY', 'COUNTED'], (key, lines)
                fields = next(s.split(';') for s in Path(str(prefix) + '.perf').read_text().splitlines()
                              if ';instructions:u;' in s)
                count = int(fields[0])
                assert count > 0 and float(fields[4].replace(',', '.')) == 100., fields
                row = dict(case=x['case'], op=op, arm=arm, rep=rep, repetitions=n, exit=0,
                           instructions=count, instructions_per_call=count / n,
                           running_percent=100.0, output_sha256=sha(out), output_bytes=out.stat().st_size)
                out.unlink()
                old = [r for r in rows if (r['case'], r['op'], r['arm']) == key[:3]]
                assert not old or old[0]['output_sha256'] == row['output_sha256'], ('unstable', key)
                with counts.open('a') as f:
                    f.write(json.dumps(row) + '\n')
                    f.flush()
                    os.fsync(f.fileno())
                rows.append(row)
                keys.add(key)
        cell = [r for r in rows if r['case'] == x['case'] and r['op'] == op]
        med = {a: statistics.median(r['instructions_per_call'] for r in cell if r['arm'] == a) for a in arms}
        line = ' '.join(f'{a}={med[a]:.0f}' for a in arms)
        if 'base' in arms and 'candidate' in arms:
            hb = {r['output_sha256'] for r in cell if r['arm'] == 'base'}
            hc = {r['output_sha256'] for r in cell if r['arm'] == 'candidate'}
            assert hb == hc and len(hb) == 1, ('output difference', x['case'], op)
            ratio = med['candidate'] / med['base']
            line += f' candidate/base={ratio:.6f}'
            if ratio > 1.01:
                flags.append(dict(case=x['case'], op=op, ratio=ratio))
        print(x['case'], op, line, flush=True)
(out_dir / 'measurement-exit.json').write_text(json.dumps(dict(exit=0, samples=len(rows),
                                                                regressions=flags), indent=2) + '\n')
