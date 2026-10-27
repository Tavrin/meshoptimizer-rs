"""Widened exact differential fuzzing for at least five child CPU minutes."""
from pathlib import Path
import json
import os
import resource
import subprocess

ART = Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
assert os.environ.get('MESHOPT_ADMISSION_ID')
# CPU accounting, not a performance timing. No wall durations are measured.
def cpu():
    r = resource.getrusage(resource.RUSAGE_CHILDREN)
    return r.ru_utime + r.ru_stime

start = cpu()
seed = 20261027
rows = []
while cpu() - start < 300:
    log = ART / 'fuzz-output.log'
    with log.open('a') as output:
        result = subprocess.run([str(ART / 'exact-std'), 'fuzz', str(seed)], stdout=output, stderr=subprocess.STDOUT)
    rows.append({'seed': seed, 'exit': result.returncode, 'cpu_seconds': cpu() - start})
    with (ART / 'fuzz.jsonl').open('a') as stream: stream.write(json.dumps(rows[-1]) + '\n')
    if result.returncode:
        raise SystemExit(result.returncode)
    seed += 1
