"""One scalar instructions:u campaign for the 0.2.15 release table.

Usage: measure.py (inside the CPU-only admission runner). Environment:
QUALIFY_SCRATCH (campaign/ from build.py; inputs/ and inputs.json),
PEERS_PERF (perf 6.17), optional PEERS_CPU (default 9).
Arms: base (v0.2.14), candidate (train), cpp (C++ 1.3 via meshopt 0.7.0,
MESHOPTIMIZER_NO_SIMD), rust (optimesh 1.1.0 without simd); SIMD is off in
every arm. Interleaved resident-loop samples, three repetitions with the arm
order reversed on the second; median per cell. One fsynced record per sample
in campaign/counts.jsonl; finished samples are skipped on restart and an
interrupted sample's files move aside. Base and candidate outputs must be
byte-identical per cell (the campaign stops otherwise).
"""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import tempfile

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
scratch = Path(os.environ['QUALIFY_SCRATCH'])
art = scratch / 'campaign'
perf = os.environ['PEERS_PERF']
cpu = int(os.environ.get('PEERS_CPU', '9'))
assert cpu in os.sched_getaffinity(0)
ops = ['simplify', 'simplify-attributes', 'simplify-sloppy', 'simplify-update', 'simplify-prune',
       'simplify-points', 'cache', 'overdraw', 'fetch', 'meshlets', 'meshlets-flex', 'meshlets-spatial',
       'cluster-lod', 'vertex-encode', 'vertex-decode', 'index-encode', 'index-decode',
       'remap-vertex', 'remap-index']
arms = ['base', 'candidate', 'cpp', 'rust']


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
inputs = json.loads((scratch / 'inputs.json').read_text())
for x in inputs:
    assert sha(scratch / 'inputs' / f"{x['case']}.bin") == x['sha256']
registration = dict(arms=arms, binaries=binding['binaries'], inputs=inputs, cpu=cpu, ops=ops,
                    runner_sha256=sha(Path(__file__)),
                    perf=subprocess.check_output([perf, '--version'], text=True).strip())
registration_path = art / 'campaign.json'
if registration_path.exists():
    assert json.loads(registration_path.read_text()) == registration
else:
    registration_path.write_text(json.dumps(registration, indent=2) + '\n')
for directory in ['samples', 'outputs', 'interrupted']:
    (art / directory).mkdir(exist_ok=True)
counts = art / 'counts.jsonl'
rows = [json.loads(line) for line in counts.read_text().splitlines()] if counts.exists() else []
keys = {(r['case'], r['op'], r['arm'], r['rep']) for r in rows}
assert len(keys) == len(rows)
for x in inputs:
    for op in ops:
        op_arms = arms_for(op)
        for rep in range(3):
            for arm in (op_arms if rep != 1 else list(reversed(op_arms))):
                key = (x['case'], op, arm, rep)
                if key in keys:
                    continue
                name = '-'.join(map(str, key))
                prefix = art / 'samples' / name
                out = art / 'outputs' / (name + '.bin')
                leftovers = [*prefix.parent.glob(name + '.*'), *out.parent.glob(out.name + '*')]
                if leftovers:
                    dest = Path(tempfile.mkdtemp(prefix=name + '-', dir=art / 'interrupted'))
                    for file in leftovers:
                        file.rename(dest / file.name)
                n = 1000 if x['case'] == 'tiny-smooth' else 3 if x['case'].startswith('medium') else 1
                cr, cw = os.pipe()
                ar, aw = os.pipe()
                cmd = [perf, 'stat', '-e', 'instructions:u', '-x', ';', '-o', str(prefix) + '.perf',
                       '-D', '-1', '--control', f'fd:{cr},{aw}', '--', 'taskset', '-c', str(cpu),
                       str(art / f'driver-{arm}'), op, str(scratch / 'inputs' / f"{x['case']}.bin"), str(out)]
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
                assert result.stdout.splitlines() == ['READY', 'COUNTED'], (key, result.stdout)
                fields = next(s.split(';') for s in Path(str(prefix) + '.perf').read_text().splitlines()
                              if ';instructions:u;' in s)
                count = int(fields[0])
                assert count > 0 and float(fields[4]) == 100., fields
                row = dict(case=x['case'], op=op, arm=arm, rep=rep, repetitions=n,
                           instructions=count, instructions_per_call=count / n,
                           output_sha256=sha(out), output_bytes=out.stat().st_size)
                old = [r for r in rows if (r['case'], r['op'], r['arm']) == key[:3]]
                assert not old or old[0]['output_sha256'] == row['output_sha256'], ('unstable', key)
                out.unlink()
                with counts.open('a') as f:
                    f.write(json.dumps(row) + '\n')
                    f.flush()
                    os.fsync(f.fileno())
                rows.append(row)
                keys.add(key)
        cell = [r for r in rows if r['case'] == x['case'] and r['op'] == op]
        med = {a: statistics.median(r['instructions_per_call'] for r in cell if r['arm'] == a) for a in op_arms}
        hashes = {a: {r['output_sha256'] for r in cell if r['arm'] == a} for a in ('base', 'candidate')}
        if hashes['base'] != hashes['candidate']:
            (art / 'output-difference.json').write_text(json.dumps(
                dict(case=x['case'], op=op, hashes={a: sorted(v) for a, v in hashes.items()}), indent=2) + '\n')
            raise SystemExit('output difference: ' + x['case'] + '/' + op)
        print(x['case'], op, {a: round(v) for a, v in med.items()}, flush=True)
(art / 'measurement-exit.json').write_text(json.dumps(dict(exit=0, samples=len(rows)), indent=2) + '\n')
