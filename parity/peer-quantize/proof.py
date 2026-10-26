#!/usr/bin/env python3
"""Targeted exact protocol checks; run under moss-heavy after build-proof.sh."""
import hashlib,json,os,struct,subprocess,sys
from pathlib import Path
S=Path(os.environ.get('PEER_SCRATCH','/mnt/linux-extra/moss-scratch/meshopt-peer-a'))
assert os.environ.get('MOSS_HEAVY_ACTIVE')
sys.path.insert(0,str(S/'helpers'))
import p01x
rows=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def check(label,data,cpp=True):
    outs={}
    for arm in ['baseline','candidate']:
        p=subprocess.run([str(S/f'protocol-{arm}-driver')],input=data,capture_output=True,timeout=60)
        outs[arm]=(p.returncode,p.stdout,p.stderr)
    assert outs['baseline']==outs['candidate'],(label,outs)
    rc,out,err=outs['candidate']
    # Invalid fuzz transport and invalid widths have no C++ API equivalent.
    if cpp and rc==0:
        p=subprocess.run([str(S/'protocol-cpp-driver')],input=data,capture_output=True,timeout=60)
        assert p.returncode==0 and p.stdout==out,(label,p.returncode,p.stderr,out[:32],p.stdout[:32])
    rows.append(dict(label=label,input_sha256=sha(data),exit=rc,output_sha256=sha(out),error=err.decode(),cpp_checked=cpp and rc==0))
# Full frozen position bit patterns, every supported float width and invalid widths.
for f in sorted((S/'inputs').glob('*.bin')):
    b=f.read_bytes();n=struct.unpack_from('<I',b)[0];values=b[8:8+n*12]
    for op in [30,31]:
        for width in ([0] if op==30 else list(range(24))+[24,31,32,0xffffffff]):
            data=struct.pack('<4s13I',b'MO02',op,0,n*3,0,0,1,1,0,0,width,0,0,0)+values
            check(f'{f.stem}/{op}/{width}',data)
# Existing differential fixtures: same geometry/variant generator, scoped to our families.
for variant,(name,p,ib) in enumerate(p01x.geometries()):
    for op in [30,31]:check(f'existing/{name}/{op}',p01x.message(op,p,ib,variant))
# Exact first 256 generated messages for each existing default fuzz seed; mutation
# order/RNG are retained from fuzz/src/p01x_main.rs, not a substitute RNG.
for f in sorted((S/'seeds').glob('*.input')):check(f'fuzz/{f.name}',f.read_bytes())
# Threshold neighborhoods, signed special values, and every mantissa width.
words=[]
for exponent in range(256):
    for mantissa in [0,1,0xfff,0x1000,0x1fff,0x3fffff,0x7fefff,0x7ff000,0x7ffffe,0x7fffff]:
        for sign in [0,0x80000000]:words.append(sign|(exponent<<23)|mantissa)
values=struct.pack('<'+'I'*len(words),*words)
for op in [30,31]:
    for width in ([0] if op==30 else list(range(24))+[24,32,0xffffffff]):
        check(f'boundaries/{op}/{width}',struct.pack('<4s13I',b'MO02',op,0,len(words),0,0,1,1,0,0,width,0,0,0)+values)
result=dict(passed=True,cases=len(rows),cpp_cases=sum(r['cpp_checked']for r in rows),rows=rows,binaries={f.name:sha(f.read_bytes())for f in S.glob('protocol-*-driver')})
(S/'proof.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items()if k not in ['rows','binaries']}))
