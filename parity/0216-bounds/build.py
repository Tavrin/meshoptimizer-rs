"""Freeze/build P3 arms under admission; shared inputs are read-only."""
import hashlib, io, json, os, shutil, subprocess, sys, tarfile
from pathlib import Path
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
root=Path(__file__).resolve().parents[2]
s=Path(os.environ['BOUNDS_SCRATCH']); s.mkdir(exist_ok=True)
target=Path(os.environ['CARGO_TARGET_DIR'])
(s/'environment.json').write_text(json.dumps({'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cxx':subprocess.check_output(['c++','--version'],text=True),'flags':{k:os.environ.get(k,'') for k in ['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_INCREMENTAL','LC_ALL']},'affinity':sorted(os.sched_getaffinity(0))},indent=2)+'\n')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
if not (s/'base').exists():
    data=subprocess.check_output(['git','archive','v0.2.15'],cwd=root)
    (s/'base').mkdir()
    with tarfile.open(fileobj=io.BytesIO(data)) as t: t.extractall(s/'base',filter='data')
    (s/'baseline.json').write_text(json.dumps({'commit':subprocess.check_output(['git','rev-parse','v0.2.15^{commit}'],cwd=root,text=True).strip()}))
if not (s/'inputs').exists():
    (s/'inputs').symlink_to(Path(os.environ.get('BOUNDS_INPUTS', str(s.parent/'0216-remap/inputs'))),target_is_directory=True)
    shutil.copy2(Path(os.environ.get('BOUNDS_INPUTS', str(s.parent/'0216-remap/inputs'))).parent/'inputs.json',s/'inputs.json')
