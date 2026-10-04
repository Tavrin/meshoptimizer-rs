#!/usr/bin/env python3
"""RFC113 S2 bridge differential and same-core interleaved cook timing."""
import argparse
import hashlib
import json
import os
import platform
import statistics
import struct
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
MOSS = Path(os.environ.get('MESHOPT_RFC113_MOSS_DIR',
    '/mnt/linux-extra/moss-lane-worktrees/cluster-lod-s2/crates/moss_cooker'))
VENDOR = MOSS / 'vendor/meshoptimizer'
MESHES = Path(os.environ.get('MESHOPT_RFC113_MESH_DIR',
    '/mnt/linux-extra/moss-spikes/cluster-lod-s0b/meshes'))
ART = Path(os.environ.get('MESHOPT_RFC113_ART_DIR',
    '/mnt/linux-extra/meshopt-artifacts/rfc113'))
TARGET = Path(os.environ.get('CARGO_TARGET_DIR',
    '/mnt/linux-extra/moss-cargo-targets/codex-meshopt-clodperf'))
LAYOUTS = {'pos3-norm3-uv2': 32, 'pos3-norm3-uv2-tangent4': 48,
           'pos3-norm3-uv2-tangent4-color4': 64,
           'pos3-norm3-uv2-tangent4-joints4-weights4': 72,
           'pos3-norm3-uv2-tangent4-color4-joints4-weights4': 88}
ENV = {**os.environ, 'CARGO_TARGET_DIR': str(TARGET), 'CARGO_NET_OFFLINE': 'true',
       'RUSTC_WRAPPER': '', 'RUSTC_WORKSPACE_WRAPPER': '', 'RUSTFLAGS': '', 'CARGO_ENCODED_RUSTFLAGS': ''}
FIELDS = [('header', 12, 4), ('vertices', 16, 0), ('indices', 4, 0),
          ('clusters', 8, 0), ('groups', 8, 0), ('nodes', 8, 0)]

def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def cpu_snapshot():
    rows = {}
    for line in Path('/proc/stat').read_text().splitlines():
        words = line.split()
        if words and words[0].startswith('cpu') and words[0][3:].isdigit():
            values = list(map(int, words[1:]))
            rows[int(words[0][3:])] = (sum(values), values[3] + values[4])
    return rows

def utilization(before, after, core):
    total = after[core][0] - before[core][0]
    idle = after[core][1] - before[core][1]
    return 1 - idle / total if total else 0

def least_busy_core():
    before = cpu_snapshot()
    time.sleep(1)
    after = cpu_snapshot()
    allowed = os.sched_getaffinity(0)
    excluded = {int(v) for v in os.environ.get('MESHOPT_RFC113_EXCLUDE_CORES','0,1').split(',') if v}
    choices = allowed - excluded
    if not choices: choices = allowed
    core = min(choices, key=lambda core: (utilization(before, after, core), core))
    return core, {'sample_seconds':1,'pre_run_utilization':utilization(before, after, core),
                  'excluded_cores':sorted(excluded)}

def build(rust_profile='consumer'):
    TARGET.mkdir(parents=True, exist_ok=True); ART.mkdir(parents=True, exist_ok=True)
    assert sha(HERE/'bridge.cpp') == sha(MOSS/'src/cluster_lod_bridge.cpp'), 'S2 bridge copy drifted'
    sources = sorted((VENDOR/'src').glob('*.cpp'))
    cpp = TARGET/'rfc113-cpp'
    subprocess.run(['c++', '-std=c++17', '-O3', '-DNDEBUG', '-DMESHOPTIMIZER_NO_SIMD', '-fno-fast-math',
                    '-ffp-contract=off', '-I'+str(VENDOR/'src'), '-I'+str(VENDOR),
                    str(HERE/'bridge.cpp'), str(HERE/'reference.cpp'), *map(str,sources), '-o', str(cpp)],
                   check=True, env=ENV)
    moss_cpp = TARGET/'rfc113-moss-cpp'
    subprocess.run(['c++', '-std=c++17', '-O3', '-DNDEBUG', '-fPIC',
                    '-ffunction-sections', '-fdata-sections', '-m64',
                    '-I'+str(VENDOR/'src'), '-I'+str(VENDOR),
                    str(HERE/'bridge.cpp'), str(HERE/'reference.cpp'), *map(str,sources), '-o', str(moss_cpp)],
                   check=True, env=ENV)
    profile_args = ['--profile', 'consumer'] if rust_profile == 'consumer' else ['--release']
    subprocess.run(['cargo','build','--offline',*profile_args,'--manifest-path',str(HERE/'Cargo.toml')],check=True,env=ENV)
    rust = TARGET/rust_profile/'rfc113-rust'
    inputs = [HERE/'bridge.cpp', HERE/'reference.cpp', HERE/'main.rs', HERE/'run.py',
              ROOT/'Cargo.toml', ROOT/'Cargo.lock', MOSS/'build.rs', MOSS/'src/cluster_lod.rs',
              MOSS/'src/cluster_lod_bridge.cpp', VENDOR/'clusterlod.h', *sources,
              *sorted((ROOT/'src').rglob('*.rs'))]
    return {'cpp':cpp,'moss_cpp':moss_cpp,'rust':rust}, {str(p):sha(p) for p in inputs}

