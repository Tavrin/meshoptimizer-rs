#!/usr/bin/env python3
"""P03 tiny: immutable diagnosed epochs and registered 5-20-pair streams."""
import hashlib,json,math,os,shutil,statistics,struct,subprocess,sys,time
from pathlib import Path
import run as lane
T95={5:2.776,6:2.571,7:2.447,8:2.365,9:2.306,10:2.262,11:2.228,12:2.201,13:2.179,14:2.160,15:2.145,16:2.131,17:2.120,18:2.110,19:2.101,20:2.093}
def interval(xs):
 logs=[math.log(x) for x in xs];mean=statistics.mean(logs);r=T95[len(xs)]*statistics.stdev(logs)/math.sqrt(len(xs));return [math.exp(mean-r),math.exp(mean+r)]
OLD=lane.ART
A=Path('/mnt/linux-extra/moss-scratch/meshopt-v03-tiny')
TARGET=Path('/mnt/linux-extra/moss-cargo-targets/codex-meshopt-tiny')
lane.ART=A;lane.TARGET=TARGET
lane.ENV.update(CARGO_TARGET_DIR=str(TARGET),TMPDIR=str(TARGET/'tmp'))
os.environ.update(CARGO_TARGET_DIR=str(TARGET))
def save(name,obj):lane.save(name,obj)
def sources():return {str(p.relative_to(lane.ROOT)):lane.sha(p) for p in (lane.ROOT/'src').rglob('*.rs')}
def prepare():
 A.mkdir(exist_ok=True);TARGET.mkdir(exist_ok=True);(TARGET/'tmp').mkdir(exist_ok=True)
 m=json.loads((OLD/'build-after.json').read_text());assert m['source']==sources();assert m['binary']==lane.sha(OLD/'rust-after')
 for src,dst in [('rust-after','rust-before'),('cpp','cpp'),('cpp-scalar','cpp-scalar'),('crate','crate'),('inputs.json','inputs.json')]:shutil.copy2(OLD/src,A/dst)
 assert lane.sha(A/'cpp')==json.loads((OLD/'timing.json').read_text())['binaries']['cpp']
 save('build-before',dict(m,consumer={str(p.relative_to(lane.ROOT)):lane.sha(p) for p in (lane.ROOT/'parity/codec').rglob('*') if p.is_file() and '__pycache__' not in p.parts}))
 (A/'SCOUT.md').write_text('Tiny path: src/codec/encode.rs:319 layout; :369 caller accounting; :390 allocating accounting. src/codec/filter_encode.rs:92 Oct4-lane prologue and scalar tail; :187 Quat4-lane prologue and scalar tail. Frozen tiny inputs contain 17 records: four vector groups plus one scalar tail. No elapsed campaign yet.\n')
 save('registration',{'baseline_revision':'a14cbc1','target':str(TARGET),'before_source':sources(),'cpp_pin':'4c203430ca565cb59a468a91922c76c208169536','pairs':[5,20],'batch_seconds':.008,'max_upper95':1.5,'aa_bounds':[.8,1.25],'cpu':26,'scope':'four tiny rows first, then full Oct/Quat confirmation; no completed stream repeated'})
