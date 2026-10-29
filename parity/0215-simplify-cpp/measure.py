"""S6 instruction-only campaign; retain one durable record per completed sample."""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import tempfile

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'
assert os.environ['LC_ALL'] == 'C'
art = Path(os.environ['MESHOPT_SCRATCH'])/os.environ.get('MESHOPT_CAMPAIGN', 'campaign-1')
perf = os.environ['PEERS_PERF']
cpu = os.environ.get('PEERS_CPU', '9')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
binding = json.loads((art/'build-binding.json').read_text())
for name, digest in binding['sources'].items(): assert sha(art.parent/name) == digest
for arm, digest in binding['binaries'].items(): assert sha(art/f'driver-{arm}') == digest
inputs = json.loads((art/'inputs.json').read_text())
for x in inputs: assert sha(art/'inputs'/f"{x['case']}.bin") == x['sha256']
registration = dict(binding=binding, inputs=inputs, cpu=cpu, runner_sha256=sha(Path(__file__)),
                    perf=subprocess.check_output([perf,'--version'],text=True).strip(),
                    operations=['simplify','simplify-sloppy'])
reg = art/'campaign.json'
if reg.exists(): assert json.loads(reg.read_text()) == registration
else: reg.write_text(json.dumps(registration,indent=2)+'\n')
for d in ['samples','outputs','interrupted']: (art/d).mkdir(exist_ok=True)
path = art/'counts.jsonl'
rows = [json.loads(s) for s in path.read_text().splitlines()] if path.exists() else []
keys = {(r['case'],r['op'],r['arm'],r['rep']) for r in rows}
assert len(keys) == len(rows)
for x in inputs:
 for op in registration['operations']:
  for rep in range(3):
   for arm in (['base','candidate','cpp','rust'] if rep != 1 else ['rust','cpp','candidate','base']):
    key = (x['case'],op,arm,rep)
    if key in keys: continue
    name = '-'.join(map(str,key)); prefix = art/'samples'/name; out = art/'outputs'/(name+'.bin')
    leftovers = [*prefix.parent.glob(name+'.*'), *out.parent.glob(out.name+'*')]
    if leftovers:
        dest = Path(tempfile.mkdtemp(prefix=name+'-',dir=art/'interrupted'))
        for p in leftovers: p.rename(dest/p.name)
    n = 1000 if x['case']=='tiny-smooth' else 3 if x['case'].startswith('medium') else 1
    cr,cw=os.pipe(); ar,aw=os.pipe()
    try:
        r=subprocess.run([perf,'stat','-e','instructions:u','-x',';','-o',str(prefix)+'.perf','-D','-1',
            '--control',f'fd:{cr},{aw}','--','taskset','-c',cpu,str(art/f'driver-{arm}'),op,
            str(art/'inputs'/f"{x['case']}.bin"),str(out)],input=f'{n}\nstop\n',
            env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),pass_fds=(cr,cw,ar,aw),
            capture_output=True,text=True,timeout=1800)
    finally:
        for fd in [cr,cw,ar,aw]: os.close(fd)
    Path(str(prefix)+'.stdout').write_text(r.stdout); Path(str(prefix)+'.stderr').write_text(r.stderr)
    assert r.returncode==0 and r.stdout.splitlines()==['READY','COUNTED'], (key,r.stdout,r.stderr)
    fields=next(s.split(';') for s in Path(str(prefix)+'.perf').read_text().splitlines() if ';instructions:u;' in s)
    assert int(fields[0])>0 and float(fields[4])==100., fields
    row=dict(case=x['case'],op=op,arm=arm,rep=rep,repetitions=n,exit=0,
        instructions=int(fields[0]),instructions_per_call=int(fields[0])/n,running_percent=float(fields[4]),
        output_sha256=sha(out),output_bytes=out.stat().st_size)
    with path.open('a') as f: f.write(json.dumps(row)+'\n'); f.flush(); os.fsync(f.fileno())
    rows.append(row); keys.add(key)
  cell=[r for r in rows if r['case']==x['case'] and r['op']==op]
  assert len({r['output_sha256'] for r in cell})==1, ('output differs',x['case'],op)
  med={a:statistics.median(r['instructions_per_call'] for r in cell if r['arm']==a) for a in binding['binaries']}
  ratio=med['candidate']/med['base']
  print(x['case'],op,'candidate/base',ratio,flush=True)
  if ratio>1.01:
      with (art/'regressions.jsonl').open('a') as f:
          f.write(json.dumps(dict(case=x['case'],op=op,ratio=ratio))+'\n')
exit_code = int((art/'regressions.jsonl').exists())
(art/'measurement-exit.json').write_text(json.dumps(dict(exit=exit_code,samples=len(rows)))+'\n')
raise SystemExit(exit_code)
