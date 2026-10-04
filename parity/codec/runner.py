#!/usr/bin/env python3
"""P02 seeded differential, fixtures, malformed inputs and executed WASM identity."""
import argparse
import base64
import json
import os
from pathlib import Path
import random
import struct
import subprocess
import time
import zipfile
from measure import ROOT, REF, TARGET, ART, ENV, FLAGS, Driver, request, decode, sha, cmd, build_cpp

def sources():
    files=[*ROOT.glob('src/**/*.rs'),*ROOT.glob('tests/**/*.rs'),*ROOT.glob('parity/codec/**/*.rs'),*ROOT.glob('parity/codec/*.py'),*ROOT.glob('parity/codec/*.cpp'),*ROOT.glob('parity/codec/*.mjs'),ROOT/'Cargo.lock',ROOT/'parity/codec/Cargo.lock',ROOT/'parity/DECODER_BAR.md']
    files += list((REF/'src').glob('*'))+[REF/'demo/tests.cpp',REF/'js/meshopt_decoder.test.js',ROOT/'Cargo.toml',ROOT/'parity/codec/Cargo.toml',ROOT/'fuzz/codec/Cargo.toml',ROOT/'fuzz/codec/Cargo.lock']
    metadata=json.loads(subprocess.check_output(['cargo','metadata','--offline','--locked','--format-version','1'],cwd=ROOT,env=ENV))
    for package in metadata['packages']:
        if package['source'] is not None:
            files+= [p for p in Path(package['manifest_path']).parent.rglob('*') if p.is_file()]
    return {str(p):sha(p) for p in sorted(set(files)) if p.is_file()}
def build():
    before=sources()
    scalar=build_cpp(True);simd=build_cpp()
    cmd(['cargo','build','--offline','--locked','--release','--manifest-path',ROOT/'parity/codec/Cargo.toml'])
    cmd(['cargo','build','--offline','--locked','--release','--target','wasm32-unknown-unknown','--manifest-path',ROOT/'parity/codec/Cargo.toml','--lib'])
    if before!=sources():raise ValueError('source changed during build')
    return {'scalar':scalar,'simd':simd,'rust':TARGET/'release/codec-driver','wasm':TARGET/'wasm32-unknown-unknown/release/meshopt_codec_parity.wasm'}
class Wasm:
    def __init__(self,binary):
        self.p=subprocess.Popen(['node',ROOT/'parity/wasm.cjs',binary],stdin=subprocess.PIPE,stdout=subprocess.PIPE,env=ENV)
    def call(self,b):
        self.p.stdin.write(base64.b64encode(b)+b'\n');self.p.stdin.flush()
        response=self.p.stdout.readline()
        if not response:raise RuntimeError('WASM exited')
        return decode(base64.b64decode(response))
    def close(self):self.p.stdin.close();assert self.p.wait()==0

def fixtures():
    directory=ART/'fixtures';directory.mkdir(exist_ok=True)
    for suite in sorted((REF/'js').glob('*.test.js')):cmd(['node',suite])
    cmd(['node',ROOT/'parity/codec/js-fixtures.mjs',REF,directory])
    generated=ART/'native-fixtures.cpp'
    cmd(['python3',ROOT/'parity/codec/native-fixtures.py',REF,generated,directory])
    binary=TARGET/'codec-native-fixtures'
    cmd(['c++',*[f for f in FLAGS if f!='-DNDEBUG'],'-DMESHOPTIMIZER_NO_SIMD','-I',REF/'src',generated,*sorted((REF/'src').glob('*.cpp')),'-o',binary])
    cmd([binary,directory])
    cases=[(p.stem,p.read_bytes()) for p in sorted(directory.glob('*.input'))]
    if len(cases)!=287:raise ValueError('missing or extra pinned decoder fixtures')
    return cases

