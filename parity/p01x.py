"""Preprocessing qualification, protocol 2; source and byte identities are retained."""
import array
import contextlib
import json
import math
import os
import random
import statistics
import struct
import subprocess
import sys
import time
import zipfile
import runner as r

FAMILIES = dict(enumerate([
    'vertex_cache_strip','vertex_cache_fifo','generate_vertex_remap','generate_vertex_remap_multi',
    'generate_vertex_remap_custom','remap_vertex_buffer','remap_index_buffer','filter_index_buffer',
    'filter_index_buffer_multi','generate_shadow_index_buffer','generate_shadow_index_buffer_multi',
    'generate_position_remap','generate_adjacency_index_buffer','generate_tessellation_index_buffer',
    'generate_provoking_index_buffer','vertex_fetch','vertex_fetch_remap','simplify_sloppy',
    'simplify_prune','simplify_points','simplify_with_update','quantize_unorm','quantize_snorm',
    'quantize_half','quantize_float','dequantize_half','compute_position_exponent','simplify_sparse',
    'simplify_prune_option','simplify_preserve_folds','simplify_error_clamped','simplify_regularize_light'], 7))

def bits(f): return struct.unpack('<I',struct.pack('<f',f))[0]

def message(op,p,ib,variant=0,mode=0,samples=0,attribute_count=None,target_ratio=None):
    vc=len(p)//3; rng=random.Random(20261003+variant)
    size=[1,3,4,8,12,16,33,64,256][variant%9]
    sc=1 if op in [9,12,14,16,22] else [1,2,3,16][variant%4] if op in [10,15,17] else 0
    stride=size if op in [9,12,22] else size+[0,1,7,64][variant%4]
    options=[0,1,2,4,8,16,32,64,33,36,96,128,256,160,288,384][variant%16]
    if op in range(34,39): options=[2,8,128,256,64][op-34] | (32 if variant%2 else 0)
    target=int(len(ib)*[0,.125,.25,.5,.9,1][variant%6]);p0=bits([0.,.001,.01,.1,.5,1.][variant%6]);p1=0
    if op==8:p0=[3,4,8,16,32,64,256,0xffffffff][variant%8]
    if op in [9,10,11,13] and variant%5==0:p1|=1
    if op==24 and variant%3==0:p1|=2
    if op==26:target=int(vc*[0,.125,.5,.9,1][variant%5]);p0=bits([0.,.1,1.,10.,-1.][variant%5]);p1=(variant%2)*2
    ac=[0,1,3,8,13,32][variant%6] if op==27 or op>=34 else 0
    if attribute_count is not None and (op==27 or op>=34):ac=attribute_count
    if target_ratio is not None:
        target=int((vc if op==26 else len(ib))*target_ratio);p0=bits(max(1-target_ratio,.05))
    weights=[[0.,.1,1.,2.][(k+variant)%4] for k in range(ac)]
    attrs=[float((i*3+k)%17)/8 for i in range(vc) for k in range(ac)]
    flags=[1 if op==24 and i%7==0 else (i+variant)%8 if op==27 or op>=34 else 0 for i in range(vc)]
    streams=b''.join(bytes((int(abs(p[i*3+k%3])*32)+j*7+k)%256 for k in range(stride)) for j in range(sc) for i in range(vc))
    original_count=len(ib)
    if 28<=op<=32:
        vc=0;p=[];flags=[];size=1;stride=1;sc=0
        if op==32:ib=[0,1,0x3ff,0x400,0x3c00,0x7bff,0x7c00,0x7e00,0x8000,0xffff]+[rng.getrandbits(16)for _ in range(32)]
        else:ib=[bits(x)for x in [-math.inf,-1.,-0.,0.,.5,1.,math.inf]]+[0x1,0x80000001,0x7f800001,0x7fc01234,0xffffffff]+[rng.getrandbits(32)for _ in range(32)]
        if mode!=0:
            values=ib;ib=[values[i%len(values)] for i in range(max(32,original_count))]
        target=(variant%31)if op==28 else(1+variant%31)if op==29 else variant%24
    if op==33:
        minimum=[min(p[k::3],default=0.)for k in range(3)];maximum=[max(p[k::3],default=0.)for k in range(3)]
        p=minimum+maximum;vc=2;ib=[];flags=[0,0];p0=(-126+variant%140)&0xffffffff;p1=2+variant%23
    if mode==0 and variant%2 and op not in [27,28,29,30,31,32,33]:options|=0x80000000
    header=struct.pack('<4s13I',b'MO02',op,vc,len(ib),mode,samples,size,stride,sc,options,target,p0,p1,ac)
    return header+r.packed(p,'f')+r.packed(ib,'I')+streams+r.packed(weights,'f')+r.packed(attrs,'f')+r.packed(flags,'I')

