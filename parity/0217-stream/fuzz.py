"""Retain an exit receipt and run at least ten child CPU minutes after build."""
import json
import os
from pathlib import Path
import resource
import subprocess

root = Path.cwd()
target = Path(os.environ['CARGO_TARGET_DIR'])
out = root / 'parity/0217-stream'
# cargo-fuzz uses the host-triple release directory for its instrumented binary.
binaries = list(target.glob('*/release/buffer_stream'))
if len(binaries) != 1:
    raise SystemExit('expected exactly one instrumented buffer_stream executable')
corpus = target / 'stream-proof/seeds'
artifacts = target / 'stream-proof/artifacts'
artifacts.mkdir(parents=True, exist_ok=True)
records = []
cpu = 0.0
while cpu < 600:
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    seconds = max(30, int(610-cpu))
    command = [str(binaries[0]), str(corpus), '-max_total_time='+str(seconds),
               '-rss_limit_mb=2048', '-timeout=30', '-max_len=16384',
               '-print_final_stats=1', '-artifact_prefix='+str(artifacts)+'/']
    raw = target / 'fuzz-raw.log'
    with raw.open('w') as log:
        result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    used = after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime
    cpu += used
    text = raw.read_text(errors='replace').replace(str(root), '<worktree>').replace(str(target), '<target>')
    index = len(list(out.glob('fuzz-run-*.log')))
    name = 'fuzz-run-'+str(index)+'.log'
    (out / name).write_text(text)
    records.append(dict(exit_code=result.returncode, cpu_seconds=used, log=name))
    (out / 'fuzz.json').write_text(json.dumps(dict(cpu_seconds=cpu, runs=records), indent=2)+'\n')
    print('fuzz:', result.returncode, 'total CPU seconds:', cpu, flush=True)
    if result.returncode != 0:
        raise SystemExit(result.returncode)
