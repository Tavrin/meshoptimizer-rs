"""Serial admitted qualification. Infrastructure locations are supplied by the caller."""
import json, os, subprocess, sys
import run as q
A=q.A;L=q.LOGS;L.mkdir(parents=True,exist_ok=True)
q.T.mkdir(parents=True,exist_ok=True)
steps=[('prepare',4,['run.py','prepare']),('fmt-and-4gb-campaign',4,['gate.py','fmt']),('all-features-and-report',9,['complete.py'])]
for name,gb,args in steps:
 print('SUBMIT',name,gb,flush=True)
 with (L/(name+'-admission.log')).open('w') as f:
  p=subprocess.run([os.environ['QUALIFY_ADMISSION'],str(gb),'python3',*[str(A/args[0]),*args[1:]]],stdout=f,stderr=subprocess.STDOUT)
 with (L/'submissions.jsonl').open('a') as f:f.write(json.dumps(dict(stage=name,reservation_gb=gb,exit=p.returncode))+'\n')
 print('COMPLETE',name,p.returncode,flush=True)
 # Coordinator refusal stops further submissions; keep all other failures as evidence.
 if p.returncode in [75,143,130]:
  print('STOPPED admission/hold',flush=True);sys.exit(p.returncode)
