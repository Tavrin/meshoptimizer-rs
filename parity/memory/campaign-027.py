#!/usr/bin/python3
from pathlib import Path
import os,subprocess,time,json,re,hashlib,math,random,traceback
s=Path(__file__).parent
os.sched_setaffinity(0,{8})
root=s/'registered';root.mkdir(exist_ok=False)
pool=root/'blobs';pool.mkdir()
cases=[('214e','jungle-scoreboard-214e85c568a64e8a-entity'),('b6fd','jungle-scoreboard-b6fdb93d8a1709e2-entity'),('a082','jungle-scoreboard-a082ca1fd19bb86a-entity'),('micro','jungle-scoreboard-214e85c568a64e8a-entity')]
reference={};samples={};rows=[]
sha=lambda p:hashlib.file_digest(p.open('rb'),'sha256').hexdigest()
binaries={a:sha(s/'bin'/a) for a in ['A','B']}

def emit(o):
 with (root/'execution.jsonl').open('a') as f:f.write(json.dumps(o)+'\n')
 print(json.dumps(o),flush=True)

def run(case,fixture,release,pair,label,arm):
 name=f'{case}-{release}-{pair:02}-{label}'
 d=root/name;d.mkdir()
 env=dict(os.environ)
 for k in ['MESHOPT_DIAG_RELEASE_MESH','MESHOPT_DIAG_RELEASE_CALL','MESHOPT_REPEAT_SIX']:env.pop(k,None)
 if case=='micro':env['MESHOPT_REPEAT_SIX']='1'
 cmd=['/usr/bin/time','-v','-o',str(d/'time.txt'),str(s/'bin'/arm),'--manifest',str(s/'corpus.toml'),'--fixtures',fixture,'--output',str(d/'result.json')]
 emit({'event':'start','name':name,'arm':arm,'binary_sha256':binaries[arm],'utc':time.time(),'command':cmd})
 with (d/'stdout.log').open('w') as out,(d/'stderr.log').open('w') as err:
  start=time.monotonic_ns();p=subprocess.run(cmd,stdout=out,stderr=err,env=env);wall=time.monotonic_ns()-start
 (d/'exit.json').write_text(json.dumps({'rc':p.returncode,'wall_ns':wall})+'\n')
 if p.returncode:raise RuntimeError(f'{name}: exit {p.returncode}')
 tm=(d/'time.txt').read_text();rss=int(re.search(r'Maximum resident set size \(kbytes\): (\d+)',tm)[1])
 if case=='micro':
  doc=json.loads((d/'result.json').read_text());wall=int(doc.pop('ns'))
  assert len(doc['calls'])==6 and all('error' not in r for r in doc['calls']),doc
 else:
  doc=json.loads((d/'result'/f'{fixture}.json').read_text());doc.pop('instrumented_processing_ns')
  assert doc['status']=='ok' and doc['levels'] and doc['legacy_levels'],doc['status']
 if case not in reference:reference[case]=doc
 assert reference[case]==doc,f'{name}: output/error mismatch'
 stable=hashlib.sha256(json.dumps(doc,sort_keys=True,separators=(',',':')).encode()).hexdigest()
 # This is outside the measured region. Verify real bytes before deduplicating.
 for b in (d/'result').glob('*.blobs/*.bin'):
  digest=sha(b);assert b.name==digest+'.bin',(b,digest)
  canon=pool/b.name
  if not canon.exists():os.link(b,canon)
  else:
   assert b.stat().st_size==canon.stat().st_size
   b.unlink();os.link(canon,b)
 receipt={'event':'finish','name':name,'case':case,'release':release,'pair':pair,'label':label,'arm':arm,'wall_ns':wall,'rss_kib':rss,'exact':True,'stable_sha256':stable,'utc':time.time()}
 (d/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');emit(receipt)
 return receipt

def upper(ratios,rng):
 logs=[math.log(r) for r in ratios]
 boots=sorted(math.exp(sum(rng.choices(logs,k=10))/10) for _ in range(10000))
 return math.exp(sum(logs)/10),boots[9499]

try:
 for case,fixture in cases:
  samples[case]={}
  for release in ['AA','AB']:
   pairs=[]
   for pair in range(11):
    labels=['A1','A2'] if release=='AA' else ['A','B']
    if pair%2:labels.reverse()
    got={}
    for label in labels:got[label]=run(case,fixture,release,pair,label,'A' if release=='AA' else label)
    if pair:pairs.append(got)
   samples[case][release]=pairs
 rng=random.Random(20261027)
 for case,_ in cases:
  ps=samples[case];aa=[p['A2']['wall_ns']/p['A1']['wall_ns'] for p in ps['AA']];ab=[p['B']['wall_ns']/p['A']['wall_ns'] for p in ps['AB']]
  aa_mean,aa_upper=upper(aa,rng);mean,hi=upper(ab,rng);floor=max(0,aa_upper-1)
  arss=max(p['A']['rss_kib'] for p in ps['AB']);brss=max(p['B']['rss_kib'] for p in ps['AB'])
  rows.append({'case':case,'AA_geomean':aa_mean,'AA_upper95':aa_upper,'F':floor,'AB_geomean':mean,'AB_upper95':hi,'wall_bar':1.02+floor,'wall_pass':hi<=1.02+floor,'A_rss_kib':arss,'B_rss_kib':brss,'A_rss_gib':arss/2**20,'B_rss_gib':brss/2**20,'rss_saved_gib':(arss-brss)/2**20,'exact':True})
 rss=rows[0];memory_pass=rss['B_rss_kib']<=1.10*2209608 and rss['B_rss_gib']<=2.318 and rss['rss_saved_gib']>=.5
 gates=all(l.split('\t')[1]=='0' for l in (s/'GATES.tsv').read_text().splitlines())
 result={'verdict':'PASS' if memory_pass and gates and all(r['wall_pass'] for r in rows) else 'FAIL','memory_pass':memory_pass,'gates_pass':gates,'rows':rows,'bootstrap_seed':20261027,'bootstrap_samples':10000,'reruns':0}
except Exception as e:
 result={'verdict':'FAIL','reason':str(e),'traceback':traceback.format_exc(),'rows':rows,'reruns':0}
(root/'VERDICT.json').write_text(json.dumps(result,indent=2)+'\n');emit(result)
raise SystemExit(0 if result['verdict']=='PASS' else 1)
