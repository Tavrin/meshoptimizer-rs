"""Supplement full-output proofs with real/corpus error-prefix and C++ checks.

Derive a second entry point from the retained exact observer, leaving all
observation logic unchanged. Run through CPU admission after checks.sh.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import struct
import subprocess

assert os.environ.get('MOSS_HEAVY_ACTIVE')
lane=Path(__file__).resolve().parent
scratch=Path(os.environ['CARGO_TARGET_DIR'])/'proof-work'
source=(lane/'exact.rs').read_text().split('fn main() {')[0]+r'''
fn main() {
    let unlimited=(usize::MAX,u64::MAX);
    let mut files=Vec::new();
    for dir in ["lod-corpus","real-corpus"] {
        files.extend(std::fs::read_dir(std::env::args().nth(1).unwrap()+"/"+dir).unwrap()
            .map(|f|f.unwrap().path()).filter(|p|p.extension().is_some_and(|e|e=="bin")));
    }
    files.sort();let mut comparisons=0;
    let pools:Vec<_>=[1,2,4,8,32].into_iter().map(|n|rayon::ThreadPoolBuilder::new().num_threads(n).build().unwrap()).collect();
    for path in files {
        let c=read(&path);
        let mut checks=Vec::new();
        for fail in [1,2] {checks.push((c.clone(),unlimited,fail));}
        for work in [0,c.p.len() as u64] {checks.push((c.clone(),(usize::MAX,work),usize::MAX));}
        if !c.i.is_empty() {let mut bad=c.clone();*bad.i.last_mut().unwrap()=u32::MAX;checks.push((bad,unlimited,usize::MAX));}
        if !c.p.is_empty() {let mut bad=c.clone();bad.p[0][0]=f32::NAN;checks.push((bad,unlimited,usize::MAX));}
        for (case,limits,fail) in checks {
            let expected=base(&case,false,limits,fail);
            for pool in &pools {
                let got=pool.install(||candidate(&case,true,limits,fail));
                assert_eq!(expected,got,"prefix {:?} {:?} {fail}",path.file_name(),limits);
                comparisons+=1;
            }
        }
        println!("prefix {:?} comparisons {comparisons}",path.file_name().unwrap());
    }
    println!("PASS prefix comparisons {comparisons}");
}
'''
derived=scratch/'prefix.rs';derived.write_text(source)
records=[]
for mode in ['simd','scalar']:
    project=scratch/('prefix-'+mode);project.mkdir(exist_ok=True)
    manifest=(scratch/('exact-'+mode)/'Cargo.toml').read_text()
    manifest=manifest.replace('name = "clod-exact"','name = "clod-prefix"').replace(str(lane/'exact.rs'),str(derived))
    (project/'Cargo.toml').write_text(manifest)
    r=subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(project/'Cargo.toml')]);assert r.returncode==0
    binary=scratch/('prefix-'+mode+'.bin');shutil.copy2(Path(os.environ['CARGO_TARGET_DIR'])/'release/clod-prefix',binary)
    with (scratch/('prefix-'+mode+'.log')).open('w') as f:
        r=subprocess.run([str(binary),str(scratch)],stdout=f,stderr=subprocess.STDOUT)
    print('prefix',mode,r.returncode,flush=True);assert r.returncode==0
    records.append(dict(mode=mode,source_sha256=hashlib.sha256(source.encode()).hexdigest(),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),exit=r.returncode))
# C++ differential on the additional LOD corpus. Add the peer protocol's seven
# empty auxiliary buffers outside the operation; positions and topology unchanged.
comparisons=0;outputs=[]
for path in sorted((scratch/'lod-corpus').glob('*.bin')):
    peer_input=scratch/'lod-peer.bin';peer_input.write_bytes(path.read_bytes()+struct.pack('<7I',*([0]*7)))
    hashes={}
    for arm in ['base','serial','candidate','cpp-scalar','cpp-default']:
        output=scratch/'lod-peer-output.bin'
        r=subprocess.run([str(scratch/('driver-'+arm)),'cluster-lod',str(peer_input),str(output)],input='stop\n',capture_output=True,text=True,env=dict(os.environ,RAYON_NUM_THREADS='4'))
        assert r.returncode==0,(path.name,arm,r.stderr)
        hashes[arm]=hashlib.sha256(output.read_bytes()).hexdigest()
    assert len(set(hashes.values()))==1,(path.name,hashes)
    outputs.append(dict(case=path.name,input_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),output_sha256=next(iter(hashes.values()))))
    comparisons+=4
(lane/'prefix-results.json').write_text(json.dumps(dict(drivers=records,cpp_lod_comparisons=comparisons,cpp_lod_outputs=outputs),indent=2)+'\n')
print('PASS C++ LOD comparisons',comparisons,flush=True)
# The API requires std through parallel. Separately exercise the extracted
# serial path in a truly no_std crate against the baseline, at every work limit.
nostd=scratch/'exact-no-std.rs'
nostd.write_text((lane/'exact.rs').read_text().replace('candidate::clusterlod::build_parallel_with_output','candidate::clusterlod::build_with_output'))
project=scratch/'exact-no-std';project.mkdir(exist_ok=True)
manifest=(scratch/'exact-scalar/Cargo.toml').read_text().replace('name = "clod-exact"','name = "clod-nostd"').replace(str(lane/'exact.rs'),str(nostd)).replace('["parallel", "clusterlod", "parity-internals"]','["clusterlod"]')
(project/'Cargo.toml').write_text(manifest)
r=subprocess.run(['cargo','build','--offline','--release','--manifest-path',str(project/'Cargo.toml')]);assert r.returncode==0
with (scratch/'exact-no-std.log').open('w') as f:r=subprocess.run([str(Path(os.environ['CARGO_TARGET_DIR'])/'release/clod-nostd')],stdout=f,stderr=subprocess.STDOUT)
assert r.returncode==0
print('PASS no_std serial extraction',flush=True)
result=json.loads((lane/'prefix-results.json').read_text())
result['no_std_serial']=dict(source_sha256=hashlib.sha256(nostd.read_bytes()).hexdigest(),binary_sha256=hashlib.sha256((Path(os.environ['CARGO_TARGET_DIR'])/'release/clod-nostd').read_bytes()).hexdigest(),exit=r.returncode)
(lane/'prefix-results.json').write_text(json.dumps(result,indent=2)+'\n')
