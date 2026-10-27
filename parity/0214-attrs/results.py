"""Summarize the pre-registered instruction gate and output identity."""
import json,os,statistics
from pathlib import Path
art=Path(os.environ['ATTR_ART']); rows=[json.loads(x) for x in (art/'full-counts.jsonl').read_text().splitlines()]
metadata=json.loads((art/'full-campaign.json').read_text())['inputs']; cases=[x['case'] for x in metadata];ops=['simplify-attributes','simplify','overdraw'];arms=['base','candidate','cpp','rust']
assert len(rows)==len(cases)*len(ops)*len(arms)*3
out=[]
for case in cases:
 for op in ops:
  vals={}; hashes={}
  for arm in arms:
   rr=[r for r in rows if r['case']==case and r['op']==op and r['arm']==arm]
   assert len(rr)==3 and {r['rep'] for r in rr}=={0,1,2} and all(r['exit']==0 and r['running_percent']==100 for r in rr)
   vals[arm]=statistics.median(r['instructions_per_call'] for r in rr);hashes[arm]={r['output_sha256'] for r in rr};assert len(hashes[arm])==1
  assert hashes['base']==hashes['candidate'],(case,op,hashes)
  out.append(dict(case=case,op=op,**vals,candidate_base=vals['candidate']/vals['base'],candidate_cpp=vals['candidate']/vals['cpp'],candidate_rust=vals['candidate']/vals['rust'],output_equal_base=True))
real=[r for r in out if r['case'].startswith('real-')]
regressions=[r for r in out if r['candidate_base']>1.01]
pass_both=all(r['candidate_rust']<1 for r in real if r['op'] in ('simplify-attributes','overdraw'))
partial={op:all(r['candidate_base']<=0.90 for r in real if r['op']==op) for op in ('simplify-attributes','overdraw')}
verdict='FAIL' if regressions else 'PASS' if pass_both else 'PARTIAL' if any(partial.values()) else 'NO-GAIN'
result=dict(verdict=verdict,partial=partial,regressions=regressions,rows=out)
(art/'full-results.json').write_text(json.dumps(result,indent=2)+'\n');print(verdict,flush=True)