def prepare(path, stride):
    data=path.read_bytes()
    if data[:4]!=b'S0BM': raise ValueError(path)
    nv,ni,flags,scale,cutoff=struct.unpack_from('<IIIff',data,4)
    assert len(data)==24+nv*32+ni*4 and ni%3==0
    pos=memoryview(data)[24:24+nv*12]; nrm=memoryview(data)[24+nv*12:24+nv*24]
    uv=memoryview(data)[24+nv*24:24+nv*32]; index_bytes=data[24+nv*32:]
    indices=struct.unpack('<'+str(ni)+'I',index_bytes)
    assert all(i<nv for i in indices)
    verts=bytearray(nv*stride)
    # The 32-byte prefix is the S0b source; deterministic larger layouts
    # append tangent, color, and packed joint/weight attributes in S2's order.
    for i in range(nv):
        off=i*stride; verts[off:off+12]=pos[i*12:i*12+12]
        verts[off+12:off+24]=nrm[i*12:i*12+12]
        verts[off+24:off+32]=uv[i*8:i*8+8]
        if stride>32: struct.pack_into('<4f',verts,off+32,1.,0.,0.,1.)
        if stride in (64,88): struct.pack_into('<4f',verts,off+48,1.,1.,1.,1.)
        if stride in (72,88):
            joint=off+(64 if stride==88 else 48)
            struct.pack_into('<4H4f',verts,joint,0,0,0,0,1.,0.,0.,0.)
    # Exact S2 boundary_locks: position-key IDs in first-seen vertex order,
    # +0/-0 canonicalization, directed edge count and orientation balance.
    ids={}; vertex_ids=[]
    for i in range(nv):
        xyz=struct.unpack_from('<3I',pos,i*12)
        key=tuple(0 if x==0x80000000 else x for x in xyz)
        vertex_ids.append(ids.setdefault(key,len(ids)))
    edges={}
    for a,b,c in zip(indices[::3],indices[1::3],indices[2::3]):
        tri=(vertex_ids[a],vertex_ids[b],vertex_ids[c])
        for x,y in ((tri[0],tri[1]),(tri[1],tri[2]),(tri[2],tri[0])):
            if x==y: continue
            k=(min(x,y),max(x,y));count,balance=edges.get(k,(0,0))
            edges[k]=(count+1,balance+(1 if x<y else -1))
    locked=bytearray(len(ids))
    for (a,b),(count,balance) in edges.items():
        if count!=2 or balance!=0:locked[a]=locked[b]=1
    locks=bytes(locked[i] for i in vertex_ids)
    payload=struct.pack('<4sIII',b'R113',nv,ni,stride)+verts+index_bytes+locks
    return payload, {'vertices':nv,'indices':ni,'stride':stride,'flags':flags,
                     'scale':scale,'alpha_cutoff':cutoff,'locks':sum(locks),'input_sha256':hashlib.sha256(payload).hexdigest()}