def counters(epoch):
 assert os.environ.get('MOSS_HEAVY_ACTIVE')
 rows=[r for r in json.loads((A/'inputs.json').read_text()) if r['name'] in ['oct-tiny-s8','quat-tiny']];records=[]
 setup=os.environ.get('MESHOPT_TINY_SETUP')=='1'
 if setup:
  expanded=[]
  for row in rows:
   for n in [0,1]:
    b=bytearray(Path(row['path']).read_bytes()[:44+n*16]);struct.pack_into('<I',b,8,n);struct.pack_into('<I',b,40,n*16)
    p=A/(row['name']+'-setup-'+str(n)+'.input');p.write_bytes(b)
    expanded.append(dict(row,path=str(p),count=n,name=row['name']+'-n'+str(n)))
  rows=expanded
 for row in rows:
  for into in [False,True]:
   for backend in ['cpp','rust-'+epoch]:
    totals=[]
    for it in [10000,20000]:
     b=bytearray(Path(row['path']).read_bytes());struct.pack_into('<II',b,32,1,it)
     if into:struct.pack_into('<I',b,16,128)
     key=f'{epoch}-{row["name"]}-{into}-{backend}-{it}';stat=A/(key+'.stat')
     p=subprocess.run([str(lane.PERF),'stat','-x',';','-e','instructions:u,cycles:u,branches:u,branch-misses:u','-o',str(stat),'--','taskset','-c','26',str(A/backend)],input=struct.pack('<I',len(b))+b,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=lane.ENV)
     assert p.returncode==0,p.stderr.decode();(A/(key+'.response')).write_bytes(p.stdout);assert lane.decode(p.stdout[4:])[0]==0
     counts={}
     for line in stat.read_text().splitlines():
      v=line.split(';')
      if len(v)>2 and v[2].endswith(':u'):counts[v[2]]=float(v[0]);assert float(v[4])>=99,v
     assert len(counts)==4;totals.append(counts)
    records.append({'case':row['name'],'api':'caller' if into else 'allocating','backend':backend,'per_call':{k:(totals[1][k]-totals[0][k])/10000 for k in totals[0]}})
 save('counters-'+epoch+('-setup' if setup else ''),{'rows':records,'binaries':{k:lane.sha(A/k) for k in ['cpp','rust-'+epoch]},'timing_assessed':False})
 with (A/('assembly-'+epoch+'.txt')).open('w') as f:subprocess.run(['objdump','-d','-C',str(A/('rust-'+epoch))],stdout=f,check=True)
 with (A/'assembly-cpp.txt').open('w') as f:subprocess.run(['objdump','-d','-C',str(A/'cpp')],stdout=f,check=True)
 print(json.dumps(records,indent=2),flush=True)
def measure(stage):
 assert os.environ.get('MOSS_HEAVY_ACTIVE');assert sources()==json.loads((A/'build-after.json').read_text())['source']
 path=A/('timing-'+stage+'.json');assert not path.exists(),'immutable stream'
 cases=[r for r in json.loads((A/'inputs.json').read_text()) if r['name'] in ['oct-tiny-s8','quat-tiny']] if stage.startswith('tiny') else [r for r in json.loads((A/'inputs.json').read_text()) if r['name'].startswith(('oct-','quat-'))]
 ds={k:lane.Driver(A/v) for k,v in {'before':'rust-before','after':'rust-after','cpp':'cpp','rust-aa':'rust-after','cpp-aa':'cpp'}.items()}
 result={'stage':stage,'policy_sha256':lane.sha(Path(__file__)),'source':sources(),'binaries':{k:lane.sha(A/v) for k,v in {'before':'rust-before','after':'rust-after','cpp':'cpp'}.items()},'rows':[],'completed':False,'admission':os.environ['MOSS_HEAVY_ACTIVE']};start=time.monotonic()
 for row in cases:
  for api in [0,1]:
   b=bytearray(Path(row['path']).read_bytes());assert hashlib.sha256(b).hexdigest()==row['sha256']
   if api:struct.pack_into('<I',b,16,128)
   outputs={k:d.call(b)[:2] for k,d in ds.items()};assert all(v==outputs['after'] for v in outputs.values()) and outputs['after'][0]==0
   probe=bytearray(b);struct.pack_into('<II',probe,32,1,1);trial=ds['cpp'].call(probe)[2][0];it=max(1,min(1000000,math.ceil(.008/max(trial,1e-9))));struct.pack_into('<I',probe,36,it)
   samples={k:[] for k in ds}
   for i in range(20):
    keys=list(ds);keys=keys[i%len(keys):]+keys[:i%len(keys)]
    if i%2:keys=keys[::-1]
    for k in keys:
     st,out,ts=ds[k].call(probe);assert (st,out)==outputs[k];samples[k].append(ts[0])
    if i>=4:
     ci=interval([x/y for x,y in zip(samples['after'],samples['cpp'])]);aa=interval([x/y for x,y in zip(samples['rust-aa'],samples['after'])]);ca=interval([x/y for x,y in zip(samples['cpp-aa'],samples['cpp'])])
     if (ci[1]<=1.5 or ci[0]>1.5) and aa[0]>=.8 and aa[1]<=1.25 and ca[0]>=.8 and ca[1]<=1.25:break
   r={'case':row['name'],'api':'caller' if api else 'allocating','pairs':i+1,'iterations':it,'samples':samples,'before_cpp':statistics.median(x/y for x,y in zip(samples['before'],samples['cpp'])),'after_cpp':statistics.median(x/y for x,y in zip(samples['after'],samples['cpp'])),'after_cpp_ci':ci,'rust_aa_ci':aa,'cpp_aa_ci':ca,'pass':ci[1]<=1.5 and aa[0]>=.8 and aa[1]<=1.25 and ca[0]>=.8 and ca[1]<=1.25,'input_sha256':row['sha256'],'output_sha256':hashlib.sha256(outputs['after'][1]).hexdigest()}
   result['rows'].append(r);save('timing-'+stage,result);print(row['name'],r['api'],r['after_cpp'],ci,r['pass'],flush=True)
   assert time.monotonic()-start<690
 for d in ds.values():d.close()
 result.update(completed=True,seconds=time.monotonic()-start);save('timing-'+stage,result)
