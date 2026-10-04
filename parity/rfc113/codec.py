#!/usr/bin/env python3
"""Compare RFC113 mesh codec payloads against Moss's vendored C++ sources."""
import argparse
import hashlib
import json
import os
import struct
import subprocess
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
sys.path.insert(0,str(HERE))
import run as clod

def sha(b):return hashlib.sha256(b).hexdigest()
def request(op,count,stride,data=b'',mode=0,version=1,level=2):
    return b'MC02'+struct.pack('<10I',op,count,stride,mode,0,version,level,0,1,len(data))+data
class Driver:
    def __init__(self,path,core):
        self.proc=subprocess.Popen(['taskset','-c',str(core),str(path)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,env=clod.ENV)
    def call(self,b):
        self.proc.stdin.write(struct.pack('<I',len(b))+b);self.proc.stdin.flush()
        n=self.proc.stdout.read(4)
        if len(n)!=4:raise RuntimeError(f'codec driver exited {self.proc.poll()}')
        size=struct.unpack('<I',n)[0];out=self.proc.stdout.read(size)
        if len(out)!=size or out[:4]!=b'MD02':raise RuntimeError('bad codec response')
        status,length,samples=struct.unpack_from('<iII',out,4)
        if len(out)!=16+length+samples*8:raise RuntimeError('bad codec length')
        return status,out[16:16+length]
    def close(self):
        self.proc.stdin.close();self.proc.wait()
        if self.proc.returncode:raise RuntimeError(f'codec driver exit {self.proc.returncode}')

def build():
    src=clod.VENDOR/'src';out=clod.TARGET/'rfc113-codec-cpp'
    sources=[src/n for n in ('vertexcodec.cpp','indexcodec.cpp','vertexfilter.cpp','meshletcodec.cpp','allocator.cpp')]
    cmd=['c++','-std=c++17','-O3','-DNDEBUG','-DMESHOPTIMIZER_NO_SIMD','-fno-fast-math','-ffp-contract=off',
         '-I'+str(src),str(ROOT/'parity/codec/reference.cpp'),*map(str,sources),'-o',str(out)]
    subprocess.run(cmd,check=True,env=clod.ENV)
    subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(ROOT/'parity/codec/Cargo.toml')],
                   check=True,env=clod.ENV)
    rust=clod.TARGET/'release/codec-driver'
    identity_files=[HERE/'codec.py',HERE/'run.py',ROOT/'Cargo.toml',ROOT/'Cargo.lock',
        ROOT/'parity/codec/Cargo.toml',ROOT/'parity/codec/Cargo.lock',
        ROOT/'parity/codec/reference.cpp',ROOT/'parity/codec/src/lib.rs',ROOT/'parity/codec/main.rs',
        *sorted((ROOT/'src').rglob('*.rs')),*sources]
    return {'cpp':out,'rust':rust},{str(p):clod.sha(p) for p in identity_files}

