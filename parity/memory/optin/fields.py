from pathlib import Path
import os,subprocess,json,time,hashlib,re,tomllib
s=Path(__file__).parent
os.sched_setaffinity(0,{8})
perf='/usr/lib/linux-tools/6.17.0-22-generic/perf'
fixtures={k:next(f['id'] for f in tomllib.loads((s/'corpus.toml').read_text())['fixture'] if f['id'].startswith('jungle-scoreboard-'+k)) for k in ['214e','b6fd','a082']}
fixtures['micro']=fixtures['214e']
root=s/'fields-measurements';root.mkdir();pool=root/'blobs';pool.mkdir()
sha=lambda p:hashlib.file_digest(p.open('rb'),'sha256').hexdigest()
refs={};rows=[];observations={}
baseenv=dict(os.environ)
for k in ['MESHOPT_LEAN','MESHOPT_REPEAT_SIX','MESHOPT_DIAG_RELEASE_MESH','MESHOPT_DIAG_RELEASE_CALL']:baseenv.pop(k,None)
def emit(o):
 with (root/'execution.jsonl').open('a') as f:f.write(json.dumps(o)+'\n')
 print(json.dumps(o),flush=True)
def run(name,case,arm,probe=False,lean=False,instructions=False):
 d=root/name;d.mkdir();binary=s/'bin'/('probe-'+arm if probe else arm)
 env=dict(baseenv)
 if lean:env['MESHOPT_LEAN']='1'
 if case=='micro':env['MESHOPT_REPEAT_SIX']='1'
 cmd=[str(binary),'--manifest',str(s/'corpus.toml'),'--fixtures',fixtures[case],'--output',str(d/'result.json')]
 if instructions:cmd=[perf,'stat','-x',';','-e','instructions:u','-o',str(d/'perf.txt'),'--']+cmd
 if lean:cmd=['/usr/bin/time','-v','-o',str(d/'rss.txt')]+cmd
 emit({'event':'start','name':name,'binary_sha256':sha(binary),'utc':time.time(),'command':cmd,'env':{k:env[k] for k in ['MESHOPT_LEAN','MESHOPT_REPEAT_SIX'] if k in env}})
 with (d/'stdout.log').open('w') as out,(d/'stderr.log').open('w') as err:p=subprocess.run(cmd,env=env,stdout=out,stderr=err)
 (d/'exit.json').write_text(json.dumps({'rc':p.returncode})+'\n')
 assert p.returncode==0,(name,p.returncode)
 if case=='micro':
  doc=json.loads((d/'result.json').read_text());doc.pop('ns');assert len(doc['calls'])==6 and all('error' not in c for c in doc['calls'])
 else:
  doc=json.loads((d/'result'/f'{fixtures[case]}.json').read_text());doc.pop('instrumented_processing_ns');assert doc['status']=='ok' and doc['levels'] and doc['legacy_levels']
 if case in refs:assert refs[case]==doc,('mismatch',name)
 else:refs[case]=doc
 # Verify all actual sidecar bytes, then hardlink identical files outside counters.
 for b in (d/'result').glob('*.blobs/*.bin'):
  digest=sha(b);assert b.name==digest+'.bin'
  canon=pool/b.name
  if not canon.exists():os.link(b,canon)
  else:
   assert b.stat().st_size==canon.stat().st_size
   b.unlink();os.link(canon,b)
 receipt={'event':'finish','name':name,'rc':p.returncode,'exact':True,'stable_sha256':hashlib.sha256(json.dumps(doc,sort_keys=True,separators=(',',':')).encode()).hexdigest(),'utc':time.time()}
 if instructions:
  txt=(d/'perf.txt').read_text();line=next(l for l in txt.splitlines() if ';instructions:u;' in l);receipt['instructions']=int(line.split(';')[0]);assert receipt['instructions']>0
  # perf must have counted the entire process without multiplexing.
  receipt['counter_line']=line
 if probe:
  points=[]
  for line in (d/'stderr.log').read_text().splitlines():
   if not line.startswith('HARNESS_DIAG '):continue
   rec=json.loads(line.removeprefix('HARNESS_DIAG '))
   if rec['stage'].startswith('observe:'):points.append({'stage':rec['stage'],**rec['detail']})
  assert points and all('retained' in p for p in points)
  observations[arm]=points
  receipt['observe_points']=len(points)
 if lean:
  receipt['rss_kib']=int(re.search(r'Maximum resident set size \(kbytes\): (\d+)',(d/'rss.txt').read_text())[1]);receipt['rss_gib']=receipt['rss_kib']/2**20;receipt['rss_pass']=receipt['rss_gib']<=2.318
 (d/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');emit(receipt);return receipt
run('observe-A','214e','A',probe=True)
run('observe-B','214e','B',probe=True)
assert observations['A']==observations['B']
assert all(len(p['fields']) > 0 for p in observations['A'] if p['retained'][1] > 0)
(root/'OBSERVATIONS.json').write_text(json.dumps(observations,indent=2)+'\n')
result={'verdict':'PASS','points':len(observations['A']),'capacity_exact':True,'simplify_buffers':max(len(p['fields']) for p in observations['A'])}
(root/'VERDICT.json').write_text(json.dumps(result,indent=2)+'\n');emit(result)
