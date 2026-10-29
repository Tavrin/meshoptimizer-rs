"""Resume an interleaved, resident-loop instructions:u campaign under admission.

Usage: measure.py CAMPAIGN ARM... (CAMPAIGN is a subdirectory of PEERS_ART).
One record per finished sample is appended (fsync) to counts.jsonl; finished
samples are skipped on restart and leftovers of an interrupted sample move
aside. Arm order is reversed on the second repetition. When base and
candidate are both measured, their outputs must be byte-identical per cell.
"""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import sys
import tempfile

assert os.environ.get('MOSS_HEAVY_ACTIVE')
art = Path(os.environ['PEERS_ART'])
out_dir = art / sys.argv[1]
arms = sys.argv[2:]
out_dir.mkdir(exist_ok=True)
perf = os.environ['PEERS_PERF']
cpu = int(os.environ.get('PEERS_CPU', '9'))
assert cpu in os.sched_getaffinity(0)
ops = ['remap-vertex', 'remap-index']


def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


binding = json.loads((art / 'build-binding.json').read_text())
for arm in arms:
    assert sha(art / f'driver-{arm}') == binding['binaries'][arm]
for name, digest in binding['sources'].items():
    assert sha(Path(os.environ['REMAP_SCRATCH']) / name) == digest, ('source changed', name)
inputs = json.loads((art / 'inputs.json').read_text())
for x in inputs:
    assert sha(art / 'inputs' / f"{x['case']}.bin") == x['sha256']
registration = dict(arms=arms, binaries={a: binding['binaries'][a] for a in arms}, inputs=inputs, cpu=cpu,
                    ops=ops, runner_sha256=sha(Path(__file__)),
                    perf=subprocess.check_output([perf, '--version'], text=True).strip())
registration_path = out_dir / 'campaign.json'
if registration_path.exists():
    assert json.loads(registration_path.read_text()) == registration
else:
    registration_path.write_text(json.dumps(registration, indent=2) + '\n')
for directory in ['samples', 'outputs', 'interrupted']:
    (out_dir / directory).mkdir(exist_ok=True)
counts = out_dir / 'counts.jsonl'
rows = [json.loads(line) for line in counts.read_text().splitlines()] if counts.exists() else []
keys = {(r['case'], r['op'], r['arm'], r['rep']) for r in rows}
assert len(keys) == len(rows)

for x in inputs:
    for op in ops:
        for rep in range(3):
            for arm in (arms if rep != 1 else list(reversed(arms))):
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
                                            env=dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar),
                                                     LC_ALL='C'),
                                            pass_fds=(cr, cw, ar, aw), capture_output=True, text=True,
                                            timeout=3600)
                finally:
                    for fd in [cr, cw, ar, aw]:
                        os.close(fd)
                Path(str(prefix) + '.stderr').write_text(result.stderr)
                assert result.returncode == 0, (key, result.stderr[-2000:])
                assert result.stdout.splitlines()[0] == 'READY' and result.stdout.splitlines()[1].startswith('COUNTED '), (key, result.stdout)
                seconds = float(result.stdout.splitlines()[1].split()[1])
                fields = next(s.split(';') for s in Path(str(prefix) + '.perf').read_text().splitlines()
                              if ';instructions:u;' in s)
                count = int(fields[0])
                assert count > 0 and float(fields[4]) == 100., fields
                row = dict(case=x['case'], op=op, arm=arm, rep=rep, repetitions=n,
                           instructions=count, instructions_per_call=count / n, seconds_per_call=seconds / n,
                           output_sha256=sha(out), output_bytes=out.stat().st_size)
                old = [r for r in rows if (r['case'], r['op'], r['arm']) == key[:3]]
                assert not old or old[0]['output_sha256'] == row['output_sha256'], ('unstable', key)
                if old:
                    out.unlink()
                with counts.open('a') as f:
                    f.write(json.dumps(row) + '\n')
                    f.flush()
                    os.fsync(f.fileno())
                rows.append(row)
                keys.add(key)
        cell = [r for r in rows if r['case'] == x['case'] and r['op'] == op]
        med = {a: statistics.median(r['instructions_per_call'] for r in cell if r['arm'] == a) for a in arms}
        if 'base' in arms and 'candidate' in arms:
            hashes = {a: {r['output_sha256'] for r in cell if r['arm'] == a} for a in ('base', 'candidate')}
            if hashes['base'] != hashes['candidate']:
                (out_dir / 'output-difference.json').write_text(json.dumps(
                    dict(case=x['case'], op=op, hashes={a: sorted(v) for a, v in hashes.items()}), indent=2) + '\n')
                raise SystemExit('output difference: ' + x['case'] + '/' + op)
        hashes = {r['output_sha256'] for r in cell}
        assert len(hashes) == 1, ('arm output difference', x['case'], op)
        print(x['case'], op, {a: round(v) for a, v in med.items()}, flush=True)
(out_dir / 'measurement-exit.json').write_text(json.dumps(dict(exit=0, samples=len(rows)), indent=2) + '\n')
