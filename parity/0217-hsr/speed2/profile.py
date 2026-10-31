"""Admitted phase profile of the pinned reference, without editing its source."""
import io,json,os,subprocess,tarfile,time
from pathlib import Path
root=Path.cwd(); receipt=root/'parity/0217-hsr/speed2'; target=Path(os.environ['CARGO_TARGET_DIR']); scratch=target/'profile'; scratch.mkdir(parents=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE')
base=scratch/'reference';base.mkdir()
raw=subprocess.check_output(['git','archive','f9859cc','Cargo.toml','Cargo.lock','src','docs','README.md'])
with tarfile.open(fileobj=io.BytesIO(raw)) as archive: archive.extractall(base,filter='data')
p=base/'src/hidden.rs';s=p.read_text()
s=s.replace('    let mut min = [f64::INFINITY; 3];\n    let mut max', '    let mut min = [f64::INFINITY; 3];\n    let mut max')
s=s.replace('    let guard = epsilon + side * 1e-12;', '    let guard = epsilon + side * 1e-12;\n    let raster_start = std::time::Instant::now();\n    let mut raster_time = 0.0;\n    let mut query_time = 0.0;')
s=s.replace('        work.add(occluders.len())?;', '        let phase = std::time::Instant::now();\n        work.add(occluders.len())?;')
s=s.replace('        work.add(triangles.len())?;\n        for t in triangles.iter() {\n            if !keep[t.id] || visible[t.id]', '        raster_time += phase.elapsed().as_secs_f64();\n        let phase = std::time::Instant::now();\n        work.add(triangles.len())?;\n        for t in triangles.iter() {\n            if !keep[t.id] || visible[t.id]')
s=s.replace('    budget.release(depth);', '        query_time += phase.elapsed().as_secs_f64();\n    }\n    budget.release(depth);',1).replace('        }\n    }\n        query_time', '        }\n        query_time',1)
s=s.replace('    if remaining != 0 {\n        // Preserve', '    eprintln!("PROFILE raster={raster_time:.6} query={query_time:.6} outside={:.6} candidates={remaining}", raster_start.elapsed().as_secs_f64());\n    let ray_start = std::time::Instant::now();\n    if remaining != 0 {\n        // Preserve')
s=s.replace('    work.add(keep.len())?;\n    for (k, v)', '    eprintln!("PROFILE rays={:.6}", ray_start.elapsed().as_secs_f64());\n    work.add(keep.len())?;\n    for (k, v)')
p.write_text(s)
(scratch/'Cargo.toml').write_text(f'''[package]
name="hsr-profile"
version="0.0.0"
edition="2021"
[dependencies]
meshoptimizer-rs={{path={json.dumps(str(base))}}}
[[bin]]
name="hsr-profile"
path={json.dumps(str(root/'parity/0217-hsr/eval/bench.rs'))}
[profile.release]
lto="fat"
codegen-units=1
debug=1
''')
def run(name,args,**kw):
    p=subprocess.run(list(map(str,args)),capture_output=True,text=True,**kw)
    (receipt/(name+'.log')).write_text((p.stdout+p.stderr).replace(str(root),'<worktree>').replace(str(target),'<target>').replace(str(Path(os.environ['HSR_SCENES'])),'<scenes>'))
    with (receipt/'profile-exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\n')
    print(name,p.returncode,p.stdout+p.stderr,flush=True)
    if p.returncode:raise SystemExit(p.returncode)
run('profile-build',['cargo','build','--release','--offline','--manifest-path',scratch/'Cargo.toml'])
cpu=min(os.sched_getaffinity(0))
for scene in ['breakfast_room','conference','living_room']:
    run('profile-'+scene,['taskset','-c',cpu,target/'release/hsr-profile','hidden',Path(os.environ['HSR_SCENES'])/(scene+'.bin'),scratch/(scene+'.out')],input='stop\n')
