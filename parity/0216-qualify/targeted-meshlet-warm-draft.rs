//! Compare selected meshlet cases over every supported position stride and work limit.
macro_rules! observe {
    ($lib:ident, $p:expr, $indices:expr, $variant:expr, $setting:expr, $kind:expr, $stride:expr, $offset:expr, $short:expr, $limit:expr) => {{
        use $lib as m;
        let p = $p;
        let indices = $indices;
        let kind = $kind;
        let stride = $stride;
        let offset = $offset;
        let mut floats = vec![f32::NAN; if kind == 1 { offset + p.len() * stride } else { 0 }];
        let mut bytes = vec![0xa5u8; if kind > 1 { offset + p.len() * stride } else { 0 }];
        for (i, v) in p.iter().enumerate() {
            if kind == 1 {
                floats[offset + i * stride..offset + i * stride + 3].copy_from_slice(v);
            } else {
                for (j, x) in v.iter().enumerate() {
                    let bits = if kind == 2 { x.to_le_bytes() } else { x.to_be_bytes() };
                    let at = offset + i * stride + j * 4;
                    bytes[at..at + 4].copy_from_slice(&bits);
                }
            }
        }
        if $short && !p.is_empty() {
            if kind == 1 { floats.truncate(offset + (p.len() - 1) * stride + 3); }
            else { bytes.truncate(offset + (p.len() - 1) * stride + 12); }
        }
        let positions = if kind == 1 {
            m::Positions::from_interleaved(&floats, p.len(), stride, offset).unwrap()
        } else {
            m::Positions::from_bytes(&bytes, p.len(), stride, offset,
                if kind == 2 { m::ByteOrder::LittleEndian } else { m::ByteOrder::BigEndian }).unwrap()
        };
        let settings = m::MeshletSettings { max_vertices: $setting.0, max_triangles: $setting.1 };
        let bound = m::build_meshlets_bound(indices.len(), $setting.0, 1).unwrap();
        let marker = m::Meshlet { vertex_offset: 123, triangle_offset: 234, vertex_count: 45, triangle_count: 56 };
        let mut ws = m::Workspace::new(m::Limits { max_work: $limit, ..Default::default() });
        let mut owned = m::Workspace::new(m::Limits { max_work: $limit, ..Default::default() });
        let pp = [[0.0f32,0.0,0.0], [1.0,0.0,0.0], [0.0,1.0,0.0]];
        let ps = m::MeshletSettings { max_vertices: 3, max_triangles: 1 };
        let pa = m::build_meshlets(&[0,1,2], m::Positions::from_packed(&pp), ps, 0.0, &mut ws);
        let pa_obs = (format!("{pa:?}"), pa.is_ok(), ws.usage().work, ws.usage().bytes);
        let pb = m::build_meshlets(&[0,1,2], m::Positions::from_packed(&pp), ps, 0.0, &mut owned);
        let pb_obs = (format!("{pb:?}"), pb.is_ok(), owned.usage().work, owned.usage().bytes);
        let mut records = Vec::new();
        let mut work = 0;
        for _ in 0..2 {
        let mut ms = vec![marker; bound + 2];
        let mut vs = vec![0xdeadbeef; indices.len() + 2];
        let mut ts = vec![0xa5; indices.len() + 2];
        let result = match $variant {
            0 => m::build_meshlets_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 0., &mut ws),
            1 => m::build_meshlets_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 0.75, &mut ws),
            2 => m::build_meshlets_flex_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 1, 0., 2., &mut ws),
            3 => m::build_meshlets_flex_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 1, 0.75, 0., &mut ws),
            _ => m::build_meshlets_spatial_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 1, 0.5, &mut ws),
        };
        let allocated = match $variant {
            0 => m::build_meshlets(indices, positions, settings, 0., &mut owned),
            1 => m::build_meshlets(indices, positions, settings, 0.75, &mut owned),
            2 => m::build_meshlets_flex(indices, positions, settings, 1, 0., 2., &mut owned),
            3 => m::build_meshlets_flex(indices, positions, settings, 1, 0.75, 0., &mut owned),
            _ => m::build_meshlets_spatial(indices, positions, settings, 1, 0.5, &mut owned),
        };
        work = work.max(ws.usage().work.max(owned.usage().work));
        records.push(format!("{result:?} {:?} {ms:?} {vs:?} {ts:?} {allocated:?} {:?}", ws.usage(), owned.usage()));
        }
        (records, work, pa_obs, pb_obs)
    }};
}
fn main() {
    let mut p = Vec::new(); let mut indices = Vec::new();
    for i in 0..24 { let x = (i % 6) as f32 * 10.; let y = (i / 6) as f32 * 10.;
        p.extend([[x,y,0.], [x+1.,y,0.], [x,y+1.,0.]]); indices.extend([3*i,3*i+1,3*i+2]); }
    let mut cases = vec![(p.clone(),indices.clone()),(vec![[0.;3];p.len()],indices.clone()),(vec![[0.;3];3],vec![0,1,2])];
    p[0][0] = 1.0e9; cases.push((p, indices.clone()));
    let collinear = (0..72).map(|i| [i as f32,0.,0.]).collect(); cases.push((collinear,indices.clone()));
    let clustered = (0..72).map(|i| [(i/6) as f32 * 100.,(i%2) as f32 * 0.001,(i%3) as f32 * 0.001]).collect(); cases.push((clustered,indices.clone()));
    let grid = (0..72).map(|i| [(i%2) as f32,((i/2)%2) as f32,((i/4)%2) as f32]).collect(); cases.push((grid,indices.clone()));
    let settings = [(3usize,1usize),(8,4),(64,124),(255,512)];
    let mut checks=[0u64;5]; let mut attempts=0u64; let mut successes=0u64;
    for (case,(p,indices)) in cases.iter().enumerate() {
        for setting in settings { for variant in 0..5 {
            let a=observe!(baseline,p,indices,variant,setting,1,3,0,false,u64::MAX);
            let b=observe!(candidate,p,indices,variant,setting,1,3,0,false,u64::MAX);
            assert_eq!(a,b,"unlimited case={case} setting={setting:?} variant={variant}");
            let end=a.1 + 4 * (indices.len() as u64 / 3) + 16;
            for limit in 0..=end {
                let a=observe!(baseline,p,indices,variant,setting,1,3,0,false,limit);
                let b=observe!(candidate,p,indices,variant,setting,1,3,0,false,limit);
                assert_eq!(a,b,"case={case} setting={setting:?} variant={variant} limit={limit}"); checks[variant]+=2; attempts+=2; successes+=u64::from(a.2.1)+u64::from(a.3.1);
            }
        }}
        println!("case={case} checks={checks:?}");
    }
    println!("PASS cells=140 checks={checks:?} priming_attempts={attempts} priming_success={successes}");
}
