"""Apply the pre-registered instruction gate without selecting samples."""
from pathlib import Path
import json,math
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H1')
rows=json.loads((S/'summary.json').read_text())
def aggregate(name,cases,weighted=False):
 values=[]
 for c in cases:
  weight=c['multiplicity'] if weighted else 1
  values.extend([c['median']]*weight)
 assert values,name
 gm=lambda denominator:math.exp(sum(math.log(x['candidate']/x[denominator]) for x in values)/len(values))
 base,cpp=gm('base'),gm('M7');worst=max(x['candidate']/x['base'] for x in values)
 verdict='FAIL' if worst>1.01 else 'PASS' if cpp<=1 else 'PARTIAL' if base<=.95 else 'NO-GAIN'
 return dict(name=name,cases=len(cases),calls=len(values),candidate_base_gm=base,candidate_M7_gm=cpp,worst_candidate_base=worst,verdict=verdict)
result=[]
for family in ['cache','cache-fifo']:
 subset=[c for c in rows if c['family']==family]
 result.append(aggregate(family+' frozen',[c for c in subset if c['kind']=='frozen']))
 result.append(aggregate(family+' all registered calls',subset,True))
for chain in sorted(set(c['chain'] for c in rows if c['kind']=='chain')):
 cases=[c for c in rows if c.get('chain')==chain]
 a=aggregate(chain[:4]+' cache',cases,True)
 sums={arm:sum(c['median'][arm]*c['multiplicity'] for c in cases) for arm in ['base','candidate','M7']}
 a['candidate_base_sum']=sums['candidate']/sums['base'];a['candidate_M7_sum']=sums['candidate']/sums['M7'];result.append(a)
(S/'results.json').write_text(json.dumps(dict(aggregates=result,cases=rows),indent=2)+'\n')
for r in result:print(r)
# FIFO was NO-GAIN in campaign 1 (results-campaign1.json) and its source was reverted
# to 0.2.9; its rows here only show candidate == base, so it is exempt.
REVERTED=('cache-fifo',)
assert all(r['verdict'] not in ['NO-GAIN','FAIL'] for r in result if not r['name'].startswith(REVERTED)), 'Fix or revert family; retain this campaign'
assert all(r['worst_candidate_base']<=1.01 for r in result), 'FAIL: reverted family regressed'
