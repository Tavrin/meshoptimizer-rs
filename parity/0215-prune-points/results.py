"""Audit raw instruction samples and export portable compact qualification receipts."""
import hashlib
import json
import math
import os
from pathlib import Path
import re
import statistics

scratch = Path(os.environ['MESHOPT_SCRATCH'])
art = scratch/'campaign-1'
lane = Path(__file__).resolve().parent
root = lane.parents[1]
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def emit(name, value): (lane/name).write_text(json.dumps(value,indent=2)+'\n')
rows = [json.loads(s) for s in (art/'counts.jsonl').read_text().splitlines()]
inputs = json.loads((art/'inputs.json').read_text())
assert len(rows)==len({(r['case'],r['op'],r['arm'],r['rep']) for r in rows})==192
binding = json.loads((art/'build-binding.json').read_text())
for name, digest in binding['sources'].items():
    assert sha(scratch/name)==digest
    if name.startswith('candidate/src/'):
        assert sha(root/name.removeprefix('candidate/'))==digest
for arm, digest in binding['binaries'].items(): assert sha(art/f'driver-{arm}')==digest
raw = {}
for r in rows:
    name = f"{r['case']}-{r['op']}-{r['arm']}-{r['rep']}"
    p = art/'samples'/(name+'.perf')
    fields = next(s.split(';') for s in p.read_text().splitlines() if ';instructions:u;' in s)
    assert int(fields[0])==r['instructions'] and float(fields[4])==r['running_percent']==100.
    assert r['exit']==0 and r['instructions_per_call']==r['instructions']/r['repetitions']
    out = art/'outputs'/(name+'.bin')
    assert sha(out)==r['output_sha256'] and out.stat().st_size==r['output_bytes']
    raw[name]=sha(p)
cases = {}
for x in inputs:
    assert sha(art/'inputs'/f"{x['case']}.bin")==x['sha256']
    for op in ['simplify-prune','simplify-points']:
        cell=[r for r in rows if r['case']==x['case'] and r['op']==op]
        assert len(cell)==12 and len({r['output_sha256'] for r in cell})==1
        med={a:statistics.median(r['instructions_per_call'] for r in cell if r['arm']==a) for a in binding['binaries']}
        cases[x['case']+'/'+op]=dict(medians=med,
            candidate_base=med['candidate']/med['base'], candidate_cpp=med['candidate']/med['cpp'],
            candidate_optimesh=med['candidate']/med['rust'],output_sha256=cell[0]['output_sha256'])
summary={}
for op in ['simplify-prune','simplify-points']:
    real=[v for k,v in cases.items() if k.startswith('real-') and k.endswith('/'+op)]
    gm={ratio:math.exp(sum(math.log(v[ratio]) for v in real)/3)
        for ratio in ['candidate_base','candidate_cpp','candidate_optimesh']}
    regressions={k:v['candidate_base'] for k,v in cases.items() if k.endswith('/'+op) and v['candidate_base']>1.01}
    verdict='REVERT' if regressions else 'SHIPS' if gm['candidate_base']<=.97 and gm['candidate_optimesh']<=.99 else 'IMPROVES' if gm['candidate_base']<=.97 else 'NO-GAIN'
    summary[op]=dict(**gm,verdict=verdict,regressions=regressions)
emit('results.json',dict(summary=summary,cases=cases))
emit('binding.json',binding)
emit('inputs.json',inputs)
(lane/'counts.jsonl').write_text((art/'counts.jsonl').read_text())
fuzz=[json.loads(s) for s in (scratch/'proof/fuzz.jsonl').read_text().splitlines()]
assert all(r['exit']==0 for r in fuzz) and sum(r['cpu_seconds'] for r in fuzz)>=300
proof={}
for mode in ['all','none']:
    log=scratch/'proof'/f'exact-{mode}.log'
    n=int(re.search(r'PASS observations=(\d+)',log.read_text())[1])
    protocol=json.loads((scratch/'proof/0216b-protocol'/f'exit-{mode}.json').read_text())
    assert protocol['exit']==0
    protocolrows=[json.loads(s) for s in (scratch/'proof/0216b-protocol'/f'cases-{mode}.jsonl').read_text().splitlines()]
    proof[mode]=dict(exact_exit=0,exact_observations=n,exact_log_sha256=sha(log),
        protocol=protocol,prune_points_cpp_cases=sum(r['operation'] in [25,26] for r in protocolrows),
        protocol_cases_sha256=sha(scratch/'proof/0216b-protocol'/f'cases-{mode}.jsonl'))
proof['fuzz']=dict(exit=0,seeds=len(fuzz),child_cpu_seconds=sum(r['cpu_seconds'] for r in fuzz),
    records_sha256=sha(scratch/'proof/fuzz.jsonl'))
proof['binding']=json.loads((scratch/'proof/binding.json').read_text())
emit('proof-results.json',proof)
logs={p.name:sha(p) for p in (scratch/'logs').glob('*.log') if 'audit' not in p.name}
emit('receipts.json',dict(raw_perf_sha256=raw,logs_sha256=logs,
    build_logs_sha256={str(p.relative_to(art)):sha(p) for p in art.rglob('*.log')},
    measurement=json.loads((art/'measurement-exit.json').read_text()),
    campaign_sha256=sha(art/'campaign.json'),exact_generator_sha256=sha(lane/'exact-main.rs')))
print(json.dumps(summary,indent=2))
for k,v in cases.items():
    print(k,*(f'{v[r]:.6f}' for r in ['candidate_base','candidate_optimesh','candidate_cpp']))
print(json.dumps({k:v for k,v in proof.items() if k!='binding'},indent=2))
