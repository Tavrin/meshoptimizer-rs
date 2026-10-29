"""Bind final proof receipts and retained binaries to the measured source."""
from pathlib import Path
import hashlib
import json
import os
import re
import shutil

root=Path(__file__).resolve().parents[2]
lane=Path(__file__).resolve().parent
scratch=Path(os.environ['P4_SCRATCH'])

def sha(p):
    with p.open('rb') as f:
        return hashlib.file_digest(f,'sha256').hexdigest()

receipts=json.loads((lane/'proof-receipts.json').read_text())
final={k:v for k,v in receipts.items() if k.startswith('final-')}
assert len(final)==19
assert all(v['exit']==0 for v in final.values())
binding=json.loads((scratch/'proof/source-binding.json').read_text())
assert binding['exact']==sha(lane/'exact.rs')
assert all(sha(root/k)==v for k,v in binding['sources'].items())
measured=json.loads((lane/'experiment2-binding.json').read_text())['candidate-default']['sources']
assert measured==binding['sources']
summary={'final_exits':{k:v['exit'] for k,v in final.items()},'exact':{},'fuzz':{},'cpp':{},
    'targeted_child_cpu_seconds':json.loads((scratch/'proof/cpu.json').read_text())['child_cpu_seconds'],
    'initial_attempt':{'exit':101,'reason':'proof driver unwrapped feature-disabled option mask in no-default mode; no library mismatch reported','retained':True},
    'source_binding':binding,'binaries':{}}
for mode in ['all','none']:
    log=scratch/'logs'/('final-exact-'+mode+'.log')
    summary['exact'][mode]={'checks':int(re.search(r'PASS observations=(\d+)',log.read_text()).group(1)),
        'exit':0,'log_sha256':sha(log)}
    journal=scratch/'proof'/('fuzz-'+mode+'.jsonl')
    rows=[json.loads(v) for v in journal.read_text().splitlines()]
    assert rows and all(v['exit']==0 for v in rows)
    seconds=sum(v['cpu_seconds'] for v in rows)
    assert seconds>=150
    summary['fuzz'][mode]={'batches':len(rows),'generated_cases':128*len(rows),
        'child_cpu_seconds':seconds,'exit':0,'journal_sha256':sha(journal)}
    summary['cpp'][mode]=json.loads((lane/('cpp-'+mode+'.json')).read_text())
    assert summary['cpp'][mode]['exit']==0 and summary['cpp'][mode]['cases']==1246
    for name in ['exact-'+mode,'0216b-protocol/rust-'+mode]:
        summary['binaries']['proof/'+name]=sha(scratch/'proof'/name)
summary['binaries']['proof/0216b-protocol/cpp-reference']=sha(scratch/'proof/0216b-protocol/cpp-reference')
for arm in ['base-scalar','base-default','candidate-scalar','candidate-default','cpp-scalar','cpp-default']:
    summary['binaries']['driver-'+arm]=sha(scratch/('driver-'+arm))
assert summary['targeted_child_cpu_seconds']<7200
(lane/'proof-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
shutil.copy2(scratch/'proof/source-binding.json',lane/'final-source-binding.json')
print('Final proof stages:',len(final),'all exit 0')
print('Exact:',{k:v['checks'] for k,v in summary['exact'].items()})
print('Fuzz:',summary['fuzz'])
print('Targeted child CPU:',summary['targeted_child_cpu_seconds'])
