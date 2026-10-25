import contextlib,hashlib,json,math,os,struct,sys,time,zipfile,datetime
from pathlib import Path
from stats import analyze
s=Path(__file__).parent;sys.path.insert(0,'/home/etienne/dev/meshoptimizer-rs-wt/simplify-perf/parity')
import performance,runner as r
r.benchmark_cpu=8;os.sched_setaffinity(0,{8})
arm=sys.argv[1] if len(sys.argv)>1 else 'C';attempt=int(sys.argv[2]) if len(sys.argv)>2 else 1
out=s/f'{arm}-attempt-{attempt}';out.mkdir(exist_ok=True)
ids=json.loads((s/'identities.json').read_text());frozen=json.loads((s.parent/'r2/frozen-final-1/benchmark.json').read_text());base=json.loads((s.parent/'r2/frozen-baseline-1/benchmark.json').read_text())
assert len(frozen['workloads'])==204 and set(frozen['workloads'])==set(base['workloads'])
reg=s/('REGISTRATION.md' if arm=='C' else 'F-REGISTRATION.md')
recordpath=out/'record.json'
record=json.loads(recordpath.read_text()) if recordpath.exists() else {'arm':arm,'attempt':attempt,'cpu':8,'affinity':sorted(os.sched_getaffinity(0)),'started_unix':time.time(),'registration_sha256':r.sha(reg),'identities':ids,'main':{},'AA':{},'replication':{},'completed':False}
assert not record['completed'];assert r.sha(reg)==record['registration_sha256']
def save():recordpath.write_text(json.dumps(record,indent=2)+'\n')
def boundary():
 coord=Path('/mnt/linux-extra/moss-coord');waiting=[]
 for q in ['heavy.waiting','heavy.waiting-exclusive']:
  for line in (coord/q).read_text().splitlines():
   fields=line.split()
   if not fields:continue
   pid=int(fields[0])
   try:cwd=os.readlink(f'/proc/{pid}/cwd');state=Path(f'/proc/{pid}/stat').read_text().split(') ')[1].split()[0]
   except (FileNotFoundError,PermissionError,ProcessLookupError):continue
   if cwd.startswith('/mnt/linux-extra/moss-lane-worktrees/train-') and state not in ['Z','X']:
    waiting.append({'pid':pid,'cwd':cwd,'queue':q})
 if waiting:
  pause={'event':'PAUSE','utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'arm':arm,'attempt':attempt,'completed_cases':{k:len(record[k]) for k in ['main','AA','replication']},'waiting':waiting}
  with (s/'pauses.jsonl').open('a') as f:f.write(json.dumps(pause)+'\n')
  save();print(json.dumps(pause),flush=True);raise SystemExit(75)
for a in [arm,'B','Bprime',arm+'p','Bp']:
 assert r.sha(Path(ids[a]['binary']))==ids[a]['sha256']
save()
with zipfile.ZipFile(s.parent/'r2/frozen-final-1-artifacts/benchmark-buffers.zip') as archive:
 for phase,a,b in [('main',arm,'B'),('AA','Bprime','B'),('replication',arm+'p','Bp')]:
  if phase=='replication':
   main=analyze(record['main']);aa=analyze(record['AA']);F=max(abs(v['median']-1) for v in aa['cases'].values());T=math.ceil((1+F+.02)*1000)/1000
   flagged=[n for n,c in main['cases'].items() if c['interval'][0]>T]
   record.update(F=F,T=T,flagged=flagged,main_analysis=main,AA_analysis=aa);save()
  else:flagged=None
  for name,w in frozen['workloads'].items():
   if name in record[phase] or (flagged is not None and name not in flagged):continue
   boundary()
   data=archive.read(w['input']['member']);assert r.digest(data)==w['input']['sha256']==base['workloads'][name]['input']['sha256']
   path=out/'case.input';path.write_bytes(data)
   work={'arm':a,'base':b,'family':w['family'],'input':w['input'],'warmup_pairs':1,'warmup_order':[a,b],'samples':[],'started_unix':time.time()}
   with contextlib.ExitStack() as cleanup:
    ps={}
    for v in [a,b]:ps[v]=performance.Paired(r,ids[v]['binary'],path);cleanup.callback(ps[v].close)
    for pair in range(12):
     order=[a,b] if pair%2==0 else [b,a];sample={'pair':pair,'order':order,'before_unix':time.time(),'load_before':os.getloadavg(),'seconds':{}}
     for v in order:
      t=ps[v].sample();assert math.isfinite(t) and t>0,(name,pair,v,t);sample['seconds'][v]=t
     sample.update(ratio=sample['seconds'][a]/sample['seconds'][b],after_unix=time.time());work['samples'].append(sample)
     with (out/'samples.jsonl').open('a') as f:f.write(json.dumps({'phase':phase,'case':name,**sample})+'\n')
    outputs=[]
    for v in [a,b]:
     raw=ps[v].finish();count=struct.unpack_from('<I',raw,12)[0];assert count==12;output,times=r.response(raw[:-8],samples=count);assert times==[p['seconds'][v] for p in work['samples']]
     (out/(phase+'-'+hashlib.sha256(name.encode()).hexdigest()+'-'+v+'.output')).write_bytes(raw);outputs.append(output)
    assert outputs[0]==outputs[1],('output mismatch',phase,name)
   record[phase][name]=work;save();print(f'{arm} {phase} {len(record[phase])} {name}',flush=True)
record.update(completed=True,finished_unix=time.time(),replication_analysis=analyze(record['replication']));save()
for a in [arm,'B','Bprime',arm+'p','Bp']:assert r.sha(Path(ids[a]['binary']))==ids[a]['sha256']
print('COMPLETE',arm,'F',record['F'],'T',record['T'],'flagged',record['flagged'],flush=True)
