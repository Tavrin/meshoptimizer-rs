//! Compare selected meshlet cases over every supported position stride and work limit.
macro_rules! observe {
    ($lib:ident, $p:expr, $indices:expr, $variant:expr, $kind:expr, $stride:expr, $offset:expr, $short:expr, $limit:expr) => {{
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
        let settings = m::MeshletSettings { max_vertices: 8, max_triangles: 4 };
        let bound = m::build_meshlets_bound(indices.len(), 8, 1).unwrap();
        let marker = m::Meshlet { vertex_offset: 123, triangle_offset: 234, vertex_count: 45, triangle_count: 56 };
        let mut ms = vec![marker; bound + 2];
        let mut vs = vec![0xdeadbeef; indices.len() + 2];
        let mut ts = vec![0xa5; indices.len() + 2];
        let mut ws = m::Workspace::new(m::Limits { max_work: $limit, ..Default::default() });
        let result = match $variant {
            0 => m::build_meshlets_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 0., &mut ws),
            1 => m::build_meshlets_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 0.75, &mut ws),
            2 => m::build_meshlets_flex_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 1, 0., 2., &mut ws),
            3 => m::build_meshlets_flex_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 2, 0.75, 0., &mut ws),
            _ => m::build_meshlets_spatial_into(&mut ms, &mut vs, &mut ts, indices, positions, settings, 1, 0.5, &mut ws),
        };
        let work = ws.usage().work;
        (format!("{result:?} {:?} {ms:?} {vs:?} {ts:?}", ws.usage()), work)
    }};
}
fn main() {
    let mut p = Vec::new();
    let mut indices = Vec::new();
    for i in 0..24 {
        let x = (i % 6) as f32 * 10.;
        let y = (i / 6) as f32 * 10.;
        p.extend([[x,y,0.], [x+1.,y,0.], [x,y+1.,0.]]);
        indices.extend([3*i,3*i+1,3*i+2]);
    }
    let mut cases = vec![(p.clone(), indices.clone()), (vec![[0.;3];p.len()], indices.clone()), (vec![[0.;3];3], vec![0,1,2])];
    p[0][0] = 1.0e9;
    cases.push((p,indices));
    let mut checks = [0u64; 5];
    for (case, (p, indices)) in cases.iter().enumerate() {
        for kind in 1..=3usize {
            let strides: Vec<_> = if kind == 1 { (3..=64).collect() } else { (12..=256).step_by(4).collect() };
            for stride in strides {
                for offset in 0..4 {
                    for short in [false, true] {
                        for variant in 0..5 {
                            let a = observe!(baseline, p, indices, variant, kind, stride, offset, short, u64::MAX);
                            let b = observe!(candidate, p, indices, variant, kind, stride, offset, short, u64::MAX);
                            assert_eq!(a, b, "unlimited case={case} kind={kind} stride={stride} offset={offset} short={short} variant={variant}");
                            for limit in 0..=a.1 + 1 {
                                let a = observe!(baseline, p, indices, variant, kind, stride, offset, short, limit);
                                let b = observe!(candidate, p, indices, variant, kind, stride, offset, short, limit);
                                assert_eq!(a, b, "case={case} kind={kind} stride={stride} offset={offset} short={short} variant={variant} limit={limit}");
                                checks[variant] += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("case={case} observations={checks:?}");
    }
    println!("PASS observations={checks:?}");
}
