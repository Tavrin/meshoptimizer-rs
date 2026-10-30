"""Admitted deterministic conservative-raster qualification; old receipts stay intact."""
import hashlib,json,os,random,shutil,statistics,struct,subprocess,time
from pathlib import Path
root=Path.cwd();lane=root/'parity/0217-hsr';receipt=lane/'eval';target=Path(os.environ['CARGO_TARGET_DIR']);scratch=target/'speed';scratch.mkdir(parents=True,exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU','0')=='0'
os.environ.update(CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',LC_ALL='C',GIT_OPTIONAL_LOCKS='0')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def binary(arm):return scratch/('driver-'+arm)
def run(name,args,**kw):
    start=time.monotonic();p=subprocess.run(list(map(str,args)),text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,**kw)
    (receipt/(name+'.log')).write_text(p.stdout.replace(str(target),'<target>').replace(str(root),'<worktree>'))
    with (receipt/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\t{time.monotonic()-start:.3f}\n')
    print(name,p.returncode,p.stdout[-1200:],flush=True)
    if p.returncode:raise SystemExit(p.returncode)
    return p.stdout
old=scratch/'old';old.mkdir(exist_ok=True)
for directory in ['src','docs']:
    shutil.copytree(root/directory,old/directory,dirs_exist_ok=True)
for name in ['Cargo.toml','Cargo.lock','README.md']:shutil.copy2(root/name,old/name)
shutil.copy2(lane/'speed/before.rs',old/'src/hidden.rs')
prior=scratch/'prior';prior.mkdir(exist_ok=True)
for directory in ['src','docs']:
    shutil.copytree(root/directory,prior/directory,dirs_exist_ok=True)
for name in ['Cargo.toml','Cargo.lock','README.md']:shutil.copy2(root/name,prior/name)
(prior/'src/hidden.rs').write_bytes(subprocess.check_output(['git','show','HEAD:src/hidden.rs']))
binaries={}
for arm in ['before','prior','after','none','parallel']:
    assert not binary(arm).is_dir(),binary(arm)
for arm,source,features in [('before',old,['std','simd']),('prior',prior,['std','simd']),('after',root,['std','simd']),('none',root,[]),('parallel',root,['std','simd','parallel'])]:
    folder=scratch/('build-'+arm);folder.mkdir(exist_ok=True)
    (folder/'Cargo.toml').write_text(f'''[package]
name="hsr-speed"
version="0.0.0"
edition="2021"
[dependencies]
meshoptimizer-rs={{path={json.dumps(str(source))},default-features=false,features={json.dumps(features)}}}
[[bin]]
name="hsr-speed"
path={json.dumps(str(lane/'speed/bench.rs' if arm=='before' else receipt/'bench.rs'))}
[profile.release]
codegen-units=1
lto="fat"
''')
    run('build-'+arm,['cargo','build','--offline','--release','--manifest-path',folder/'Cargo.toml'])
    shutil.copy2(target/'release/hsr-speed',binary(arm));binaries[arm]=sha(binary(arm))
run('oracle-build',['c++','-std=c++17','-O2','-ffp-contract=off',lane/'oracle.cpp','-o',scratch/'oracle'])
# Independent C++ oracle and preserved old implementation on the same 80 scenes.
rng=random.Random(217);scenes=scratch/'scenes';scenes.mkdir(exist_ok=True);observations=0;extra=0
for case in range(80):
    n=1+case%24;p=[];idx=[]
    for t in range(n):
        p.extend([[rng.randrange(-8,9)/4 for _ in range(3)] for _ in range(3)]);idx.extend([t*3,t*3+1,t*3+2])
    if case%2==0:
        offset=len(p);p.extend([[x,y,z] for z in [-4.,4.] for y in [-4.,4.] for x in [-4.,4.]])
        idx.extend(offset+v for v in [0,3,1,0,2,3,4,5,7,4,7,6,0,1,5,0,5,4,1,3,7,1,7,5,3,2,6,3,6,7,2,0,4,2,4,6])
    path=scenes/(str(case)+'.bin');path.write_bytes(struct.pack('<II',len(p),len(idx))+b''.join(struct.pack('<3f',*v) for v in p)+struct.pack('<'+'I'*len(idx),*idx))
    for s in [1,2,4]:
        expected=scratch/'oracle.out';subprocess.run([str(scratch/'oracle'),str(path),str(expected),str(s)],check=True)
        expected_bytes=expected.read_bytes();expected_tris=set(struct.iter_unpack('<III',expected_bytes));hashes=[]
        for arm in ['before','prior','after','none','parallel']:
            actual=scratch/(arm+'.out');pr=subprocess.run([str(binary(arm)),'hidden',str(path),str(actual)],input='stop\n',text=True,capture_output=True,env=dict(os.environ,HSR_SUBDIVISIONS=str(s)))
            assert pr.returncode==0,(case,s,arm,pr.stderr)
            data=actual.read_bytes()
            if arm=='before':assert data==expected_bytes,(case,s,arm)
            else:
                actual_tris=set(struct.iter_unpack('<III',data));assert expected_tris<=actual_tris,(case,s,arm)
                extra+=len(actual_tris-expected_tris);hashes.append(sha(actual))
            observations+=1
        assert len(set(hashes))==1,(case,s,hashes)
(receipt/'oracle.json').write_text(json.dumps(dict(exit=0,scenes=80,subdivisions=[1,2,4],observations=observations,candidate_checks=960,extra_kept_total=extra,criterion='old exact C++; candidate superset, pre-fix exact identity and feature-mode identity',oracle_sha256=sha(scratch/'oracle')))+'\n')
print('PASS conservative C++/old differential',observations,flush=True)

(receipt/'binaries-oracle.json').write_text(json.dumps(binaries,indent=2)+'\n')

# Same complete merged scenes/densities as the measurements, using pre-fix HEAD.
scene_identity=[]
scene_root=Path(os.environ['HSR_SCENES'])
for row in json.loads((receipt/'summary.json').read_text()):
    out=scratch/'prior-scene.out'
    pr=run('prior-'+row['scene']+'-'+row['density'],
        [binary('prior'),'hidden',scene_root/(row['scene']+'.bin'),out],
        input='stop\n',env=dict(os.environ,HSR_PIXEL_SIZE=str(row['pixel_size'])))
    expected=scene_root/(row['scene']+'-'+row['density']+'-0.out')
    assert out.read_bytes()==expected.read_bytes()
    scene_identity.append(dict(scene=row['scene'],density=row['density'],sha256=sha(out)))
(receipt/'prior-scene-identity.json').write_text(json.dumps(scene_identity,indent=2)+'\n')

with (receipt/'exits.tsv').open('a') as f:f.write('oracle\t0\n')
print('PASS complete C++/pre-fix/feature-mode and complete-scene identity checks',flush=True)
