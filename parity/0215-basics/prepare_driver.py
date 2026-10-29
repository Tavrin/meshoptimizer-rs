"""Adapt a scratch copy of the peer consumer for the 0.2.15 basics lane.

Usage: prepare_driver.py PEERS_COPY_DIR. Apply after
parity/0215-contract-meshlets/prepare_driver.py (which removes the elapsed-time
collection). Adds the remap family ops and turns SIMD off in every arm:
meshoptimizer-rs without its `simd` feature, optimesh without `simd`; the C++
arm is built with MESHOPTIMIZER_NO_SIMD by build.py (indexgenerator.cpp and
indexcodec.cpp have no SIMD paths either way). The remap and index inputs of
remap-vertex/remap-index are computed outside the counted region, in
first-reference order of positions, identically for every arm.
"""
from pathlib import Path
import sys

d = Path(sys.argv[1])
main = d / 'src/main.rs'
s = main.read_text()


def edit(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new, 1)


edit('struct Input { p: Vec<[f32;3]>,', 'struct Input { remap: Vec<u32>, unique: usize, p: Vec<[f32;3]>,')
edit('  Self{p,i,attrs,vb:', '  let (remap,unique)=first_remap(&p,&i);Self{remap,unique,p,i,attrs,vb:')
edit('enum Output { ', 'enum Output { Remap(Vec<u32>,usize), ')
edit('   Self::RawMesh(ms,v,t)=>',
     '   Self::Remap(r,k)=>{let mut b=(*k as u32).to_le_bytes().to_vec();b.extend(ws(r));b},Self::RawMesh(ms,v,t)=>')

ours = '''  "remap"|"remap-multi"|"remap-custom"=>{let s=m::VertexStream::new(bytes(&x.p),n,12,12,0).unwrap();let r=match op{"remap"=>m::generate_vertex_remap(Some(&x.i),s,w),"remap-multi"=>m::generate_vertex_remap_multi(Some(&x.i),&[s,m::VertexStream::new(abytes(&x.attrs),n,16,16,0).unwrap()],w),_=>{let a=&x.attrs;m::generate_vertex_remap_custom(Some(&x.i),p,|u,v|close(a,u,v),w)}}.unwrap();Output::Remap(r.remap,r.vertex_count)},
  "position-remap"=>Output::Indices(m::generate_position_remap(p,w).unwrap()),
  "remap-vertex"=>Output::Bytes(m::remap_vertex_buffer(m::VertexStream::new(bytes(&x.p),n,12,12,0).unwrap(),&x.remap,w).unwrap()),
  "remap-index"=>Output::Indices(m::remap_index_buffer(Some(&x.i),&x.remap,w).unwrap()),
  "shadow"|"shadow-multi"=>{let s=m::VertexStream::new(bytes(&x.p),n,12,12,0).unwrap();Output::Indices(if op=="shadow"{m::generate_shadow_index_buffer(&x.i,s,w)}else{m::generate_shadow_index_buffer_multi(&x.i,&[s,m::VertexStream::new(abytes(&x.attrs),n,16,16,0).unwrap()],w)}.unwrap())},
'''
cpp = '''  "remap"|"remap-multi"|"remap-custom"=>{let mut b=vec![0;n];let s=[f::meshopt_Stream{data:x.p.as_ptr().cast(),size:12,stride:12},f::meshopt_Stream{data:x.attrs.as_ptr().cast(),size:16,stride:16}];let k=match op{"remap"=>f::meshopt_generateVertexRemap(b.as_mut_ptr(),i,ni,p.cast(),n,12),"remap-multi"=>f::meshopt_generateVertexRemapMulti(b.as_mut_ptr(),i,ni,n,s.as_ptr(),2),_=>f::meshopt_generateVertexRemapCustom(b.as_mut_ptr(),i,ni,p,n,12,Some(close_c),x.attrs.as_ptr() as *mut std::ffi::c_void)};Output::Remap(b,k)},
  "position-remap"=>{let mut b=vec![0;n];f::meshopt_generatePositionRemap(b.as_mut_ptr(),p,n,12);Output::Indices(b)},
  "remap-vertex"=>{let mut b=vec![0u8;x.unique*12];f::meshopt_remapVertexBuffer(b.as_mut_ptr().cast(),p.cast(),n,12,x.remap.as_ptr());Output::Bytes(b)},
  "remap-index"=>{let mut b=vec![0;ni];f::meshopt_remapIndexBuffer(b.as_mut_ptr(),i,ni,x.remap.as_ptr());Output::Indices(b)},
  "shadow"|"shadow-multi"=>{let mut b=vec![0;ni];let s=[f::meshopt_Stream{data:x.p.as_ptr().cast(),size:12,stride:12},f::meshopt_Stream{data:x.attrs.as_ptr().cast(),size:16,stride:16}];if op=="shadow"{f::meshopt_generateShadowIndexBuffer(b.as_mut_ptr(),i,ni,p.cast(),n,12,12)}else{f::meshopt_generateShadowIndexBufferMulti(b.as_mut_ptr(),i,ni,n,s.as_ptr(),2)};Output::Indices(b)},
'''
rust = '''  "remap"|"remap-multi"|"remap-custom"=>{use optimesh::indexgenerator as g;let mut b=vec![0;n];let k=match op{"remap"=>g::generate_vertex_remap(&mut b,Some(&x.i),bytes(&x.p),12),"remap-multi"=>g::generate_vertex_remap_multi(&mut b,Some(&x.i),&[g::Stream{data:bytes(&x.p),size:12,stride:12},g::Stream{data:abytes(&x.attrs),size:16,stride:16}]),_=>{let a=&x.attrs;g::generate_vertex_remap_custom(&mut b,Some(&x.i),positions,12,Some(&|u,v|close(a,u,v)))}};Output::Remap(b,k)},
  "position-remap"=>{let mut b=vec![0;n];optimesh::indexgenerator::generate_position_remap(&mut b,positions,n,12);Output::Indices(b)},
  "remap-vertex"=>{let mut b=vec![0u8;x.unique*12];optimesh::indexgenerator::remap_vertex_buffer(&mut b,bytes(&x.p),12,&x.remap);Output::Bytes(b)},
  "remap-index"=>{let mut b=vec![0;ni];optimesh::indexgenerator::remap_index_buffer(&mut b,Some(&x.i),&x.remap);Output::Indices(b)},
  "shadow"|"shadow-multi"=>{use optimesh::indexgenerator as g;let mut b=vec![0;ni];if op=="shadow"{g::generate_shadow_index_buffer(&mut b,&x.i,bytes(&x.p),n,12,12)}else{g::generate_shadow_index_buffer_multi(&mut b,&x.i,n,&[g::Stream{data:bytes(&x.p),size:12,stride:12},g::Stream{data:abytes(&x.attrs),size:16,stride:16}])};Output::Indices(b)},
'''
for anchor, addition in [('  "index-decode"=>Output::Bytes(m::codec::decode_index_buffer', ours),
                         ('  "vertex-decode"|"index-decode"=>{let mut b=vec![0;if op=="vertex-decode"{n*12}', cpp),
                         ('  "index-decode"=>{let mut b=vec![0;ni];i::decode_index_buffer', rust)]:
    edit(anchor, addition + anchor)

