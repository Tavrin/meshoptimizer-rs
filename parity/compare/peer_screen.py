#!/usr/bin/env python3
"""Extend the retained compare harness: oracle validation, paired samples, train yielding."""
from pathlib import Path
import os,sys,json,hashlib,time,subprocess,math,statistics,select,shutil,datetime
import measure as m
ROOT=m.ROOT; ART=m.ART
BURST_STARTED=time.monotonic()
def completed_lane_runtime():
 start=datetime.datetime.strptime(ART.name,'campaign-%Y%m%dT%H%M%SZ').replace(tzinfo=datetime.timezone.utc).timestamp()
 total=0.;receipts=[]
 with Path('/mnt/linux-extra/moss-coord/heavy.log').open()as f:
  for line in f:
   fields=line.rstrip('\n').replace('\\t','\t').split('\t')
   if len(fields)<11 or fields[10]!='meshopt-peer-screen':continue
   try:ended=float(fields[9]);seconds=float(fields[3])
   except ValueError:continue
   if ended>=start and str(ROOT)in fields[6]:
    total+=max(0.,seconds)+1.;receipts.append({'pid':fields[0],'ended':ended,'runtime_seconds':seconds,'exit':fields[4]})
 return total,receipts
COMPLETED_LANE_SECONDS,LANE_TIME_RECEIPTS=completed_lane_runtime()
OPS=m.PERF+m.EXTRA
XOPS=set(OPS)
DENSE=['simplify','meshlets','meshlets-spatial']
REAL=[('214e', '214e85c568a64e8a'),('b6fd','b6fdb93d8a1709e2'),('a082','a082ca1fd19bb86a')]
def cases():
 return m.CASES+[(f'real-{short}',0,'replay')for short,_ in REAL]
def input_path(name,op):
 return ART/'inputs'/f'{name}{"-"+op if name.startswith("real-") else "-strip" if op=="unstripify" else ""}.bin'
def operations(name):return ['simplify','overdraw']if name.startswith('real-')else OPS
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,x):
 t=p.with_suffix('.tmp');t.write_text(json.dumps(x,indent=2)+'\n');t.replace(p)
def event(**kw):
 with (ART/'events.jsonl').open('a')as f:f.write(json.dumps(dict(utc=time.time(),**kw))+'\n')
def train_waiters():
 found=[]
 for p in Path('/mnt/linux-extra/moss-coord').glob('heavy.queued.*.json'):
  try:
   x=json.loads(p.read_text());pid=int(x['pid']);os.kill(pid,0)
   wt=x.get('worktree_id','');cmd=Path(f'/proc/{pid}/cmdline').read_bytes().decode(errors='replace')
   cwd=str(Path(f'/proc/{pid}/cwd').resolve())
   if '/mnt/linux-extra/moss-lane-worktrees/train-' in wt+cwd+cmd:found.append(x)
  except (OSError,ValueError,KeyError):continue
 return found
def boundary():
 if time.monotonic()-BURST_STARTED>720:
  event(event="PAUSE",reason="720-second checkpoint between cases");raise SystemExit(75)
 m.disk();waiting=train_waiters()
 if waiting:event(event='PAUSE',reason='train first',waiting=waiting);raise SystemExit(75)
 assert COMPLETED_LANE_SECONDS+time.monotonic()-BURST_STARTED<6*3600,'six hours of admitted lane runtime exhausted'
def source_check():
 x=json.loads((ART/'epoch.json').read_text());assert all(sha(ROOT/p)==h for p,h in x['source_sha256'].items()),'source epoch changed'
