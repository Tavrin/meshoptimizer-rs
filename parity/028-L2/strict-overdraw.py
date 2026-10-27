#!/usr/bin/env python3
"""Discriminate historical r2 output binding from current C++ 1.3 behavior."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib,json,os,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
ART=Path(_public_path('<scratch>/meshopt-028-L2'))
REF=Path(_public_path('<refs>/meshoptimizer'))
assert os.environ.get('PARITY_RUNNER_ACTIVE')
binary=ART/'strict-overdraw'
command=['c++','-std=c++17','-O3','-DNDEBUG','-DMESHOPTIMIZER_NO_SIMD','-fno-fast-math','-ffp-contract=off','-march=x86-64','-mtune=generic','-I',str(REF/'src'),str(ROOT/'parity/028-L2/strict-overdraw.cpp'),*[str(REF/'src'/f) for f in ['overdrawoptimizer.cpp','allocator.cpp']],'-o',str(binary)]
subprocess.run(command,check=True)
rows=[]
for trace in json.loads((ART/'stages.json').read_text()):
 vertex=Path(trace['manifest']).read_text().splitlines()[0].split('\t')[0]
 call=next(c for c in trace['calls'] if c['operation']=='overdraw')
 out=ART/('strict-overdraw-'+trace['case']+'.bin')
 subprocess.run(['taskset','-c','9',str(binary),vertex,call['input_path'],str(out)],check=True)
 rows.append(dict(case=trace['case'],sha256=hashlib.sha256(out.read_bytes()).hexdigest(),capture=call['output_sha256']))
(ART/'strict-overdraw.json').write_text(json.dumps(dict(command=command,rows=rows),indent=2)+'\n')
print(json.dumps(rows),flush=True)
