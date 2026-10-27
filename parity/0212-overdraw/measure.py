from pathlib import Path
import subprocess,os,json,sys,hashlib,statistics,types
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
RES=Path(os.environ['MESHOPT_RESCREEN_SCRATCH'])
PHASE=sys.argv[1]
ARMS=['base','A','B','AB','C','M7','X'] if PHASE=='attribution' else ['base','D','E','F','G','H','I','M7','X'] if PHASE=='predicate' else ['base','I','J','M7','X'] if PHASE=='block' else ['base','candidate'] if PHASE=='packed-pilot' else ['base','candidate','M7','X']
assert os.environ.get('MESHOPT_ADMISSION_ID')
PERF=os.environ['MESHOPT_PERF']
assert subprocess.check_output([PERF,'--version'],text=True).startswith('perf version 6.17')
CPU=int(os.environ.get('MESHOPT_CPU', '9'))
assert CPU in os.sched_getaffinity(0)
# Reuse only frozen validators; no geometry generation, calibration or timing code runs.
for name in ['runner','performance','quiet']:sys.modules[name]=types.ModuleType(name)
sys.path.insert(0,str(RES/'parity/compare'))
import measure as m
m.ART=A
CASES=[v[0] for v in m.CASES]
(A/PHASE).mkdir(parents=True,exist_ok=True)
launcher=Path(__file__).read_bytes()
launcher_sha=hashlib.sha256(launcher).hexdigest()
launcher_copy=A/PHASE/'launcher.py'
if launcher_copy.exists():assert launcher_copy.read_bytes()==launcher
else:launcher_copy.write_bytes(launcher)
(A/PHASE/'samples').mkdir(parents=True,exist_ok=True);(A/PHASE/'outputs').mkdir(exist_ok=True)
RAW=A/PHASE/'raw.jsonl'
existing=[json.loads(s) for s in RAW.read_text().splitlines()] if RAW.exists() else []
keys={(r['variant'],r['case'],r['operation'],r['arm'],r['sample']) for r in existing}
records={}
for r in existing:records.setdefault((r['variant'],r['case'],r['operation'],r['arm']),[]).append(r)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def inputpath(case,op):return RES/'inputs'/f'{case}{"-"+op if case.startswith("real-") else "-strip" if op=="unstripify" else ""}.bin'
def cross(backend,profile,op,path,suffix=''):
 arm='base' if backend=='ours' else 'M7'
 dst=A/PHASE/'outputs'/f'cross-{arm}-{op}.bin'
 r=subprocess.run([str(A/f'{PHASE}-driver-{arm}'),op,str(path),str(dst)],input='stop\n',text=True,capture_output=True,timeout=180)
 assert r.returncode==0,(r.returncode,r.stderr)
 data=dst.read_bytes();dst.unlink();return data
m.execute=cross

def sample(variant,case,op,arm,rep):
 key=(variant,case,op,arm,rep)
 if key in keys:return
 path=inputpath(case,op)
 fast=op.endswith(('-bound','-version')) or op in ['dequantize-half','optimize-meshlet','meshlet-bounds','cluster-bounds']
 n=10000 if fast or case=='tiny-smooth' else 1 if case.startswith('real-') or case=='million-smooth' else 10
 prefix=A/PHASE/'samples'/f'{variant}-{case}-{op}-{arm}-{rep}'
 assert not prefix.with_suffix('.perf').exists(),'incomplete attempt retained; use explicit disposition'
 dst=A/PHASE/'outputs'/f'{variant}-{case}-{op}-{arm}-{rep}.bin'
 cr,cw=os.pipe();ar,aw=os.pipe()
 cmd=[PERF,'stat','-e','instructions:u','-x',';','-D','-1','--control',f'fd:{cr},{aw}','-o',str(prefix.with_suffix('.perf')),'--','taskset','-c',str(CPU),str(A/f'{PHASE}-driver-{arm}'),op,str(path),str(dst)]
 try:
  r=subprocess.run(cmd,env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),input=f'{n}\nstop\n',pass_fds=(cr,cw,ar,aw),capture_output=True,text=True,timeout=240)
 finally:
  for fd in [cr,cw,ar,aw]:os.close(fd)
 prefix.with_suffix('.stderr').write_text(r.stderr)
 prefix.with_suffix('.stdout').write_text(r.stdout)
 receipt={'launcher_sha256':launcher_sha,'variant':variant,'case':case,'operation':op,'arm':arm,'sample':rep,'repetitions':n,'exit':r.returncode,'cpu':CPU,'command':cmd,'admission_pid':os.environ['MESHOPT_ADMISSION_ID']}
 prefix.with_suffix('.json').write_text(json.dumps(receipt,indent=2)+'\n')
 assert r.returncode==0,(receipt,r.stderr)
 assert r.stdout=='READY\nCOUNTED\n',r.stdout
 line=next(s for s in prefix.with_suffix('.perf').read_text().splitlines() if ';instructions:u;' in s)
 fields=line.split(';');count=int(fields[0]);running=float(fields[4])
 assert count>0 and running>=99,(line,count,running)
 output_sha=sha(dst);cell=(variant,case,op,arm)
 previous=records.get(cell,[])
 if previous:assert previous[0]['output_sha256']==output_sha,'unstable output'
 valid=previous[0]['validation'] if previous else m.valid(op,dst.read_bytes(),path,'theirs' if arm in ['M7','X'] else 'ours',os.environ['MESHOPT_PROFILE'])
 receipt.update(instructions=count,instructions_per_call=count/n,running_percent=running,output_sha256=output_sha,output_bytes=dst.stat().st_size,validation=valid,input_sha256=sha(path),binary_sha256=sha(A/f'{PHASE}-driver-{arm}'))
 prefix.with_suffix('.json').write_text(json.dumps(receipt,indent=2)+'\n')
 with RAW.open('a') as f:f.write(json.dumps(receipt)+'\n')
 keys.add(key);records.setdefault(cell,[]).append(receipt)


for case in (['tiny-smooth','million-smooth'] if PHASE=='packed-pilot' else ['real-214e','real-b6fd','real-a082'] if PHASE in ['attribution','predicate','block'] else CASES+['real-214e','real-b6fd','real-a082']):
 for op in (['overdraw'] if case.startswith('real-') or PHASE in ['attribution','predicate','block','packed-pilot'] else ['overdraw','analyze-overdraw']):
  for rep in range(3):
   order=ARMS[rep:]+ARMS[:rep]
   for arm in order:sample('fair',case,op,arm,rep)
  med={arm:statistics.median(r['instructions_per_call'] for r in records['fair',case,op,arm]) for arm in ARMS}
  for arm in ARMS:
   assert len({r['output_sha256'] for r in records['fair',case,op,arm]})==1
   if arm not in ['M7','X']:assert records['fair',case,op,arm][0]['output_sha256']==records['fair',case,op,'base'][0]['output_sha256']
  if PHASE=='packed-pilot':assert med['candidate']/med['base']<=1.01, ('packed precheck failed',case,med)
  print(PHASE,case,op,{arm:round(med[arm]/med['base'],6) for arm in ARMS},flush=True)
(A/PHASE/'measurement-exit.json').write_text(json.dumps({'exit':0,'samples':len(keys),'admission':os.environ['MESHOPT_ADMISSION_ID']},indent=2)+'\n')
