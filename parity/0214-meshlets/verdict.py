#!/usr/bin/env python3
"""Compute the pre-registered median instruction gate from finished rows."""
import json,os,statistics
from pathlib import Path
art=Path(os.environ['MESHLETS_ART'])
rows=[json.loads(s) for s in (art/'campaign.jsonl').read_text().splitlines()]
cases=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth','real-214e','real-a082','real-b6fd']
ops=['meshlets','meshlets-flex','meshlets-spatial','cluster-lod']
med={}
for case in cases:
 for op in ops:
  for arm in ('base','candidate','cpp','rust'):
   if op=='cluster-lod' and arm=='rust':continue
   v=[r for r in rows if r['case']==case and r['op']==op and r['arm']==arm]
   assert sorted(r['rep'] for r in v)==[0,1,2],(case,op,arm)
   assert len({r['output_sha256'] for r in v})==1,(case,op,arm)
   med[case,op,arm]=statistics.median(r['per_call'] for r in v)
  outputs={r['output_sha256'] for r in rows if r['case']==case and r['op']==op and r['arm'] in ('base','candidate')}
  assert len(outputs)==1,(case,op,'base/candidate output mismatch')
real=cases[-3:]
regressions=[(c,o,med[c,o,'candidate']/med[c,o,'base']) for c in cases for o in ops if med[c,o,'candidate']/med[c,o,'base']>1.01]
full=all(med[c,'meshlets','candidate']<med[c,'meshlets','rust'] and med[c,'cluster-lod','candidate']/med[c,'cluster-lod','base']<=.90 for c in real)
partial=all(med[c,'meshlets','candidate']/med[c,'meshlets','base']<=.90 for c in real)
verdict='FAIL' if regressions else 'PASS' if full else 'PARTIAL' if partial else 'NO-GAIN'
result=dict(verdict=verdict,regressions=regressions,rows=[dict(case=c,op=o,base=med[c,o,'base'],candidate=med[c,o,'candidate'],cpp=med[c,o,'cpp'],optimesh=med.get((c,o,'rust')),candidate_over_base=med[c,o,'candidate']/med[c,o,'base']) for c in cases for o in ops])
(art/'verdict.json').write_text(json.dumps(result,indent=2)+'\n')
print(verdict, 'regressions',regressions)
