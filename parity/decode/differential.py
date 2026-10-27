#!/usr/bin/env python3
"""Targeted C++ 1.3 filter decode parity, including integer/boundary records."""
import hashlib, json, os, pathlib, random, struct, sys
ROOT = pathlib.Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT/'parity/codec'))
from measure import Driver, request
from runner import filter_floats
ART = pathlib.Path(os.environ['MESHOPT_ARTIFACTS'])
assert os.environ.get('PARITY_RUNNER_ACTIVE')
cpp = Driver(ART/'cpp-scalar')
rs = Driver(ART/'codec-scalar-std')
no_std = Driver(ART/'codec-scalar-nostd')
simd = Driver(ART/'codec-simd')
simd_before = Driver(ART/'codec-simd-before')
rng = random.Random(260026)
rows = []
def compare(name, kind, stride, data):
    b = request(kind+3, len(data)//stride, stride, data)
    c = cpp.call(b)
    assert c[0] == 0, name
    digest = hashlib.sha256(c[1]).hexdigest()
    for arm,d in [('std',rs),('nostd',no_std)]:
        r = d.call(b)
        assert r[:2] == c[:2], (name,arm,next((i for i,(a,b) in enumerate(zip(r[1],c[1])) if a!=b),None))
        # Caller-buffer entry and untouched tail behavior is tested in crate tests.
        q=bytearray(b);struct.pack_into('<I',q,16,128)
        assert d.call(q)[:2] == c[:2], (name,arm,'into')
    # No SIMD source changes; retain existing +/-1 conformance policy.
    r=simd.call(b)
    assert r[0] == 0
    assert r[:2] == simd_before.call(b)[:2], (name, 'SIMD baseline mismatch')
    fmt='b' if kind==1 and stride==4 else 'h'
    a=struct.unpack('<'+fmt*(len(r[1])//struct.calcsize(fmt)),r[1])
    v=struct.unpack('<'+fmt*(len(c[1])//struct.calcsize(fmt)),c[1])
    if name.startswith('encoded-'):
        assert max([abs(a-b) for a,b in zip(a,v)]+[0]) <= 1, (name,'SIMD conformance')
    rows.append(dict(case=name, kind=kind, stride=stride, count=len(data)//stride,
                     input_sha256=hashlib.sha256(data).hexdigest(), output_sha256=digest))
for kind,stride in [(1,4),(1,8),(2,8)]:
    for bits in range(4 if kind==2 else 2, 9 if stride==4 else 17):
        for count in [1,2,3,4,7,8,15,16,17,63,64,65,257,513]:
            floats=filter_floats(rng,kind,count,stride)
            c=cpp.call(request(13+kind,count,stride,floats,level=bits))
            assert c[0]==0
            compare(f'encoded-{kind}-{stride}-{bits}-{count}',kind,stride,c[1])
    # Repeated keys, changing alpha, sign/reflection and signed integer extremes.
    if kind==1:
        limit=128 if stride==4 else 32768
        vals=[-limit,-limit+1,-1,0,1,limit-2,limit-1]
        records=[]
        for x in vals:
            for y in vals:
                for z in vals:
                    if (x,y,z)==(0,0,0):continue
                    for alpha in [0,limit-1]:
                        records.extend([struct.pack('<'+('4b' if stride==4 else '4h'),x,y,z,alpha)]*2)
        compare(f'integer-{stride}',kind,stride,b''.join(records))
    else:
        # C++ conversion is defined for these arbitrary signed scales too.
        rec=[]
        for s in [-32768,-16384,-4,0,4,16384,32764]:
            for largest in range(4):
                for v in [-32768,-1,0,1,32767]:
                    rec.extend([struct.pack('<4h',v,0,0,s|largest)]*2)
        compare('quat-scales-and-rotation',kind,stride,b''.join(rec))
# Touched vertex reconstruction/parser: both versions, all relevant block
# edges and varied byte/float-like data. Index and sequence codecs are untouched.
for version in [0,1]:
    for stride in [4,12,48,64,256]:
        for count in [0,1,15,16,17,127,128,129,255,256,257,513]:
            for shape in [0,1,2]:
                if shape==0:
                    raw=bytes(rng.randrange(256)for _ in range(count*stride))
                elif shape==1:
                    raw=bytes((i//stride+(i%stride)*13)&255 for i in range(count*stride))
                else:
                    raw=b''.join(struct.pack('<f',rng.uniform(-10,10))for _ in range(count*stride//4))
                status,encoded,_=cpp.call(request(11,count,stride,raw,version=version,level=2))
                assert status==0
                b=request(1,count,stride,encoded)
                c=cpp.call(b)
                assert c[:2]==(0,raw)
                for arm,d in [('std',rs),('nostd',no_std),('simd',simd)]:
                    assert d.call(b)[:2]==c[:2], (version,stride,count,shape,arm)
                    into=bytearray(b);struct.pack_into('<I',into,16,128)
                    assert d.call(into)[:2]==c[:2], (version,stride,count,shape,arm,'into')
                rows.append(dict(case=f'vertex-v{version}-s{stride}-n{count}-k{shape}',kind=0,stride=stride,count=count,
                                 input_sha256=hashlib.sha256(encoded).hexdigest(),output_sha256=hashlib.sha256(raw).hexdigest()))
                if count in [1,17] and stride in [4,48,64]:
                    for cut in sorted(set([0,1,2,len(encoded)//2,len(encoded)-1])):
                        q=request(1,count,stride,encoded[:cut]);c=cpp.call(q)
                        for arm,d in [('std',rs),('nostd',no_std),('simd',simd)]:
                            r=d.call(q)
                            assert (c[0]==0)==(r[0]==0), (version,stride,count,shape,arm,cut)
                            if c[0]==0: assert r[1]==c[1]

for d in [cpp,rs,no_std,simd,simd_before]:d.close()
(ART/'differential.json').write_text(json.dumps(dict(cases=len(rows), records=sum(r['count'] for r in rows), mismatches=0, rows=rows),indent=2)+'\n')
print('PASS',len(rows),'cases',sum(r['count']for r in rows),'records',flush=True)
