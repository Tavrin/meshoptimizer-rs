from pathlib import Path
import hashlib,json,os,statistics,subprocess,struct,select,math
S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-H2');O=S/'meshopt-028-L2';OUT=S/os.environ.get('H2_SAMPLES','samples');OUT.mkdir(exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_MEASUREMENT')=='1'
PERF='/usr/lib/linux-tools/6.17.0-22-generic/perf'
ARMS=['base','candidate','M7'];FAMILIES=['simplify-scale','cache','cache-fifo','overdraw','partition','meshlets-spatial']
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def save(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def line(f):
 assert select.select([f],[],[],180)[0], 'protocol timeout'
 return f.readline().strip('\x00\n\r ')
cases=json.loads((S/'cases.json').read_text())
if os.environ.get('H2_PILOT'): cases=[c for c in cases if c['id'] in ['frozen-simplify-scale-tiny-smooth','frozen-simplify-scale-medium-smooth','frozen-overdraw-tiny-smooth','frozen-overdraw-medium-smooth','realfixture-overdraw-214e']]

referenced={}
for case in cases:
 assert sha(case['source'])==case['source_sha256'],case['id']
 referenced[case['source']]=sha(case['source'])
 if case['kind']=='chain':
  fields=Path(case['source']).read_text().splitlines()[case['index']+1].split('\t')
  assert sha(fields[6])==case['vertex_sha256']
  referenced[fields[6]]=sha(fields[6])
  if fields[1]!='-':
   assert sha(fields[1])==case['index_sha256']
   referenced[fields[1]]=sha(fields[1])
source={str(p):sha(p) for p in Path.cwd().joinpath('src').rglob('*.rs')}
save(OUT/'binding.json',dict(sources=source,inputs=referenced,binaries={a:sha(S/f'driver-{a}') for a in ARMS},cases_sha256=sha(S/'cases.json'),admission=os.environ['MOSS_HEAVY_ACTIVE'],perf=subprocess.check_output([PERF,'--version'],text=True).strip()))
rows=[];summary=[];exits=[]
for case in cases:
 baseline=None
 for rep in range(3):
  order=ARMS[rep:]+ARMS[:rep]
  for arm in order:
   stem=OUT/f"{case['id']}-{rep}-{arm}";assert not stem.with_suffix('.json').exists(),str(stem)
   cr,cw=os.pipe();ar,aw=os.pipe();output=stem.with_suffix('.bin')
   args=case['args']+[str(output)]+([str(case['index'])] if 'index' in case else [])
   cmd=[PERF,'stat','-e','instructions:u','-x',',','-o',str(stem.with_suffix('.perf')),'--delay=-1',f'--control=fd:{cr},{aw}','--','taskset','-c','9',str(S/f'driver-{arm}'),*args]
   with stem.with_suffix('.stderr').open('w') as err:
    p=subprocess.Popen(cmd,pass_fds=(cr,aw),stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=err,text=True,bufsize=1)
    os.close(cr);os.close(aw)
    try:
     ready=line(p.stdout);assert ready=='READY',(case['id'],arm,ready,stem.with_suffix('.stderr').read_text())
     digest=sha(output)
     if arm=='base' and baseline is None:baseline=digest
     assert digest==baseline,(case['id'],arm,'output mismatch',digest,baseline)
     if 'expected' in case:assert digest==case['expected'],(case['id'],arm,'historical mismatch',digest,case['expected'])
     with os.fdopen(cw,'w',buffering=1) as ctl,os.fdopen(ar,'r',buffering=1) as ack:
      ctl.write('enable\n');assert line(ack)=='ack'
      p.stdin.write(f"count {case['iterations']}\n");p.stdin.flush();assert line(p.stdout)=='COUNTED'
      ctl.write('disable\n');assert line(ack)=='ack'
     p.stdin.write('stop\n');p.stdin.flush();p.communicate(timeout=180);assert p.returncode==0
    except:
     p.kill();p.wait();raise
   exits.append(dict(case=case['id'],arm=arm,rep=rep,exit=p.returncode))
   fields=next(l.split(',') for l in stem.with_suffix('.perf').read_text().splitlines() if ',instructions:u,' in l);n=int(fields[0]);assert float(fields[4])>=99.0,fields
   r=dict(case=case['id'],family=case['family'],arm=arm,rep=rep,iterations=case['iterations'],instructions=n,per_call=n/case['iterations'],output_sha256=digest,cpp_equal=digest==baseline,exit=p.returncode,counter_running_percent=float(fields[4]));rows.append(r);save(stem.with_suffix('.json'),r);save(OUT/'raw.json',rows);save(OUT/'measurement-exits.json',exits)
 med={a:statistics.median(r['per_call'] for r in rows if r['case']==case['id'] and r['arm']==a) for a in ARMS};ratio={a:med[a]/med['M7'] for a in ARMS[:-1]};gap=med['base']-med['M7'];shares={};summary.append(dict(**case,median=med,ratio=ratio,gap_shares=shares));save(OUT/'summary.json',summary)
 print(case['id'],{a:round(v,5) for a,v in ratio.items()},flush=True)
assert source=={str(p):sha(p) for p in Path.cwd().joinpath('src').rglob('*.rs')}
print('COMPLETE',len(cases),len(rows),flush=True)