def geometries():
    yield 'empty',[],[]
    yield 'triangle',[0.,0.,0.,1.,0.,0.,0.,1.,0.],[0,1,2]
    yield 'degenerate',[0.]*9,[0,0,0,0,1,1,1,0,1]
    yield 'signed-zero',[-0.,0.,0.,0.,-0.,0.,0.,0.,-0.],[0,1,2]*3
    yield 'signed-zero-disconnected',[-0.,0.,0.,1.,0.,0.,0.,1.,0.,0.,0.,0.,-1.,0.,0.,0.,-1.,0.],[0,1,2,3,4,5]
    yield 'duplicates',[0.,0.,0.,1.,0.,0.,0.,1.,0.],[0,1,2,1,2,0,2,1,0]*3
    for n in [2,32,128,1024]:
        p,ib=r.grid(n);yield f'grid-{n}',p,ib
    for split in [False,True]:
        p,ib=r.sphere(8,split);yield f'sphere-{split}',p,ib
    p,ib=r.grid(32);p=list(p)+[100.,200.,300.];yield 'unused-extreme',p,ib
    yield 'tiny-finite-scale',[0.,0.,0.,1e-30,0.,0.,0.,1e-30,0.],[0,1,2]
    yield 'large-finite-scale',[0.,0.,0.,1e30,0.,0.,0.,1e30,0.],[0,1,2]
    yield 'normalized-subnormal-feature',[0.,0.,0.,1.,0.,0.,0.,float.fromhex('0x1p-149'),0.,0.,1.,0.],[0,1,2]

