#!/usr/bin/env python3
"""Run the pinned scalar C++ differential for the meshlet families."""
import importlib.util,json,os,struct
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
root=Path(__file__).resolve().parents[2]
art=Path(os.environ['MESHLETS_ART'])
os.environ['MESHOPT_REFERENCE']=os.environ['MESHLETS_REFERENCE']
os.environ['MESHOPT_ARTIFACTS']=str(art/'p03')
spec=importlib.util.spec_from_file_location('p03',root/'parity/p03/runner.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
bins,identity=r.build(False)
clients={n:r.Client(n,p) for n,p in bins.items() if n in ('cpp','rust')}
folder=art/'cpp-diff';folder.mkdir(exist_ok=True)
try:
 with (folder/'corpus.bin').open('wb') as records:
  for op in (1,3,4):
   for seed in range(20261004,20263004):
    data=r.case(op,seed);r.compare(clients,data,f'{op}-{seed}',folder,records)
    into=bytearray(data);struct.pack_into('<I',into,4,op|0x10000);r.compare(clients,into,f'{op}-{seed}-into',folder,records)
   print('cpp differential',op,2000,flush=True)
finally:
 for c in clients.values():c.close()
(folder/'record.json').write_text(json.dumps(dict(exit=0,seeds_per_family=2000,variants=['alloc','into'],identity=identity),indent=2)+'\n')
