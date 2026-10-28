"""Drive coverage-instrumented cluster LOD using the fuzz generator's inputs."""
import importlib.util
import os
from pathlib import Path
root=Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location('p03_runner',root/'parity/p03/runner.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
c=r.Client('std',Path(os.environ['CLOD_COVERAGE_BINARY']))
try:
    for seed in range(512):
        p,idx=r.grid(2+seed%12,seed%2);flags=(seed*37)&511
        if seed%3==0:flags|=1024|2048|4096|((seed%10)<<13)
        c.run(r.message(16,p,idx,mv=2+seed%5,min=2+seed%4,mt=[4,8,16,32][seed%4],target=flags,split=0.1,radii=[i/100 for i in range(len(p))]))
finally:c.close()