def differential(args):
    reference,target,results=r.paths();binaries,identity=r.build(reference,target);artifacts=r.artifacts_path()
    action=args.action;record={'schema':1,'phase':'0.1.x','action':action,'seed':args.seed,'identities':identity,'counts':{f:0 for f in FAMILIES.values()},'cases':[],'mismatches':0,'passed':False}
    record['upstream_fixture_inventory']=[];record['intentional_deviations']=[]
    started=time.time();wasm=r.Wasm(binaries['wasm'])
    try:
        with zipfile.ZipFile(artifacts/f'p01x-{action}-buffers.zip','w',compression=zipfile.ZIP_DEFLATED,compresslevel=1) as archive:
            rng=random.Random(args.seed)
            inputs=geometries() if action=='run' else ((f'seed-{i}',*random_geometry(rng,i))for i in range(args.cases_per_family))
            for number,(label,p,ib)in enumerate(inputs):
                for op,family in FAMILIES.items():
                    data=message(op,p,ib,number)
                    outputs={name:r.execute(binaries[name],data)for name in ['cpp','rust']}
                    outputs['wasm']=wasm.execute(data)
                    case={'id':label,'family':family,'input':r.retain(archive,f'{family}/{label}.input',data),'outputs':{name:r.retain(archive,f'{family}/{label}-{name}.output',out)for name,out in outputs.items()},'match':len(set(outputs.values()))==1}
                    record['cases'].append(case);record['counts'][family]+=1
                    if not case['match']:
                        record['mismatches']+=1;(results/f'p01x-{action}.json').write_text(json.dumps(record,indent=2)+'\n');raise ValueError(f'preprocessing mismatch: {family}/{label}')
                if number%25==0: print(f'0.1.x {action}: {number+1} cases per function, exact native/C++/WASM',flush=True)
            if action=='run':
                for width in range(31):
                    values=[0x3f7fffff,0x3f800000,0x3f800001,0x7f800000,0x7fc01234,0x80000000]
                    data=struct.pack('<4s13I',b'MO02',28,0,len(values),0,0,1,1,0,0,width,0,0,0)+r.packed(values,'I')
                    outputs={name:r.execute(binaries[name],data)for name in ['cpp','rust']};outputs['wasm']=wasm.execute(data)
                    label=f'unorm-endpoints-{width}';family=FAMILIES[28]
                    case={'id':label,'family':family,'input':r.retain(archive,label+'.input',data),'outputs':{name:r.retain(archive,label+'-'+name+'.output',out)for name,out in outputs.items()},'match':len(set(outputs.values()))==1}
                    record['cases'].append(case);record['counts'][family]+=1
                    if not case['match']:
                        record['mismatches']+=1;(results/f'p01x-{action}.json').write_text(json.dumps(record,indent=2)+'\n');raise ValueError('UNorm endpoint mismatch')
                # Both sides of the fetch output strategy need native/WASM proof.
                packed_positions, typed_indices=r.grid(4096)
                typed_positions=list(packed_positions)+[0.]*len(packed_positions)
                for width,variant in [(4,2),(8,3),(12,4),(16,5)]:
                    data=bytearray(message(22,typed_positions,typed_indices,variant))
                    struct.pack_into('<I',data,36,struct.unpack_from('<I',data,36)[0]&0x7fffffff)
                    data=bytes(data);outputs={name:r.execute(binaries[name],data)for name in ['cpp','rust']};outputs['wasm']=wasm.execute(data)
                    label=f'typed-fetch-unused-width-{width}';family=FAMILIES[22]
                    case={'id':label,'family':family,'input':r.retain(archive,label+'.input',data),'outputs':{name:r.retain(archive,label+'-'+name+'.output',out)for name,out in outputs.items()},'match':len(set(outputs.values()))==1}
                    record['cases'].append(case);record['counts'][family]+=1
                    if not case['match']:raise ValueError('typed fetch fixture mismatch')
                # Upstream returns before normalization for a zero point target.
                extreme=message(26,[-float.fromhex('0x1.fffffep127'),0.,0.,float.fromhex('0x1.fffffep127'),0.,0.],[],0)
                outputs={name:r.execute(binaries[name],extreme)for name in ['cpp','rust']};outputs['wasm']=wasm.execute(extreme)
                label='zero-point-target-extreme-finite';family=FAMILIES[26]
                case={'id':label,'family':family,'input':r.retain(archive,label+'.input',extreme),'outputs':{name:r.retain(archive,label+'-'+name+'.output',out)for name,out in outputs.items()},'match':len(set(outputs.values()))==1}
                record['cases'].append(case);record['counts'][family]+=1
                if not case['match']:raise ValueError('zero point target mismatch')
                native_fixtures=sorted((target/'upstream-fixtures').glob('p01x-native-*.input'))
                js_fixtures=sorted((target/'upstream-fixtures').glob('p01x-js-*.input'))
                if len(native_fixtures)!=88 or len(js_fixtures)!=10:raise ValueError('missing preprocessing upstream fixture')
                for fixture in native_fixtures+js_fixtures:
                    data=fixture.read_bytes();op,=struct.unpack_from('<I',data,4);family=FAMILIES[op];label=fixture.stem
                    vc,=struct.unpack_from('<I',data,8);flags=struct.unpack_from(f'<{vc}I',data,len(data)-vc*4)
                    record['upstream_fixture_inventory'].append({'id':label,'family':family,'input':r.retain(archive,label+'.input',data)})
                    if any(f&~7 for f in flags):
                        status=subprocess.run([binaries['rust']],input=data,capture_output=True,env=r.ENV)
                        if status.returncode==0 or b'flag'not in status.stderr:raise ValueError('unknown flag fixture did not reject')
                        record['intentional_deviations'].append({'id':label,'classification':'typed Rust flags reject unknown bits even on unreferenced sparse vertices; upstream only reads referenced flags','rust_exit':status.returncode,'stderr':status.stderr.decode()});continue
                    outputs={name:r.execute(binaries[name],data)for name in ['cpp','rust']};outputs['wasm']=wasm.execute(data)
                    case={'id':label,'family':family,'input':record['upstream_fixture_inventory'][-1]['input'],'outputs':{name:r.retain(archive,label+'-'+name+'.output',out)for name,out in outputs.items()},'match':len(set(outputs.values()))==1}
                    record['cases'].append(case);record['counts'][family]+=1
                    if not case['match']:
                        record['mismatches']+=1;(results/f'p01x-{action}.json').write_text(json.dumps(record,indent=2)+'\n');raise ValueError(f'upstream preprocessing mismatch: {label}')
    finally:wasm.close()
    r.check_unchanged(reference,binaries,identity);record['artifacts']={f'p01x-{action}-buffers.zip':r.sha(artifacts/f'p01x-{action}-buffers.zip')};record['seconds']=time.time()-started;record['passed']=True
    (results/f'p01x-{action}.json').write_text(json.dumps(record,indent=2)+'\n')
    print(f'0.1.x {action}: passed {sum(record["counts"].values())} comparisons',flush=True)

