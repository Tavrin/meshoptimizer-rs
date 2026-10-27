"""Widened exact differential fuzzing for at least five child CPU minutes."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import json
import os
import resource
import subprocess

ART = Path(_public_path('<scratch>/meshopt-0212-simplify'))
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
