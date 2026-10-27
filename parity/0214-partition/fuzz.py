"""Differential generation measured in child CPU time, not wall time."""
import json
import os
from pathlib import Path
import resource
import subprocess

scratch = Path(os.environ['MESHOPT_SCRATCH'])
assert os.environ.get('PARITY_RUNNER_ACTIVE')
def cpu():
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    return usage.ru_utime + usage.ru_stime
start = cpu()
rows = []
seed = 20261027
while cpu() - start < 300:
    log = scratch / f'fuzz-{seed}.log'
    with log.open('x') as out:
        result = subprocess.run([str(scratch / 'exact-std'), str(seed)], stdout=out, stderr=subprocess.STDOUT)
    rows.append({'seed': seed, 'exit': result.returncode, 'cpu_seconds': cpu() - start})
    (scratch / 'fuzz.json').write_text(json.dumps(rows, indent=2) + '\n')
    if result.returncode:
        raise SystemExit(result.returncode)
    seed += 1
print('PASS', len(rows) * 2000, 'cases;', round(cpu() - start, 1), 'CPU seconds')
