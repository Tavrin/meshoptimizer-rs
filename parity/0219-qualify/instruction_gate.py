"""Apply the instruction and output-identity bars to the completed campaign."""
import json,sys
from pathlib import Path
p=Path(__file__).with_name('release-results.json');d=json.loads(p.read_text())
identity={k:sum(c['identity'][k] for c in d['cells']) for k in d['cells'][0]['identity']}
worst=max(max(c['ratios'][k] for k in ['scalar_base','default_base']) for c in d['cells'])
r=dict(samples=d['samples'],cells=len(d['cells']),identity=identity,regressions=d['regressions'],worst_ratio=worst)
print(json.dumps(r,indent=2));sys.exit(0 if d['samples']==3024 and len(d['cells'])==168 and all(v==168 for v in identity.values()) and not d['regressions'] else 1)
