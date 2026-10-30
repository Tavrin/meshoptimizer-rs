"""CPU-admitted whole-scene timing, independent renders and unchanged C++ checks."""
import hashlib
import json
import os
import statistics
import struct
import subprocess
import time
from pathlib import Path

root=Path.cwd(); receipt=root/'parity/0217-hsr/eval'
target=Path(os.environ['CARGO_TARGET_DIR']); scratch=target/'evaluation'
scenes=Path(os.environ['HSR_SCENES'])
scratch.mkdir(parents=True,exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU','0')=='0'
os.environ.update(CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',LC_ALL='C',GIT_OPTIONAL_LOCKS='0',PYTHONDONTWRITEBYTECODE='1')
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def run(name,args,**kwargs):
    start=time.monotonic()
    p=subprocess.run(list(map(str,args)),text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,**kwargs)
    (receipt/(name+'.log')).write_text(p.stdout.replace(str(root),'<worktree>').replace(str(target),'<target>').replace(str(scenes),'<scene-scratch>'))
    with (receipt/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\t{time.monotonic()-start:.3f}\n')
    print(name,p.returncode,p.stdout[-800:],flush=True)
    if p.returncode:raise SystemExit(p.returncode)
    return p.stdout
run('checks-retry',['python3',receipt/'checks.py'])
run('prepare',['python3',receipt/'prepare.py'])
manifest=scratch/'Cargo.toml'
manifest.write_text(f'''[package]
name="hsr-eval"
version="0.0.0"
edition="2021"
[dependencies]
meshoptimizer-rs={{path={json.dumps(str(root))}}}
[[bin]]
name="hsr-eval"
path={json.dumps(str(receipt/'bench.rs'))}
[[bin]]
name="hsr-render"
path={json.dumps(str(receipt/'render.rs'))}
[profile.release]
codegen-units=1
lto="fat"
overflow-checks=true
''')
run('build',['cargo','build','--offline','--release','--manifest-path',manifest])
# Positive control: the visual metric must detect deletion of an exposed face.
control=scenes/'render-control.bin'
control.write_bytes(struct.pack('<II9f3I',3,3,-1.,-1.,0.,1.,-1.,0.,0.,1.,0.,0,1,2))
empty=scenes/'render-control-empty.out';empty.write_bytes(b'')
check=subprocess.run([str(target/'release/hsr-render'),str(control),str(empty),str(scenes/'render-control')],capture_output=True,text=True)
assert check.returncode==1 and json.loads(check.stdout)['changed_pixels']>0,check
(receipt/'render-control.json').write_text(json.dumps(dict(expected_exit=1,actual_exit=check.returncode,**json.loads(check.stdout)),indent=2)+'\n')
cpu=min(os.sched_getaffinity(0)); rows=[]; outputs={}; visuals={}
for scene in json.loads((receipt/'scenes.json').read_text()):
    name=scene['name']; mesh=scenes/(name+'.bin')
    assert sha(mesh)==scene['mesh_sha256']
    with mesh.open('rb') as f:
        nv,ni=struct.unpack('<II',f.read(8))
        p=[v for v in struct.iter_unpack('<3f',f.read(nv*12))]
    side=max(max(v[k] for v in p)-min(v[k] for v in p) for k in range(3))
    del p
    arms=[('hidden','coarse',side/64),('hidden','default',0.),('hidden','fine',side/256),('simplify','simplify',0.)]
    for rep in range(3):
        for op,density,pixel_size in arms if rep!=1 else list(reversed(arms)):
            label=f'{name}-{density}-{rep}'; out=scenes/(label+'.out')
            log=run(label,['taskset','-c',cpu,target/'release/hsr-eval',op,mesh,out],
                    input='1\nstop\n',env=dict(os.environ,HSR_PIXEL_SIZE=str(pixel_size)))
            lines=log.splitlines();report=json.loads(next(s.split(' ',2)[2] for s in lines if s.startswith('RESULT ')))
            seconds=float(next(s.split()[1] for s in lines if s.startswith('COUNTED ')))
            digest=sha(out);key=(name,density)
            if key in outputs:assert digest==outputs[key]
            outputs[key]=digest
            rows.append(dict(scene=name,op=op,density=density,rep=rep,pixel_size=pixel_size,
                resolution={'coarse':128,'default':256,'fine':512,'simplify':None}[density],
                seconds=seconds,report=report,output_sha256=digest))
            (receipt/'samples.json').write_text(json.dumps(rows,indent=2)+'\n')
            if op=='hidden' and rep==0:
                # All binary depth images stay beside the scene data in scratch.
                prefix=scenes/(name+'-'+density)
                visual=json.loads(run(label+'-render',[target/'release/hsr-render',mesh,out,prefix]).splitlines()[-1])
                visual['image_sha256']={p.name:sha(p) for p in scenes.glob(name+'-'+density+'-*.pgm')}
                visuals[name+'-'+density]=visual
                (receipt/'visuals.json').write_text(json.dumps(visuals,indent=2)+'\n')
summary=[]
for scene in json.loads((receipt/'scenes.json').read_text()):
    name=scene['name']
    simplify=statistics.median(r['seconds'] for r in rows if r['scene']==name and r['op']=='simplify')
    for density in ['coarse','default','fine']:
        cell=[r for r in rows if r['scene']==name and r['density']==density]
        r=cell[0];report=r['report']; elapsed=statistics.median(r['seconds'] for r in cell)
        summary.append(dict(scene=name,density=density,resolution=r['resolution'],pixel_size=r['pixel_size'],
            triangles=scene['triangles'],removed_percent=100*report['removed']/report['input'],report=report,
            seconds=elapsed,simplify_seconds=simplify,hsr_vs_simplify=elapsed/simplify,
            changed_pixels=visuals[name+'-'+density]['changed_pixels']))
(receipt/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(receipt/'environment.json').write_text(json.dumps(dict(cpu=cpu,rustc=subprocess.check_output(['rustc','-vV'],text=True),
    repetitions=3,order='coarse/default/fine/simplify; reversed in repetition 2',
    timing='one warm resident call per process, parsing/initial warmup/emission outside timer; monotonic wall time; single thread',
    workspace_limits=dict(max_bytes=3<<30,max_work=1<<34),
    simplify_settings=dict(target_fraction=0.5,target_error=0.5,options='EMPTY'),
    render='independent two-sided CPU center-sample depth, 8 orthographic +/-1 corner directions, 512x512, 16-bit quantized depth incl coverage; exact pixel comparison',
    binary_sha256={name:sha(target/'release'/name) for name in ['hsr-eval','hsr-render']}),indent=2)+'\n')
run('oracle',['python3',receipt/'oracle.py'])
print('PASS scene campaign and unchanged oracle checks',flush=True)