helpers = '''// Remap input for remap-vertex/remap-index: first reference of each position.
#[allow(clippy::ptr_arg)] // fixes the inferred type of the caller's collect
fn first_remap(p:&Vec<[f32;3]>,i:&[u32])->(Vec<u32>,usize) {
 let mut seen=std::collections::HashMap::new();let mut r=vec![u32::MAX;p.len()];
 for &v in i {if r[v as usize]==u32::MAX {let next=seen.len() as u32;r[v as usize]=*seen.entry(p[v as usize].map(f32::to_bits)).or_insert(next);}}
 let k=seen.len();(r,k)
}
fn abytes(a:&[f32])->&[u8]{ assert!(cfg!(target_endian="little")); unsafe{std::slice::from_raw_parts(a.as_ptr().cast(),a.len()*4)} }
// Data-dependent custom-remap predicate shared by every arm.
fn close(a:&[f32],u:u32,v:u32)->bool{ (a[u as usize*4]-a[v as usize*4]).abs()<0.25 }
#[cfg(feature="cpp")]
unsafe extern "C" fn close_c(ctx:*mut std::ffi::c_void,u:std::ffi::c_uint,v:std::ffi::c_uint)->std::ffi::c_int{
 let a=ctx as *const f32;((*a.add(u as usize*4)-*a.add(v as usize*4)).abs()<0.25) as std::ffi::c_int
}
'''
edit('// Normalize only during serialization, outside the counted region.',
     helpers + '// Normalize only during serialization, outside the counted region.')
main.write_text(s)

cargo = d / 'Cargo.toml'
c = cargo.read_text()
for old, new in [('meshoptimizer-rs = { path = "../..", optional = true, features = ["clusterlod"] }',
                  'meshoptimizer-rs = { path = "../..", optional = true, default-features = false, features = ["std", "clusterlod"] }'),
                 ('optimesh = { version = "=1.1.0", optional = true, features = ["simd"] }',
                  'optimesh = { version = "=1.1.0", optional = true }')]:
    assert c.count(old) == 1, old
    c = c.replace(old, new, 1)
cargo.write_text(c)