for arm in sys.argv[1:]:
    dest=s/arm
    if dest.exists(): raise SystemExit('refuse overwrite '+str(dest))
    source=s/'base' if arm.startswith('base') else root
    shutil.copytree(source/'src',dest/'src')
    shutil.copytree(source/'docs',dest/'docs')
    for f in ['Cargo.toml','Cargo.lock','README.md']: shutil.copy2(source/f,dest/f)
    d=dest/'parity/peers'; shutil.copytree(root/'parity/peers',d)
    subprocess.run(['python3',str(root/'parity/0215-contract-meshlets/prepare_driver.py'),str(d/'src/main.rs')],check=True)
    p=d/'src/main.rs'; code=p.read_text()
    code=code.replace('control("enable");for','control("enable");let start=Instant::now();for').replace('control("disable");println!("COUNTED")','let elapsed=start.elapsed().as_secs_f64();control("disable");println!("COUNTED {elapsed:.9}")')
    assert 'let start=Instant::now()' in code
    # Identical frozen baseline meshlets, localized outside counts.
    code=code.replace('struct Input {','struct Input { groups: Vec<BoundGroup>,')
    code=code.replace('let p=b.chunks_exact(12)', 'let p:Vec<[f32;3]>=b.chunks_exact(12)')
    code=code.replace('let i=words(&b);','let i=words(&b);let groups=if std::env::args().nth(1).is_some_and(|op|op.ends_with("bounds")){bound_groups(path,&p)}else{Vec::new()};')
    code=code.replace('Self{p,i,attrs,','Self{groups,p,i,attrs,')
    group_setup = r'''struct BoundGroup { p:Vec<[f32;3]>, i:Vec<u32>, v:Vec<u32>, t:Vec<u8> }
fn bound_groups(path:&str,p:&[[f32;3]])->Vec<BoundGroup>{
 let name=std::path::Path::new(path).file_name().unwrap();
 let path=std::path::Path::new(&std::env::var("BOUNDS_SCRATCH").unwrap()).join("bound-inputs").join(name);
 let mut f=std::io::BufReader::new(std::fs::File::open(path).unwrap());
 fn word(f:&mut impl Read)->usize{let mut b=[0;4];f.read_exact(&mut b).unwrap();u32::from_le_bytes(b)as usize}
 let count=word(&mut f);(0..count).map(|_|{let nv=word(&mut f);let nt=word(&mut f);let local=(0..nv).map(|_|p[word(&mut f)]).collect();let mut t=vec![0;nt];f.read_exact(&mut t).unwrap();BoundGroup{p:local,i:t.iter().map(|&v|v as u32).collect(),v:(0..nv as u32).collect(),t}}).collect()
}
'''
    code=code.replace('struct Input {',group_setup+'struct Input {',1)
    ours='''  "cluster-bounds"|"meshlet-bounds"=>{let mut o=Vec::new();for g in &x.groups{let p=m::Positions::from_packed(&g.p);let b=if op=="cluster-bounds"{m::compute_cluster_bounds(&g.i,p,w)}else{m::compute_meshlet_bounds(&g.v,&g.t,p,w)}.unwrap();o.extend(b.center.map(f32::to_bits));o.push(b.radius.to_bits());o.extend(b.cone_apex.map(f32::to_bits));o.extend(b.cone_axis.map(f32::to_bits));o.push(b.cone_cutoff.to_bits());o.extend(b.cone_axis_s8.map(|v|v as u32));o.push(b.cone_cutoff_s8 as u32);}Output::Indices(o)},\n'''
    cpp='''  "cluster-bounds"|"meshlet-bounds"=>{let mut o=Vec::new();for g in &x.groups{let b=if op=="cluster-bounds"{f::meshopt_computeClusterBounds(g.i.as_ptr(),g.i.len(),g.p.as_ptr().cast(),g.p.len(),12)}else{f::meshopt_computeMeshletBounds(g.v.as_ptr(),g.t.as_ptr(),g.t.len()/3,g.p.as_ptr().cast(),g.p.len(),12)};o.extend(b.center.map(f32::to_bits));o.push(b.radius.to_bits());o.extend(b.cone_apex.map(f32::to_bits));o.extend(b.cone_axis.map(f32::to_bits));o.push(b.cone_cutoff.to_bits());o.extend(b.cone_axis_s8.map(|v|v as u32));o.push(b.cone_cutoff_s8 as u32);}Output::Indices(o)},\n'''
    code=code.replace('  "vertex-encode"=>Output::Bytes(m::codec',ours+'  "vertex-encode"=>Output::Bytes(m::codec',1)
    code=code.replace('  "vertex-encode"|"index-encode"=>{let mut b=vec![0;if op=="vertex-encode"{f::',cpp+'  "vertex-encode"|"index-encode"=>{let mut b=vec![0;if op=="vertex-encode"{f::',1)
    p.write_text(code)
    c=(d/'Cargo.toml').read_text().replace('features = ["clusterlod"]','default-features = false, features = ["std", "clusterlod"'+(', "simd"' if arm.endswith('default') else '')+']')
    c=c.replace('lto = "thin"','lto = "thin"\ndebug = 1')
    (d/'Cargo.toml').write_text(c)
    feature='cpp' if arm.startswith('cpp') else 'ours'
    env=dict(os.environ,CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD' if arm=='cpp-scalar' else '')
    cmd=['cargo','build','--offline','--locked','--release','--manifest-path',str(d/'Cargo.toml'),'--no-default-features','--features',feature,'-j','2']
    with (s/f'build-{arm}.log').open('w') as log: result=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT)
    assert result.returncode==0,(arm,(s/f'build-{arm}.log').read_text()[-3000:])
    shutil.copy2(target/'release/meshopt-peers',s/f'driver-{arm}')
    binding={str(f.relative_to(dest)):sha(f) for f in [*dest.glob('src/**/*.rs'),dest/'src/codec/simd/SAFETY.md',dest/'Cargo.toml',dest/'Cargo.lock',d/'Cargo.toml',d/'Cargo.lock',d/'src/main.rs']}
    (s/f'binding-{arm}.json').write_text(json.dumps({'binary':sha(s/f'driver-{arm}'),'sources':binding},indent=2)+'\n')
    print('built',arm,flush=True)
if 'base-default' in sys.argv[1:]:
    prep=s/'prepare';prep.mkdir()
    manifest='[package]\nname="bounds-prepare"\nversion="0.0.0"\nedition="2021"\n[dependencies]\nmeshoptimizer-rs={path="'+str(s/'base')+'"}\n[[bin]]\nname="bounds-prepare"\npath="'+str(root/'parity/0216-bounds/prepare.rs')+'"\n'
    (prep/'Cargo.toml').write_text(manifest)
    subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(prep/'Cargo.toml'),'-j','2'],check=True)
    shutil.copy2(target/'release/bounds-prepare',s/'bounds-prepare')
    groups=s/'bound-inputs';groups.mkdir()
    group_binding={}
    for item in json.loads((s/'inputs.json').read_text()):
        name=item['case']+'.bin'
        subprocess.run([str(s/'bounds-prepare'),str(s/'inputs'/name),str(groups/name)],check=True)
        group_binding[name]=sha(groups/name)
    (s/'bound-inputs.json').write_text(json.dumps({'preparer':sha(root/'parity/0216-bounds/prepare.rs'),'binary':sha(s/'bounds-prepare'),'inputs':group_binding},indent=2)+'\n')
    subprocess.run(['python3',str(root/'parity/0216-bounds/profile.py')],check=True)
