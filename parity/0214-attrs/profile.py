"""Resumable instruction-sampled profiles of the peer harness real cases."""
import hashlib, json, os, subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'
art = Path(os.environ['ATTR_ART'])
inputs = Path(os.environ['ATTR_INPUTS'])
perf = os.environ['ATTR_PERF']
(art / 'profiles').mkdir(exist_ok=True)
rows_path = art / 'profiles.jsonl'
rows = [json.loads(x) for x in rows_path.read_text().splitlines()] if rows_path.exists() else []
keys = {(x['case'], x['op'], x['arm']) for x in rows}
for case in ['real-214e', 'real-a082', 'real-b6fd']:
 for op in ['simplify-attributes', 'simplify', 'overdraw']:
  for arm in ['ours', 'cpp', 'rust']:
   key = (case, op, arm)
   if key in keys: continue
   name = '-'.join(key)
   prefix = art / 'profiles' / name
   out = Path(str(prefix) + '.bin')
   cr, cw = os.pipe(); ar, aw = os.pipe()
   cmd = [perf, 'record', '-e', 'instructions:u', '-c', '1000000', '-o', str(prefix)+'.data', '-D', '-1', '--control', f'fd:{cr},{aw}', '--', 'taskset', '-c', '9', str(art/f'driver-{arm}'), op, str(inputs/f'{case}.bin'), str(out)]
   try:
    p = subprocess.run(cmd, input='1\nstop\n', env=dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar)), pass_fds=(cr,cw,ar,aw), capture_output=True, text=True, timeout=1200)
   finally:
    for fd in (cr,cw,ar,aw): os.close(fd)
   Path(str(prefix)+'.stdout').write_text(p.stdout); Path(str(prefix)+'.stderr').write_text(p.stderr)
   if p.returncode or not p.stdout.startswith('READY\nCOUNTED '): raise RuntimeError((key,p.returncode,p.stderr,p.stdout))
   report = subprocess.run([perf, 'report', '--stdio', '--no-children', '--sort', 'symbol', '-i', str(prefix)+'.data'], capture_output=True, text=True)
   Path(str(prefix)+'.report').write_text(report.stdout); Path(str(prefix)+'.report.stderr').write_text(report.stderr)
   if report.returncode: raise RuntimeError((key, report.stderr))
   def sha(path):
    with path.open('rb') as f: return hashlib.file_digest(f, 'sha256').hexdigest()
   row = dict(case=case, op=op, arm=arm, exit=p.returncode, report_exit=report.returncode, input_sha256=sha(inputs/f'{case}.bin'), binary_sha256=sha(art/f'driver-{arm}'), output_sha256=sha(out))
   with rows_path.open('a') as f: f.write(json.dumps(row)+'\n')
   keys.add(key)
   print(name, 'complete', flush=True)
