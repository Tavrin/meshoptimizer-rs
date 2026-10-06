#!/usr/bin/env python3
"""Instruction sampling without an elapsed-time verdict or a rebuild."""
from pathlib import Path
import subprocess,os
ROOT=Path(__file__).resolve().parents[2];ART=Path('/mnt/linux-extra/moss-scratch/meshopt-v03');OLD=Path('/mnt/linux-extra/moss-scratch/meshopt-cmp');perf='/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf'
assert os.environ.get('MOSS_HEAVY_ACTIVE')
for op in ['vertex-encode','sequence-encode','oct-encode','quat-encode','exp-encode']:
 p=ART/f'scout-sample-{op}.data'
 subprocess.run([perf,'record','-q','-e','instructions:u','-c','100003','-o',str(p),'--','taskset','-c','26',str(OLD/'cmp-driver-defaults-ours'),op,str(OLD/'inputs/medium-smooth.bin'),str(ART/('sample-'+op+'.bin'))],input=b'10000\nstop\n',stdout=subprocess.DEVNULL,check=True)
 with (ART/f'scout-sample-{op}.txt').open('w') as f:subprocess.run([perf,'report','--stdio','--no-children','-i',str(p)],stdout=f,check=True)
 with (ART/f'scout-annotate-{op}.txt').open('w') as f:subprocess.run([perf,'annotate','--stdio','-i',str(p)],stdout=f,check=True)
 print('sampled',op,flush=True)
with (ART/'scout-cpp.asm').open('w') as f:subprocess.run(['objdump','-Cd','/mnt/linux-extra/meshopt-artifacts/rel020/binaries/2ae0a9f7cf2485919968f388e1ed3913366ab78b1f003458801ad37657ec3e58'],stdout=f,check=True)
with (ART/'scout-rust.asm').open('w') as f:subprocess.run(['objdump','-d',str(OLD/'cmp-driver-defaults-ours')],stdout=f,check=True)
