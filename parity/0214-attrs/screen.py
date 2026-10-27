"""Resumable diagnostic subset, not the registered full campaign."""
import hashlib, json, os, subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'
a=Path(os.environ['ATTR_ART']); inputs=Path(os.environ['ATTR_INPUTS']); perf=os.environ['ATTR_PERF']
rowsfile=a/'screen.jsonl'; rows=[json.loads(s) for s in rowsfile.read_text().splitlines()] if rowsfile.exists() else []
keys={(r['case'],r['arm'],r['rep']) for r in rows}
(a/'screen').mkdir(exist_ok=True)
for case in ['tiny-smooth','million-smooth','real-214e','real-a082','real-b6fd']:
 for rep in range(3):
  for arm in (['base','candidate'] if rep != 1 else ['candidate','base']):
   key=(case,arm,rep)
   if key in keys: continue
   name='-'.join(map(str,key)); p=a/'screen'/name; out=Path(str(p)+'.bin')
   n=1000 if case=='tiny-smooth' else 1
   cr,cw=os.pipe(); ar,aw=os.pipe()
   cmd=[perf,'stat','-e','instructions:u','-x',';','-o',str(p)+'.perf','-D','-1','--control',f'fd:{cr},{aw}','--','taskset','-c','9',str(a/f'driver-{arm}'),'simplify-attributes',str(inputs/f'{case}.bin'),str(out)]
   try: r=subprocess.run(cmd,input=f'{n}\nstop\n',env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),pass_fds=(cr,cw,ar,aw),capture_output=True,text=True,timeout=1200)
   finally:
    for fd in (cr,cw,ar,aw): os.close(fd)
   Path(str(p)+'.stdout').write_text(r.stdout); Path(str(p)+'.stderr').write_text(r.stderr)
   if r.returncode or not r.stdout.startswith('READY\nCOUNTED '): raise RuntimeError((key,r.returncode,r.stderr))
   fields=next(s.split(';') for s in Path(str(p)+'.perf').read_text().splitlines() if ';instructions:u;' in s)
   assert float(fields[4])==100.0
   def sha(path):
    with path.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
   row=dict(case=case,arm=arm,rep=rep,instructions=int(fields[0])/n,input_sha256=sha(inputs/f'{case}.bin'),binary_sha256=sha(a/f'driver-{arm}'),output_sha256=sha(out),exit=r.returncode)
   with rowsfile.open('a') as f:f.write(json.dumps(row)+'\n')
   keys.add(key); print(name,flush=True)