def generated(cpp,seed,cases):
    rng=random.Random(seed)
    for op in range(1,8):
        for i in range(cases):
            count=rng.choice([1,2,3,15,16,17,31,32,33,63,64,65,127,128,129,255,256,257,513,rng.randrange(1,600)])
            version=i%2;filter=0;mode=0
            if op in [1,7]:
                if op==7:
                    mode=i%3;version=0 if mode==0 else version;filter=rng.randrange(4) if mode==0 else 0
                raw=1 if op==1 else mode+1
            else:raw=op
            if raw==1:
                stride=rng.choice([4,8,12,16,20,32,48,64,128,252,256]);
                if filter==1:stride=rng.choice([4,8])
                if filter==2:stride=8
                if filter:
                    f=request(13+filter,count,stride,filter_floats(rng,filter,count,stride),level=rng.randint(2,8 if stride==4 else (24 if filter==3 else 16)))
                    status,data,_=cpp.call(f);assert status==0
                else:
                    kind=i%5
                    data=bytes(rng.randrange(256) if kind==0 else (j//stride*kind+j%stride)%256 if kind<4 else 0 for j in range(count*stride))
                status,encoded,_=cpp.call(request(11,count,stride,data,version=version,level=i%10));assert status==0
            elif raw in [2,3]:
                count=count//3*3+3 if raw==2 else count;stride=rng.choice([2,4]);kind=i%5
                indices=[rng.randrange(65536) if kind==0 else j if kind==1 else j//3 if kind==2 else rng.randrange(0xffffffff) if kind==3 else 0 for j in range(count)]
                status,encoded,_=cpp.call(request(raw+10,count,4,struct.pack('<'+'I'*count,*indices),version=version));assert status==0
            else:
                f=raw-3;stride=rng.choice([4,8]) if f==1 else 8 if f==2 else rng.choice([4,8,12,16,32,64,256])
                status,encoded,_=cpp.call(request(13+f,count,stride,filter_floats(rng,f,count,stride),level=rng.randint(2,8 if stride==4 and f==1 else 16 if f in [1,2] else 24)));assert status==0
            yield f'seed-{seed}-op{op}-{i}',request(op,count,stride,encoded,mode=mode,filter=filter)

def filter_floats(rng,f,count,stride):
    data=[]
    for _ in range(count):
        if f in [1,2]:
            v=[rng.uniform(-1,1) for _ in range(3 if f==1 else 4)]
            length=sum(x*x for x in v)**0.5;v=[x/length for x in v]
            if f==1:v+=[rng.choice([-1.,1.])]
        else:v=[rng.uniform(-1e6,1e6) for _ in range(stride//4)]
        data+=v
    return struct.pack('<'+'f'*len(data),*data)

def compare(cases,binaries,label,seed=None):
    before=sources();identities={k:sha(v) for k,v in binaries.items()}
    cpp=Driver(binaries['scalar']);simd=Driver(binaries['simd']);rust=Driver(binaries['rust']);wasm=Wasm(binaries['wasm'])
    archive=ART/(label+'.zip');records=[];counts={};conformance=0
    with zipfile.ZipFile(archive,'w',compression=zipfile.ZIP_DEFLATED) as z:
        for name,b in cases:
            c=cpp.call(b);r=rust.call(b);w=wasm.call(b)
            if (c[0]==0)!=(r[0]==0) or (r[:2]!=w[:2]) or (c[0]==0 and c[1]!=r[1]):
                (ART/'failure.input').write_bytes(b);raise AssertionError((name,c[0],r[0],w[0],next((i for i,(x,y) in enumerate(zip(c[1],r[1])) if x!=y),None)))
            op=struct.unpack_from('<I',b,4)[0];counts[op]=counts.get(op,0)+1
            # Both APIs and unused-tail preservation are independently unit-tested.
            if op not in [8,9]:
                into=bytearray(b);struct.pack_into('<I',into,16,struct.unpack_from('<I',b,16)[0]|128)
                ri=rust.call(into);wi=wasm.call(into)
                assert ri[:2]==r[:2]==wi[:2],name
            if c[0]==0 and (op in [4,5,6] or (op==7 and struct.unpack_from('<I',b,20)[0])):
                s=simd.call(b);assert s[0]==0
                filter=op-3 if op in [4,5,6] else struct.unpack_from('<I',b,20)[0];stride=struct.unpack_from('<I',b,12)[0]
                if filter==3:assert s[1]==r[1],name
                else:
                    fmt='b' if filter==1 and stride==4 else 'h'
                    a=struct.unpack('<'+fmt*(len(s[1])//struct.calcsize(fmt)),s[1]);v=struct.unpack('<'+fmt*(len(r[1])//struct.calcsize(fmt)),r[1])
                    assert max([abs(x-y) for x,y in zip(a,v)]+[0])<=1,name
                conformance+=1
            files={}
            for key,data in [('input',b),('cpp',c[1]),('rust',r[1]),('wasm',w[1])]:
                member=name+'.'+key;z.writestr(member,data);files[key]={'member':member,'sha256':__import__('hashlib').sha256(data).hexdigest()}
            records.append({'case':name,'operation':op,'cpp_status':c[0],'rust_status':r[0],'wasm_status':w[0],'files':files})
            if len(records)%500==0:print(label,len(records),'zero mismatches',flush=True)
    for d in [cpp,simd,rust,wasm]:d.close()
    assert before==sources();assert identities=={k:sha(v) for k,v in binaries.items()}
    record={'phase':'0.2','seed':seed,'counts':counts,'mismatches':0,'simd_filter_conformance_cases':conformance,'sources':before,'executable_sha256':identities,'archive':str(archive),'archive_sha256':sha(archive),'cases':records,'node':subprocess.check_output(['node','--version'],text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cxx':subprocess.check_output(['c++','--version'],text=True),'cxx_scalar_flags':FLAGS+['-DMESHOPTIMIZER_NO_SIMD'],'cxx_simd_flags':FLAGS,'rust_flags':'empty; generic target; release; fat LTO; one codegen unit; no_std core','reference_revision':'4c203430ca565cb59a468a91922c76c208169536'}
    (ART/(label+'.json')).write_text(json.dumps(record,indent=2))
    print(label,len(records),'PASS',counts,flush=True)

def malformed(fixtures):
    result=[]
    # Every-byte truncation for fixed upstream raw codec fixtures, including
    # tables/tails. Defined reference failure outputs are never compared.
    for name,b in fixtures:
        op=struct.unpack_from('<I',b,4)[0]
        if op not in [1,2,3] or 'MemorySafe' in name:continue
        if len(b)>2048:continue
        for n in range(len(b)-44):
            truncated=bytearray(b[:44+n]);struct.pack_into('<I',truncated,40,n)
            result.append((name+f'-truncated-{n}',truncated))
        for header in [0,255,(b[44]&240)|15] if len(b)>44 else []:
            wrong=bytearray(b);wrong[44]=header;result.append((name+f'-header-{header}',wrong))
    return result

def main():
    p=argparse.ArgumentParser();p.add_argument('action',choices=['run','sweep']);p.add_argument('--phase',default='0.2');p.add_argument('--profile',default='scalar-strict');p.add_argument('--cases-per-family',type=int,default=2000);p.add_argument('--seed',type=int,default=20261004);args=p.parse_args()
    if args.phase!='0.2' or args.profile!='scalar-strict' or args.cases_per_family<2000:raise ValueError('unsupported phase/profile/count')
    binaries=build()
    if args.action=='run':
        fixed=fixtures();compare(fixed,binaries,'fixtures');compare(malformed(fixed),binaries,'malformed')
    else:
        cpp=Driver(binaries['scalar'])
        compare(generated(cpp,args.seed,args.cases_per_family),binaries,'sweep',args.seed);cpp.close()
if __name__=='__main__':main()
