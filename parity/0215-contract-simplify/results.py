import json,statistics,os
from collections import defaultdict
from pathlib import Path
s=Path(os.environ['MESHOPT_SCRATCH'])
rows=[json.loads(v) for v in (s/'full-counts.jsonl').read_text().splitlines()]
groups=defaultdict(list)
for r in rows:groups[(r['case'],r['op'],r['arm'])].append(r)
cases=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth','real-214e','real-a082','real-b6fd']
ops=['simplify','simplify-attributes','simplify-update','simplify-sloppy','simplify-prune','simplify-points','overdraw']
arms=['base','candidate','cpp','rust']
assert len(rows)==len(cases)*len(ops)*len(arms)*3
result=[]
for case in cases:
 for op in ops:
  cell={a:groups[(case,op,a)] for a in arms}
  assert all(len(v)==3 and len({x['rep'] for x in v})==3 and len({x['output'] for x in v})==1 for v in cell.values())
  median={a:statistics.median(x['count']/x['repetitions'] for x in v) for a,v in cell.items()}
  assert cell['base'][0]['output']==cell['candidate'][0]['output']==cell['cpp'][0]['output'],(case,op,'identity')
  result.append(dict(case=case,op=op,counts=median,base=median['candidate']/median['base'],cpp=median['candidate']/median['cpp'],rust=median['candidate']/median['rust']))
real=[x for x in result if x['case'].startswith('real-')]
regressions=[x for x in result if x['base']>1.01]
passed=all(x['rust']<1 for x in real if x['op'] in ['simplify','simplify-attributes','overdraw'])
partial=any(all(x['base']<=.90 for x in real if x['op']==op) for op in ['simplify-attributes','overdraw'])
verdict='FAIL' if regressions else 'PASS' if passed else 'PARTIAL' if partial else 'NO-GAIN'
(s/'full-results.json').write_text(json.dumps(dict(verdict=verdict,results=result,regressions=regressions),indent=2)+'\n')
print(verdict, 'regressions',len(regressions))
