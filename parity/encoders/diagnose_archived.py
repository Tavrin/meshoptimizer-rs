#!/usr/bin/env python3
"""Instruction sampling without an elapsed-time verdict or a rebuild."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import subprocess,os
ROOT=Path(__file__).resolve().parents[2];ART=Path(_public_path('<scratch>/meshopt-v03'));OLD=Path(_public_path('<scratch>/meshopt-cmp'));perf='/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf'
assert os.environ.get('MOSS_HEAVY_ACTIVE')
for op in ['vertex-encode','sequence-encode','oct-encode','quat-encode','exp-encode']:
 p=ART/f'scout-sample-{op}.data'
 subprocess.run([perf,'record','-q','-e','instructions:u','-c','100003','-o',str(p),'--','taskset','-c','26',str(OLD/'cmp-driver-defaults-ours'),op,str(OLD/'inputs/medium-smooth.bin'),str(ART/('sample-'+op+'.bin'))],input=b'10000\nstop\n',stdout=subprocess.DEVNULL,check=True)
 with (ART/f'scout-sample-{op}.txt').open('w') as f:subprocess.run([perf,'report','--stdio','--no-children','-i',str(p)],stdout=f,check=True)
 with (ART/f'scout-annotate-{op}.txt').open('w') as f:subprocess.run([perf,'annotate','--stdio','-i',str(p)],stdout=f,check=True)
 print('sampled',op,flush=True)
with (ART/'scout-cpp.asm').open('w') as f:subprocess.run(['objdump','-Cd',_public_path('<artifacts>/rel020/binaries/2ae0a9f7cf2485919968f388e1ed3913366ab78b1f003458801ad37657ec3e58')],stdout=f,check=True)
with (ART/'scout-rust.asm').open('w') as f:subprocess.run(['objdump','-d',str(OLD/'cmp-driver-defaults-ours')],stdout=f,check=True)
