#!/usr/bin/env python3
"""SPEC-026 proof only: exact Rust Result/bytes, no clocks or library edits.

Run with decode-proof.sh through <job-runner>. Sources, corpus, executables,
compiler logs, counters and comparison receipts are retained in fresh scratch.
The embedded Rust program is a driver, never a permanent crate test.
"""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

import hashlib
import json
import os
from pathlib import Path
import random
import re
import shutil
import struct
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[2]
ART = Path(_public_path('<scratch>/meshopt-decode-proof'))
TARGET = Path(_public_path('<target-dir>'))
OLD = Path(_public_path('<scratch>/meshopt-decode'))
PROBES = {
    'codec/filter.rs': ['filter_root', 'scalar_oct', 'oct_records',
                        'scalar_oct_with_root', 'scalar_quat', 'quat_cached',
                        'trunc_normal', 'quat_records'],
    'codec/vertex.rs': ['packed', 'scalar_deltas', 'strided_bytes'],
}
DRIVER = r'''
use meshoptimizer_rs::{codec::*, Error, Workspace};
use std::io::{Read, Write};
fn word(r: &mut impl Read) -> std::io::Result<usize> {
    let mut b = [0; 4]; r.read_exact(&mut b)?; Ok(u32::from_le_bytes(b) as usize)
}
fn emit(out: &mut impl Write, id: &str, result: Result<Vec<u8>, Error>) {
    // Debug serializes the actual Error discriminant and every field (the
    // pinned Error enum has no fields). Success carries every output byte.
    let (status, bytes) = match result {
        Ok(v) => ("Ok".to_owned(), v),
        Err(e) => (format!("Err({e:?})"), Vec::new()),
    };
    let header = format!("{id} {status}");
    let size = 4 + header.len() + bytes.len();
    out.write_all(&(size as u32).to_le_bytes()).unwrap();
    out.write_all(&(header.len() as u32).to_le_bytes()).unwrap();
    out.write_all(header.as_bytes()).unwrap(); out.write_all(&bytes).unwrap();
}
fn vertex(out: &mut impl Write, id: &str, count: usize, stride: usize, data: &[u8]) {
    let mut ws = Workspace::default();
    let a = decode_vertex_buffer(count, stride, data, &mut ws);
    let mut into = vec![0xa5; count * stride + 37];
    let b = decode_vertex_buffer_into(&mut into, count, stride, data, &mut ws);
    assert!(into[count*stride..].iter().all(|&v| v == 0xa5), "{id}: tail");
    assert_eq!(a.as_ref().map(|_| ()), b.as_ref().map(|_| ()), "{id}: API Result");
    if let Ok(ref v) = a { assert_eq!(v, &into[..count*stride], "{id}: API bytes"); }
    emit(out, &format!("{id}/alloc"), a);
    emit(out, &format!("{id}/into"), b.map(|()| into[..count*stride].to_vec()));
}
fn filter(out: &mut impl Write, id: &str, kind: usize, count: usize, stride: usize, data: &[u8]) {
    let mut ws = Workspace::default();
    let enc = encode_vertex_buffer(data, count, stride, VertexEncoding::new(0,2).unwrap(), &mut ws).unwrap();
    let f = if kind == 1 { Filter::Octahedral } else { Filter::Quaternion };
    #[cfg(feature="probe")]
    let hits = || if kind == 1 {
        meshoptimizer_rs::decode_proof::OCT_RECORDS.load(std::sync::atomic::Ordering::Relaxed)
    } else {
        meshoptimizer_rs::decode_proof::QUAT_RECORDS.load(std::sync::atomic::Ordering::Relaxed)
    };
    #[cfg(feature="probe")] let before_alloc = hits();
    let a = decode_buffer_view(Mode::Attributes, f, count, stride, &enc, &mut ws);
    #[cfg(feature="probe")] if count >= 4 { assert!(hits() > before_alloc, "{id}: allocating bulk reachability"); }
    let mut into = vec![0xa5; count*stride+37];
    #[cfg(feature="probe")] let before_into = hits();
    let b = decode_buffer_view_into(&mut into, Mode::Attributes, f, count, stride, &enc, &mut ws);
    #[cfg(feature="probe")] if count >= 4 { assert!(hits() > before_into, "{id}: caller-buffer bulk reachability"); }
    assert!(into[count*stride..].iter().all(|&v| v == 0xa5), "{id}: view tail");
    assert_eq!(a.as_ref().map(|_| ()), b.as_ref().map(|_| ()), "{id}: view Result");
    if let Ok(ref v) = a { assert_eq!(v, &into[..count*stride], "{id}: view bytes"); }
    emit(out, &format!("{id}/alloc"), a);
    emit(out, &format!("{id}/into"), b.map(|()| into[..count*stride].to_vec()));
    // Also retain partial bytes on error for direct in-place filters, so zero
    // record failure-prefix behavior is compared, not merely the error kind.
    let mut direct = data.to_vec(); direct.extend([0xa5;37]);
    let r = if kind == 1 { decode_filter_oct(&mut direct,count,stride,&mut ws) }
        else { decode_filter_quat(&mut direct,count,stride,&mut ws) };
    assert!(direct[count*stride..].iter().all(|&v| v == 0xa5), "{id}: direct tail");
    emit(out, &format!("{id}/direct"), r.map(|()| Vec::new()));
    emit(out, &format!("{id}/partial"), Ok(direct[..count*stride].to_vec()));
}
fn run() {
    let suite = std::env::args().nth(1).unwrap();
    let path = std::env::args().nth(2).unwrap();
    let mut input = std::fs::File::open(path).unwrap();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let mut fixture = 0;
    while let Ok(kind) = word(&mut input) {
        let count = word(&mut input).unwrap(); let stride = word(&mut input).unwrap();
        let n = word(&mut input).unwrap(); let mut data = vec![0; n]; input.read_exact(&mut data).unwrap();
        if kind == 0 && suite == "vertex" {
            // Every truncation offset, including empty and full streams.
            for cut in 0..=n { vertex(&mut out,&format!("v{fixture}/cut{cut}"),count,stride,&data[..cut]); }
            // Every byte is mutated once: covers every control/group header,
            // block boundary and channel selector, without assuming a parser.
            for pos in 0..n {
                let mut m = data.clone(); m[pos] ^= 255;
                vertex(&mut out,&format!("v{fixture}/xor{pos}"),count,stride,&m);
            }
            // Exhaust all channel selector and first-block control byte values
            // on the small 17-record fixture for each version/stride/shape.
            if count == 17 {
                let version = data[0] & 15;
                let mut positions = vec![0];
                if version == 1 {
                    positions.extend(1..1+stride/4);
                    positions.extend(n-stride/4..n);
                }
                for pos in positions { for value in 0..=255u8 {
                    let mut m = data.clone(); m[pos] = value;
                    vertex(&mut out,&format!("v{fixture}/set{pos}={value}"),count,stride,&m);
                } }
            }
        } else if kind != 0 && suite == "filter" {
            filter(&mut out,&format!("f{fixture}"),kind,count,stride,&data);
        } else if kind == 0 && suite == "filter" {
            // Include valid vertex fixtures in WASM to reach strided_bytes
            // with both static strides, full groups and short final records.
            vertex(&mut out,&format!("v{fixture}/full"),count,stride,&data);
        }
        fixture += 1;
    }
    out.flush().unwrap();
}
fn main() {
    #[cfg(feature="scalar-ceiling")]
    with_level(Level::Scalar, run).unwrap();
    #[cfg(not(feature="scalar-ceiling"))]
    run();
    #[cfg(feature="probe")]
    for (name, counter) in meshoptimizer_rs::decode_proof::COUNTERS {
        eprintln!("PROBE {name} {}",counter.load(std::sync::atomic::Ordering::Relaxed));
    }
}
'''
NODE = '''const { WASI } = require('node:wasi');
const fs = require('node:fs');
const wasi = new WASI({version:'preview1',args:process.argv.slice(2),env:{},preopens:{'/': '/'},returnOnExit:true});
const mod = new WebAssembly.Module(fs.readFileSync(process.argv[2]));
const instance = new WebAssembly.Instance(mod, {wasi_snapshot_preview1:wasi.wasiImport});
process.exitCode = wasi.start(instance);
'''


