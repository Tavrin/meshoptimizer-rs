#!/usr/bin/python3
from pathlib import Path
import subprocess, os, json, time, hashlib
s=Path(__file__).parent
os.sched_setaffinity(0,{8})
case='jungle-scoreboard-214e85c568a64e8a-entity'
for arm,binary,env in [('reuse','c',{}),('clear-mesh','c',{'MESHOPT_DIAG_RELEASE_MESH':'1'}),('cpp13','b',{}),('clear-call-diagnostic','c',{'MESHOPT_DIAG_RELEASE_CALL':'1'})]:
 out=s/'step1'/arm;out.mkdir(parents=True,exist_ok=False)
 cmd=['/usr/bin/time','-v','-o',str(out/'time.txt'),str(s/'bin'/f'migration-{binary}'),'--manifest',str(s/'214e-corpus.toml'),'--fixtures',case,'--output',str(out/'result.json')]
 print(json.dumps({'start':arm,'utc':time.time(),'cmd':cmd}),flush=True)
 with (out/'stdout.log').open('w') as stdout,(out/'stderr.log').open('w') as stderr:
  p=subprocess.run(cmd,stdout=stdout,stderr=stderr,env={**os.environ,**env})
 print(json.dumps({'finish':arm,'rc':p.returncode,'utc':time.time()}),flush=True)
 if p.returncode:raise SystemExit(p.returncode)
