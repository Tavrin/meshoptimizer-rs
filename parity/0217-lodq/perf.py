"""Four-arm simplify regression guard; resident counted loop, three repetitions."""
import hashlib,io,json,os,shutil,statistics,subprocess,tarfile
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
r=Path.cwd();lane=r/'parity/0217-lodq';out=lane/'receipts';t=Path(os.environ['CARGO_TARGET_DIR']);s=t/'perf';s.mkdir(exist_ok=True)
perf=os.environ['LOD_PERF'];source=Path(os.environ['LOD_INPUTS'])
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(name,cmd,env=None):
 with (out/(name+'.log')).open('w') as f:result=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,env=env)
 with (out/'exits.tsv').open('a') as f:f.write(f'{name}\t{result.returncode}\n')
 assert result.returncode==0,(name,(out/(name+'.log')).read_text()[-3000:])
 print(name,'built',flush=True)
binding={};arms=['base','candidate','cpp-scalar','cpp-default']
for arm in arms:
 tree=s/arm;tree.mkdir(exist_ok=True)
 archive=subprocess.check_output(['git','archive','v0.2.16','src','docs','Cargo.toml','Cargo.lock','README.md'])
 tarfile.open(fileobj=io.BytesIO(archive)).extractall(tree,filter='data')
 if arm=='candidate':
  shutil.copytree(r/'src',tree/'src',dirs_exist_ok=True)
  shutil.copy2(r/'Cargo.toml',tree/'Cargo.toml')
 peers=tree/'parity/peers';shutil.copytree(r/'parity/peers',peers,dirs_exist_ok=True)
 if arm=='candidate':
  p=peers/'Cargo.toml';p.write_text(p.read_text().replace('features = ["clusterlod"]','features = ["clusterlod", "lod-quality"]'))
 env=dict(os.environ,CARGO_TARGET_DIR=str(t/('perf-'+arm)),CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD' if arm=='cpp-scalar' else '')
 run('perf-build-'+arm,['cargo','build','--offline','--locked','--release','--manifest-path',str(peers/'Cargo.toml'),'--features','cpp' if arm.startswith('cpp') else 'ours'],env)
 b=s/('driver-'+arm);shutil.copy2(t/('perf-'+arm)/'release/meshopt-peers',b)
 binding[arm]={'binary':sha(b),'sources':{str(p.relative_to(tree)):sha(p) for p in [*tree.glob('src/**/*.rs'),tree/'Cargo.toml',peers/'Cargo.toml',peers/'src/main.rs']}}
(out/'perf-binding.json').write_text(json.dumps(binding,indent=2)+'\n')
inputs=json.loads((r/'parity/peers/inputs.json').read_text())
for x in inputs:assert sha(source/(x['case']+'.bin'))==x['sha256'],x['case']
(out/'inputs.json').write_text(json.dumps(inputs,indent=2)+'\n')
cpu=min(os.sched_getaffinity(0));rows=[];samples=out/'samples';samples.mkdir(exist_ok=True)
for x in inputs:
 case=x['case'];outputs=set()
 for rep in range(3):
  for arm in (arms if rep!=1 else list(reversed(arms))):
   name=f'{case}-{arm}-{rep}';prefix=samples/name;output=s/(name+'.bin');n=100 if case=='tiny-smooth' else 1
   cr,cw=os.pipe();ar,aw=os.pipe()
   try:
    p=subprocess.run([perf,'stat','-e','instructions:u','-x',';','-o',str(prefix)+'.perf','-D','-1','--control',f'fd:{cr},{aw}','--','taskset','-c',str(cpu),str(s/('driver-'+arm)),'simplify',str(source/(case+'.bin')),str(output)],
       input=f'{n}\nstop\n',env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar),LC_ALL='C'),pass_fds=(cr,cw,ar,aw),capture_output=True,text=True)
   finally:
    for fd in [cr,cw,ar,aw]:os.close(fd)
   Path(str(prefix)+'.log').write_text(p.stdout+p.stderr);assert p.returncode==0,(name,p.stderr)
   fields=next(line.split(';') for line in Path(str(prefix)+'.perf').read_text().splitlines() if ';instructions:u;' in line)
   assert float(fields[4])==100.,fields
   count=int(fields[0]);seconds=float(next(v for v in p.stdout.splitlines() if v.startswith('COUNTED ')).split()[1])
   h=sha(output);outputs.add(h);output.unlink()
   row=dict(case=case,arm=arm,rep=rep,repetitions=n,instructions=count/n,seconds=seconds/n,output_sha256=h)
   rows.append(row)
   with (out/'counts.jsonl').open('a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
 assert len(outputs)==1,(case,outputs)
 print(case,'four arms byte-identical',flush=True)
summary=[]
for x in inputs:
 case=x['case'];med={a:statistics.median(v['instructions'] for v in rows if v['case']==case and v['arm']==a) for a in arms}
 wall={a:statistics.median(v['seconds'] for v in rows if v['case']==case and v['arm']==a) for a in arms}
 summary.append(dict(case=case,instructions=med,wall_seconds=wall,candidate_over_base=med['candidate']/med['base'],candidate_over_cpp_scalar=med['candidate']/med['cpp-scalar'],candidate_over_cpp_default=med['candidate']/med['cpp-default']))
(out/'perf-results.json').write_text(json.dumps(dict(cpu=cpu,event='instructions:u',repetitions=3,results=summary),indent=2)+'\n')
with (out/'exits.tsv').open('a') as f:f.write('perf-measure\t0\n')
print(json.dumps(summary,indent=2),flush=True)
