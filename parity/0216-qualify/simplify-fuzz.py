"""Resumable differential fuzzing for five child CPU minutes."""
import json
import os
from pathlib import Path
import resource
import subprocess

art = Path(os.environ['MESHOPT_SCRATCH'])
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
path = art / 'fuzz.jsonl'
rows = [json.loads(x) for x in path.read_text().splitlines()] if path.exists() else []
assert all(row['exit'] == 0 for row in rows)
total = sum(row['cpu_seconds'] for row in rows)
seed = 20261027 + len(rows)
while total < 300:
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    with (art / f'fuzz-{seed}.log').open('x') as log:
        result = subprocess.run([str(art / 'exact-std'), 'fuzz', str(seed)], stdout=log, stderr=subprocess.STDOUT)
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    used = (after.ru_utime + after.ru_stime) - (before.ru_utime + before.ru_stime)
    row = dict(seed=seed, exit=result.returncode, cpu_seconds=used)
    with path.open('a') as output:
        output.write(json.dumps(row) + '\n')
    if result.returncode:
        raise SystemExit(result.returncode)
    total += used
    seed += 1
print(f'PASS {seed - 20261027} seeds; {total:.3f} child CPU seconds')
