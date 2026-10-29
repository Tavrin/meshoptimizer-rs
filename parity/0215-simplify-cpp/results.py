"""Audit raw instruction samples and export portable compact qualification receipts."""
import hashlib
import json
import math
import os
from pathlib import Path
import re
import statistics

scratch = Path(os.environ['MESHOPT_SCRATCH'])
campaign = os.environ.get('MESHOPT_CAMPAIGN', 'campaign-1')
art = scratch/campaign
here = Path(__file__).resolve().parent
root = here.parents[1]
# Later campaigns keep their receipts beside the first campaign's.
lane = here if campaign == 'campaign-1' else here/campaign
lane.mkdir(exist_ok=True)
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def emit(name, value): (lane/name).write_text(json.dumps(value,indent=2)+'\n')
rows = [json.loads(s) for s in (art/'counts.jsonl').read_text().splitlines()]
inputs = json.loads((art/'inputs.json').read_text())
assert len(rows)==len({(r['case'],r['op'],r['arm'],r['rep']) for r in rows})==192
binding = json.loads((art/'build-binding.json').read_text())
for name, digest in binding['sources'].items():
    assert sha(scratch/name)==digest
    tree, _, rest = name.partition('/')
    if tree.startswith('candidate') and rest.startswith('src/'):
        assert sha(root/rest)==digest
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
    for op in ['simplify','simplify-sloppy']:
        cell=[r for r in rows if r['case']==x['case'] and r['op']==op]
        assert len(cell)==12 and len({r['output_sha256'] for r in cell})==1
        med={a:statistics.median(r['instructions_per_call'] for r in cell if r['arm']==a) for a in binding['binaries']}
        cases[x['case']+'/'+op]=dict(medians=med,
            candidate_base=med['candidate']/med['base'], candidate_cpp=med['candidate']/med['cpp'],
            base_cpp=med['base']/med['cpp'],
            candidate_optimesh=med['candidate']/med['rust'],output_sha256=cell[0]['output_sha256'])
summary={}
for op in ['simplify','simplify-sloppy']:
    real=[v for k,v in cases.items() if k.startswith('real-') and k.endswith('/'+op)]
    gm={ratio:math.exp(sum(math.log(v[ratio]) for v in real)/3)
        for ratio in ['candidate_base','candidate_cpp','candidate_optimesh','base_cpp']}
    regressions={k:v['candidate_base'] for k,v in cases.items() if k.endswith('/'+op) and v['candidate_base']>1.01}
    # The spec replaces the optimesh condition for these two functions with
    # candidate/C++ improving on base/C++ by at least five percentage points.
    cpp_gain=gm['base_cpp']-gm['candidate_cpp']
    verdict='REVERT' if regressions else 'SHIPS' if gm['candidate_base']<=.97 and cpp_gain>=.05 else 'IMPROVES' if gm['candidate_base']<=.97 else 'NO-GAIN'
    summary[op]=dict(**gm,cpp_gain_points=cpp_gain*100,verdict=verdict,regressions=regressions)
emit('results.json',dict(summary=summary,cases=cases))
emit('binding.json',binding)
emit('inputs.json',inputs)
(lane/'counts.jsonl').write_text((art/'counts.jsonl').read_text())
print(json.dumps(summary, indent=2))
