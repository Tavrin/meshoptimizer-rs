"""Apply the instruction and output-identity bars to the completed campaign."""
import json,sys
import run as q
p=q.OUT/'release-results.json';d=json.loads(p.read_text())
identity={k:sum(c['identity'][k] for c in d['cells']) for k in d['cells'][0]['identity']}
worst=max(max(c['ratios'][k] for k in ['scalar_base','default_base']) for c in d['cells'])
r=dict(samples=d['samples'],cells=len(d['cells']),identity=identity,regressions=d['regressions'],worst_ratio=worst)
p.with_name('instruction-bars.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r,indent=2));sys.exit(0 if d['samples']==1008 and len(d['cells'])==168 and all(v==168 for v in identity.values()) and not d['regressions'] else 1)