def verify():
 assert os.environ.get('MOSS_HEAVY_ACTIVE')
 before=sources();results=[]
 commands=[['cargo','test','--offline','--locked'],['cargo','test','--offline','--locked','--no-default-features','--test','codec'],['cargo','clippy','--offline','--locked','--lib','--all-features','--','-D','warnings'],['cargo','fmt','--all','--','--check'],[str(lane.ROOT/'parity/check-reference.sh')]]
 for i,cmd in enumerate(commands):
  log=A/('verify-'+str(i)+'.log')
  with log.open('w') as f:p=subprocess.run(cmd,cwd=lane.ROOT,env=lane.ENV,stdout=f,stderr=subprocess.STDOUT)
  results.append({'command':cmd,'exit_code':p.returncode,'log':str(log),'sha256':lane.sha(log)})
  save('verification',{'source':before,'commands':results});print(cmd,'exit',p.returncode,flush=True)
  assert p.returncode==0,log.read_text()[-6000:]
 assert sources()==before

def freeze():
 m=json.loads((A/('build-'+os.environ.get('MESHOPT_TINY_FINAL','specialized')+'.json')).read_text());assert m['source']==sources()
 v=json.loads((A/'verification.json').read_text());assert v['source']==sources() and all(c['exit_code']==0 for c in v['commands'])
 assert m['binary']==lane.sha(A/('rust-'+os.environ.get('MESHOPT_TINY_FINAL','specialized')))
 consumer={str(p.relative_to(lane.ROOT)):lane.sha(p) for p in (lane.ROOT/'parity/codec').rglob('*') if p.is_file() and '__pycache__' not in p.parts}
 assert consumer==json.loads((A/'build-before.json').read_text())['consumer']
 shutil.copy2(A/('rust-'+os.environ.get('MESHOPT_TINY_FINAL','specialized')),A/'rust-after');save('build-after',dict(m,consumer=consumer))
 for file in [*lane.ROOT.joinpath('src').rglob('*.rs'),lane.ROOT/'Cargo.toml',lane.ROOT/'Cargo.lock',Path(__file__)]:
  dst=A/'source-final'/file.relative_to(lane.ROOT);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(file,dst)
 print('frozen',m['binary'],flush=True)

if __name__=='__main__':
 action=sys.argv[1];arg=sys.argv[2] if len(sys.argv)>2 else ''
 {'prepare':prepare,'verify':verify,'freeze':freeze,'counters':lambda:counters(arg),'build':lambda:lane.build(arg),'parity':lambda:lane.parity(arg),'measure':lambda:measure(arg)}[action]()
