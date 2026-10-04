#!/usr/bin/env python3
"""Paired scalar/production timing and requested-heap matrix for phase 0.3."""
import argparse,json,math,os,statistics,struct,time
from pathlib import Path
import runner as r
KINDS=['smooth','seams','disconnected','sparse']

def geometry(n,kind):
 p,idx=r.grid(n,1);base=len(p)
 if kind=='seams':
  p+=p[:];idx=[v+base*((i//6)%2) for i,v in enumerate(idx)]
 elif kind=='disconnected':
  p=[(x+(layer%2)*n*4,y+(layer//2)*n*4,z) for layer in range(4) for x,y,z in p]
  idx=[v+base*((i//6//(n-1)>=((n-1)//2))*2+(i//6%(n-1)>=((n-1)//2))) for i,v in enumerate(idx)]
 elif kind=='sparse':p += [(x+n*4,y,z) for x,y,z in p]
 return p,idx

def cases(op,quick=False):
 sizes=[('tiny',5),('medium',65),('million-triangles',729)]
 if quick:sizes=sizes[:2]
 for label,n in sizes:
  for kind in (KINDS[:1] if quick else KINDS):
   case_label=label
   if op in [6,7,9,10,11]:n={'tiny':4,'medium':6,'million-triangles':8}[label]
   p,idx=geometry(n,kind);counts=[];target=8
   if op in [6,7,9,10,11] and label=='million-triangles':idx=(idx*math.ceil(1536/len(idx)))[:1536];case_label='maximum-meshlet'
   if op==8:target=1
   if op==10:target=3
   if op==12:
    size=64*3;counts=[min(size,len(idx)-i) for i in range(0,len(idx),size)]
   if op==15:target=64
   data=r.message(op,p,idx,mv=64,min=16,mt=64,weight=0.5,split=2.,target=target,counts=counts,radii=[0.1]*len(p))
   for api in (['value'] if op in [5,6,7,8] else ['allocating','into']):
    msg=bytearray(data)
    if api=='into':struct.pack_into('<I',msg,4,op|0x10000)
    yield f'{case_label}-{kind}-{api}',msg

def select_cpu(folder):
 allowed=sorted(os.sched_getaffinity(0))
 def stat():
  result={}
  for line in Path('/proc/stat').read_text().splitlines():
   f=line.split()
   if f[0].startswith('cpu') and f[0][3:].isdigit():
    cpu=int(f[0][3:]);v=list(map(int,f[1:9]));result[cpu]=(sum(v),v[3]+v[4])
  return result
 a=stat();time.sleep(0.5);b=stat();busy={c:1-(b[c][1]-a[c][1])/max(1,b[c][0]-a[c][0]) for c in allowed};cpu=min(allowed,key=lambda c:(busy[c],c));siblings=[]
 for part in Path(f'/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list').read_text().strip().split(','):
  lo,_,hi=part.partition('-');siblings.extend(range(int(lo),int(hi or lo)+1))
 record={'cpu':cpu,'siblings':siblings,'busy_fraction':busy,'selection_seconds':0.5};(folder/'cpu.json').write_text(json.dumps(record,indent=2)+'\n');return record

def summary(values):
 s=sorted(values);return {'median':statistics.median(s),'mean':statistics.mean(s),'min':s[0],'max':s[-1],'quartiles':statistics.quantiles(s,n=4),'stdev':statistics.stdev(s),'cv':statistics.stdev(s)/statistics.mean(s)}

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--phase',default='0.3');parser.add_argument('--profile',default='scalar-strict');parser.add_argument('--enforce',action='store_true');parser.add_argument('--quick',action='store_true');parser.add_argument('--families',nargs='*',choices=r.NAMES);a=parser.parse_args()
 bins,identity=r.build(False);folder=r.ART/('benchmark' if r.PROFILE=='release' else f'benchmark-{r.PROFILE}');folder.mkdir(exist_ok=True);cpu=select_cpu(folder);os.sched_setaffinity(0,{cpu['cpu']});clients={name:r.Client(name,p) for name,p in bins.items()};rows=[];start=time.time();finished=False
 record={'schema':'meshopt-p03-performance/2','identity':identity,'cpu':cpu,'method':'single thread; paired same-core resident drivers; validation, allocation, copies and operation timed; serialization, generation, I/O, startup excluded; output+scratch requested heap measured at untimed warm-up; fixed stack excluded on both sides','baselines':{'cpp':'scalar strict','cpp-simd':'same ISA availability, upstream SSE paths enabled; f32 contraction disabled'},'rows':rows,'quick':a.quick,'families':a.families,'rust_profile':r.PROFILE,'load_start':os.getloadavg()}
 def save():
  verdicts={}
  for baseline in ['cpp','cpp-simd']:
   verdicts[baseline]={}
   for name in r.NAMES:
    entries=[v for v in rows if v['family']==name and v['baseline']==baseline]
    if not entries:continue
    ratios=[v['ratio'] for v in entries];gm=math.exp(sum(map(math.log,ratios))/len(ratios));mm=max(v['memory_ratio'] for v in entries)
    verdicts[baseline][name]={'cases':len(entries),'geometric_mean':gm,'maximum':max(ratios),'maximum_memory_ratio':mm,'pass':gm<=1.25 and max(ratios)<=1.5 and mm<=1.25}
  record.update(verdicts=verdicts,complete=finished,elapsed=time.time()-start,load_end=os.getloadavg(),source_unchanged=identity['sources']==r.snapshot())
  (folder/'record.json').write_text(json.dumps(record,indent=2)+'\n')
 try:
  for op,name in enumerate(r.NAMES,1):
   if a.families and name not in a.families:continue
   for label,data in cases(op,a.quick):
    expected={n:c.run(data) for n,c in clients.items()};payloads={n:v[0] for n,v in expected.items()}
    if payloads['cpp']!=payloads['rust']:raise RuntimeError(f'{name}/{label}: scalar parity mismatch')
    filename=f'{name}-{label}';(folder/f'{filename}.input').write_bytes(data)
    for backend,payload in payloads.items():(folder/f'{filename}.{backend}.output').write_bytes(payload)
    for baseline in ['cpp','cpp-simd']:
     probe=bytearray(data);struct.pack_into('<I',probe,44,1);t=clients[baseline].run(probe)[1][0];repeats=min(2000000,max(1,math.ceil(0.008/max(t,1e-9))));struct.pack_into('<I',probe,44,repeats);struct.pack_into('<I',probe,4,struct.unpack_from('<I',probe,4)[0]|0x20000)
     samples=[]
     for i in range(30):
      order=[baseline,'rust'] if i%2==0 else ['rust',baseline];times={};before=os.getloadavg()
      for backend in order:
       for attempt in range(3):
        elapsed=clients[backend].run(probe)[1][0]
        if elapsed>0:break
       if elapsed<=0:raise RuntimeError('nonpositive timing')
       times[backend]=elapsed
      samples.append({'seconds':times,'ratio':times['rust']/times[baseline],'order':order,'load_before':before,'load_after':os.getloadavg()})
      # D78: at least twenty pairs per case; thirty when quartiles cross a bar.
      if i==19:
       qs=statistics.quantiles([v['ratio'] for v in samples],n=4)
       if not (qs[0]<=1.25<=qs[2] or qs[0]<=1.5<=qs[2]):break
     cpp=statistics.median(s['seconds'][baseline] for s in samples);rust=statistics.median(s['seconds']['rust'] for s in samples);ratios=[s['ratio'] for s in samples];ratio=statistics.median(ratios);mem_cpp=expected[baseline][1][1];mem_rust=expected['rust'][1][1];memory_ratio=mem_rust/mem_cpp if mem_cpp else (1. if mem_rust==0 else math.inf)
     row={'family':name,'case':label,'baseline':baseline,'repeats':repeats,'samples':samples,'cpp_seconds':cpp,'rust_seconds':rust,'ratio':ratio,'dispersion':summary(ratios),'memory_cpp':mem_cpp,'memory_rust':mem_rust,'memory_ratio':memory_ratio,'scalar_identity':True,'optimized_output_identity':payloads['cpp-simd']==payloads['rust'],'input_sha256':r.sha(folder/f'{filename}.input'),'outputs':{backend:r.sha(folder/f'{filename}.{backend}.output') for backend in payloads}};rows.append(row);save();print(name,label,baseline,round(ratio,4),round(memory_ratio,4),flush=True)
  finished=True;save()
  # D82: the registered 0.3 gate uses scalar C++; SIMD C++ remains a
  # published production comparison, not an acceptance baseline.
  if not record['source_unchanged'] or (a.enforce and (a.quick or a.families or any(not v['pass'] for v in record['verdicts']['cpp'].values()))):raise SystemExit('benchmark bar failed')
 finally:
  save()
  for c in clients.values():c.close()
if __name__=='__main__':main()
