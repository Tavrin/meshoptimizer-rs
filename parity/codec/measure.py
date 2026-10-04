#!/usr/bin/env python3
"""Decoder pre-registration and candidate measurement; persistent paired drivers."""
import argparse
import hashlib
import itertools
import json
import math
import os
from pathlib import Path
import platform
import random
import statistics
import struct
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
REF = Path(os.environ['MESHOPT_REFERENCE'])
TARGET = Path(os.environ['CARGO_TARGET_DIR'])
ART = Path(os.environ['MESHOPT_ARTIFACTS'])
ENV = dict(os.environ, RUSTFLAGS='', CARGO_ENCODED_RUSTFLAGS='', RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='', CARGO_NET_OFFLINE='true', TMPDIR=str(TARGET/'tmp'))
FLAGS = ['-std=c++17', '-O3', '-DNDEBUG', '-fno-fast-math', '-ffp-contract=off']

def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def cmd(args): subprocess.run(list(map(str,args)),env=ENV,check=True)
def build_cpp(scalar=False):
    TARGET.mkdir(parents=True,exist_ok=True);(TARGET/'tmp').mkdir(exist_ok=True)
    cmd([ROOT/'parity/check-reference.sh'])
    out=TARGET/('codec-scalar' if scalar else 'codec-simd')
    cmd([os.environ.get('CXX','c++'),*FLAGS,*(['-DMESHOPTIMIZER_NO_SIMD'] if scalar else []),'-I',REF/'src',ROOT/'parity/codec/reference.cpp',*[REF/'src'/f for f in ['vertexcodec.cpp','indexcodec.cpp','vertexfilter.cpp','allocator.cpp']],'-o',out])
    return out

def request(op,count,stride,data,mode=0,filter=0,version=0,level=2,samples=0,iterations=1):
    return b'MC02'+struct.pack('<10I',op,count,stride,mode,filter,version,level,samples,iterations,len(data))+data
def decode(b):
    assert b[:4]==b'MD02';status,n,samples=struct.unpack_from('<iII',b,4)
    assert len(b)==16+n+samples*8
    return status,b[16:16+n],list(struct.unpack_from('<'+'d'*samples,b,16+n))
