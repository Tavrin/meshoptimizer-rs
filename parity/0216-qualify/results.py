"""Summarize the preregistered instruction campaign; no proof inference."""
import json
import math
import os
from pathlib import Path
import statistics

art = Path(os.environ['PEERS_ART'])
rows = [json.loads(line) for line in (art / 'counts.jsonl').read_text().splitlines()]
inputs = json.loads((art / 'inputs.json').read_text())
ops = ['meshlets', 'meshlets-flex', 'meshlets-spatial', 'cluster-lod', 'simplify',
       'simplify-attributes', 'simplify-update', 'simplify-sloppy', 'simplify-prune',
       'simplify-points', 'overdraw']
result = {}
for case in inputs:
    for op in ops:
        arms = ['base', 'candidate', 'cpp'] + ([] if op == 'cluster-lod' else ['rust'])
        samples = {arm: [r for r in rows if (r['case'], r['op'], r['arm']) == (case['case'], op, arm)] for arm in arms}
        assert all(len(v) == 3 and sorted(r['rep'] for r in v) == [0, 1, 2] for v in samples.values())
        assert all(r['exit'] == 0 and r['running_percent'] == 100 for v in samples.values() for r in v)
        assert all(len({r['output_sha256'] for r in v}) == 1 for v in samples.values())
        med = {arm: statistics.median(r['instructions_per_call'] for r in v) for arm, v in samples.items()}
        output = {arm: v[0]['output_sha256'] for arm, v in samples.items()}
        result[f"{case['case']}/{op}"] = dict(medians=med, output_sha256=output,
            candidate_base=med['candidate']/med['base'], candidate_cpp=med['candidate']/med['cpp'],
            candidate_optimesh=med['candidate']/med['rust'] if 'rust' in med else None)
real = [c['case'] for c in inputs if c['case'].startswith('real-')]
assert len(real) == 3
regressions = {k: v['candidate_base'] for k, v in result.items() if v['candidate_base'] > 1.01}
geomeans = {op: math.exp(sum(math.log(result[f'{c}/{op}']['candidate_base']) for c in real)/3) for op in ops}
(art / 'results.json').write_text(json.dumps(dict(regressions=regressions, geomeans=geomeans, cases=result), indent=2)+'\n')
print(json.dumps(dict(regressions=regressions, geomeans=geomeans), indent=2))