def main():
    p=argparse.ArgumentParser();p.add_argument('--case',default=None);p.add_argument('--core',type=int,default=min(os.sched_getaffinity(0)))
    args=p.parse_args();assert args.core in os.sched_getaffinity(0)
    binaries,sources=build();drivers={k:Driver(v,args.core) for k,v in binaries.items()};records=[]
    try:
        for mesh in sorted(clod.MESHES.glob('*.mesh')):
            strides=[48,64,72,88] if mesh.stem in ('pyramid','sponza_lionhead') else [48]
            for stride in strides:
                name=f'{mesh.stem}:{stride}'
                if args.case and args.case not in (mesh.stem,name):continue
                payload,meta=clod.prepare(mesh,stride)
                nv,ni=meta['vertices'],meta['indices']
                vertices=payload[16:16+nv*stride];indices=payload[16+nv*stride:16+nv*stride+ni*4]
                sample=min(nv,128)
                xyz=[];octa=[];quat=[]
                for i in range(sample):
                    p=struct.unpack_from('<3f',vertices,i*stride);n=struct.unpack_from('<3f',vertices,i*stride+12)
                    xyz.extend(p);octa.extend((*n,0.));quat.extend((0.,0.,0.,1.))
                cases=[('vertex',11,nv,stride,vertices,1,2),('index',12,ni,4,indices,1,2),
                       ('sequence',13,ni,4,indices,1,2),
                       ('oct',14,sample,4,struct.pack('<'+str(len(octa))+'f',*octa),1,8),
                       ('quat',15,sample,8,struct.pack('<'+str(len(quat))+'f',*quat),1,12),
                       ('exp',16,sample,12,struct.pack('<'+str(len(xyz))+'f',*xyz),1,16)]
                row={'case':name,'mesh_sha256':clod.sha(mesh),'payloads':{},'mismatches':[]}
                for label,op,count,s,data,version,level in cases:
                    b=request(op,count,s,data,version=version,level=level)
                    cpp=drivers['cpp'].call(b);rust=drivers['rust'].call(b)
                    rec={'input_sha256':sha(b),'cpp_status':cpp[0],'rust_status':rust[0],
                         'cpp_sha256':sha(cpp[1]),'rust_sha256':sha(rust[1]),'cpp_bytes':len(cpp[1]),'rust_bytes':len(rust[1])}
                    if cpp[0]!=rust[0] or cpp[1]!=rust[1]:
                        at=next((i for i,(x,y) in enumerate(zip(cpp[1],rust[1])) if x!=y),min(len(cpp[1]),len(rust[1])))
                        rec['mismatch']={'field':f'{label}.encoded[{at}]','reproduce':f'python3 parity/rfc113/codec.py --case {name}'}
                        row['mismatches'].append(rec['mismatch'])
                    if cpp[0]==0 and rust[0]==0:
                        if label in ('vertex','index','sequence'):
                            version_op=8 if label=='vertex' else 9
                            vr={side:d.call(request(version_op,0,0,cpp[1])) for side,d in drivers.items()}
                            if vr['cpp']!=vr['rust'] or vr['cpp']!=(0,struct.pack('<I',version)):
                                row['mismatches'].append({'field':f'{label}.version','reproduce':f'python3 parity/rfc113/codec.py --case {name}'})
                            bound_op={'vertex':22,'index':23,'sequence':24}[label]
                            bound_data=b'' if label=='vertex' else struct.pack('<Q',nv)
                            bound={side:d.call(request(bound_op,count,s,bound_data)) for side,d in drivers.items()}
                            rec['bound_sha256']=sha(bound['cpp'][1])
                            if bound['cpp']!=bound['rust'] or bound['cpp'][0]!=0 or struct.unpack('<Q',bound['cpp'][1])[0]<len(cpp[1]):
                                row['mismatches'].append({'field':f'{label}.bound','reproduce':f'python3 parity/rfc113/codec.py --case {name}'})
                        decode_op={'vertex':1,'index':2,'sequence':3,'oct':4,'quat':5,'exp':6}[label]
                        ds=(4 if label in ('index','sequence') else s)
                        decoded={}
                        for origin,encoded in [('cpp',cpp[1]),('rust',rust[1])]:
                            q=request(decode_op,count,ds,encoded)
                            decoded[origin]={side:d.call(q) for side,d in drivers.items()}
                        if len({v for m in decoded.values() for v in m.values()})!=1:
                            row['mismatches'].append({'field':f'{label}.decode','reproduce':f'python3 parity/rfc113/codec.py --case {name}'})
                        rec['decode_status']=decoded['cpp']['cpp'][0]
                        rec['decode_sha256']=sha(decoded['cpp']['cpp'][1])
                        if label in ('vertex','sequence') and decoded['cpp']['cpp']!=(0,data):
                            row['mismatches'].append({'field':f'{label}.round_trip','reproduce':f'python3 parity/rfc113/codec.py --case {name}'})
                        if label=='index':
                            result=decoded['cpp']['cpp'][1]
                            same=len(result)==len(data)
                            if same:
                                for at in range(0,len(data),12):
                                    a,b,c=struct.unpack_from('<III',data,at)
                                    x,y,z=struct.unpack_from('<III',result,at)
                                    if (x,y,z) not in ((a,b,c),(b,c,a),(c,a,b)):
                                        same=False;break
                            if not same:
                                row['mismatches'].append({'field':'index.round_trip_rotation','reproduce':f'python3 parity/rfc113/codec.py --case {name}'})
                    row['payloads'][label]=rec
                records.append(row)
                print(name,'MATCH' if not row['mismatches'] else row['mismatches'],flush=True)
        assert sources=={p:clod.sha(p) for p in sources}
        clod.ART.mkdir(parents=True,exist_ok=True)
        result={'schema':'rfc113-codec/1','cases':records,'source_hashes':sources,
                'executable_hashes':{k:clod.sha(v) for k,v in binaries.items()},'core':args.core}
        (clod.ART/'codec.json').write_text(json.dumps(result,indent=2)+'\n')
    finally:
        for d in drivers.values():d.close()
if __name__=='__main__':main()
