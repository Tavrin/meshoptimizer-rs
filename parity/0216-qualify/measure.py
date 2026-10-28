"""Resume an interleaved, resident-loop instructions:u campaign under admission."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import statistics
import tempfile

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
art = Path(os.environ['PEERS_ART'])
perf = os.environ['PEERS_PERF']
cpu = int(os.environ.get('PEERS_CPU', '9'))
assert cpu in os.sched_getaffinity(0)
ops = ['meshlets', 'meshlets-flex', 'meshlets-spatial', 'cluster-lod', 'simplify', 'simplify-attributes', 'simplify-update', 'simplify-sloppy', 'simplify-prune', 'simplify-points', 'overdraw']
arms = ['base', 'candidate', 'cpp', 'rust']

def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

binding = json.loads((art / 'build-binding.json').read_text())
for arm in arms:
    assert sha(art / f'driver-{arm}') == binding['binaries'][arm]
for name, digest in binding['sources'].items():
    assert sha(art.parent / name) == digest, ('source changed', name)
inputs = json.loads((art / 'inputs.json').read_text())
for x in inputs:
    assert sha(art / 'inputs' / f"{x['case']}.bin") == x['sha256']
registration = dict(binding=binding, inputs=inputs, cpu=cpu, ops=ops,
                    runner_sha256=sha(Path(__file__)),
                    perf=subprocess.check_output([perf, '--version'], text=True).strip())
registration_path = art / 'campaign.json'
if registration_path.exists():
    assert json.loads(registration_path.read_text()) == registration
else:
    registration_path.write_text(json.dumps(registration, indent=2) + '\n')
for directory in ['samples', 'outputs', 'interrupted', 'objects']:
    (art / directory).mkdir(exist_ok=True)
counts = art / 'counts.jsonl'
rows = [json.loads(line) for line in counts.read_text().splitlines()] if counts.exists() else []
keys = {(r['case'], r['op'], r['arm'], r['rep']) for r in rows}
assert len(keys) == len(rows)
for x in inputs:
    for op in ops:
        for rep in range(3):
            for arm in (arms if rep != 1 else list(reversed(arms))):
                if op == 'cluster-lod' and arm == 'rust':
                    continue
                key = (x['case'], op, arm, rep)
                if key in keys:
                    continue
                name = '-'.join(map(str, key))
                prefix = art / 'samples' / name
                out = art / 'outputs' / (name + '.bin')
                leftovers = [*prefix.parent.glob(name + '.*'), *out.parent.glob(out.name + '*')]
                if leftovers:
                    dest = Path(tempfile.mkdtemp(prefix=name+'-', dir=art/'interrupted'))
                    for file in leftovers:
                        file.rename(dest/file.name)
                n = 1000 if x['case'] == 'tiny-smooth' else 3 if x['case'].startswith('medium') else 1
                if op == 'cluster-lod':
                    n = 10 if x['case'] == 'tiny-smooth' else 1
                cr, cw = os.pipe()
                ar, aw = os.pipe()
                cmd = [perf, 'stat', '-e', 'instructions:u', '-x', ';', '-o', str(prefix)+'.perf',
                       '-D', '-1', '--control', f'fd:{cr},{aw}', '--', 'taskset', '-c', str(cpu),
                       str(art/f'driver-{arm}'), op, str(art/'inputs'/f"{x['case']}.bin"), str(out)]
                try:
                    result = subprocess.run(cmd, input=f'{n}\nstop\n',
                        env=dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar)),
                        pass_fds=(cr,cw,ar,aw), capture_output=True, text=True, timeout=1200)
                finally:
                    for fd in [cr,cw,ar,aw]:
                        os.close(fd)
                Path(str(prefix)+'.stdout').write_text(result.stdout)
                Path(str(prefix)+'.stderr').write_text(result.stderr)
                assert result.returncode == 0, (key, result.stderr)
                lines = result.stdout.splitlines()
                assert len(lines) == 2 and lines[0] == 'READY' and lines[1] == 'COUNTED', (key, lines)
                fields = next(s.split(';') for s in Path(str(prefix)+'.perf').read_text().splitlines() if ';instructions:u;' in s)
                count = int(fields[0])
                assert count > 0 and float(fields[4]) == 100., fields
                row = dict(case=x['case'], op=op, arm=arm, rep=rep, repetitions=n,
                           exit=result.returncode, instructions=count, instructions_per_call=count/n,
                           running_percent=float(fields[4]), output_sha256=sha(out), output_bytes=out.stat().st_size)
                def retain(path):
                    digest = sha(path)
                    object_path = art/'objects'/digest
                    if object_path.exists():
                        assert sha(object_path) == digest
                        path.unlink()
                    else:
                        path.rename(object_path)
                    os.link(object_path, path)
                    return digest
                assert retain(out) == row['output_sha256']
                raw = Path(str(out)+'.raw')
                if raw.exists():
                    row['raw_sha256'] = retain(raw)
                old = [r for r in rows if (r['case'], r['op'], r['arm']) == key[:3]]
                assert not old or old[0]['output_sha256'] == row['output_sha256'], ('unstable', key)
                Path(str(prefix)+'.json').write_text(json.dumps(row, indent=2)+'\n')
                with counts.open('a') as f:
                    f.write(json.dumps(row)+'\n')
                    f.flush()
                    os.fsync(f.fileno())
                rows.append(row)
                keys.add(key)
        cell = [r for r in rows if r['case'] == x['case'] and r['op'] == op]
        hashes = {a: {r['output_sha256'] for r in cell if r['arm'] == a} for a in ('base', 'candidate')}
        if any(len(hashes[a]) != 1 for a in hashes) or hashes['base'] != hashes['candidate']:
            (art/'output-difference.json').write_text(json.dumps(dict(case=x['case'], op=op, hashes={a: sorted(v) for a, v in hashes.items()}), indent=2)+'\n')
            raise SystemExit('output difference: ' + x['case'] + '/' + op)
        med = {a: statistics.median(r['instructions_per_call'] for r in cell if r['arm'] == a) for a in ('base', 'candidate')}
        ratio = med['candidate'] / med['base']
        if ratio > 1.01:
            (art/'regression.json').write_text(json.dumps(dict(case=x['case'], op=op, ratio=ratio, medians=med), indent=2)+'\n')
            raise SystemExit('regression: ' + x['case'] + '/' + op + ' ratio=' + str(ratio))
        print(x['case'], op, 'complete', 'candidate/base', ratio, flush=True)
(art/'measurement-exit.json').write_text(json.dumps(dict(exit=0, samples=len(rows)), indent=2)+'\n')
