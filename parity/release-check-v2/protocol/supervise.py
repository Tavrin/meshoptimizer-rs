from pathlib import Path
import subprocess,os,time,json
s=Path(__file__).parent
arm=__import__('sys').argv[1] if len(__import__('sys').argv)>1 else 'C'
attempt=1;segment=0
while True:
 segment+=1
 with (s/f'{arm}-attempt-{attempt}-segment-{segment}.log').open('x') as log:
  p=subprocess.run(['/mnt/linux-extra/moss-coord/bin/moss-heavy.sh','4',str(s/'run.sh'),arm,str(attempt)],stdout=log,stderr=subprocess.STDOUT,env={**os.environ,'MOSS_HEAVY_MEASUREMENT':'1','MOSS_LANE':'meshopt-release-v2'})
 with (s/'attempts.jsonl').open('a') as f:f.write(json.dumps({'arm':arm,'attempt':attempt,'segment':segment,'exit':p.returncode,'unix':time.time()})+'\n')
 if p.returncode==0:break
 if p.returncode==75:
  # Release admission during PAUSE. The next admission retains this session's case checkpoints.
  while True:
   waiting=[]
   for q in ['heavy.waiting','heavy.waiting-exclusive']:
    for line in (Path('/mnt/linux-extra/moss-coord')/q).read_text().splitlines():
     try:cwd=os.readlink('/proc/'+line.split()[0]+'/cwd')
     except (FileNotFoundError,ProcessLookupError,PermissionError):continue
     if cwd.startswith('/mnt/linux-extra/moss-lane-worktrees/train-'):waiting.append(cwd)
   if not waiting:break
   time.sleep(5)
  continue
 text=(s/f'{arm}-attempt-{attempt}-segment-{segment}.log').read_text()
 if any(x in text for x in ['paired driver timeout','short paired response','TimeoutExpired','paired driver failed']):
  if attempt==1:attempt=2;continue
 raise SystemExit(p.returncode)