def random_geometry(rng,number):
    if number%5==0:return r.grid(rng.randint(1,50)*2)
    if number%5==1:return r.sphere(rng.randint(3,7),number%2==0)
    count=rng.randint(0,60);triangles=rng.randint(0,100)if count else 0
    p=[float(rng.randint(-8,8)) / 4 for _ in range(count*3)]
    if number%7==0:p=[0.]*(count*3)
    ib=[rng.randrange(count)for _ in range(triangles*3)]
    return p,ib

def benchmark(args):
    import performance
    import quiet
    profile=args.consumer_profile
    overrides={'CARGO_PROFILE_RELEASE_OPT_LEVEL':'3','CARGO_PROFILE_RELEASE_DEBUG':'0',
               'CARGO_PROFILE_RELEASE_LTO':'thin'if profile=='moss'else'false',
               'CARGO_PROFILE_RELEASE_CODEGEN_UNITS':'1'if profile=='moss'else'16'}if profile!='local'else{}
    r.ENV.update(overrides)
    reference,target,results=r.paths();binaries,identity=r.build(reference,target,False);artifacts=r.artifacts_path()
    selection=quiet.physical_core();
    if os.environ.get('MESHOPT_BENCHMARK_CPU'):
        forced=int(os.environ['MESHOPT_BENCHMARK_CPU'])
        if forced!=selection['cpu']:raise ValueError('benchmark core override differs from measured core; pass a complete retained selection')
        selection['override']='reuse measured physical core and exclude both siblings from fuzz affinity'
        if selection['cpu']not in selection['allowed_cpus']:raise ValueError('unavailable benchmark core')
    r.benchmark_cpu=selection['cpu']
    record={'schema':1,'phase':'0.1.x','profile':'scalar-strict','consumer_profile':profile,'profile_overrides':overrides,'identities':identity,'cpu_selection':selection,'samples_minimum':10,'samples_maximum':30,'batch_repetitions':{'position_exponent':262144,'index_count_below_3000':32768,'index_count_below_300000':16,'otherwise':1},'timing':'validation, required copies, allocating output, scratch and execution; excludes input generation, transport output, serialization, I/O and startup','workloads':{},'families':{},'passed':False}
    with zipfile.ZipFile(artifacts/f'p01x-benchmark-{profile}-buffers.zip','w',compression=zipfile.ZIP_DEFLATED,compresslevel=1)as archive:
        for size,n in [('million',1000000),('tiny',32),('medium',8192)]:
            geometries={shape:performance.geometry(r,n,shape) for shape in ['smooth','seam-heavy','disconnected','sparse']}
            schedule=[(shape,op,FAMILIES[op]) for op in [22,24,9] for shape in geometries]
            schedule.extend((shape,op,family) for shape in geometries for op,family in FAMILIES.items() if op not in [22,24,9])
            for shape,op,family in schedule:
                p,ib=geometries[shape]
                for mode in [1,2]:
                    variants=[(None,None,'')]
                    if op==27 or op>=34:
                        extra={'smooth':(0,.25),'seam-heavy':(12,.75),'disconnected':(1,.5)}.get(shape)
                        if extra:variants.append((*extra,f'/a{extra[0]}-r{extra[1]}'))
                    if op==26 and shape in ['smooth','seam-heavy']:variants.append((None,.5,'/half-points'))
                    if op==26:variants.append((None,.5,'/colored-half-points'))
                    for ac,ratio,suffix in variants:
                        variant=3 if op==27 or op>=34 or suffix=='/colored-half-points' else 4
                        data=message(op,p,ib,variant,mode,100,ac,ratio);path=target/'p01x-paired.input';path.write_bytes(data)
                        samples={'rust':[],'cpp':[]};loads=[]
                        with contextlib.ExitStack()as cleanup:
                            backends={name:performance.Paired(r,binary,path)for name,binary in binaries.items()if name in ['rust','cpp']}
                            for backend in backends.values():cleanup.callback(backend.close)
                            pairs=10;i=0
                            while i<pairs:
                                loads.append(quiet.load())
                                for name in (['rust','cpp']if i%2==0 else ['cpp','rust']):
                                    seconds=backends[name].sample()
                                    if not math.isfinite(seconds) or seconds<=0:raise ValueError('invalid paired clock interval')
                                    samples[name].append(seconds)
                                i+=1
                                if i==pairs and i<30:
                                    ratios=sorted(a/b for a,b in zip(samples['rust'],samples['cpp']));lo,hi=ratios[len(ratios)//4],ratios[3*len(ratios)//4]
                                    if any(lo<=bar<=hi for bar in [1.25,1.5]):pairs+=10
                            outputs={name:backend.finish()for name,backend in backends.items()}
                        expected=None;memory={}
                        for name,out in outputs.items():
                            memory[name],=struct.unpack('<Q',out[-8:])
                            values,times=r.response(out[:-8],samples=len(samples[name]));
                            if times!=samples[name]:raise ValueError('paired sample stream mismatch')
                            if expected is not None and expected!=values:raise ValueError(f'benchmark mismatch: {family}')
                            expected=values
                        key=f'{family}/{size}/{shape}/mode-{mode}'+suffix;st=performance.paired_stats(samples)
                        memory_ratio=memory['rust']/memory['cpp']if memory['cpp']else(1.0 if memory['rust']==0 else None)
                        record['workloads'][key]={'family':family,'size':size,'shape':shape,'mode':mode,'attributes':struct.unpack_from('<I',data,52)[0],'target':struct.unpack_from('<I',data,40)[0],'paired_ratio_stats':st,'stats':performance.stats(samples),'loads':loads,'memory_bytes':memory,'memory_ratio':memory_ratio,'input':r.retain(archive,key+'.input',data),'outputs':{name:r.retain(archive,key+'-'+name+'.output',out)for name,out in outputs.items()}}
                        print(f'{key}: {st["median"]:.3f}',flush=True)
                        (results/f'p01x-benchmark-{profile}.partial.json').write_text(json.dumps(record,indent=2)+'\n')
    for family in FAMILIES.values():
        values=[w['paired_ratio_stats']['median']for w in record['workloads'].values()if w['family']==family];gm=math.exp(statistics.mean(math.log(v)for v in values));maximum=max(values)
        memory=max(w['memory_ratio']if w['memory_ratio']is not None else math.inf for w in record['workloads'].values()if w['family']==family)
        record['families'][family]={'geometric_mean':gm,'maximum':maximum,'maximum_memory_ratio':memory,'passed':gm<=1.25 and maximum<=1.5 and memory<=1.25}
    r.check_unchanged(reference,binaries,identity);record['artifacts']={f'p01x-benchmark-{profile}-buffers.zip':r.sha(artifacts/f'p01x-benchmark-{profile}-buffers.zip')};record['passed']=all(f['passed']for f in record['families'].values());(results/f'p01x-benchmark-{profile}.json').write_text(json.dumps(record,indent=2)+'\n')
    if args.enforce and not record['passed']:raise ValueError('preprocessing benchmark bar failed')
