"""Adapt the existing peer consumer in scratch; retain its measured allocation policy."""
from pathlib import Path
import sys
p = Path(sys.argv[1])
s = p.read_text()
ours = '''  "meshlets-flex"|"meshlets-spatial"=>{let s=m::MeshletSettings{max_vertices:64,max_triangles:124};let r=if op=="meshlets-flex"{m::build_meshlets_flex(&x.i,p,s,41,0.,2.,w)}else{m::build_meshlets_spatial(&x.i,p,s,41,0.5,w)}.unwrap();Output::Mesh(r.meshlets.iter().map(|v|[v.vertex_offset,v.triangle_offset,v.vertex_count,v.triangle_count]).collect(),r.vertices,r.triangles)},\n'''
cpp = '''  "meshlets-flex"|"meshlets-spatial"=>{let bound=f::meshopt_buildMeshletsBound(ni,64,41);let mut ms=vec![std::mem::zeroed::<f::meshopt_Meshlet>();bound];let mut v=vec![0;bound*64];let mut t=vec![0;bound*124*3];let k=if op=="meshlets-flex"{f::meshopt_buildMeshletsFlex(ms.as_mut_ptr(),v.as_mut_ptr(),t.as_mut_ptr(),i,ni,p,n,12,64,41,124,0.,2.)}else{f::meshopt_buildMeshletsSpatial(ms.as_mut_ptr(),v.as_mut_ptr(),t.as_mut_ptr(),i,ni,p,n,12,64,41,124,0.5)};ms.truncate(k);mesh(ms.iter().map(|m|[m.vertex_offset,m.triangle_offset,m.vertex_count,m.triangle_count]).collect(),v,t)},\n'''
rust = '''  "meshlets-flex"|"meshlets-spatial"=>{let bound=c::build_meshlets_bound(ni,64,41);let mut ms=vec![c::Meshlet::default();bound];let mut v=vec![0;bound*64];let mut t=vec![0;bound*124*3];let mut output=c::MeshletBuffers{meshlets:&mut ms,vertices:&mut v,triangles:&mut t};let pos=c::Positions{data:positions,count:n,stride:12};let k=if op=="meshlets-flex"{c::build_meshlets_flex(&mut output,&x.i,&pos,&c::MeshletOptions{max_vertices:64,min_triangles:41,max_triangles:124,cone_weight:0.,split_factor:2.})}else{c::build_meshlets_spatial(&mut output,&x.i,&pos,64,41,124,0.5)};ms.truncate(k);mesh(ms.iter().map(|m|[m.vertex_offset,m.triangle_offset,m.vertex_count,m.triangle_count]).collect(),v,t)},\n'''
for anchor, addition in [('  "vertex-encode"=>Output::Bytes(m::codec',ours),('  "vertex-encode"|"index-encode"=>{let mut b=vec![0;if op=="vertex-encode"{f::',cpp),('  "vertex-encode"|"index-encode"=>{let mut b=vec![0;if op=="vertex-encode"{v::',rust)]:
    assert s.count(anchor)==1
    s=s.replace(anchor, addition+anchor,1)
# The inherited consumer times the loop. This lane counts instructions only.
s = s.replace('let start=Instant::now();', '')
s = s.replace('let elapsed=start.elapsed().as_secs_f64();', '')
s = s.replace('println!("COUNTED {elapsed:.9}")', 'println!("COUNTED")')
p.write_text(s)
