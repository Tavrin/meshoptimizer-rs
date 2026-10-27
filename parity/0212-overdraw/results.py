import os
from pathlib import Path
import collections,json,math,statistics,sys
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
phase=sys.argv[1]
rows=[json.loads(s) for s in (A/phase/'raw.jsonl').read_text().splitlines()]
cells=collections.defaultdict(lambda:collections.defaultdict(list))
for r in rows:cells[r['case'],r['operation']][r['arm']].append(r['instructions_per_call'])
results=[]
for (case,op),arms in sorted(cells.items()):
    assert all(len(v)==3 for v in arms.values())
    med={arm:statistics.median(values) for arm,values in arms.items()}
    result={'case':case,'operation':op,'median_instructions':med,'ratios':{a:{peer:med[a]/med[peer] for peer in ['base','M7','X']} for a in med}}
    if phase=='final':
        q=med['candidate']/med['base']
        result['verdict']='FAIL' if q>1.01 else ('PASS' if q<=.80 else 'PARTIAL' if q<=.95 else 'NO-GAIN') if case.startswith('real-') else 'WITHIN-REGRESSION-LIMIT'
    results.append(result)
(A/phase/'results.json').write_text(json.dumps(results,indent=2)+'\n')
if phase=='final':
    out=['case\toperation\tbase\tcandidate\tM7\tX\tcandidate/base\tcandidate/M7\tcandidate/X\tverdict']
    for r in results:
        m=r['median_instructions'];q=r['ratios']['candidate']
        out.append('\t'.join([r['case'],r['operation'],*[str(m[a]) for a in ['base','candidate','M7','X']],*[str(q[a]) for a in ['base','M7','X']],r['verdict']]))
    (A/'instructions.tsv').write_text('\n'.join(out)+'\n')
    assert all(r['verdict']!='FAIL' for r in results), 'per-case regression gate failed'
for r in results:print(r['case'],r['operation'],r.get('verdict','ATTRIBUTION'),r['ratios'].get('candidate',r['ratios']))