class Driver(m.Driver):
 def __init__(self,arm,profile,mode,op,path,cpu=None,suffix=''):
  self.output=ART/'outputs'/f'{profile}-{mode}-{path.stem}-{op}-{arm}{suffix}.bin';self.output.parent.mkdir(exist_ok=True)
  self.binary=ART/f'cmp-driver-{profile}-{arm}-{mode}'
  cmd=[str(self.binary),op,str(path),str(self.output)]
  if cpu is not None:cmd=['taskset','-c',str(cpu)]+cmd
  self.p=subprocess.Popen(cmd,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
  try:assert self.read()=='READY'
  except BaseException:
   self.p.kill();self.p.wait();raise
 def close(self):
  if self.p.poll() is None:
   self.p.stdin.write('stop\n');self.p.stdin.flush();self.p.communicate(timeout=180)
  assert self.p.returncode==0,self.p.returncode

def execute(backend,profile,op,path,suffix=''):
 arm='O' if backend=='ours' else 'M7'
 with Driver(arm,profile,'scalar',op,path,suffix=suffix)as d:return d.output.read_bytes()
m.execute=execute

def prepare():
 assert os.environ.get('MOSS_HEAVY_ACTIVE');m.prepare()
 frozen=json.loads((ROOT/'parity/compare/results.json').read_text())['outputs']
 for name,_,_ in m.CASES:
  expected=next(r['input_sha256']for r in frozen if r['case']==name and r['operation']=='vertex-decode');assert sha(ART/'inputs'/f'{name}.bin')==expected,'frozen input mismatch'
 real_prepare()
 # Frozen source, input, executable identities; no timing sample yet.
 files=list((ROOT/'src').rglob('*.rs'))+list((ROOT/'parity/compare/src').glob('*.rs'))+list((ROOT/'parity/compare').glob('peer_*.py'))+list((ROOT/'parity/compare').glob('peer_*.mjs'))+[ROOT/'Cargo.toml',ROOT/'parity/compare/Cargo.toml',ROOT/'parity/compare/Cargo.lock',ROOT/'parity/compare/measure.py',Path(__file__).resolve()]
 save(ART/'epoch.json',{'started':time.time(),'revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'source_sha256':{str(p.relative_to(ROOT)):sha(p) for p in files},'binaries':{p.name:sha(p) for p in ART.glob('cmp-driver-*')},'core':m.quiet.physical_core()})

def real_prepare():
 import struct,array
 base=Path('/mnt/linux-extra/moss-scratch/meshopt-migration-uz67q/r4-measurements')
 provenance=[]
 for short,ident in REAL:
  p=base/f'jungle-scoreboard-{ident}-entity/B/jungle-scoreboard-{ident}-entity.json';doc=json.loads(p.read_text());blobs=p.parent/doc['byte_blobs'];v=doc['records'][0]['data'];desc=v['vertex_bytes'];raw=(blobs/desc['$bytes']).read_bytes();assert len(raw)==desc['length'] and hashlib.sha256(raw).hexdigest()==desc['sha256']
  positions=array.array('f')
  for off in range(0,len(raw),64):positions.frombytes(raw[off:off+12])
  vertex_raw=raw
  for op in ['simplify','overdraw']:
   r=next(r for r in doc['records'] if r['stage']==op);desc=r['data']['input_indices'];raw=(blobs/desc['$bytes']).read_bytes();assert len(raw)==desc['length'] and hashlib.sha256(raw).hexdigest()==desc['sha256'];indices=array.array('I');indices.frombytes(raw);del raw
   d=r['data'];params={'target_indices':d.get('target_indices',len(indices)//2//3*3),'target_error_bits':d.get('target_error_bits',0x3f000000),'options':d.get('options',0),'threshold_bits':d.get('threshold_bits',0x3f866666)}
   out=input_path('real-'+short,op)
   with out.open('wb')as f:
    f.write(m.pack(positions,indices,[b'']*7));f.write(struct.pack('<III',0x52504c59,64,len(vertex_raw)));f.write(vertex_raw);f.write(struct.pack('<IIII',params['target_indices'],params['target_error_bits'],params['options'],params['threshold_bits']))
   provenance.append({'case':short,'operation':op,'replay':str(p),'replay_sha256':sha(p),'vertex_blob':v['vertex_bytes'],'index_blob':desc,'input_sha256':sha(out),'vertex_stride':64,'parameters':params,'representation':'full captured stride-64 vertex bytes; packed XYZ prefix is for independent validators only'})
 save(ART/'real-inputs.json',provenance)

def correctness():
 source_check();p=ART/'correctness.json';rows=json.loads(p.read_text())if p.exists()else[]
 for profile in ['defaults','moss']:
  for mode in ['scalar','simd']:
   for name,nt,shape in cases():
    boundary()
    for op in operations(name):
     if all(any(r['key']==[profile,mode,name,op,arm] for r in rows) for arm in ['O','M7','X']):continue
     boundary()
     path=input_path(name,op)
     ref=execute('theirs','defaults',op,path,suffix='-reference')
     for arm in ['O','M7','X']:
      key=[profile,mode,name,op,arm]
      if any(r['key']==key for r in rows):continue
      if arm=='X' and op not in XOPS:
       rows.append({'key':key,'status':'UNAVAILABLE','reason':'public API adapter not available in this bounded screen'});save(p,rows);continue
      row={'key':key,'checked_utc':time.time(),'input_sha256':sha(path),'oracle_sha256':hashlib.sha256(ref).hexdigest()}
      try:
       with Driver(arm,profile,mode,op,path,suffix='-correctness')as d:out=d.output.read_bytes()
       row.update(output_sha256=hashlib.sha256(out).hexdigest(),bytes=len(out))
       row['validation']=m.valid(op,out,path,arm,profile)
       row['status']='identical' if out==ref else 'different but valid'
       if op in ['simplify','simplify-attributes','simplify-sloppy']:
        import struct
        row['triangles']=(len(out)-4)//12;row['reported_error']=struct.unpack('<f',out[-4:])[0];row['oracle_triangles']=(len(ref)-4)//12;row['oracle_reported_error']=struct.unpack('<f',ref[-4:])[0]
       elif op.startswith('meshlets'):
        import struct
        row['meshlets']=struct.unpack_from('<I',out)[0];row['triangles']=nt
      except (RuntimeError,TimeoutError)as e:
       row.update(status='unmeasured',reason='driver produced no completed output: '+repr(e));event(event='correctness-infrastructure-failure',key=key,reason=repr(e))
      except (AssertionError,ValueError)as e:row.update(status='invalid',reason=repr(e))
      rows.append(row);save(p,rows)
    print('correctness',profile,mode,name,flush=True)
 event(event='correctness-complete',rows=len(rows))

def timing(profile,mode,arm,aa=False):
 source_check();assert os.environ.get('MOSS_HEAVY_ACTIVE');assert os.environ.get('MOSS_HEAVY_MEASUREMENT')=='1'
 checks=json.loads((ART/'correctness.json').read_text());core=json.loads((ART/'epoch.json').read_text())['core']['cpu']
 out=ART/f'timing-{profile}-{mode}-{arm}.json';rows=json.loads(out.read_text())if out.exists()else[]
 for name,nt,shape in cases():
  for op in (DENSE if aa else operations(name)):
   if aa and name.startswith('real-'):continue
   if any(r['case']==name and r['operation']==op for r in rows):continue
   boundary()
   labels=[r for r in checks if r['key'][:4]==[profile,mode,name,op] and r['key'][4] in ['O', 'O' if aa else arm]]
   if any(r['status'] in ['invalid','UNAVAILABLE','unmeasured']for r in labels):continue
   path=input_path(name,op)
   row={'case':name,'operation':op,'labels':labels,'input_sha256':sha(path),'samples':[],'binary_sha256':{b:sha(ART/f'cmp-driver-{profile}-{b}-{mode}')for b in ['O',arm]},'cpu':core}
   attempt=0
   while True:
    try:
     with Driver('O',profile,mode,op,path,core)as ours,Driver(arm,profile,mode,op,path,core)as peer:
      # Validate the exact resident outputs before calibration and sampling.
      for d,b in [(ours,'O'),(peer,'O'if aa else arm)]:
       expected=next(r for r in checks if r['key']==[profile,mode,name,op,b])
       assert sha(d.output)==expected['output_sha256']
      calibration={b:d.sample(1) for b,d in [('O',ours),(arm,peer)]};iterations=max(1,min(1000000,math.ceil(25000000/min(calibration.values()))));row.update(iterations=iterations,calibration_ns=calibration)
      for pair in range(20):
       if pair>=5 and not row['five_pair_ci'][0]<=1<=row['five_pair_ci'][1]:break
       order=['O',arm]if pair%2==0 else[arm,'O'];v={};row['pending_pair']={'pair':pair,'ns':v};save(out.with_suffix('.pending.json'),row)
       for b in order:
        v[b]={'O':ours,arm:peer}[b].sample(iterations);row['pending_pair']['ns']=dict(v);save(out.with_suffix('.pending.json'),row)
       row['samples'].append({'pair':pair,'order':order,'ns':v,'ratio':v[arm]/v['O'],'utc':time.time(),'load':os.getloadavg()});del row['pending_pair']
       if pair==4:row['five_pair_ci']=m.interval([s['ratio']for s in row['samples']])
       save(out.with_suffix('.pending.json'),row)
      logs=[math.log(s['ratio'])for s in row['samples']];row.update(gm=math.exp(statistics.mean(logs)),ci95=m.interval([s['ratio']for s in row['samples']]),completed=True)
     break
    except (TimeoutError,RuntimeError)as e:
     prior_failures=sum(1 for line in (ART/'events.jsonl').read_text().splitlines() if (lambda x:x.get('event')=='infrastructure-failure' and x.get('family')==op and x.get('arm')==arm and x.get('profile')==profile and x.get('mode')==mode)(json.loads(line))) if (ART/'events.jsonl').exists() else 0
     event(event='infrastructure-failure',profile=profile,mode=mode,family=op,case=name,arm=arm,attempt=attempt,reason=repr(e),retained=row)
     if prior_failures:row.update(completed=False,unmeasured_reason='timeout/crash twice');break
     attempt+=1;row['samples']=[]
   rows.append(row);save(out,rows);print(profile,mode,arm,name,op,row.get('gm'),flush=True)
if __name__=='__main__':
 cmd=sys.argv[1]
 if cmd=='prepare':prepare()
 elif cmd=='real-prepare':real_prepare()
 elif cmd=='correctness':correctness()
 else:timing(*sys.argv[2:],aa=sys.argv[2+2]=='AA')
