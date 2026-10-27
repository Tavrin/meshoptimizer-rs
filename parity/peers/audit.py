"""Audit coverage, measured bytes, source constancy, counters and exported receipts."""
import collections,hashlib,json,os,subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
root=Path(__file__).resolve().parents[2];art=Path(os.environ['PEERS_ART']);dest=root/'parity/peers'
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
b=json.loads((dest/'binding.json').read_text())
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['entry_commit']
assert not subprocess.check_output(['git','diff','HEAD','--','src','Cargo.toml','Cargo.lock','README.md','CHANGELOG.md'])
for name,h in b['source'].items():assert sha(root/name)==h,('source changed',name)
for name,h in b['scripts'].items():assert sha(root/name)==h,('script changed',name)
for arm,h in b['binaries'].items():assert sha(art/f'driver-{arm}')==h
summary={}
for prefix,expected in [('',840),('stage-',162)]:
 rows=[json.loads(s)for s in(dest/(prefix+'counts.jsonl')).read_text().splitlines()];assert len(rows)==expected
 keys={(r['case'],r['op'],r['arm'],r['rep'])for r in rows};assert len(keys)==expected
 groups=collections.defaultdict(list)
 for r in rows:
  assert r['binary_sha256']==b['binaries'][r['arm']]
  assert r['exit']==0 and r['running_percent']==100 and r['instructions']>0
  assert r['instructions_per_call']==r['instructions']/r['repetitions']
  path=art/'outputs'/('-'.join(map(str,(r['case'],r['op'],r['arm'],r['rep'])))+'.bin')
  assert sha(path)==r['output_sha256'] and path.stat().st_size==r['output_bytes']
  if 'raw_sha256'in r:assert sha(Path(str(path)+'.raw'))==r['raw_sha256']
  groups[(r['case'],r['op'],r['arm'])].append(r)
 for key,rs in groups.items():
  assert {r['rep']for r in rs}=={0,1,2}
  assert len({r['output_sha256']for r in rs})==1
  if 'raw_sha256'in rs[0]:assert len({r['raw_sha256']for r in rs})==1
 summary[prefix or 'main']=dict(samples=len(rows),cells=len(groups),stable_outputs=True,all_counters_running_100_percent=True)
for name in ['inputs.json','stage-inputs.json']:
 for r in json.loads((dest/name).read_text()):assert sha(art/'inputs'/(r['case']+'.bin'))==r['sha256']
for name in ['validation.json','stage-validation.json']:assert json.loads((dest/name).read_text())['exit']==0
inventory=json.loads((dest/'inventory.json').read_text());assert len(inventory['functions'])==238 and len(inventory['options'])==106
for r in inventory['functions']+inventory['options']:
 assert r['status']in ['present','partial','missing']
 for ref in r['port_source'].split(', '):
  if ';'in ref:continue
  path,number=ref.rsplit(':',1)
  assert 0<int(number)<=len((root/path).read_text().splitlines()),ref
# All source references stay portable. Every committed lane file is relative text.
receipt=dict(exit=0,coverage=summary,inventory_functions=len(inventory['functions']),inventory_options=len(inventory['options']),max_process_rss_kib=max(json.loads(s)['peak_rss_kib']for name in ['counts.jsonl','stage-counts.jsonl']for s in(dest/name).read_text().splitlines()),entry_commit=b['entry_commit'],library_changes=False)
(dest/'audit.json').write_text(json.dumps(receipt,indent=2)+'\n');(art/'audit.json').write_text(json.dumps(receipt,indent=2)+'\n');print('PASS',summary)
