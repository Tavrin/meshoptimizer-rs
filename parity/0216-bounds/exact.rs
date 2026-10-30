//! Bitwise observable snapshots, every work budget and workspace reuse.
macro_rules! driver {
    ($name:ident, $krate:ident) => {
        mod $name {
            use $krate as m;
            fn bits(b: m::Bounds) -> Vec<u32> {
                let mut v = b.center.map(f32::to_bits).to_vec();
                v.push(b.radius.to_bits());
                v.extend(b.cone_apex.map(f32::to_bits));
                v.extend(b.cone_axis.map(f32::to_bits));
                v.push(b.cone_cutoff.to_bits());
                v.extend(b.cone_axis_s8.map(|x| x as u32));
                v.push(b.cone_cutoff_s8 as u32);
                v
            }
            fn call(p: &[[f32; 3]], i: &[u32], op: usize, w: &mut m::Workspace) -> String {
                let pos = m::Positions::from_packed(p);
                match op {
                    0..=3 => {
                        let s = m::MeshletSettings {max_vertices: 8, max_triangles: 4};
                        let mut ms = vec![m::Meshlet {vertex_offset: 123, triangle_offset: 234, vertex_count: 45, triangle_count: 56}; i.len()+2];
                        let mut vs = vec![0xdeadbeef; i.len()+2];
                        let mut ts = vec![0xa5; i.len()+2];
                        let r = if op < 2 {m::build_meshlets_into(&mut ms, &mut vs, &mut ts, i, pos, s, op as f32 * 0.75, w)} else {m::build_meshlets_flex_into(&mut ms, &mut vs, &mut ts, i, pos, s, 1, (op-2) as f32 * 0.75, 2., w)};
                        let first = format!("{r:?} {:?} {ms:?} {vs:?} {ts:?}",w.usage());
                        let owned = if op < 2 {m::build_meshlets(i, pos, s, op as f32 * 0.75, w)} else {m::build_meshlets_flex(i, pos, s, 1, (op-2) as f32 * 0.75, 2., w)};
                        format!("{first} {owned:?}")
                    }
                    4..=7 => {
                        let vertices: Vec<_> = (0..p.len() as u32).collect();
                        let triangles: Vec<_> = i.iter().map(|&v|v as u8).collect();
                        let cached = m::BoundsPositions::new(pos);
                        let r = match op {
                            4 => m::compute_cluster_bounds(i,pos,w),
                            5 => m::compute_cluster_bounds_with_positions(i,cached,w),
                            6 => m::compute_meshlet_bounds(&vertices,&triangles,pos,w),
                            _ => m::compute_meshlet_bounds_with_positions(&vertices,&triangles,cached,w),
                        }.map(bits);
                        format!("{r:?}")
                    }
                    11 => {
                        let radii:Vec<_>=(0..p.len()).map(|j| (j%3) as f32*0.1).collect();
                        let r=m::Attributes::from_interleaved(&radii,p.len(),1,1,0).unwrap();
                        format!("{:?}",m::compute_sphere_bounds(pos,Some(r),w).map(bits))
                    }
                    10 => format!("{:?}",m::compute_sphere_bounds(pos,None,w).map(bits)),
                    _ => {
                        let mut p = p.to_vec();
                        let mut records = Vec::<u32>::new();
                        let mut cfg = m::clusterlod::default_config(4).unwrap();
                        cfg.simplify_dilate_borders = op == 9;
                        let r=m::clusterlod::build_with_output(cfg,m::clusterlod::Mesh {indices:i,positions:&mut p,attributes:None,vertex_lock:None,attribute_weights:&[],attribute_protect_mask:0}, |g,cs| {
                            records.extend([g.depth as u32,cs.len() as u32]);
                            records.extend(g.simplified.center.map(f32::to_bits));
                            records.extend([g.simplified.radius.to_bits(),g.simplified.error.to_bits()]);
                            for c in cs {records.extend([c.refined as u32,c.vertex_count as u32]);records.extend(c.bounds.center.map(f32::to_bits));records.extend([c.bounds.radius.to_bits(),c.bounds.error.to_bits()]);records.extend(&c.indices);}
                            Ok(records.len() as i32)
                        },w);
                        let p:Vec<_>=p.iter().flat_map(|p|p.map(f32::to_bits)).collect();
                        format!("{r:?} {records:?} {p:?}")
                    }
                }
            }
            pub fn run(p: &[[f32;3]], i: &[u32], op:usize, work:u64, bytes:usize, state:usize) -> (String,u64,usize) {
                let mut w=m::Workspace::new(m::Limits{max_work:work,max_bytes:bytes});
                if state==1 {let _=call(p,i,op,&mut w);}
                if state==2 {
                    let _=call(p,i,op,&mut w);
                    let _=m::build_meshlets_into(&mut [], &mut [], &mut [], &[0,1,2], m::Positions::from_packed(p),m::MeshletSettings{max_vertices:8,max_triangles:4},0.,&mut w);
                }
                if state==3 {let _=m::optimize_vertex_cache(&[0,1,2,2,1,3],4,&mut w);}
                let r=call(p,i,op,&mut w);
                (format!("{r} {:?}",w.usage()),w.usage().work,w.usage().bytes)
            }
        }
    }
}
driver!(base,baseline);
driver!(cand,candidate);
fn main() {
    let mut total=0u64;
    for case in 0..12 {
        let p:Vec<[f32;3]>=(0..12).map(|v| {
            let a=(v%4) as f32;let b=(v/4) as f32;
            match case {
                0=>[a,b,a*b*0.1], 1=>[0.;3], 2=>[a*1e-20,b*1e-20,0.],
                3=>[a*1e9,b*1e9,0.],4=>[if v%2==0 {-0.}else{0.},b,a],
                5=>[a,b,if v==2 {f32::NAN}else{0.}],6=>[a,b,if v==1 {f32::MAX}else{0.}],
                _=>[a,b,((v*37+case*11)%17) as f32*0.03]
            }
        }).collect();
        let mut i=vec![0,1,4,4,1,5,1,2,5,5,2,6,2,3,6,6,3,7,4,5,8,8,5,9,5,6,9,9,6,10,6,7,10,10,7,11];
        if case==7 {i[0]=99;}
        if case==8 {i.pop();}
        if case==9 {i=vec![0,0,0,1,1,1];}
        if case==10 {i.clear();}
        for op in 0..12 {
            let (_,max,peak)=base::run(&p,&i,op,u64::MAX,usize::MAX,0);
            for state in 0..4 {
                for work in 0..=max+1 {
                    let a=base::run(&p,&i,op,work,usize::MAX,state);
                    let b=cand::run(&p,&i,op,work,usize::MAX,state);
                    assert_eq!(a,b,"case {case} op {op} work {work} state {state}");total+=1;
                }
                for bytes in [0,1,peak.saturating_sub(1),peak,peak.saturating_add(1)] {
                    for work in [0,max/2,max,max+1] {
                        assert_eq!(base::run(&p,&i,op,work,bytes,state),cand::run(&p,&i,op,work,bytes,state),"bytes case {case} op {op}");total+=1;
                    }
                }
            }
        }
    }
    // Cross the 1/8-lane, 32/128/512-triangle stack buckets and maximum
    // local-vertex boundary; sample limits on these larger cases.
    for triangles in [1,7,8,9,31,32,33,127,128,129,511,512] {
        let p:Vec<_>=(0..256).map(|v|[(v%16) as f32,(v/16) as f32,((v*37)%29) as f32*0.01]).collect();
        let i:Vec<_>=(0..triangles*3).map(|v|((v*17+v/3)%256) as u32).collect();
        for op in 4..8 {
            let (_,max,peak)=base::run(&p,&i,op,u64::MAX,usize::MAX,0);
            for state in 0..4 {
                for work in [0,1,max/2,max.saturating_sub(1),max,max+1] {
                    for bytes in [0,peak,usize::MAX] {
                        assert_eq!(base::run(&p,&i,op,work,bytes,state),cand::run(&p,&i,op,work,bytes,state),"boundary {triangles} op {op}");total+=1;
                    }
                }
            }
        }
    }
    println!("PASS {total} exact comparisons");
}
