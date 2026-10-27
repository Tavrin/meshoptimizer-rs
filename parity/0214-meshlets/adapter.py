#!/usr/bin/env python3
"""Extend the peer consumer in scratch with the registered flex/spatial cases."""
from pathlib import Path
import os, shutil, sys
root=Path(__file__).resolve().parents[2]
art=Path(os.environ['MESHLETS_ART'])
library=Path(sys.argv[1]).resolve()
out=Path(sys.argv[2]).resolve()
out.mkdir(parents=True,exist_ok=True)
(out/'src').mkdir(exist_ok=True)
manifest=(root/'parity/peers/Cargo.toml').read_text().replace('path = "../.."',f'path = "{library}"')
(out/'Cargo.toml').write_text(manifest)
shutil.copy2(root/'parity/peers/Cargo.lock',out/'Cargo.lock')
s=(root/'parity/peers/src/main.rs').read_text()
ours='''  "meshlets-flex"|"meshlets-spatial"=>{let settings=m::MeshletSettings{max_vertices:64,max_triangles:124};let r=if op=="meshlets-flex"{m::build_meshlets_flex(&x.i,p,settings,64,0.,0.,w)}else{m::build_meshlets_spatial(&x.i,p,settings,64,0.5,w)}.unwrap();Output::Mesh(r.meshlets.iter().map(|v|[v.vertex_offset,v.triangle_offset,v.vertex_count,v.triangle_count]).collect(),r.vertices,r.triangles)},
'''
cpp='''  "meshlets-flex"|"meshlets-spatial"=>{let bound=f::meshopt_buildMeshletsBound(ni,64,64);let mut ms=vec![std::mem::zeroed::<f::meshopt_Meshlet>();bound];let mut v=vec![0;bound*64];let mut t=vec![0;bound*124*3];let k=if op=="meshlets-flex"{f::meshopt_buildMeshletsFlex(ms.as_mut_ptr(),v.as_mut_ptr(),t.as_mut_ptr(),i,ni,p,n,12,64,64,124,0.,0.)}else{f::meshopt_buildMeshletsSpatial(ms.as_mut_ptr(),v.as_mut_ptr(),t.as_mut_ptr(),i,ni,p,n,12,64,64,124,0.5)};ms.truncate(k);mesh(ms.iter().map(|m|[m.vertex_offset,m.triangle_offset,m.vertex_count,m.triangle_count]).collect(),v,t)},
'''
rust='''  "meshlets-flex"|"meshlets-spatial"=>{let bound=c::build_meshlets_bound(ni,64,64);let mut ms=vec![c::Meshlet::default();bound];let mut v=vec![0;bound*64];let mut t=vec![0;bound*124*3];let mut buffers=c::MeshletBuffers{meshlets:&mut ms,vertices:&mut v,triangles:&mut t};let p=c::Positions{data:positions,count:n,stride:12};let k=if op=="meshlets-flex"{c::build_meshlets_flex(&mut buffers,&x.i,&p,&c::MeshletOptions{max_vertices:64,min_triangles:64,max_triangles:124,cone_weight:0.,split_factor:0.})}else{c::build_meshlets_spatial(&mut buffers,&x.i,&p,64,64,124,0.5)};ms.truncate(k);mesh(ms.iter().map(|m|[m.vertex_offset,m.triangle_offset,m.vertex_count,m.triangle_count]).collect(),v,t)},
'''
needle='  "meshlets"=>'
assert s.count(needle)==3
parts=s.split(needle)
s=parts[0]+ours+needle+parts[1]+cpp+needle+parts[2]+rust+needle+parts[3]
s=s.replace('if op=="meshlets"{','if op.starts_with("meshlets"){')
(out/'src/main.rs').write_text(s)