class Driver:
    def __init__(self,binary):
        cpu=int(os.environ.get('MESHOPT_BENCH_CPU',min(os.sched_getaffinity(0))))
        assert cpu in os.sched_getaffinity(0)
        self.p=subprocess.Popen(['taskset','-c',str(cpu),str(binary)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,env=ENV)
    def call(self,b):
        self.p.stdin.write(struct.pack('<I',len(b))+b);self.p.stdin.flush()
        header=self.p.stdout.read(4)
        if len(header)!=4: raise RuntimeError('driver closed')
        n=struct.unpack('<I',header)[0];data=bytearray()
        while len(data)<n:
            chunk=self.p.stdout.read(n-len(data))
            if not chunk: raise RuntimeError('truncated response')
            data+=chunk
        return decode(data)
    def close(self):
        self.p.stdin.close();assert self.p.wait()==0

def corpus(cpp):
    cases=[]
    for count,label in [(17,'tiny'),(4097,'resident'),(2097153,'streaming')]:
        for stride in [4,12,32]:
            data=bytes((i//stride+(i%stride)*13)&255 for i in range(count*stride))
            status,encoded,_=cpp.call(request(11,count,stride,data));assert status==0
            cases.append((f'vertex-v0-{label}-s{stride}',request(1,count,stride,encoded)))
            cases.append((f'view-none-{label}-s{stride}',request(7,count,stride,encoded)))
        for f,stride in [(1,4),(1,8),(2,8),(3,12)]:
            float_count=4 if f in (1,2) else stride//4
            values=[v for i in range(count) for v in ([0.,0.,1.,1.] if f in (1,2) else [float(i%100)/10,0.25,-3.5])]
            data=struct.pack('<'+'f'*len(values),*values)
            status,filtered,_=cpp.call(request(13+f,count,stride,data,level=8 if stride==4 else 12));assert status==0
            status,encoded,_=cpp.call(request(11,count,stride,filtered));assert status==0
            cases.append((f'filter-{f}-{label}-s{stride}',request(3+f,count,stride,filtered)))
            cases.append((f'view-{f}-{label}-s{stride}',request(7,count,stride,encoded,filter=f)))
        for op in [2,3]:
            n=count if op==3 else count//3*3
            data=struct.pack('<'+'I'*n,*[i%60000 for i in range(n)])
            for version in [0,1]:
                status,encoded,_=cpp.call(request(10+op,n,4,data,version=version));assert status==0
                for stride in [2,4]: cases.append((f'index-{op}-v{version}-{label}-s{stride}',request(op,n,stride,encoded)))
    return cases

def stats(values):
    median=statistics.median(values)
    return {'median':median,'min':min(values),'max':max(values),'stdev':statistics.stdev(values),'mad':statistics.median(abs(v-median) for v in values),'cv':statistics.stdev(values)/statistics.mean(values)}

def measure(cases,binaries,samples=12,target_seconds=0.008,adaptive=False):
    drivers={k:Driver(v) for k,v in binaries.items()}; records=[]
    for name,b in cases:
        outbytes=struct.unpack_from('<I',b,8)[0]*struct.unpack_from('<I',b,12)[0]
        probe=bytearray(b);struct.pack_into('<I',probe,32,1)
        trial=drivers['simd'].call(probe)[2][0]
        iterations=max(1,min(1000000 if adaptive else 10000,math.ceil(target_seconds/max(trial,1e-9))))
        struct.pack_into('<I',probe,36,iterations)
        raw={k:[] for k in binaries}; hashes={}
        orders=list(itertools.permutations(drivers))
        limit=samples;s=0
        while s<limit:
            names=orders[s%len(orders)] if adaptive else (list(drivers) if s%2==0 else list(drivers)[::-1])
            for k in names:
                status,data,t=drivers[k].call(probe);assert status==0,(name,k,status)
                raw[k]+=t;hashes[k]=hashlib.sha256(data).hexdigest()
            s+=1
            if adaptive and s==limit and limit<60:
                cv=max(stats(v)['cv'] for v in raw.values())
                ratios=[x/y for x,y in zip(raw['rust'],raw['scalar'])]
                logs=[math.log(v) for v in ratios];center=statistics.mean(logs);uncertainty=1.96*statistics.stdev(logs)/math.sqrt(len(logs))
                crosses=math.exp(center-uncertainty)<=1.50<=math.exp(center+uncertainty)
                if cv>0.15 or crosses:limit+=12
        row={'case':name,'decoded_bytes':outbytes,'iterations':iterations,'raw_seconds':raw,'statistics':{k:stats(v) for k,v in raw.items()},'output_sha256':hashes}
        row['throughput_bytes_second']={k:outbytes/v['median'] for k,v in row['statistics'].items()}
        if 'moss' in raw:
            row['moss_simd_throughput_ratio']=row['throughput_bytes_second']['moss']/row['throughput_bytes_second']['simd']
            # Fixed non-regression rule, registered before candidate measurements:
            # 95% of the worst baseline sample, excluding only stated dispersion.
            row['minimum_bytes_second']=0.95*outbytes/max(raw['moss'])
        records.append(row);print(name,{k:round(v/1e6,2) for k,v in row['throughput_bytes_second'].items()},flush=True)
    for driver in drivers.values():driver.close()
    return records

def baseline():
    ART.mkdir(parents=True,exist_ok=True)
    if not (ART/'moss_codec.rs').exists():
        moss=Path(os.environ['MESHOPT_MOSS_REFERENCE'])
        source=subprocess.check_output(['git','--no-optional-locks','-C',moss,'show','dc4af42a5e94f8a0f22932c53977f66cd88aadc2:crates/moss_asset_loader/src/meshopt_codec.rs'])
        (ART/'moss_codec.rs').write_bytes(source)
    if (ROOT/'parity/DECODER_BAR.md').exists():raise ValueError('registered bar already exists; do not overwrite')
    scratch=ART/'baseline';scratch.mkdir(exist_ok=True)
    source=(ART/'moss_codec.rs').read_text().split('#[cfg(test)]')[0]
    for fn in ['decode_vertex_buffer','decode_index_buffer','decode_index_sequence','filter_octahedral','filter_quaternion','filter_exponential']:
        source=source.replace('fn '+fn+'(', 'pub fn '+fn+'(')
    (scratch/'moss.rs').write_text(source)
    (scratch/'main.rs').write_bytes((ROOT/'parity/codec/baseline.rs').read_bytes())
    (scratch/'Cargo.toml').write_text('[package]\nname="meshopt-moss-baseline"\nversion="0.0.0"\nedition="2021"\n[[bin]]\nname="moss-baseline"\npath="main.rs"\n[profile.release]\ncodegen-units=1\nlto="fat"\n')
    cmd(['cargo','build','--release','--offline','--manifest-path',scratch/'Cargo.toml'])
    simd=build_cpp();scalar=build_cpp(True)
    if (ART/'benchmark-inputs.json').exists():
        manifest=json.loads((ART/'benchmark-inputs.json').read_text())
        cases=[]
        for entry in manifest:
            assert sha(entry['path'])==entry['sha256']
            cases.append((entry['case'],Path(entry['path']).read_bytes()))
    else:
        driver=Driver(simd);cases=corpus(driver);driver.close()
    manifest=[]
    for name,b in cases:
        p=ART/(name+'.input');p.write_bytes(b);manifest.append({'case':name,'path':str(p),'sha256':sha(p)})
    (ART/'benchmark-inputs.json').write_text(json.dumps(manifest,indent=2))
    binaries={'moss':TARGET/'release/moss-baseline','simd':simd,'scalar':scalar}
    before={k:sha(v) for k,v in binaries.items()}
    rows=measure(cases,binaries)
    assert before=={k:sha(v) for k,v in binaries.items()}
    record={'phase':'0.2','registered_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),'source_revision':'dc4af42a5e94f8a0f22932c53977f66cd88aadc2','moss_source_sha256':sha(ART/'moss_codec.rs'),'binary_sha256':before,'cpu':min(os.sched_getaffinity(0)),'hardware':platform.machine(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cxx_flags':FLAGS,'scalar_flags':FLAGS+['-DMESHOPTIMIZER_NO_SIMD'],'raw':rows,'input_manifest_sha256':sha(ART/'benchmark-inputs.json'),'uncertainty_rule':'minimum = 0.95 * decoded_bytes / maximum measured Moss seconds; 12 alternating paired samples; shared host load; worst sample plus 5% margin'}
    (ART/'baseline.json').write_text(json.dumps(record,indent=2))
    lines=['# P02 registered decoder bar','',f'Registered {record["registered_utc"]}, before port measurement.','',f'Moss commit `{record["source_revision"]}`, source SHA-256 `{record["moss_source_sha256"]}`.','',f'Baseline evidence: `{ART}/baseline.json`, SHA-256 `{sha(ART/"baseline.json")}`.','',record['uncertainty_rule']+'. Minima apply to allocating and caller-buffer APIs on this Linux x86-64 target. Raw v1 is measured separately against scalar C++; it has no Moss baseline.','', 'Optimized C++ uses enabled upstream SIMD; scalar canonical output is separate. Below 80% ships safe scalar under RFC 6.2, subject to these measured non-regression minima.','', '| Case | Moss MB/s | SIMD MB/s | Moss/SIMD | Minimum MB/s |','|---|---:|---:|---:|---:|']
    for r in rows:
        t=r['throughput_bytes_second'];lines.append(f'| {r["case"]} | {t["moss"]/1e6:.6f} | {t["simd"]/1e6:.6f} | {r["moss_simd_throughput_ratio"]:.6f} | {r["minimum_bytes_second"]/1e6:.6f} |')
    (ROOT/'parity/DECODER_BAR.md').write_text('\n'.join(lines)+'\n')
    print('BAR REGISTERED',sha(ROOT/'parity/DECODER_BAR.md'),flush=True)

def candidate():
    from runner import build,sources
    bar=ROOT/'parity/DECODER_BAR.md'
    if not bar.exists():raise ValueError('register the baseline before candidate measurement')
    registered=sha(bar);baseline_record=json.loads((ART/'baseline.json').read_text())
    assert baseline_record['registered_utc'] < time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())
    binaries=build();before=sources();identities={k:sha(v) for k,v in binaries.items()}
    cases=[]
    for entry in json.loads((ART/'benchmark-inputs.json').read_text()):
        assert sha(entry['path'])==entry['sha256']
        cases.append((entry['case'],Path(entry['path']).read_bytes()))
    # Extend the matrix with v1 on identical decoded data; no Moss result
    # is fabricated for its unsupported raw version.
    cpp=Driver(binaries['scalar'])
    for name,b in list(cases):
        if name.startswith('vertex-v0'):
            _,data,_=cpp.call(b);count,stride=struct.unpack_from('<II',b,8)
            _,encoded,_=cpp.call(request(11,count,stride,data,version=1,level=2))
            cases.append((name.replace('-v0-','-v1-'),request(1,count,stride,encoded)))
    cpp.close()
    cpp=Driver(binaries['scalar'])
    resident=next(b for name,b in cases if name=='vertex-v0-resident-s12')
    _,data,_=cpp.call(resident);count,stride=struct.unpack_from('<II',resident,8)
    for version in [0,1]:
        for level in [0,9]:
            _,encoded,_=cpp.call(request(11,count,stride,data,version=version,level=level))
            cases.append((f'vertex-v{version}-resident-s12-level{level}',request(1,count,stride,encoded)))
    # A triangle buffer with over two million triangles, independent of
    # the attribute streaming cases and without inferring a Moss baseline.
    n=6291456;indices=struct.pack('<'+'I'*n,*[i%60000 for i in range(n)])
    for version in [0,1]:
        _,encoded,_=cpp.call(request(12,n,4,indices,version=version))
        cases.append((f'index-2-v{version}-millions-s4',request(2,n,4,encoded)))
    cpp.close()
    allocating=measure(cases,{k:binaries[k] for k in ['rust','simd','scalar']},target_seconds=0.04,adaptive=True)
    into=[]
    for name,b in cases:
        req=bytearray(b);struct.pack_into('<I',req,16,struct.unpack_from('<I',b,16)[0]|128)
        into.append((name,req))
    caller=measure(into,{k:binaries[k] for k in ['rust','simd','scalar']},target_seconds=0.04,adaptive=True)
    minima={r['case']:r['minimum_bytes_second'] for r in baseline_record['raw']}
    verdicts={};raw_times={}
    for api,rows in [('allocating',allocating),('caller_buffer',caller)]:
        failures=[];ratios=[]
        for row in rows:
            t=row['throughput_bytes_second'];row['rust_simd_throughput_ratio']=t['rust']/t['simd']
            row['rust_scalar_time_ratio']=t['scalar']/t['rust']
            if row['case'] in minima:
                row['minimum_bytes_second']=minima[row['case']];row['bar_pass']=t['rust']>=minima[row['case']]
                if not row['bar_pass']:failures.append(row['case'])
            if row['case'].startswith(('vertex-','index-')):ratios.append(row['rust_scalar_time_ratio'])
        gm=statistics.geometric_mean(ratios);maximum=max(ratios)
        raw_times[api]={'geometric_mean_rust_scalar_time_ratio':gm,'maximum_rust_scalar_time_ratio':maximum,'pass':gm<=1.25 and maximum<=1.50}
        verdicts[api]={'failed_registered_minima':failures,'minimum_bar_pass':not failures,'raw_scalar_bar':raw_times[api]}
    assert before==sources() and registered==sha(bar) and identities=={k:sha(v) for k,v in binaries.items()}
    record={'phase':'0.2','candidate_measured_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),'registered_bar_sha256':registered,'baseline_sha256':sha(ART/'baseline.json'),'sources':before,'executables':identities,'allocating':allocating,'caller_buffer':caller,'verdicts':verdicts,'single_thread_cpu':int(os.environ.get('MESHOPT_BENCH_CPU',min(os.sched_getaffinity(0)))),'load_after':os.getloadavg(),'stack_scratch_excluded':True,'heap_memory_bar':'raw into has zero heap scratch in Rust and C++; allocating output bytes match; retained Workspace capacity counted; no C++ global allocator used by raw decoders'}
    (ART/'candidate.json').write_text(json.dumps(record,indent=2))
    lines=['# P02 measured decoder performance','',f'Bar SHA-256 `{registered}`. Evidence `{ART}/candidate.json`.','', 'At least twelve paired samples, increasing to sixty when dispersion exceeds 15% or uncertainty crosses a gate. Samples target 40 ms and rotate all backend orders. Validation, required copies and allocation timed; driver startup, I/O, generation and serialization excluded. Caller buffers are allocated before timing. Source and executable identities checked before and after. Linux x86-64 only; shared host load.','', 'Safe scalar remains the selected implementation under RFC 6.2. SIMD filter comparisons use decoded-unit conformance; canonical output follows scalar C++.','', 'Standalone Oct/Quat timing inputs contain repeated encoded directions/quaternions. Exact last-record reuse explains their throughput; these ratios do not establish throughput on varied filter data. Varied filters are covered by correctness sweeps. The registered Moss minima remain unchanged. The C++ standalone filter allocation was corrected to copy-construct without redundant zeroing; the separate baseline correction record retains paired measurements and does not change any minimum.','', '| API | Case | Rust MB/s | SIMD MB/s | Rust/SIMD | Rust/scalar time | Min bar |','|---|---|---:|---:|---:|---:|---|']
    for api,rows in [('allocating',allocating),('caller_buffer',caller)]:
        for row in rows:
            t=row['throughput_bytes_second'];lines.append(f'| {api} | {row["case"]} | {t["rust"]/1e6:.3f} | {t["simd"]/1e6:.3f} | {row["rust_simd_throughput_ratio"]:.3f} | {row["rust_scalar_time_ratio"]:.3f} | {row.get("bar_pass","raw v1")} |')
    lines+=['','Verdicts: `'+json.dumps(verdicts)+'`.']
    (ROOT/'parity/P02_PERFORMANCE.md').write_text('\n'.join(lines)+'\n')
    print('CANDIDATE VERDICT',verdicts,flush=True)
    if not all(v['minimum_bar_pass'] and v['raw_scalar_bar']['pass'] for v in verdicts.values()):raise SystemExit(1)

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--baseline',action='store_true');parser.add_argument('--phase',default='0.2');parser.add_argument('--enforce',action='store_true');parser.add_argument('--register-decoder-bars',action='store_true');args=parser.parse_args()
    if args.phase!='0.2':raise ValueError('unsupported phase')
    if args.baseline or args.register_decoder_bars:baseline()
    else:candidate()
