"""Check success differences and retain inherited failed-call write observations."""
import json,sys
from pathlib import Path
import run as q
A=q.OUT;rows=[]
for family in ['decode-all','decode-none','vdecode-all','vdecode-none']:
 for p in sorted((A/family).rglob('differential*.json')):
  d=json.loads(p.read_text());rows.append(dict(receipt=str(p.relative_to(A)),**{k:d.get(k,0) for k in ['cases','forced_level_differences','cpp_success_differences','cpp_failure_write_differences','cpp_default_isa_divergences']}))
for row in rows:
 old=q.A.parent/'0219-qualify'/row['receipt']
 if old.exists():
  d=json.loads(old.read_text());row['retained_0219_failure_write_differences']=d.get('cpp_failure_write_differences',0);row['same_failure_count_as_0219']=row['cpp_failure_write_differences']==row['retained_0219_failure_write_differences']
totals={k:sum(x[k] for x in rows) for k in ['cases','forced_level_differences','cpp_success_differences','cpp_failure_write_differences','cpp_default_isa_divergences']}
result=dict(runs=rows,totals=totals,strict_zero_differences=bool(rows) and all(totals[k]==0 for k in totals if k!='cases'),note='Failure-buffer observations are reported per reference arm and run, not deduplicated inputs. The current brief requires zero success-output differences and separate disclosure of inherited failed-call writes. The strict-zero field is diagnostic only.')
(A/'cpp-observations.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2));sys.exit(0 if rows and totals['cpp_success_differences']==0 and totals['forced_level_differences']==0 and all(x.get('same_failure_count_as_0219',False) for x in rows) else 1)
