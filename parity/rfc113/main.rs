use meshoptimizer_rs::{clusterlod, Attributes, Bounds, Positions, VertexFlags, Workspace};
use std::{collections::BTreeMap, io::{self, Read, Write}};

fn word(bytes: &[u8], at: usize) -> u32 { u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()) }
fn put(out: &mut Vec<u8>, value: u32) { out.extend_from_slice(&value.to_le_bytes()); }
fn bounds(out: &mut Vec<u8>, b: clusterlod::LodBounds) {
    for v in b.center { put(out, v.to_bits()); }
    put(out, b.radius.to_bits()); put(out, b.error.to_bits());
}
fn detailed_bounds(out:&mut Vec<u8>,b:Bounds) {
    for x in b.center {put(out,x.to_bits());}
    put(out,b.radius.to_bits());
    for x in b.cone_apex {put(out,x.to_bits());}
    for x in b.cone_axis {put(out,x.to_bits());}
    put(out,b.cone_cutoff.to_bits());
    for x in b.cone_axis_s8 {out.push(x as u8);}
    out.push(b.cone_cutoff_s8 as u8);
}
fn run(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.len() < 16 || !matches!(&input[..4],b"R113"|b"R11B"|b"R11T") { return Err("bad input".into()); }
    let bounds_mode=&input[..4]==b"R11B";
    let nv=word(input,4) as usize; let ni=word(input,8) as usize; let stride=word(input,12) as usize;
    if nv > 4<<20 || ni > 16<<20 || stride < 32 || stride > 256 || stride%4 != 0 || ni%3 != 0 ||
        16usize.checked_add(nv.checked_mul(stride).ok_or("size overflow")?)
        .and_then(|v|v.checked_add(ni*4)).and_then(|v|v.checked_add(nv)) != Some(input.len()) {
        return Err("bad input sizes".into());
    }
    let floats: Vec<f32> = input[16..16+nv*stride].chunks_exact(4)
        .map(|b|f32::from_le_bytes(b.try_into().unwrap())).collect();
    let indices: Vec<u32> = input[16+nv*stride..16+nv*stride+ni*4].chunks_exact(4)
        .map(|b|u32::from_le_bytes(b.try_into().unwrap())).collect();
    let locks: Vec<VertexFlags> = input[16+nv*stride+ni*4..].iter()
        .map(|&b|VertexFlags::from_bits(b).map_err(|e|e.to_string())).collect::<Result<_,_>>()?;
    let mut positions: Vec<[f32;3]> = floats.chunks_exact(stride/4).map(|v|[v[0],v[1],v[2]]).collect();
    let width=stride/4-3;
    let attrs: Vec<f32> = floats.chunks_exact(stride/4).flat_map(|v|v[3..].iter().copied()).collect();
    let attributes=Attributes::from_interleaved(&attrs,nv,width,width,0).map_err(|e|e.to_string())?;
    let mut weights=vec![0f32;width];weights[..3].fill(0.5);
    let mut ws=Workspace::default();
    let mut cfg=clusterlod::default_config(128).map_err(|e|e.to_string())?;
    cfg.simplify_dilate_borders=false;
    let mut meta=Vec::new();
    let mut vsrc=Vec::<u32>::new(); let mut vpos=Vec::<[f32;3]>::new(); let mut out_indices=Vec::<u32>::new();
    let mut cl=Vec::new(); let mut gr=Vec::new(); let mut detail=Vec::new(); let mut dedupe=BTreeMap::<[u32;4],u32>::new();
    let mut local_ws=Workspace::default();
    clusterlod::build_with_output(cfg,clusterlod::Mesh {
        indices:&indices,positions:&mut positions,attributes:Some(attributes),vertex_lock:Some(&locks),
        attribute_weights:&weights,attribute_protect_mask:((1u32<<9)-1) | if stride==64 { (15u32)<<9 } else { 0 },
    },|g,clusters| {
        let gid=meta.len();
        let first_cluster=(cl.len() / 32) as u32;
        for c in clusters {
            let local=clusterlod::local_indices(&c.indices,&mut local_ws)?;
            if bounds_mode {
                let p=Positions::from_interleaved(&floats,nv,stride/4,0)?;
                let cb=meshoptimizer_rs::compute_cluster_bounds(&c.indices,p,&mut local_ws)?;
                let mb=meshoptimizer_rs::compute_meshlet_bounds(&local.vertices,&local.triangles,p,&mut local_ws)?;
                detailed_bounds(&mut detail,cb);detailed_bounds(&mut detail,mb);
            }
            let mut remap=Vec::with_capacity(local.vertices.len());
            for src in local.vertices {
                let at=src as usize*stride/4;let p=[floats[at],floats[at+1],floats[at+2]];
                let key=[src,p[0].to_bits(),p[1].to_bits(),p[2].to_bits()];
                let dst=if let Some(&v)=dedupe.get(&key) {v} else {
                    let v=vsrc.len() as u32; vsrc.push(src);vpos.push(p);dedupe.insert(key,v);v
                }; remap.push(dst);
            }
            put(&mut cl,out_indices.len() as u32);put(&mut cl,c.indices.len() as u32);
            put(&mut cl,gid as u32);put(&mut cl,c.refined as u32);
            for x in c.bounds.center {put(&mut cl,x.to_bits());} put(&mut cl,c.bounds.radius.to_bits());
            out_indices.extend(local.triangles.iter().map(|&i|remap[i as usize]));
        }
        bounds(&mut gr,g.simplified);put(&mut gr,g.depth as u32);
        put(&mut gr,first_cluster);put(&mut gr,clusters.len() as u32);
        meta.push(g);
        Ok(gid as i32)
    },&mut ws).map_err(|e|format!("build: {e}"))?;
    let levels=meta.iter().map(|g|g.depth as usize+1).max().unwrap_or(1);
    let bound=clusterlod::build_hierarchy_bound(meta.len(),8,levels).map_err(|e|e.to_string())?;
    if levels>64 || bound>1<<20 { return Err("hierarchy ceiling".into()); }
    let nodes=clusterlod::build_hierarchy(&meta,8,levels,&mut ws).map_err(|e|e.to_string())?;
    let mut depth=0u32;
    let mut stack:Vec<(usize,u32)>=(0..levels).map(|i|(i,1)).collect();
    while let Some((i,d))=stack.pop() {
        depth=depth.max(d); if depth>32 || i>=nodes.len() {return Err("hierarchy depth".into());}
        let n=nodes[i]; if n.group<0 { for k in 0..n.child_count {stack.push(((n.child_offset+k) as usize,d+1));} }
    }
    let mut out=Vec::new();
    for n in [0x444c434d,1,nv as u32,ni as u32,2,vsrc.len() as u32,out_indices.len() as u32,
        (cl.len()/32) as u32,meta.len() as u32,nodes.len() as u32,levels as u32,depth] {put(&mut out,n);}
    for (src,p) in vsrc.iter().zip(vpos.iter()) {put(&mut out,*src);for &x in p {put(&mut out,x.to_bits());}}
    for i in out_indices {put(&mut out,i);}
    out.extend(cl);out.extend(gr);
    for n in nodes {bounds(&mut out,n.bounds);put(&mut out,n.group as u32);put(&mut out,n.child_offset);put(&mut out,n.child_count);}
    if bounds_mode {put(&mut out,0x32444e42);put(&mut out,(detail.len()/96) as u32);out.extend(detail);}
    Ok(out)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin=io::stdin().lock();let mut stdout=io::stdout().lock();
    loop {
        let mut size=[0u8;4];match stdin.read_exact(&mut size) {Ok(())=>(),Err(e) if e.kind()==io::ErrorKind::UnexpectedEof=>break,Err(e)=>return Err(e.into())};
        let n=u32::from_le_bytes(size) as usize;if n>256<<20 {return Err("large input".into());}
        let mut input=vec![0;n];stdin.read_exact(&mut input)?;
        let started=std::time::Instant::now();
        let mut out=run(&input)?;
        if &input[..4]==b"R11T" {out.extend_from_slice(&(started.elapsed().as_nanos() as u64).to_le_bytes());}
        stdout.write_all(&(out.len() as u32).to_le_bytes())?;stdout.write_all(&out)?;stdout.flush()?;
    }
    Ok(())
}
