#[cfg(feature = "theirs")]
use meshopt::ffi as f;
#[cfg(feature = "ours")]
use meshoptimizer_rs as m;
use std::{
    hint::black_box,
    io::{self, BufRead, Write},
    time::Instant,
};

struct Input {
    p: Vec<[f32; 3]>,
    raw: Vec<u8>,
    i: Vec<u32>,
    vb: Vec<u8>,
    ib: Vec<u8>,
    seq: Vec<u8>,
    filters: [Vec<u8>; 4],
    floats: Vec<f32>,
    quat: Vec<f32>,
}
enum Output {
    #[cfg(feature = "ours")]
    MeshOurs(m::Meshlets),
    #[cfg(feature = "theirs")]
    MeshTheirs(Vec<f::meshopt_Meshlet>, Vec<u32>, Vec<u8>),
    Bytes(Vec<u8>),
    Indices(Vec<u32>),
    Simplify(Vec<u32>, f32),
    Fetch(Vec<u32>, Vec<u8>),
    Mesh(Vec<[u32; 4]>, Vec<u32>, Vec<u8>),
}
impl Output {
    fn raw_bytes(&self) -> Vec<u8> {
        #[cfg(feature = "theirs")]
        if let Self::MeshTheirs(ms, v, t) = self {
            let nv = ms
                .iter()
                .map(|m| m.vertex_offset as usize + m.vertex_count as usize)
                .max()
                .unwrap_or(0);
            let nt = ms
                .last()
                .map(|m| m.triangle_offset as usize + ((m.triangle_count as usize * 3 + 3) & !3))
                .unwrap_or(0);
            let mut b = (ms.len() as u32).to_le_bytes().to_vec();
            b.extend((nv as u32).to_le_bytes());
            for m in ms {
                for a in [
                    m.vertex_offset,
                    m.triangle_offset,
                    m.vertex_count,
                    m.triangle_count,
                ] {
                    b.extend(a.to_le_bytes());
                }
            }
            for a in &v[..nv] {
                b.extend(a.to_le_bytes());
            }
            b.extend(&t[..nt]);
            return b;
        }
        self.bytes()
    }
    fn bytes(&self) -> Vec<u8> {
        fn words(v: &[u32]) -> Vec<u8> {
            v.iter().flat_map(|x| x.to_le_bytes()).collect()
        }
        match self {
            #[cfg(feature = "ours")]
            Self::MeshOurs(r) => Output::Mesh(
                r.meshlets
                    .iter()
                    .map(|a| {
                        [
                            a.vertex_offset,
                            a.triangle_offset,
                            a.vertex_count,
                            a.triangle_count,
                        ]
                    })
                    .collect(),
                r.vertices.clone(),
                r.triangles.clone(),
            )
            .bytes(),
            #[cfg(feature = "theirs")]
            Self::MeshTheirs(ms, v, t) => {
                let mut packed = Vec::new();
                let mut desc = Vec::new();
                let mut nv = 0;
                for m in ms {
                    let off = packed.len() as u32;
                    packed.extend_from_slice(
                        &t[m.triangle_offset as usize
                            ..m.triangle_offset as usize + m.triangle_count as usize * 3],
                    );
                    nv = nv.max(m.vertex_offset as usize + m.vertex_count as usize);
                    desc.push([m.vertex_offset, off, m.vertex_count, m.triangle_count]);
                }
                Output::Mesh(desc, v[..nv].to_vec(), packed).bytes()
            }
            Self::Bytes(v) => v.clone(),
            Self::Indices(v) => words(v),
            Self::Simplify(v, e) => {
                let mut b = words(v);
                b.extend(e.to_le_bytes());
                b
            }
            Self::Fetch(i, v) => {
                let mut b = (i.len() as u32).to_le_bytes().to_vec();
                b.extend(words(i));
                b.extend(v);
                b
            }
            Self::Mesh(ms, v, t) => {
                let mut b = (ms.len() as u32).to_le_bytes().to_vec();
                b.extend((v.len() as u32).to_le_bytes());
                for d in ms {
                    b.extend(words(d));
                }
                b.extend(words(v));
                b.extend(t);
                b
            }
        }
    }
}
impl Input {
    fn read(path: &str) -> Self {
        let b = std::fs::read(path).unwrap();
        let mut c = 0;
        fn u(b: &[u8], c: &mut usize) -> u32 {
            let v = u32::from_le_bytes(b[*c..*c + 4].try_into().unwrap());
            *c += 4;
            v
        }
        let n = u(&b, &mut c) as usize;
        let ni = u(&b, &mut c) as usize;
        let p: Vec<[f32; 3]> = (0..n)
            .map(|_| std::array::from_fn(|_| f32::from_bits(u(&b, &mut c))))
            .collect();
        let i = (0..ni).map(|_| u(&b, &mut c)).collect();
        let mut buffers = Vec::new();
        for _ in 0..7 {
            let len = u(&b, &mut c) as usize;
            buffers.push(b[c..c + len].to_vec());
            c += len;
        }
        let floats: Vec<f32> = (0..n)
            .flat_map(|j| {
                let a = j as f32 * 0.01;
                [a.sin() * 0.5, a.cos() * 0.5, (1. - 0.25f32).sqrt(), 1.]
            })
            .collect();
        assert_eq!(c, b.len());
        let raw = p
            .iter()
            .flat_map(|p| p.iter().flat_map(|x| x.to_le_bytes()))
            .collect();
        let quat = floats
            .iter()
            .map(|v| v * std::f32::consts::FRAC_1_SQRT_2)
            .collect();
        Self {
            p,
            raw,
            i,
            vb: buffers.remove(0),
            ib: buffers.remove(0),
            seq: buffers.remove(0),
            filters: std::array::from_fn(|_| buffers.remove(0)),
            floats,
            quat,
        }
    }
}
#[cfg(feature = "ours")]
fn run(op: &str, x: &Input, w: &mut m::Workspace) -> Output {
    use m::codec as c;
    let n = x.p.len();
    let p = m::Positions::from_packed(&x.p);
    let settings = m::SimplifySettings {
        target_index_count: x.i.len() / 2 / 3 * 3,
        target_error: 0.5,
        options: m::SimplifyOptions::default(),
    };
    let vs = m::MeshletSettings {
        max_vertices: 64,
        max_triangles: 124,
    };
    match op {
        "vertex-decode" => Output::Bytes(c::decode_vertex_buffer(n, 12, &x.vb, w).unwrap()),
        "vertex-encode" | "vertex-encode-v0" | "vertex-encode-default" => Output::Bytes(
            c::encode_vertex_buffer(
                &x.raw,
                n,
                12,
                c::VertexEncoding::new(if op.ends_with("v0") { 0 } else { 1 }, 2).unwrap(),
                w,
            )
            .unwrap(),
        ),
        "index-decode" => Output::Bytes(c::decode_index_buffer(x.i.len(), 4, &x.ib, w).unwrap()),
        "index-encode" => {
            Output::Bytes(c::encode_index_buffer(&x.i, c::IndexEncoding::DEFAULT, w).unwrap())
        }
        "sequence-decode" => {
            Output::Bytes(c::decode_index_sequence(x.i.len(), 4, &x.seq, w).unwrap())
        }
        "sequence-encode" => {
            Output::Bytes(c::encode_index_sequence(&x.i, c::IndexEncoding::DEFAULT, w).unwrap())
        }
        "oct-decode" | "quat-decode" | "exp-decode" | "color-decode" => {
            let (k, stride) = match op {
                "oct-decode" => (0, 8),
                "quat-decode" => (1, 8),
                "exp-decode" => (2, 16),
                _ => (3, 8),
            };
            let mut b = x.filters[k].clone();
            match k {
                0 => c::decode_filter_oct(&mut b, n, stride, w),
                1 => c::decode_filter_quat(&mut b, n, stride, w),
                2 => c::decode_filter_exp(&mut b, n, stride, w),
                _ => c::decode_filter_color(&mut b, n, stride, w),
            }
            .unwrap();
            Output::Bytes(b)
        }
        "oct-encode" => Output::Bytes(c::encode_filter_oct(n, 8, 12, &x.floats, w).unwrap()),
        "quat-encode" => Output::Bytes(c::encode_filter_quat(n, 8, 12, &x.quat, w).unwrap()),
        "exp-encode" => Output::Bytes(
            c::encode_filter_exp(n, 16, 20, &x.floats, c::ExpMode::Separate, w).unwrap(),
        ),
        "color-encode" => {
            let a: Vec<f32> = x.floats.iter().map(|v| v.abs()).collect();
            Output::Bytes(c::encode_filter_color(n, 8, 12, &a, w).unwrap())
        }
        "cache" => Output::Indices(m::optimize_vertex_cache(&x.i, n, w).unwrap()),
        "cache-strip" => Output::Indices(m::optimize_vertex_cache_strip(&x.i, n, w).unwrap()),
        "cache-fifo" => Output::Indices(m::optimize_vertex_cache_fifo(&x.i, n, 16, w).unwrap()),
        "overdraw" => Output::Indices(m::optimize_overdraw(&x.i, p, 1.05, w).unwrap()),
        "fetch-remap" => Output::Indices(m::optimize_vertex_fetch_remap(&x.i, n, w).unwrap().remap),
        "fetch" => {
            let r = m::optimize_vertex_fetch(
                &x.i,
                m::VertexStream::new(&x.raw, n, 12, 12, 0).unwrap(),
                w,
            )
            .unwrap();
            Output::Fetch(r.indices, r.vertices)
        }
        "simplify" => {
            let r = m::simplify(&x.i, p, settings, w).unwrap();
            Output::Simplify(r.indices, r.error)
        }
        "simplify-scale" => Output::Bytes(m::simplify_scale(p).unwrap().to_le_bytes().to_vec()),
        "stripify" => Output::Indices(m::stripify(&x.i, n, u32::MAX, w).unwrap()),
        "unstripify" => {
            let strip = decode_words(&x.seq);
            Output::Indices(m::unstripify(&strip, u32::MAX, w).unwrap())
        }
        "meshlets" | "meshlets-scan" | "meshlets-flex" | "meshlets-spatial" => {
            let r = match op {
                "meshlets" => m::build_meshlets(&x.i, p, vs, 0., w),
                "meshlets-scan" => m::build_meshlets_scan(&x.i, n, vs, w),
                "meshlets-flex" => m::build_meshlets_flex(&x.i, p, vs, 64, 0., 0., w),
                _ => m::build_meshlets_spatial(&x.i, p, vs, 64, 0.5, w),
            }
            .unwrap();
            Output::MeshOurs(r)
        }

        "vertex-version" => Output::Bytes(vec![c::decode_vertex_version(&x.vb).unwrap()]),
        "index-version" => Output::Bytes(vec![c::decode_index_version(&x.ib).unwrap()]),
        "vertex-bound" => Output::Bytes(
            (c::encode_vertex_buffer_bound(n, 12).unwrap() as u64)
                .to_le_bytes()
                .to_vec(),
        ),
        "index-bound" => Output::Bytes(
            (c::encode_index_buffer_bound(x.i.len(), n).unwrap() as u64)
                .to_le_bytes()
                .to_vec(),
        ),
        "sequence-bound" => Output::Bytes(
            (c::encode_index_sequence_bound(x.i.len(), n).unwrap() as u64)
                .to_le_bytes()
                .to_vec(),
        ),
        "strip-bound" => Output::Bytes(
            (m::stripify_bound(x.i.len()).unwrap() as u64)
                .to_le_bytes()
                .to_vec(),
        ),
        "unstrip-bound" => Output::Bytes(
            (m::unstripify_bound(x.i.len()).unwrap() as u64)
                .to_le_bytes()
                .to_vec(),
        ),
        "meshlet-bound" => Output::Bytes(
            (m::build_meshlets_bound(x.i.len(), 64, 124).unwrap() as u64)
                .to_le_bytes()
                .to_vec(),
        ),
        "remap" | "remap-multi" | "remap-custom" | "remap-vertices" | "remap-indices" => {
            let v = m::VertexStream::new(&x.raw, n, 12, 12, 0).unwrap();
            let r = match op {
                "remap-custom" => m::generate_vertex_remap_custom(Some(&x.i), p, |_, _| true, w),
                "remap-multi" => m::generate_vertex_remap_multi(Some(&x.i), &[v], w),
                _ => m::generate_vertex_remap(Some(&x.i), v, w),
            }
            .unwrap();
            match op {
                "remap-vertices" => Output::Bytes(m::remap_vertex_buffer(v, &r.remap, w).unwrap()),
                "remap-indices" => {
                    Output::Indices(m::remap_index_buffer(Some(&x.i), &r.remap, w).unwrap())
                }
                _ => Output::Indices(r.remap),
            }
        }
        "shadow" | "shadow-multi" => {
            let v = m::VertexStream::new(&x.raw, n, 12, 12, 0).unwrap();
            Output::Indices(
                if op == "shadow" {
                    m::generate_shadow_index_buffer(&x.i, v, w)
                } else {
                    m::generate_shadow_index_buffer_multi(&x.i, &[v], w)
                }
                .unwrap(),
            )
        }
        "position-remap" => Output::Indices(m::generate_position_remap(p, w).unwrap()),
        "adjacency" => Output::Indices(m::generate_adjacency_index_buffer(&x.i, p, w).unwrap()),
        "tessellation" => {
            Output::Indices(m::generate_tessellation_index_buffer(&x.i, p, w).unwrap())
        }
        "provoking" => {
            let r = m::generate_provoking_index_buffer(&x.i, n, w).unwrap();
            let mut b = vec![r.reorder.len() as u32];
            b.extend(r.indices);
            b.extend(r.reorder);
            Output::Indices(b)
        }
        "simplify-attributes" => {
            let a = m::Attributes::from_interleaved(&x.floats, n, 4, 4, 0).unwrap();
            let r = m::simplify_with_attributes(&x.i, p, a, &[1.; 4], None, settings, w).unwrap();
            Output::Simplify(r.indices, r.error)
        }
        "simplify-sloppy" => {
            let r = m::simplify_sloppy(&x.i, p, None, settings.target_index_count, 0.5, w).unwrap();
            Output::Simplify(r.indices, r.error)
        }
        "simplify-prune" => Output::Indices(m::simplify_prune(&x.i, p, 0.5, w).unwrap()),
        "simplify-points" => Output::Indices(m::simplify_points(p, None, 0., n / 2, w).unwrap()),
        "simplify-update" => {
            let mut ib = x.i.clone();
            let mut pp = x.p.clone();
            let r = m::simplify_with_update(
                &mut ib,
                &mut m::PositionsMut::from_packed(&mut pp),
                None,
                &[],
                None,
                settings,
                w,
            )
            .unwrap();
            ib.truncate(r.index_count);
            let mut b = vec![ib.len() as u32];
            b.extend(ib);
            b.push(r.error.to_bits());
            b.extend(pp.iter().flatten().map(|v| v.to_bits()));
            Output::Indices(b)
        }
        "analyze-cache" => {
            let r = m::analyze_vertex_cache(&x.i, n, 16, 32, 128, w).unwrap();
            Output::Indices(vec![
                r.vertices_transformed,
                r.warps_executed,
                r.acmr.to_bits(),
                r.atvr.to_bits(),
            ])
        }
        "analyze-fetch" => {
            let r = m::analyze_vertex_fetch(&x.i, n, 12, w).unwrap();
            Output::Indices(vec![r.bytes_fetched, r.overfetch.to_bits()])
        }
        "analyze-overdraw" => {
            let r = m::analyze_overdraw(&x.i, p, w).unwrap();
            Output::Indices(vec![
                r.pixels_covered,
                r.pixels_shaded,
                r.overdraw.to_bits(),
            ])
        }
        "analyze-coverage" => {
            let r = m::analyze_coverage(&x.i, p, w).unwrap();
            floats(&[r.coverage[0], r.coverage[1], r.coverage[2], r.extent])
        }
        "cluster-bounds" | "sphere-bounds" | "meshlet-bounds" => {
            let r = match op {
                "cluster-bounds" => m::compute_cluster_bounds(&x.i[..x.i.len().min(124 * 3)], p, w),
                "sphere-bounds" => m::compute_sphere_bounds(p, None, w),
                _ => {
                    let v: Vec<u32> = (0..n.min(64) as u32).collect();
                    let t: Vec<u8> = vec![0, 1, 2];
                    m::compute_meshlet_bounds(&v, &t, p, w)
                }
            }
            .unwrap();
            bounds(
                r.center,
                r.radius,
                r.cone_apex,
                r.cone_axis,
                r.cone_cutoff,
                r.cone_axis_s8,
                r.cone_cutoff_s8,
            )
        }
        "optimize-meshlet" => {
            let v: Vec<u32> = (0..n.min(64) as u32).collect();
            let t: Vec<u8> = vec![0, 1, 2];
            let r = m::optimize_meshlet(&v, &t, w).unwrap();
            let mut b = (r.vertices.len() as u32).to_le_bytes().to_vec();
            for v in r.vertices {
                b.extend(v.to_le_bytes())
            }
            b.extend(r.triangles);
            Output::Bytes(b)
        }
        "partition" => {
            let counts: Vec<u32> = x.i.chunks(96).map(|c| c.len() as u32).collect();
            let r = m::partition_clusters(&x.i, &counts, n, Some(p), 4, w).unwrap();
            let mut b = vec![r.count as u32];
            b.extend(r.assignments);
            Output::Indices(b)
        }
        "spatial-remap" => Output::Indices(m::spatial_sort_remap(p, w).unwrap()),
        "spatial-triangles" => Output::Indices(m::spatial_sort_triangles(&x.i, p, w).unwrap()),
        "spatial-points" => Output::Indices(m::spatial_cluster_points(p, 16, w).unwrap()),
        "quantize-half" => Output::Indices(
            x.p.iter()
                .flatten()
                .map(|v| m::quantize_half(*v) as u32)
                .collect(),
        ),
        "dequantize-half" => floats(
            &(0..256)
                .map(|h| m::dequantize_half(h * 251))
                .collect::<Vec<_>>(),
        ),
        "quantize-float" => floats(
            &x.p.iter()
                .flatten()
                .map(|v| m::quantize_float(*v, 12).unwrap())
                .collect::<Vec<_>>(),
        ),
        "quantize-unorm" => Output::Indices(
            x.floats
                .iter()
                .map(|v| m::quantize_unorm(*v, 12).unwrap() as u32)
                .collect(),
        ),
        "quantize-snorm" => Output::Indices(
            x.floats
                .iter()
                .map(|v| m::quantize_snorm(*v, 12).unwrap() as u32)
                .collect(),
        ),
        _ => panic!("unknown operation {op}"),
    }
}
#[cfg(feature = "theirs")]
fn run(op: &str, x: &Input, _: &mut ()) -> Output {
    let n = x.p.len();
    let p = x.p.as_ptr().cast::<f32>();
    let i = x.i.as_ptr();
    let ni = x.i.len();
    // Inputs are frozen, initialized, bounded and index-valid before FFI. No hostile-input FFI calls.
    unsafe {
        match op {
            "vertex-decode" => {
                let mut b = vec![0; n * 12];
                assert_eq!(
                    f::meshopt_decodeVertexBuffer(
                        b.as_mut_ptr().cast(),
                        n,
                        12,
                        x.vb.as_ptr(),
                        x.vb.len()
                    ),
                    0
                );
                Output::Bytes(b)
            }
            "vertex-encode" | "vertex-encode-v0" | "vertex-encode-default" => {
                let mut b = vec![0; f::meshopt_encodeVertexBufferBound(n, 12)];
                let used = if op == "vertex-encode-default" {
                    f::meshopt_encodeVertexBuffer(b.as_mut_ptr(), b.len(), p.cast(), n, 12)
                } else {
                    f::meshopt_encodeVertexBufferLevel(
                        b.as_mut_ptr(),
                        b.len(),
                        p.cast(),
                        n,
                        12,
                        2,
                        if op.ends_with("v0") { 0 } else { 1 },
                    )
                };
                assert!(used > 0);
                b.truncate(used);
                Output::Bytes(b)
            }
            "index-decode" | "sequence-decode" => {
                let src = if op == "index-decode" { &x.ib } else { &x.seq };
                let mut b = vec![0; ni * 4];
                let r = if op == "index-decode" {
                    f::meshopt_decodeIndexBuffer(
                        b.as_mut_ptr().cast(),
                        ni,
                        4,
                        src.as_ptr(),
                        src.len(),
                    )
                } else {
                    f::meshopt_decodeIndexSequence(
                        b.as_mut_ptr().cast(),
                        ni,
                        4,
                        src.as_ptr(),
                        src.len(),
                    )
                };
                assert_eq!(r, 0);
                Output::Bytes(b)
            }
            "index-encode" | "sequence-encode" => {
                let mut b = vec![
                    0;
                    if op == "index-encode" {
                        f::meshopt_encodeIndexBufferBound(ni, n)
                    } else {
                        f::meshopt_encodeIndexSequenceBound(ni, n)
                    }
                ];
                let used = if op == "index-encode" {
                    f::meshopt_encodeIndexBuffer(b.as_mut_ptr(), b.len(), i, ni)
                } else {
                    f::meshopt_encodeIndexSequence(b.as_mut_ptr(), b.len(), i, ni)
                };
                assert!(used > 0);
                b.truncate(used);
                Output::Bytes(b)
            }
            "oct-decode" | "quat-decode" | "exp-decode" | "color-decode" => {
                let (k, stride) = match op {
                    "oct-decode" => (0, 8),
                    "quat-decode" => (1, 8),
                    "exp-decode" => (2, 16),
                    _ => (3, 8),
                };
                let mut b = x.filters[k].clone();
                match k {
                    0 => f::meshopt_decodeFilterOct(b.as_mut_ptr().cast(), n, stride),
                    1 => f::meshopt_decodeFilterQuat(b.as_mut_ptr().cast(), n, stride),
                    2 => f::meshopt_decodeFilterExp(b.as_mut_ptr().cast(), n, stride),
                    _ => f::meshopt_decodeFilterColor(b.as_mut_ptr().cast(), n, stride),
                };
                Output::Bytes(b)
            }
            "oct-encode" | "quat-encode" | "exp-encode" | "color-encode" => {
                let mut b = vec![0; n * if op == "exp-encode" { 16 } else { 8 }];
                match op {
                    "oct-encode" => f::meshopt_encodeFilterOct(
                        b.as_mut_ptr().cast(),
                        n,
                        8,
                        12,
                        x.floats.as_ptr(),
                    ),
                    "quat-encode" => f::meshopt_encodeFilterQuat(
                        b.as_mut_ptr().cast(),
                        n,
                        8,
                        12,
                        x.quat.as_ptr(),
                    ),
                    "exp-encode" => f::meshopt_encodeFilterExp(
                        b.as_mut_ptr().cast(),
                        n,
                        16,
                        20,
                        x.floats.as_ptr(),
                        f::meshopt_EncodeExpMode_meshopt_EncodeExpSeparate,
                    ),
                    _ => {
                        let a: Vec<f32> = x.floats.iter().map(|v| v.abs()).collect();
                        f::meshopt_encodeFilterColor(b.as_mut_ptr().cast(), n, 8, 12, a.as_ptr())
                    }
                };
                Output::Bytes(b)
            }
            "cache" | "cache-strip" | "cache-fifo" | "overdraw" => {
                let mut b = vec![0; ni];
                match op {
                    "cache" => f::meshopt_optimizeVertexCache(b.as_mut_ptr(), i, ni, n),
                    "cache-strip" => f::meshopt_optimizeVertexCacheStrip(b.as_mut_ptr(), i, ni, n),
                    "cache-fifo" => {
                        f::meshopt_optimizeVertexCacheFifo(b.as_mut_ptr(), i, ni, n, 16)
                    }
                    _ => f::meshopt_optimizeOverdraw(b.as_mut_ptr(), i, ni, p, n, 12, 1.05),
                };
                Output::Indices(b)
            }
            "fetch-remap" => {
                let mut b = vec![0; n];
                f::meshopt_optimizeVertexFetchRemap(b.as_mut_ptr(), i, ni, n);
                Output::Indices(b)
            }
            "fetch" => {
                let mut ib = x.i.clone();
                let mut b = vec![0; n * 12];
                let used = f::meshopt_optimizeVertexFetch(
                    b.as_mut_ptr().cast(),
                    ib.as_mut_ptr(),
                    ni,
                    p.cast(),
                    n,
                    12,
                );
                b.truncate(used * 12);
                Output::Fetch(ib, b)
            }
            "simplify" => {
                let mut b = vec![0; ni];
                let mut error = 0.;
                let used = f::meshopt_simplify(
                    b.as_mut_ptr(),
                    i,
                    ni,
                    p,
                    n,
                    12,
                    ni / 2 / 3 * 3,
                    0.5,
                    0,
                    &mut error,
                );
                b.truncate(used);
                Output::Simplify(b, error)
            }
            "simplify-scale" => {
                Output::Bytes(f::meshopt_simplifyScale(p, n, 12).to_le_bytes().to_vec())
            }
            "stripify" => {
                let mut b = vec![0; f::meshopt_stripifyBound(ni)];
                let used = f::meshopt_stripify(b.as_mut_ptr(), i, ni, n, u32::MAX);
                b.truncate(used);
                Output::Indices(b)
            }
            "unstripify" => {
                let strip = decode_words(&x.seq);
                let mut b = vec![0; f::meshopt_unstripifyBound(strip.len())];
                let used =
                    f::meshopt_unstripify(b.as_mut_ptr(), strip.as_ptr(), strip.len(), u32::MAX);
                b.truncate(used);
                Output::Indices(b)
            }
            "meshlets" | "meshlets-scan" | "meshlets-flex" | "meshlets-spatial" => {
                let bound = f::meshopt_buildMeshletsBound(
                    ni,
                    64,
                    if op == "meshlets-flex" || op == "meshlets-spatial" {
                        64
                    } else {
                        124
                    },
                );
                let mut ms = vec![std::mem::zeroed::<f::meshopt_Meshlet>(); bound];
                let mut v = vec![0; bound * 64];
                let mut t = vec![0; bound * 124 * 3];
                let used = match op {
                    "meshlets" => f::meshopt_buildMeshlets(
                        ms.as_mut_ptr(),
                        v.as_mut_ptr(),
                        t.as_mut_ptr(),
                        i,
                        ni,
                        p,
                        n,
                        12,
                        64,
                        124,
                        0.,
                    ),
                    "meshlets-scan" => f::meshopt_buildMeshletsScan(
                        ms.as_mut_ptr(),
                        v.as_mut_ptr(),
                        t.as_mut_ptr(),
                        i,
                        ni,
                        n,
                        64,
                        124,
                    ),
                    "meshlets-flex" => f::meshopt_buildMeshletsFlex(
                        ms.as_mut_ptr(),
                        v.as_mut_ptr(),
                        t.as_mut_ptr(),
                        i,
                        ni,
                        p,
                        n,
                        12,
                        64,
                        64,
                        124,
                        0.,
                        0.,
                    ),
                    _ => f::meshopt_buildMeshletsSpatial(
                        ms.as_mut_ptr(),
                        v.as_mut_ptr(),
                        t.as_mut_ptr(),
                        i,
                        ni,
                        p,
                        n,
                        12,
                        64,
                        64,
                        124,
                        0.5,
                    ),
                };
                ms.truncate(used);
                Output::MeshTheirs(ms, v, t)
            }

            "vertex-version" => {
                Output::Bytes(vec![
                    f::meshopt_decodeVertexVersion(x.vb.as_ptr(), x.vb.len()) as u8,
                ])
            }
            "index-version" => {
                Output::Bytes(vec![
                    f::meshopt_decodeIndexVersion(x.ib.as_ptr(), x.ib.len()) as u8,
                ])
            }
            "vertex-bound" => Output::Bytes(
                (f::meshopt_encodeVertexBufferBound(n, 12) as u64)
                    .to_le_bytes()
                    .to_vec(),
            ),
            "index-bound" => Output::Bytes(
                (f::meshopt_encodeIndexBufferBound(ni, n) as u64)
                    .to_le_bytes()
                    .to_vec(),
            ),
            "sequence-bound" => Output::Bytes(
                (f::meshopt_encodeIndexSequenceBound(ni, n) as u64)
                    .to_le_bytes()
                    .to_vec(),
            ),
            "strip-bound" => {
                Output::Bytes((f::meshopt_stripifyBound(ni) as u64).to_le_bytes().to_vec())
            }
            "unstrip-bound" => Output::Bytes(
                (f::meshopt_unstripifyBound(ni) as u64)
                    .to_le_bytes()
                    .to_vec(),
            ),
            "meshlet-bound" => Output::Bytes(
                (f::meshopt_buildMeshletsBound(ni, 64, 124) as u64)
                    .to_le_bytes()
                    .to_vec(),
            ),
            "remap" | "remap-multi" | "remap-custom" | "remap-vertices" | "remap-indices" => {
                let mut r = vec![u32::MAX; n];
                let stream = f::meshopt_Stream {
                    data: p.cast(),
                    size: 12,
                    stride: 12,
                };
                unsafe extern "C" fn yes(_: *mut std::ffi::c_void, _: u32, _: u32) -> i32 {
                    1
                }
                let count = match op {
                    "remap-multi" => {
                        f::meshopt_generateVertexRemapMulti(r.as_mut_ptr(), i, ni, n, &stream, 1)
                    }
                    "remap-custom" => f::meshopt_generateVertexRemapCustom(
                        r.as_mut_ptr(),
                        i,
                        ni,
                        p,
                        n,
                        12,
                        Some(yes),
                        std::ptr::null_mut(),
                    ),
                    _ => f::meshopt_generateVertexRemap(r.as_mut_ptr(), i, ni, p.cast(), n, 12),
                };
                match op {
                    "remap-vertices" => {
                        let mut b = vec![0; n * 12];
                        f::meshopt_remapVertexBuffer(
                            b.as_mut_ptr().cast(),
                            p.cast(),
                            n,
                            12,
                            r.as_ptr(),
                        );
                        b.truncate(count * 12);
                        Output::Bytes(b)
                    }
                    "remap-indices" => {
                        let mut b = vec![0; ni];
                        f::meshopt_remapIndexBuffer(b.as_mut_ptr(), i, ni, r.as_ptr());
                        Output::Indices(b)
                    }
                    _ => Output::Indices(r),
                }
            }
            "shadow" | "shadow-multi" => {
                let mut b = vec![0; ni];
                let stream = f::meshopt_Stream {
                    data: p.cast(),
                    size: 12,
                    stride: 12,
                };
                if op == "shadow" {
                    f::meshopt_generateShadowIndexBuffer(b.as_mut_ptr(), i, ni, p.cast(), n, 12, 12)
                } else {
                    f::meshopt_generateShadowIndexBufferMulti(b.as_mut_ptr(), i, ni, n, &stream, 1)
                };
                Output::Indices(b)
            }
            "position-remap" => {
                let mut b = vec![0; n];
                f::meshopt_generatePositionRemap(b.as_mut_ptr(), p, n, 12);
                Output::Indices(b)
            }
            "adjacency" | "tessellation" => {
                let mut b = vec![0; ni * if op == "adjacency" { 2 } else { 4 }];
                if op == "adjacency" {
                    f::meshopt_generateAdjacencyIndexBuffer(b.as_mut_ptr(), i, ni, p, n, 12)
                } else {
                    f::meshopt_generateTessellationIndexBuffer(b.as_mut_ptr(), i, ni, p, n, 12)
                };
                Output::Indices(b)
            }
            "provoking" => {
                let mut b = vec![0; ni];
                let mut r = vec![0; n + ni / 3];
                let count = f::meshopt_generateProvokingIndexBuffer(
                    b.as_mut_ptr(),
                    r.as_mut_ptr(),
                    i,
                    ni,
                    n,
                );
                r.truncate(count);
                let mut out = vec![count as u32];
                out.extend(b);
                out.extend(r);
                Output::Indices(out)
            }
            "simplify-attributes" | "simplify-sloppy" | "simplify-prune" | "simplify-points" => {
                let mut b = vec![0; ni.max(n)];
                let mut e = 0.;
                let count = match op {
                    "simplify-attributes" => f::meshopt_simplifyWithAttributes(
                        b.as_mut_ptr(),
                        i,
                        ni,
                        p,
                        n,
                        12,
                        x.floats.as_ptr(),
                        16,
                        [1.; 4].as_ptr(),
                        4,
                        std::ptr::null(),
                        ni / 2 / 3 * 3,
                        0.5,
                        0,
                        &mut e,
                    ),
                    "simplify-sloppy" => f::meshopt_simplifySloppy(
                        b.as_mut_ptr(),
                        i,
                        ni,
                        p,
                        n,
                        12,
                        std::ptr::null(),
                        ni / 2 / 3 * 3,
                        0.5,
                        &mut e,
                    ),
                    "simplify-prune" => {
                        f::meshopt_simplifyPrune(b.as_mut_ptr(), i, ni, p, n, 12, 0.5)
                    }
                    _ => f::meshopt_simplifyPoints(
                        b.as_mut_ptr(),
                        p,
                        n,
                        12,
                        std::ptr::null(),
                        0,
                        0.,
                        n / 2,
                    ),
                };
                b.truncate(count);
                if op == "simplify-attributes" || op == "simplify-sloppy" {
                    Output::Simplify(b, e)
                } else {
                    Output::Indices(b)
                }
            }
            "simplify-update" => {
                let mut ib = x.i.clone();
                let mut pp = x.p.clone();
                let mut e = 0.;
                let count = f::meshopt_simplifyWithUpdate(
                    ib.as_mut_ptr(),
                    ni,
                    pp.as_mut_ptr().cast(),
                    n,
                    12,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null(),
                    0,
                    std::ptr::null(),
                    ni / 2 / 3 * 3,
                    0.5,
                    0,
                    &mut e,
                );
                ib.truncate(count);
                let mut b = vec![count as u32];
                b.extend(ib);
                b.push(e.to_bits());
                b.extend(pp.iter().flatten().map(|v| v.to_bits()));
                Output::Indices(b)
            }
            "analyze-cache" => {
                let r = f::meshopt_analyzeVertexCache(i, ni, n, 16, 32, 128);
                Output::Indices(vec![
                    r.vertices_transformed,
                    r.warps_executed,
                    r.acmr.to_bits(),
                    r.atvr.to_bits(),
                ])
            }
            "analyze-fetch" => {
                let r = f::meshopt_analyzeVertexFetch(i, ni, n, 12);
                Output::Indices(vec![r.bytes_fetched, r.overfetch.to_bits()])
            }
            "analyze-overdraw" => {
                let r = f::meshopt_analyzeOverdraw(i, ni, p, n, 12);
                Output::Indices(vec![
                    r.pixels_covered,
                    r.pixels_shaded,
                    r.overdraw.to_bits(),
                ])
            }
            "analyze-coverage" => {
                let r = f::meshopt_analyzeCoverage(i, ni, p, n, 12);
                floats(&[r.coverage[0], r.coverage[1], r.coverage[2], r.extent])
            }
            "cluster-bounds" | "sphere-bounds" | "meshlet-bounds" => {
                let r = match op {
                    "cluster-bounds" => {
                        f::meshopt_computeClusterBounds(i, ni.min(124 * 3), p, n, 12)
                    }
                    "sphere-bounds" => {
                        f::meshopt_computeSphereBounds(p, n, 12, std::ptr::null(), 0)
                    }
                    _ => {
                        let v: Vec<u32> = (0..n.min(64) as u32).collect();
                        let t: Vec<u8> = vec![0, 1, 2];
                        f::meshopt_computeMeshletBounds(v.as_ptr(), t.as_ptr(), 1, p, n, 12)
                    }
                };
                bounds(
                    r.center,
                    r.radius,
                    r.cone_apex,
                    r.cone_axis,
                    r.cone_cutoff,
                    r.cone_axis_s8,
                    r.cone_cutoff_s8,
                )
            }
            "optimize-meshlet" => {
                let mut v: Vec<u32> = (0..n.min(64) as u32).collect();
                let mut t: Vec<u8> = vec![0, 1, 2];
                f::meshopt_optimizeMeshlet(v.as_mut_ptr(), t.as_mut_ptr(), 1, v.len());
                let mut b = (v.len() as u32).to_le_bytes().to_vec();
                for v in v {
                    b.extend(v.to_le_bytes())
                }
                b.extend(t);
                Output::Bytes(b)
            }
            "partition" => {
                let counts: Vec<u32> = x.i.chunks(96).map(|c| c.len() as u32).collect();
                let mut a = vec![0; counts.len()];
                let c = f::meshopt_partitionClusters(
                    a.as_mut_ptr(),
                    i,
                    ni,
                    counts.as_ptr(),
                    counts.len(),
                    p,
                    n,
                    12,
                    4,
                );
                let mut b = vec![c as u32];
                b.extend(a);
                Output::Indices(b)
            }
            "spatial-remap" | "spatial-points" => {
                let mut b = vec![0; n];
                if op == "spatial-remap" {
                    f::meshopt_spatialSortRemap(b.as_mut_ptr(), p, n, 12)
                } else {
                    f::meshopt_spatialClusterPoints(b.as_mut_ptr(), p, n, 12, 16)
                };
                Output::Indices(b)
            }
            "spatial-triangles" => {
                let mut b = vec![0; ni];
                f::meshopt_spatialSortTriangles(b.as_mut_ptr(), i, ni, p, n, 12);
                Output::Indices(b)
            }
            "quantize-half" => Output::Indices(
                x.p.iter()
                    .flatten()
                    .map(|v| f::meshopt_quantizeHalf(*v) as u32)
                    .collect(),
            ),
            "dequantize-half" => floats(
                &(0..256)
                    .map(|h| f::meshopt_dequantizeHalf(h * 251))
                    .collect::<Vec<_>>(),
            ),
            "quantize-float" => floats(
                &x.p.iter()
                    .flatten()
                    .map(|v| f::meshopt_quantizeFloat(*v, 12))
                    .collect::<Vec<_>>(),
            ),
            "quantize-unorm" => Output::Indices(
                x.floats
                    .iter()
                    .map(|v| meshopt::quantize_unorm(*v, 12) as u32)
                    .collect(),
            ),
            "quantize-snorm" => Output::Indices(
                x.floats
                    .iter()
                    .map(|v| meshopt::quantize_snorm(*v, 12) as u32)
                    .collect(),
            ),
            _ => panic!("unknown operation {op}"),
        }
    }
}
fn floats(v: &[f32]) -> Output {
    Output::Bytes(v.iter().flat_map(|v| v.to_le_bytes()).collect())
}
fn bounds(
    c: [f32; 3],
    r: f32,
    ap: [f32; 3],
    ax: [f32; 3],
    cut: f32,
    ax8: [i8; 3],
    cut8: i8,
) -> Output {
    let mut b = Vec::new();
    for v in c.into_iter().chain([r]).chain(ap).chain(ax).chain([cut]) {
        b.extend(v.to_le_bytes())
    }
    b.extend(ax8.map(|v| v as u8));
    b.push(cut8 as u8);
    Output::Bytes(b)
}
fn decode_words(b: &[u8]) -> Vec<u32> {
    b.chunks_exact(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect()
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let op = &a[1];
    #[cfg(feature = "theirs")]
    unsafe {
        f::meshopt_encodeVertexVersion(0);
        f::meshopt_encodeIndexVersion(1);
    }
    let x = Input::read(&a[2]);
    #[cfg(feature = "ours")]
    let mut w = m::Workspace::default();
    #[cfg(feature = "theirs")]
    let mut w = ();
    let out = run(op, &x, &mut w);
    std::fs::write(&a[3], out.bytes()).unwrap();
    if op.starts_with("meshlets") {
        std::fs::write(format!("{}.raw", a[3]), out.raw_bytes()).unwrap();
    }
    drop(out);
    println!("READY");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "stop" {
            break;
        }
        let iterations: usize = line.parse().unwrap();
        let t = Instant::now();
        for _ in 0..iterations {
            black_box(run(black_box(op), black_box(&x), &mut w));
        }
        println!("{}", t.elapsed().as_nanos() as f64 / iterations as f64);
        io::stdout().flush().unwrap();
    }
}
