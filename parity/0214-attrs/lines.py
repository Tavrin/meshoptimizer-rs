"""Resolve sampled instructions to source lines; retain a record per case."""
import json, os, subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'
art = Path(os.environ['ATTR_ART']); perf = os.environ['ATTR_PERF']
path = art / 'lines.jsonl'
rows = [json.loads(x) for x in path.read_text().splitlines()] if path.exists() else []
keys = {(x['case'], x['op'], x['arm']) for x in rows}
for row in map(json.loads, (art/'profiles.jsonl').read_text().splitlines()):
 key = tuple(row[k] for k in ('case','op','arm'))
 if key in keys: continue
 prefix = art/'profiles'/'-'.join(key)
 result = subprocess.run([perf, 'report', '--stdio', '--no-children', '--sort', 'srcline', '-i', str(prefix)+'.data'], capture_output=True, text=True)
 Path(str(prefix)+'.lines').write_text(result.stdout); Path(str(prefix)+'.lines.stderr').write_text(result.stderr)
 if result.returncode: raise RuntimeError((key,result.stderr))
 record = {k:row[k] for k in ('case','op','arm','input_sha256','binary_sha256')}; record['exit']=result.returncode
 with path.open('a') as f: f.write(json.dumps(record)+'\n')
 print(*key, flush=True)
