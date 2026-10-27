"""Compute pre-registered instruction ratios and compare retained output hashes."""
import json
import os
from pathlib import Path
import statistics

art = Path(os.environ['PEERS_ART'])
rows = [json.loads(line) for line in (art/'counts.jsonl').read_text().splitlines()]
inputs = json.loads((art/'inputs.json').read_text())
ops = ['meshlets','meshlets-flex','meshlets-spatial','cluster-lod']
summary = {}
for item in inputs:
    case = item['case']
    for op in ops:
        arms = ['base','candidate','cpp'] + ([] if op == 'cluster-lod' else ['rust'])
        medians = {}
        outputs = {}
        for arm in arms:
            samples = [r for r in rows if (r['case'],r['op'],r['arm']) == (case,op,arm)]
            assert len(samples) == 3 and sorted(r['rep'] for r in samples) == [0,1,2]
            assert all(r['exit'] == 0 and r['running_percent'] == 100 for r in samples)
            assert len({r['output_sha256'] for r in samples}) == 1
            medians[arm] = statistics.median(r['instructions_per_call'] for r in samples)
            outputs[arm] = samples[0]['output_sha256']
        assert outputs['base'] == outputs['candidate'], (case,op,'base divergence')
        assert outputs['cpp'] == outputs['candidate'], (case,op,'C++ divergence')
        summary[f'{case}/{op}'] = dict(medians=medians, outputs=outputs,
            candidate_base=medians['candidate']/medians['base'],
            candidate_cpp=medians['candidate']/medians['cpp'],
            candidate_optimesh=medians['candidate']/medians['rust'] if 'rust' in medians else None)
real = [i['case'] for i in inputs if i['case'].startswith('real-')]
assert len(real) == 3
regressions = [k for k,v in summary.items() if v['candidate_base'] > 1.01]
passed = all(summary[f'{case}/meshlets']['candidate_optimesh'] < 1 and
             summary[f'{case}/cluster-lod']['candidate_base'] <= .85 for case in real)
partial = all(summary[f'{case}/{op}']['candidate_base'] <= .90
              for case in real for op in ['meshlets','cluster-lod'])
verdict = 'FAIL' if regressions else 'PASS' if passed else 'PARTIAL' if partial else 'NO-GAIN'
result = dict(verdict=verdict, regressions=regressions, cases=summary)
(art/'results.json').write_text(json.dumps(result,indent=2)+'\n')
print(verdict, 'regressions', regressions)
