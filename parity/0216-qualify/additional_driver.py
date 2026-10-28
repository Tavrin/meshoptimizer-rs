"""Add three missing simplify operations to the peers consumer."""
from pathlib import Path
import sys
p = Path(sys.argv[1])
s = p.read_text()
ours = '''  "simplify-update"=>{let mut i=x.i.clone();let mut v=x.p.clone();let r=m::simplify_with_update(&mut i,&mut m::PositionsMut::from_packed(&mut v),None,&[],None,settings,w).unwrap();i.truncate(r.index_count);update_out(i,r.error,v)},
  "simplify-prune"=>Output::Indices(m::simplify_prune(&x.i,p,0.5,w).unwrap()),
  "simplify-points"=>Output::Indices(m::simplify_points(p,None,0.,n/2,w).unwrap()),
'''
cpp = '''  "simplify-update"=>{let mut b=x.i.clone();let mut v=x.p.clone();let mut e=0.;let k=f::meshopt_simplifyWithUpdate(b.as_mut_ptr(),ni,v.as_mut_ptr().cast(),n,12,std::ptr::null_mut(),0,std::ptr::null(),0,std::ptr::null(),x.target,x.error,0,&mut e);b.truncate(k);update_out(b,e,v)},
  "simplify-prune"=>{let mut b=vec![0;ni];let k=f::meshopt_simplifyPrune(b.as_mut_ptr(),i,ni,p,n,12,0.5);b.truncate(k);Output::Indices(b)},
  "simplify-points"=>{let mut b=vec![0;n/2];let k=f::meshopt_simplifyPoints(b.as_mut_ptr(),p,n,12,std::ptr::null(),0,0.,n/2);b.truncate(k);Output::Indices(b)},
'''
rust = '''  "simplify-update"=>{let mut b=x.i.clone();let mut v=x.p.clone();let mut attrs=[];let(k,e)=s::simplify_with_update(&mut b,&mut s::VertexDataMut{positions:v.as_flattened_mut(),count:n,stride:12},&mut s::AttributesMut{data:&mut attrs,stride:0,weights:&[],count:0},None,&s::SimplifyTarget{target_index_count:x.target,target_error:x.error,options:0});b.truncate(k);update_out(b,e,v)},
  "simplify-prune"=>{let mut b=vec![0;ni];let k=s::simplify_prune(&mut b,&x.i,&s::VertexData{positions,count:n,stride:12},0.5);b.truncate(k);Output::Indices(b)},
  "simplify-points"=>{let mut b=vec![0;n/2];let k=s::simplify_points(&mut b,&s::VertexData{positions,count:n,stride:12},None,0.,n/2);b.truncate(k);Output::Indices(b)},
'''
for anchor, add in [('  "cache"=>Output::Indices(m::',ours),('  "cache"|"overdraw"=>{let mut b=vec![0;ni];if op=="cache"{f::',cpp),('  "cache"|"overdraw"=>{let mut b=vec![0;ni];if op=="cache"{optimesh::',rust)]:
    assert s.count(anchor)==1
    s=s.replace(anchor,add+anchor,1)
s=s.replace('enum Output { Bytes(', 'enum Output { Updated(Vec<u32>,f32,Vec<[f32;3]>), Bytes(')
s=s.replace('   Self::RawMesh(ms,v,t)=>', '   Self::Updated(i,e,p)=>{let mut b=(i.len() as u32).to_le_bytes().to_vec();b.extend(ws(i));b.extend(e.to_le_bytes());b.extend(bytes(p));b},Self::RawMesh(ms,v,t)=>')
anchor='// Normalize only during serialization, outside the counted region.'
s=s.replace(anchor,'''fn update_out(i:Vec<u32>,e:f32,p:Vec<[f32;3]>)->Output {Output::Updated(i,e,p)}
'''+anchor)
p.write_text(s)
