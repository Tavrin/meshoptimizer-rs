"""Instruction sampling before P3 edits; symbol and line reports retained."""
import json, os, subprocess
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE')
s=Path(os.environ['BOUNDS_SCRATCH']); d=s/'profile';d.mkdir(exist_ok=True)
perf='/usr/lib/linux-tools/6.17.0-22-generic/perf'
for case in ['medium-smooth','real-b6fd']:
 for op in ['meshlets','meshlets-flex','cluster-lod','cluster-bounds','meshlet-bounds']:
    prefix=d/f'{case}-{op}'
    cr,cw=os.pipe(); ar,aw=os.pipe()
    cmd=[perf,'record','-D','-1','--control',f'fd:{cr},{aw}','-e','instructions:u','-c','100003','-o',str(prefix)+'.data','--','taskset','-c','9',str(s/'driver-base-default'),op,str(s/'inputs'/f'{case}.bin'),str(prefix)+'.out']
    with open(str(prefix)+'.log','w') as log:
     r=subprocess.run(cmd,input='3\nstop\n',text=True,stdout=log,stderr=subprocess.STDOUT,pass_fds=(cr,cw,ar,aw),env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)))
    for fd in [cr,cw,ar,aw]: os.close(fd)
    assert r.returncode==0
    for name,opts in [('symbols',['--sort','symbol']),('lines',['--sort','srcline'])]:
     with open(str(prefix)+f'-{name}.txt','w') as f: subprocess.run([perf,'report','-i',str(prefix)+'.data','--stdio','--percent-limit','0.5',*opts],stdout=f,check=True)
    print(case,op,'profiled',flush=True)
(d/'exit.json').write_text(json.dumps({'exit':0})+'\n')
