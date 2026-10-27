"""Widened exact differential fuzzing for at least five child CPU minutes."""
from pathlib import Path
import json
import os
import resource
import subprocess

ART = Path(os.environ['MESHOPT_SCRATCH'])
assert os.environ.get('MOSS_HEAVY_ACTIVE')
# CPU accounting, not a performance timing. No wall durations are measured.
def cpu():
    r = resource.getrusage(resource.RUSAGE_CHILDREN)
    return r.ru_utime + r.ru_stime

start = cpu()
seed = 20261026
rows = []
while cpu() - start < 300:
    log = ART / f'fuzz-{seed}.log'
    with log.open('x') as output:
        result = subprocess.run([str(ART / 'exact-std'), 'fuzz', str(seed)], stdout=output, stderr=subprocess.STDOUT)
    rows.append({'seed': seed, 'exit': result.returncode, 'cpu_seconds': cpu() - start})
    (ART / 'fuzz.json').write_text(json.dumps(rows, indent=2) + '\n')
    if result.returncode:
        raise SystemExit(result.returncode)
    seed += 1
