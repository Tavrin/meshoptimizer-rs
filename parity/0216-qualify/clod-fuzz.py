"""Resumable generated cluster-LOD protocol differential; budget candidate CPU only."""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import struct
import time

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
root = Path(__file__).resolve().parents[2]
art = Path(os.environ['PEERS_ART'])
spec = importlib.util.spec_from_file_location('p03_runner', root/'parity/p03/runner.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
paths = {'cpp': art/'p03-cpp', 'std': art/'p03-std'}
identity = {k: hashlib.sha256(v.read_bytes()).hexdigest() for k,v in paths.items()}
log = art/'clod-fuzz.jsonl'
rows = [json.loads(x) for x in log.read_text().splitlines()] if log.exists() else []
assert all(x['exit'] == 0 and x['binary'] == identity for x in rows)
total = sum(x['candidate_cpu_seconds'] for x in rows)
seed = sum(x['cases'] for x in rows)
hz = os.sysconf('SC_CLK_TCK')
clients = {k:r.Client(k,p) for k,p in paths.items()}
def cpu():
    stat = Path(f"/proc/{clients['std'].proc.pid}/stat").read_text().rsplit(')',1)[1].split()
    return (int(stat[11])+int(stat[12]))/hz
last = cpu()
try:
    while total < 300:
        begin = seed
        for _ in range(100):
            p,idx=r.grid(2+seed%12,seed%2)
            flags=(seed*37)&511
            if seed%3 == 0:
                flags |= 1024|2048|4096|((seed%10)<<13)
            data=r.message(16,p,idx,mv=2+seed%5,min=2+seed%4,mt=[4,8,16,32][seed%4],target=flags,split=0.1,radii=[i/100 for i in range(len(p))])
            outputs = {name:c.run(data)[0] for name,c in clients.items()}
            if outputs['std'] != outputs['cpp']:
                (art/'clod-fuzz-failure.bin').write_bytes(data)
                raise SystemExit(f'protocol mismatch at seed {seed}')
            seed+=1
        now=cpu(); used=now-last; last=now; total+=used
        row=dict(first_seed=begin,cases=100,candidate_cpu_seconds=used,exit=0,binary=identity)
        with log.open('a') as f:
            f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
        print('cases',seed,'candidate_cpu_seconds',round(total,2),flush=True)
finally:
    for client in clients.values():client.close()
(art/'clod-fuzz-exit.json').write_text(json.dumps(dict(exit=0,cases=seed,candidate_cpu_seconds=total))+'\n')
