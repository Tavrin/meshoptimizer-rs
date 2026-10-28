"""Differential overdraw fuzz with five minutes of child CPU time."""
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU') == '0'
scratch = Path(os.environ['MESHOPT_SCRATCH'])
root = scratch / 'proof/0216b-overdraw-fuzz'
root.mkdir(parents=True, exist_ok=False)
(root / 'empty').mkdir()
binary = scratch / 'proof/0216b-affected/overdraw-all'
digest = hashlib.sha256(binary.read_bytes()).hexdigest()
env = dict(os.environ, MESHOPT_OVERDRAW_SCRATCH=str(root))
total = 0.0
seed = 20261028
while total < 300:
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    with (root / f'{seed}.log').open('x') as log:
        result = subprocess.run([str(binary), 'fuzz', str(seed)], stdout=log, stderr=subprocess.STDOUT, env=env)
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    used = after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime
    row = {'seed': seed, 'exit': result.returncode, 'cpu_seconds': used, 'binary_sha256': digest}
    with (root / 'results.jsonl').open('a') as out:
        out.write(json.dumps(row) + '\n')
    if result.returncode:
        raise SystemExit(result.returncode)
    total += used
    seed += 1
(root / 'exit.json').write_text(json.dumps({'exit': 0, 'cpu_seconds': total, 'seeds': seed - 20261028, 'binary_sha256': digest}) + '\n')
