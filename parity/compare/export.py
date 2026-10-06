#!/usr/bin/env python3
"""Retain compact, source-bound comparison results beside the report."""
from pathlib import Path
import json,hashlib,math,statistics
ROOT=Path(__file__).resolve().parents[2]
ART=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp')
def read(name):return json.loads((ART/name).read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
result={'schema':1,'measured_revision':read('measurement-source.json')['revision'],
 'evidence_directory':str(ART),'report_sha256':sha(ROOT/'parity/COMPARE_MESHOPT_CRATE.md'),
 'source_epoch':read('measurement-source.json'),'platforms':read('platforms.json'),
 'footprint':read('footprint.json'),'builds':{},'timing':{},'outputs':[],
 'limitations':['Shared Linux x86-64 host; compile-only target checks.',
 'Allocating matched public-FFI adapters; not every idiomatic wrapper.',
 'Case medians with nominal log-mean Student-t intervals; no simultaneous/sequential significance claim.',
 'Five pairs, extend once to twenty if first interval straddles equality.',
 'Historical upstream 1.3 qualification records are not retimed or promoted to integrated acceptance.',
 'Full retained API footprint covers a broader Rust surface.'],
 'audit':{'passed':read('audit.json')['passed'],'checks':len(read('audit.json')['checks'])},
 'evidence_sha256':{}}
for prof in ['defaults','moss']:
 for backend in ['ours','theirs']:
  result['builds'][f'{prof}/{backend}']=read(f'build-{prof}-{backend}.json')
 j=read(f'measure-{prof}-timing.json');assert j['completed']
 rows=[{k:r[k] for k in ['case','operation','input_sha256','median_ratio','ci95','five_pair_ci']}|{'pairs':len(r['samples'])} for r in j['rows']]
 operations={}
 for op in dict.fromkeys(r['operation'] for r in rows):
  rr=[r for r in rows if r['operation']==op]
  operations[op]={'geometric_mean_case_medians':math.exp(statistics.mean(math.log(r['median_ratio']) for r in rr)),
  'case_ci_envelope':[min(r['ci95'][0] for r in rr),max(r['ci95'][1] for r in rr)]}
 result['timing'][prof]={'core':j['core'],'binary_sha256':j['binary_sha256'],'cases':rows,'operations':operations,'pairs':sum(r['pairs'] for r in rows)}
j=read('measure-defaults-outputs.json');assert j['completed']
for r in j['rows']:
 result['outputs'].append({k:r[k] for k in ['case','operation','input_sha256','byte_identical','outputs']}|({k:r[k] for k in ['raw_byte_identical','raw_outputs']} if 'raw_outputs' in r else {}))
for p in sorted(ART.iterdir()):
 if p.is_file() and p.name!='MILESTONES.md' and p.suffix in ['.json','.log','.tsv','.md']:
  result['evidence_sha256'][p.name]=sha(p)
(ROOT/'parity/compare/results.json').write_text(json.dumps(result,indent=2)+'\n')
print('exported',len(result['outputs']),'outputs;',sum(len(j['cases']) for j in result['timing'].values()),'timing cases;',sum(j['pairs'] for j in result['timing'].values()),'pairs')
