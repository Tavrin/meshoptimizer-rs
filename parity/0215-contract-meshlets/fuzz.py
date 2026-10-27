"""Run resumable deterministic batches until 300 child CPU seconds."""
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
art = Path(os.environ['PEERS_ART'])
binary = Path(os.environ['FUZZ_BINARY'])
identity = hashlib.sha256(binary.read_bytes()).hexdigest()
binding = art/'fuzz-binding.json'
if binding.exists():
    assert json.loads(binding.read_text()) == {'binary_sha256': identity}
else:
    binding.write_text(json.dumps({'binary_sha256': identity})+'\n')
log = art/'fuzz-batches.jsonl'
rows = [json.loads(s) for s in log.read_text().splitlines()] if log.exists() else []
assert all(r['exit'] == 0 for r in rows)
cpu = sum(r['cpu_seconds'] for r in rows)
while cpu < 300:
    index = len(rows)
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    result = subprocess.run([str(binary), str(0x02150001 + index), '2000'], capture_output=True)
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    seconds = (after.ru_utime + after.ru_stime) - (before.ru_utime + before.ru_stime)
    row = dict(batch=index, seed=0x02150001 + index, cases=2000, cpu_seconds=seconds, exit=result.returncode)
    if result.returncode:
        (art/'fuzz-failure.stderr').write_bytes(result.stderr)
        raise SystemExit(f'fuzz failure: {row}')
    with log.open('a') as f:
        f.write(json.dumps(row)+'\n')
        f.flush()
        os.fsync(f.fileno())
    rows.append(row)
    cpu += seconds
    if index % 10 == 0:
        print('batches',len(rows),'cpu_seconds',round(cpu,2),flush=True)
(art/'fuzz-exit.json').write_text(json.dumps(dict(exit=0, cpu_seconds=cpu, cases=2000*len(rows)),indent=2)+'\n')
