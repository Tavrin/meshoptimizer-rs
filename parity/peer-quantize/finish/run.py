from pathlib import Path
import subprocess,os,json,hashlib,shutil
s=Path(__file__).resolve().parent
assert os.environ['MOSS_HEAVY_ACTIVE']
rows={}
def run(name,cmd):
 with (s/(name+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 rows[name]=r.returncode;(s/'exits.json').write_text(json.dumps(rows,indent=2));assert r.returncode==0,name
run('test',['cargo','test','--all-features'])
run('clippy',['cargo','clippy','--all-targets','--all-features','--','-D','warnings'])
run('fmt',['cargo','fmt','--all','--','--check'])
run('build-diagnostic',['python3',str(s/'build.py'),'B','O','X','M7'])
run('build-proof',['bash',str(s/'build-proof.sh')])
run('exact',['python3','parity/peer-quantize/proof.py'])
run('direct',['python3','parity/peer-quantize/direct.py'])
shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release/peer-direct',s/'peer-direct')
results=[]
for op in ['quantize-half','quantize-float']:
 for arm in ['B','O','X','M7']:
  log=s/f'instructions-{op}-{arm}.txt';output=s/f'output-{op}-{arm}.bin'
  cmd=['/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf','stat','-x,','-e','instructions:u','-o',str(log),'taskset','-c',str(min(os.sched_getaffinity(0))),str(s/f'driver-{arm}'),op,str(s/'inputs/million-smooth.bin'),str(output)]
  r=subprocess.run(cmd,input='100\nstop\n',capture_output=True,text=True)
  (s/f'driver-{op}-{arm}.log').write_text(r.stdout+r.stderr)
  assert r.returncode==0,(op,arm,r.stderr)
  count=int(next(l.split(',')[0] for l in log.read_text().splitlines() if ',instructions:u,' in l))
  results.append(dict(operation=op,arm=arm,instructions=count,exit=r.returncode,binary_sha256=hashlib.sha256((s/f'driver-{arm}').read_bytes()).hexdigest(),output_sha256=hashlib.sha256(output.read_bytes()).hexdigest()))
  print(op,arm,count,flush=True)
(s/'fair-adapter.json').write_text(json.dumps(dict(rows=results,input_sha256=hashlib.sha256((s/'inputs/million-smooth.bin').read_bytes()).hexdigest()),indent=2)+'\n')