class Driver:
    def __init__(self,path,core):
        self.proc=subprocess.Popen(['taskset','-c',str(core),str(path)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,env=ENV)
    def run(self,payload):
        start=time.perf_counter_ns()
        self.proc.stdin.write(struct.pack('<I',len(payload))+payload);self.proc.stdin.flush()
        head=self.proc.stdout.read(4)
        if len(head)!=4: raise RuntimeError(f'driver exited: {self.proc.poll()}')
        n=struct.unpack('<I',head)[0]
        out=self.proc.stdout.read(n)
        if len(out)!=n: raise RuntimeError('short driver output')
        if payload[:4]==b'R11T':
            return out[:-8],struct.unpack_from('<Q',out,len(out)-8)[0]/1e9
        return out,(time.perf_counter_ns()-start)/1e9
    def close(self):
        self.proc.stdin.close();self.proc.wait()
        if self.proc.returncode: raise RuntimeError(f'driver exit {self.proc.returncode}')

def field_at(data, offset):
    if len(data)<48: return 'truncated header'
    counts=struct.unpack_from('<12I',data)
    sections=[('header',48),('vertex',counts[5]*16),('index',counts[6]*4),
              ('cluster',counts[7]*32),('group',counts[8]*32),('node',counts[9]*32)]
    labels={'vertex':['source','position.x','position.y','position.z'],
            'cluster':['first_index','index_count','group','refined','center.x','center.y','center.z','radius'],
            'group':['center.x','center.y','center.z','radius','error','depth','first_cluster','cluster_count'],
            'node':['center.x','center.y','center.z','radius','error','group','child_offset','child_count']}
    at=0
    for name,size in sections:
        if offset<at+size:
            word=(offset-at)//4
            if name=='header': return f'header word {word}'
            width=4 if name=='vertex' else 1 if name=='index' else 8
            field=labels.get(name,['value'])[word%width]
            return f'{name}[{word//width}].{field}'
        at+=size
    if len(data)>=at+8 and data[at:at+4]==b'BND2':
        if offset<at+8:return 'bounds header'
        rel=offset-at-8;record=rel//96;which=('cluster','meshlet')[(rel%96)//48]
        word=(rel%48)//4
        return f'{which}_bounds[{record}].'+(['center.x','center.y','center.z','radius','apex.x','apex.y','apex.z',
                   'axis.x','axis.y','axis.z','cutoff','packed_axis'][word])
    return f'trailing byte {offset-at}'

def main():
    p=argparse.ArgumentParser();p.add_argument('--case',default=None,help='mesh stem or stem:stride')
    p.add_argument('--pairs',type=int,default=3);p.add_argument('--core',type=int,default=None)
    p.add_argument('--rust-profile',choices=('consumer','release'),default='consumer')
    args=p.parse_args()
    selection = None
    if args.core is None: args.core, selection = least_busy_core()
    binaries,sources=build(args.rust_profile)
    assert args.core in os.sched_getaffinity(0)
    load_before = os.getloadavg()
    cpu_before = cpu_snapshot()
    cases=[]
    for mesh in sorted(MESHES.glob('*.mesh')):
        strides=[32,48,64,72,88] if mesh.stem in ('pyramid','sponza_lionhead') else [48]
        for stride in strides:
            name=f'{mesh.stem}:{stride}'
            if args.case and args.case not in (mesh.stem,name): continue
            cases.append((name,mesh,stride))
    drivers={k:Driver(v,args.core) for k,v in binaries.items()};records=[]
    try:
        for name,mesh,stride in cases:
            payload,meta=prepare(mesh,stride)
            if stride==32:
                records.append({'case':name,'mesh_sha256':sha(mesh),**meta,'mismatch':{
                    'field':'attribute_protect_mask bit 8','cpp':'clodBuild assert: mask must fit vertex_attributes_stride / 4 (8 floats)',
                    'rust':'InvalidParameter: mask exceeds attribute component count (5 floats)',
                    'reproduce':f'python3 parity/rfc113/run.py --case {name} --pairs 0'},
                    'times_s':{'cpp':[],'moss_cpp':[],'rust':[]},'median_ratio':None,'moss_ratio':None})
                print(name,'INVALID S2 stride-32 setup: protect mask bit 8',flush=True)
                continue
            outputs={};parity_payload=b'R11B'+payload[4:]
            for side in ('cpp','moss_cpp','rust'):
                outputs[side],_=drivers[side].run(parity_payload)
            cpp,rust=outputs['cpp'],outputs['rust']
            moss_cpp=outputs['moss_cpp']
            mismatch=None
            if cpp!=rust:
                at=next((i for i,(x,y) in enumerate(zip(cpp,rust)) if x!=y),min(len(cpp),len(rust)))
                mismatch={'byte':at,'field':field_at(cpp,at),'cpp_length':len(cpp),'rust_length':len(rust),
                          'cpp_word':cpp[at:at+4].hex(),'rust_word':rust[at:at+4].hex(),
                          'reproduce':f'python3 parity/rfc113/run.py --case {name} --pairs 0'}
            timing={'cpp':[],'moss_cpp':[],'rust':[]}
            if stride==48:
                timed_outputs={}
                for pair in range(args.pairs):
                    for side in (('cpp','moss_cpp','rust') if pair%2==0 else ('rust','moss_cpp','cpp')):
                        result,seconds=drivers[side].run(b'R11T'+payload[4:])
                        if side in timed_outputs and result!=timed_outputs[side]:raise RuntimeError(f'{name} nondeterministic {side}')
                        timed_outputs[side]=result
                        timing[side].append(seconds)
            ratio=(statistics.median(timing['rust'])/statistics.median(timing['cpp'])) if timing['cpp'] else None
            moss_ratio=(statistics.median(timing['rust'])/statistics.median(timing['moss_cpp'])) if timing['moss_cpp'] else None
            records.append({'case':name,'mesh_sha256':sha(mesh),**meta,
                            'cpp_sha256':hashlib.sha256(cpp).hexdigest(),
                            'moss_cpp_sha256':hashlib.sha256(moss_cpp).hexdigest(),
                            'moss_cpp_matches_scalar':moss_cpp==cpp,
                            'rust_sha256':hashlib.sha256(rust).hexdigest(),
                            'bytes':len(cpp),'group_count':struct.unpack_from('<I',cpp,32)[0],
                            'dag_depth':struct.unpack_from('<I',cpp,40)[0]-1,
                            'mismatch':mismatch,'times_s':timing,'median_ratio':ratio,'moss_ratio':moss_ratio})
            print(name,'MATCH' if mismatch is None else f'MISMATCH {mismatch}',f'ratio={ratio}',f'moss_ratio={moss_ratio}',flush=True)
        total_cpp=sum(statistics.median(r['times_s']['cpp']) for r in records if r['times_s']['cpp'])
        total_moss=sum(statistics.median(r['times_s']['moss_cpp']) for r in records if r['times_s']['moss_cpp'])
        total_rust=sum(statistics.median(r['times_s']['rust']) for r in records if r['times_s']['rust'])
        identity={'sources':sources,'executables':{k:sha(v) for k,v in binaries.items()},
                  'core':args.core,'machine':platform.platform(),
                  'core_selection':selection,
                  'load_before':load_before,'load_after':os.getloadavg(),
                  'core_utilization':utilization(cpu_before,cpu_snapshot(),args.core),
                  'cpp_flags':'-O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD -fno-fast-math -ffp-contract=off',
                  'moss_cpp_flags':'-O3 -DNDEBUG -fPIC -ffunction-sections -fdata-sections -m64; SIMD enabled, mirroring cc release defaults',
                  'rust_profile':('consumer opt-level=3 thin LTO codegen-units=1' if args.rust_profile=='consumer'
                                  else 'Cargo release defaults: opt-level=3, 16 codegen units, no LTO'),
                  'timing':'internal monotonic ns; input parse plus S2 bridge/Rust equivalent; excludes pipe transfer, process launch, detailed bounds'}
        assert sources=={p:sha(p) for p in sources} and identity['executables']=={k:sha(v) for k,v in binaries.items()}
        result={'schema':'rfc113-clod/2','cases':records,'timing':{'cpp_sum_medians_s':total_cpp,
                'moss_cpp_sum_medians_s':total_moss,
                'rust_sum_medians_s':total_rust,'ratio':total_rust/total_cpp if total_cpp else None,
                'moss_ratio':total_rust/total_moss if total_moss else None,
                'bar':'<= approximately 1.2x'},'identity':identity}
        filename = 'result.json' if args.rust_profile=='consumer' else 'result-defaults.json'
        (ART/filename).write_text(json.dumps(result,indent=2)+'\n')
        if args.pairs >= 20 and args.rust_profile == 'consumer':
            if any(r['mismatch'] is not None and r['stride'] != 32 for r in records):
                raise RuntimeError('RFC113 byte parity failed')
            if total_cpp and (total_rust / total_cpp > 1.2 or any(
                r['median_ratio'] is not None and r['median_ratio'] > 1.5 for r in records
            )):
                raise RuntimeError('RFC113 scalar cook bar failed')
    finally:
        for d in drivers.values():d.close()

if __name__=='__main__':main()
