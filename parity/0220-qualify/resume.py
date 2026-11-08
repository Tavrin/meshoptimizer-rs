"""Resume pending stages after an interrupted admitted job."""
import json,os,subprocess,sys
from pathlib import Path
import run as q
A=q.A;L=q.LOGS;L.mkdir(parents=True,exist_ok=True)
for name,gb,script in [('bundle-resume',4,'bundle.py'),('all-features-and-report',9,'complete.py')]:
 # QUALIFY_STEER (optional): the admission tool's steering file; a hold there stops new submissions.
 control=Path(os.environ['QUALIFY_STEER']) if os.environ.get('QUALIFY_STEER') else None
 state=json.loads(control.read_text()) if control and control.exists() else {}
 if state.get('holds') or state.get('lane_holds'):
  print('STOP: coordinator hold; no new submission',flush=True);sys.exit(75)
 print('SUBMIT',name,gb,flush=True)
 with (L/(name+'-admission.log')).open('w') as f:
  p=subprocess.run([os.environ['QUALIFY_ADMISSION'],str(gb),'python3',str(A/script)],stdout=f,stderr=subprocess.STDOUT)
 with (L/'submissions.jsonl').open('a') as f:f.write(json.dumps(dict(stage=name,reservation_gb=gb,exit=p.returncode))+'\n')
 print('COMPLETE',name,p.returncode,flush=True)
 if p.returncode:sys.exit(p.returncode)
