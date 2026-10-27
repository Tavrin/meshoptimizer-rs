"""Pinned C++ differential over meshlet builders and cluster LOD."""
import importlib.util
import json
import os
from pathlib import Path
import shutil
import struct
import subprocess

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
root = Path(__file__).resolve().parents[2]
art = Path(os.environ['PEERS_ART'])
ref = Path(os.environ['MESHOPT_REFERENCE'])
target = Path(os.environ['CARGO_TARGET_DIR'])
spec = importlib.util.spec_from_file_location('p03_runner', root/'parity/p03/runner.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
sources = [ref/f'src/{n}.cpp' for n in ['clusterizer','meshletutils','spatialorder','partition','allocator','simplifier','indexgenerator']]
cpp = art/'p03-cpp'
cmd = ['c++',*r.FLAGS,'-I'+str(ref/'src'),'-I'+str(ref/'demo'),str(root/'parity/p03/reference.cpp'),*[str(s) for s in sources],'-o',str(cpp)]
subprocess.run(cmd, check=True)
bins = {'cpp':cpp}
for name, features in [('nostd',[]),('std',['--features','meshoptimizer-rs/std'])]:
    subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(root/'parity/p03/Cargo.toml'),*features,'-j','1'],check=True)
    bins[name] = art/f'p03-{name}'
    shutil.copy2(target/'release/meshopt-p03-driver',bins[name])
clients = {name:r.Client(name, binary) for name,binary in bins.items()}
counts = {}
folder = art/'cpp-differential'
folder.mkdir(exist_ok=True)
try:
    with (folder/'corpus.bin').open('wb') as records:
        for op in [1,3,4]:
            for seed in range(20261004,20261204):
                data = r.case(op,seed)
                r.compare(clients,data,f'{op}-{seed}',folder,records)
                into = bytearray(data)
                struct.pack_into('<I',into,4,op|0x10000)
                r.compare(clients,into,f'{op}-{seed}-into',folder,records)
            counts[str(op)] = 400
        for seed in range(80):
            p,idx = r.grid(2+seed%12,seed%2)
            flags = seed*37 & 511
            extra = []
            if seed >= 32:
                flags |= 1024|2048|4096|((seed%10)<<13)
                if seed%2:
                    width=[1,2,8,12,32][seed%5]
                    flags |= (width-1)<<20
                    extra = [0] if width==1 else [struct.unpack('<I',struct.pack('<f',(i%7+k)/100))[0] for i in range(len(p)) for k in range(width)]
                    if seed%3==0: flags |= 512
                if seed%4==0:
                    old=len(p)
                    p += p[:]
                    idx=[v+old*(i//6%2) for i,v in enumerate(idx)]
                    if extra:
                        width=((flags>>20)&31)+1
                        extra=[0] if width==1 else [struct.unpack('<I',struct.pack('<f',(i%7+k)/100))[0] for i in range(len(p)) for k in range(width)]
            if seed==79:
                p,idx=r.grid(2)
                extra=[]
                flags=0
            data=r.message(16,p,idx,mv=2+seed%5,min=2+seed%4,mt=[4,8,16,32][seed%4],target=flags,split=0.1,counts=extra,radii=[i/100 for i in range(len(p))])
            r.compare(clients,data,f'clusterlod-{seed}',folder,records)
        counts['clusterlod']=80
finally:
    for client in clients.values():
        client.close()
(folder/'result.json').write_text(json.dumps(dict(exit=0,counts=counts,binaries={k:r.sha(v) for k,v in bins.items()},corpus=r.sha(folder/'corpus.bin'),cpp_sources={p.name:r.sha(p) for p in sources}),indent=2)+'\n')
print(counts,flush=True)
