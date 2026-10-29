"""Resume an interleaved, resident-loop instructions:u campaign under admission.

Cache-only adapter of the S5 resident-loop runner. One record per finished sample is appended to counts.jsonl and finished
samples are skipped on restart; leftovers of an interrupted sample are moved
aside. Usage: measure.py CAMPAIGN (a subdirectory of RESTORE_SCRATCH). Arms: base
(v0.2.14), candidate, cpp (C++ 1.3 via meshopt 0.7.0), rust (optimesh 1.1.0,
which has no cluster LOD).
"""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import statistics
import sys
import tempfile

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
art = Path(os.environ['RESTORE_SCRATCH'])
out_dir = art / sys.argv[1]
out_dir.mkdir(exist_ok=True)
perf = os.environ['PEERS_PERF']
cpu = int(os.environ.get('PEERS_CPU', '9'))
assert cpu in os.sched_getaffinity(0)
ops = ['cache']
arms = sys.argv[2:]
assert arms and 'base' in arms


def arms_for(op):
    return [a for a in arms if not (a == 'rust' and op == 'cluster-lod')]


def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


binding = json.loads((art / 'build-binding.json').read_text())
for arm in arms:
    assert sha(art / f'driver-{arm}') == binding['binaries'][arm]
for name, digest in binding['sources'].items():
    assert sha(art / name) == digest, ('source changed', name)
inputs = json.loads((art / 'inputs.json').read_text())
for x in inputs:
    assert sha(Path(os.environ['RESTORE_INPUTS']) / f"{x['case']}.bin") == x['sha256']
registration = dict(binding=binding, inputs=inputs, cpu=cpu, ops=ops, arms=arms,
                    runner_sha256=sha(Path(__file__)),
                    perf=subprocess.check_output([perf, '--version'], text=True).strip())
registration_path = out_dir / 'campaign.json'
if registration_path.exists():
    assert json.loads(registration_path.read_text()) == registration
else:
    registration_path.write_text(json.dumps(registration, indent=2) + '\n')
for directory in ['samples', 'outputs', 'interrupted', 'objects']:
    (out_dir / directory).mkdir(exist_ok=True)
counts = out_dir / 'counts.jsonl'
rows = [json.loads(line) for line in counts.read_text().splitlines()] if counts.exists() else []
keys = {(r['case'], r['op'], r['arm'], r['rep']) for r in rows}
assert len(keys) == len(rows)


def retain(path):
    digest = sha(path)
    object_path = out_dir / 'objects' / digest
    if object_path.exists():
        assert sha(object_path) == digest
        path.unlink()
    else:
        path.rename(object_path)
    os.link(object_path, path)
    return digest


for x in inputs:
    for op in ops:
        op_arms = arms_for(op)
        for rep in range(3):
            for arm in (op_arms if rep != 1 else list(reversed(op_arms))):
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
                       str(art / f'driver-{arm}'), op, str(Path(os.environ['RESTORE_INPUTS']) / f"{x['case']}.bin"), str(out)]
                try:
                    result = subprocess.run(cmd, input=f'{n}\nstop\n',
                                            env=dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar),
                                                     LC_ALL='C'),
                                            pass_fds=(cr, cw, ar, aw), capture_output=True, text=True,
                                            timeout=3600)
                finally:
                    for fd in [cr, cw, ar, aw]:
                        os.close(fd)
                Path(str(prefix) + '.stdout').write_text(result.stdout)
                Path(str(prefix) + '.stderr').write_text(result.stderr)
                assert result.returncode == 0, (key, result.stderr)
                lines = result.stdout.splitlines()
                assert lines == ['READY', 'COUNTED'], (key, lines)
                fields = next(s.split(';') for s in Path(str(prefix) + '.perf').read_text().splitlines()
                              if ';instructions:u;' in s)
                count = int(fields[0])
                assert count > 0 and float(fields[4]) == 100., fields
                row = dict(case=x['case'], op=op, arm=arm, rep=rep, repetitions=n,
                           exit=result.returncode, instructions=count, instructions_per_call=count / n,
                           running_percent=float(fields[4]), output_sha256=sha(out),
                           output_bytes=out.stat().st_size)
                assert retain(out) == row['output_sha256']
                for extra in out.parent.glob(out.name + '.*'):
                    retain(extra)
                old = [r for r in rows if (r['case'], r['op'], r['arm']) == key[:3]]
                assert not old or old[0]['output_sha256'] == row['output_sha256'], ('unstable', key)
                Path(str(prefix) + '.json').write_text(json.dumps(row, indent=2) + '\n')
                with counts.open('a') as f:
                    f.write(json.dumps(row) + '\n')
                    f.flush()
                    os.fsync(f.fileno())
                rows.append(row)
                keys.add(key)
        cell = [r for r in rows if r['case'] == x['case'] and r['op'] == op]
        hashes = {a: {r['output_sha256'] for r in cell if r['arm'] == a} for a in arms}
        if any(len(hashes[a]) != 1 for a in hashes) or any(hashes[a] != hashes['base'] for a in arms):
            (out_dir / 'output-difference.json').write_text(json.dumps(
                dict(case=x['case'], op=op, hashes={a: sorted(v) for a, v in hashes.items()}), indent=2) + '\n')
            raise SystemExit('output difference: ' + x['case'] + '/' + op)
        med = {a: statistics.median(r['instructions_per_call'] for r in cell if r['arm'] == a)
               for a in arms}
        print(x['case'], op, 'complete', 'ratios/base', {a: med[a] / med['base'] for a in arms}, flush=True)
(out_dir / 'measurement-exit.json').write_text(json.dumps(dict(exit=0, samples=len(rows)), indent=2) + '\n')