def sha(path):
    h = hashlib.sha256()
    with open(path,'rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''): h.update(b)
    return h.hexdigest()


def command(args, label):
    with (ART/(label+'.log')).open('wb') as log:
        p = subprocess.run(list(map(str,args)),stdout=log,stderr=subprocess.STDOUT)
    with (ART/'gates.tsv').open('a') as f: f.write(f'{label}\t{p.returncode}\n')
    print(label,p.returncode,flush=True)
    if p.returncode: raise RuntimeError(label)


def snapshot(revision, name):
    archive = ART/(name+'.tar')
    with archive.open('wb') as f: subprocess.run(['git','archive',revision,'Cargo.toml','Cargo.lock','src'],cwd=ROOT,stdout=f,check=True)
    directory = ART/name; directory.mkdir()
    with tarfile.open(archive) as f: f.extractall(directory,filter='data')
    return directory


def instrument(directory):
    # Probe only a separate frozen source copy. Primary differential builds
    # compile byte-for-byte original library sources from the two Git revisions.
    names = [n for values in PROBES.values() for n in values]
    for file, functions in PROBES.items():
        path = directory/'src'/file; text = path.read_text()
        for name in functions:
            pat = r'(\bfn '+name+r'\b[^\{]*\{)'
            text, hits = re.subn(pat,lambda m:m[0]+f'\ncrate::decode_proof::{name.upper()}.fetch_add(1, core::sync::atomic::Ordering::Relaxed);',text,count=1)
            assert hits == 1, (file,name)
        path.write_text(text)
    with (directory/'src/lib.rs').open('a') as f:
        f.write('\n#[doc(hidden)] pub mod decode_proof {\nuse core::sync::atomic::AtomicUsize;\n')
        for name in names: f.write(f'pub static {name.upper()}: AtomicUsize = AtomicUsize::new(0);\n')
        f.write('pub const COUNTERS: &[(&str, &AtomicUsize)] = &[\n')
        for name in names: f.write(f'("{name}", &{name.upper()}),\n')
        f.write('];}\n')


def package(directory, name, probe=False):
    driver = ART/(name+'-driver'); (driver/'src').mkdir(parents=True)
    (driver/'src/main.rs').write_text(DRIVER)
    (driver/'Cargo.toml').write_text(f'''[package]
name = "decode-proof-driver"
version = "0.0.0"
edition = "2021"
[dependencies]
meshoptimizer-rs = {{ path = "{directory}", default-features = false, features = ["std"] }}
[features]
scalar-ceiling = ["meshoptimizer-rs/simd", "meshoptimizer-rs/parity-internals"]
probe = []
[profile.release]
codegen-units = 1
lto = "fat"
''')
    return driver


def fixtures():
    os.environ.update(MESHOPT_REFERENCE=_public_path('<refs>/meshoptimizer'),MESHOPT_ARTIFACTS=str(OLD))
    sys.path.insert(0,str(ROOT/'parity/codec'))
    from measure import Driver, request
    from runner import filter_floats
    cpp = Driver(OLD/'cpp-scalar')
    rng = random.Random(260026)
    # Advance exactly as differential.py before its existing vertex fixtures.
    for kind,stride in [(1,4),(1,8),(2,8)]:
        for bits in range(4 if kind==2 else 2,9 if stride==4 else 17):
            for count in [1,2,3,4,7,8,15,16,17,63,64,65,257,513]:
                filter_floats(rng,kind,count,stride)
    meta = []; vertex = 0; filters = 0
    with (ART/'corpus.bin').open('wb') as f:
        def add(name,kind,count,stride,data,**extra):
            f.write(struct.pack('<4I',kind,count,stride,len(data))); f.write(data)
            meta.append(dict(id=len(meta),name=name,kind=kind,count=count,stride=stride,
                             bytes=len(data),sha256=hashlib.sha256(data).hexdigest(),**extra))
        original = {r['case']:r for r in json.loads((OLD/'differential.json').read_text())['rows']}
        for version in [0,1]:
            for stride in [4,12,48,64,256]:
                for count in [0,1,15,16,17,127,128,129,255,256,257,513]:
                    for shape in [0,1,2]:
                        if shape==0: raw=bytes(rng.randrange(256) for _ in range(count*stride))
                        elif shape==1: raw=bytes((i//stride+(i%stride)*13)&255 for i in range(count*stride))
                        else: raw=b''.join(struct.pack('<f',rng.uniform(-10,10)) for _ in range(count*stride//4))
                        status,encoded,_ = cpp.call(request(11,count,stride,raw,version=version,level=2)); assert status==0
                        name=f'vertex-v{version}-s{stride}-n{count}-k{shape}'
                        assert hashlib.sha256(encoded).hexdigest()==original[name]['input_sha256'],name
                        add(name,0,count,stride,encoded,version=version,shape=shape); vertex+=1
        # Small groups, 64-record bulk chunks and tails; vary every lane.
        counts=[0,1,2,3,4,5,7,8,15,16,17,63,64,65,127,128,129]
        for kind,stride in [(1,4),(1,8),(2,8)]:
            limit=128 if stride==4 else 32768
            def pack(v): return struct.pack('<'+('4b' if stride==4 else '4h'),*v)
            for count in counts:
                records=[]
                for i in range(count):
                    vals=[-limit,-limit+1,-1,0,1,limit-2,limit-1]
                    v=[vals[i%7], vals[(i*3+1)%7], vals[(i*5+2)%7], vals[(i+4)%7]]
                    if kind==1 and v[:3]==[0,0,0]: v[2]=1
                    records.append(pack(v))
                add(f'varied-{kind}-{stride}-{count}',kind,count,stride,b''.join(records)); filters+=1
                if kind==1:
                    for zero in range(count):
                        changed=records.copy(); changed[zero]=pack([0,0,0,zero%limit])
                        add(f'oct-zero-{stride}-{count}-{zero}',kind,count,stride,b''.join(changed)); filters+=1
                    if count:
                        add(f'oct-repeat-{stride}-{count}',kind,count,stride,pack([13,-27,63,1])*count); filters+=1
            if kind==2:
                for scale in [-32768,-260,-259,-258,-257,-256,-255,-254,-253,-252,-4,-1,0,1,3,4,248,251,252,253,254,255,256,257,258,259,260,32764,32767]:
                    for lane in range(4):
                        for count in counts:
                            records=[]
                            for i in range(count):
                                s=scale if i%4==lane else 16384
                                records.append(pack([[-32768,-1,0,32767][i%4],i%13-6,i%17-8,s]))
                            add(f'quat-scale-{scale}-lane{lane}-n{count}',2,count,8,b''.join(records)); filters+=1
                for count in counts:
                    add(f'quat-repeat-{count}',2,count,8,pack([1,-2,3,16384])*count);filters+=1
    cpp.close()
    (ART/'corpus.json').write_text(json.dumps(dict(vertex_fixtures=vertex,filter_fixtures=filters,rows=meta),indent=2)+'\n')
    print('fixtures',vertex,filters,flush=True)


def build(package_dir,label,wasm=False,release=False,probe=False):
    args=['cargo','build','--offline','--manifest-path',package_dir/'Cargo.toml']
    if wasm: args += ['--target','wasm32-wasip1']
    features=[]
    if not wasm: features.append('scalar-ceiling')
    if probe: features.append('probe')
    if features: args+=['--features',','.join(features)]
    if release: args+=['--release']
    command(args,label)
    binary=TARGET/('wasm32-wasip1' if wasm else '')/('release' if release else 'debug')/('decode-proof-driver.wasm' if wasm else 'decode-proof-driver')
    out=ART/label; shutil.copy2(binary,out)
    return out


def read_exact(stream,n):
    b=bytearray()
    while len(b)<n:
        v=stream.read(n-len(b))
        if not v: raise EOFError((len(b),n))
        b.extend(v)
    return bytes(b)


def compare(a,b,label,wasm=False,probe=False,suite=None):
    def start(binary,arm):
        cmd=['node','--disable-warning=ExperimentalWarning',str(ART/'wasi.cjs'),str(binary)] if wasm else [str(binary)]
        cmd += [suite or ('filter' if wasm else 'vertex'),str(ART/'corpus.bin')]
        err=(ART/(label+'-'+arm+'.stderr')).open('wb')
        return subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=err),err
    pa,ea=start(a,'baseline');pb,eb=start(b,'candidate')
    records=0; cases=0; success=0; errors={}; digest=hashlib.sha256()
    try:
        while True:
            ha=pa.stdout.read(4); hb=pb.stdout.read(4)
            if not ha and not hb: break
            assert ha==hb and len(ha)==4, (label,records,'record length',ha,hb)
            n=struct.unpack('<I',ha)[0]
            ra=read_exact(pa.stdout,n);rb=read_exact(pb.stdout,n)
            if ra!=rb:
                (ART/'MISMATCH-baseline.bin').write_bytes(ra);(ART/'MISMATCH-candidate.bin').write_bytes(rb)
                raise AssertionError((label,records,ra[:160],rb[:160]))
            k=struct.unpack_from('<I',ra)[0];header=ra[4:4+k].decode();status=header.split()[-1]
            if header.split()[0].endswith('/alloc'): cases+=1
            if status=='Ok':success+=1
            else:errors[status]=errors.get(status,0)+1
            digest.update(ha+ra);records+=1
    finally:
        if sys.exc_info()[0]: pa.kill();pb.kill()
        rc=[pa.wait(),pb.wait()];ea.close();eb.close()
    assert rc==[0,0],(label,rc)
    row=dict(label=label,cases=cases,api_records=records,success_records=success,errors=errors,
             mismatches=0,exit_codes=rc,transcript_sha256=digest.hexdigest())
    if probe:
        text=(ART/(label+'-candidate.stderr')).read_text()
        row['probes']={name:int(value) for name,value in re.findall(r'PROBE (\w+) (\d+)',text)}
        expected=[n for values in PROBES.values() for n in values]
        if not wasm: expected.remove('strided_bytes')
        assert all(row['probes'].get(n,0)>0 for n in expected),(label,row['probes'])
    (ART/(label+'.json')).write_text(json.dumps(row,indent=2)+'\n')
    with (ART/'gates.tsv').open('a') as f: f.write(label+'\t0\n')
    print(json.dumps(row),flush=True)
    return row


def main():
    assert os.environ.get('MOSS_HEAVY_ACTIVE'), _public_path('<job-runner> required')
    assert os.environ['CARGO_TARGET_DIR']==str(TARGET)
    assert not ART.exists(), 'fresh scratch required; never overwrite receipts'
    ART.mkdir(parents=True)
    for script in ['decode-proof.py', 'decode-proof.sh']:
        shutil.copy2(ROOT/'parity/decode'/script, ART/script)
    (ART/'wasi.cjs').write_text(NODE)
    candidate=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
    (ART/'identity.json').write_text(json.dumps(dict(candidate=candidate,baseline='bea3e04',
        rustc=subprocess.check_output(['rustc','-Vv'],text=True),node=subprocess.check_output(['node','--version'],text=True),
        library_sha256={str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'src').rglob('*.rs')},
        cpp_encoder_sha256=sha(OLD/'cpp-scalar')),indent=2)+'\n')
    baseline=snapshot('bea3e04','baseline');after=snapshot(candidate,'candidate')
    probed=ART/'probe';shutil.copytree(after,probed);instrument(probed)
    packages=[package(baseline,'baseline'),package(after,'candidate'),package(probed,'probe',True)]
    fixtures()
    rows=[]
    # Primary original-source comparisons precede separate reachability builds.
    native=[build(p,n+'-native',release=True) for p,n in zip(packages[:2],['baseline','candidate'])]
    rows.append(compare(*native,'native-exact'))
    wasm_pairs={}
    for release in [False,True]:
        mode='release' if release else 'debug'
        pair=[build(p,n+'-wasm-'+mode,wasm=True,release=release) for p,n in zip(packages[:2],['baseline','candidate'])]
        rows.append(compare(*pair,'wasm-std-'+mode,wasm=True));wasm_pairs[mode]=pair
    probe=build(packages[2],'probe-native',release=True,probe=True)
    # wasm executes the entire filter corpus plus all valid vertex fixtures.
    for release in [False,True]:
        mode='release' if release else 'debug'
        probe=build(packages[2],'probe-wasm-'+mode,wasm=True,release=release,probe=True)
        rows.append(compare(wasm_pairs[mode][0],probe,'reach-wasm-'+mode,wasm=True,probe=True))
    # The valid vertex/filter suite reaches every native changed function;
    # do not repeat millions of malformed cases solely for counters.
    probe=ART/'probe-native'
    rows.append(compare(native[0],probe,'reach-native',probe=True,suite='filter'))
    (ART/'summary.json').write_text(json.dumps(dict(verdict='PASS',comparisons=rows),indent=2)+'\n')


if __name__=='__main__':
    try: main()
    except BaseException as e:
        if ART.is_dir(): (ART/'FAILURE.txt').write_text(repr(e)+'\n')
        raise
