//! Deterministic differential fuzz, v0.2.14 (`baseline`) against the
//! candidate, over the paths changed in 0.2.15: vertex-cache optimizers,
//! vertex decoding (strides biased to the peer harness's 12-byte records),
//! meshlet builders and cluster LOD, at random work limits.
//! Usage: fuzz SEED CASES. Panics on the first difference.

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
    fn below(&mut self, n: usize) -> usize {
        self.next() as usize % n.max(1)
    }
}

fn limit(rng: &mut Rng) -> u64 {
    match rng.below(4) {
        0 => u64::MAX,
        1 => rng.below(64) as u64,
        2 => rng.below(4096) as u64,
        _ => rng.below(1 << 20) as u64,
    }
}

macro_rules! cache {
    ($m:ident, $indices:expr, $v:expr, $variant:expr, $fuel:expr) => {{
        use $m as m;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let mut out = vec![7u32; $indices.len() + 1];
        let r = match $variant {
            0 => m::optimize_vertex_cache_into(&mut out, $indices, $v, &mut ws),
            1 => m::optimize_vertex_cache_strip_into(&mut out, $indices, $v, &mut ws),
            _ => m::optimize_vertex_cache_fifo_into(&mut out, $indices, $v, 16, &mut ws),
        };
        (format!("{r:?}"), out, ws.usage().work, ws.usage().bytes)
    }};
}

macro_rules! decode {
    ($m:ident, $count:expr, $stride:expr, $source:expr, $fuel:expr) => {{
        use $m as m;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let mut out = vec![0xccu8; $count * $stride + 2];
        let r = m::codec::decode_vertex_buffer_into(&mut out, $count, $stride, $source, &mut ws);
        let owned = m::codec::decode_vertex_buffer($count, $stride, $source, &mut ws);
        (format!("{r:?} {owned:?}"), out, ws.usage().work)
    }};
}

macro_rules! meshlets {
    ($m:ident, $p:expr, $indices:expr, $variant:expr, $fuel:expr) => {{
        use $m as m;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let s = m::MeshletSettings {
            max_vertices: 16,
            max_triangles: 8,
        };
        let p = m::Positions::from_packed($p);
        let r = match $variant {
            0 => m::build_meshlets($indices, p, s, 0., &mut ws),
            1 => m::build_meshlets($indices, p, s, 0.5, &mut ws),
            2 => m::build_meshlets_flex($indices, p, s, 3, 0., 2., &mut ws),
            _ => m::build_meshlets_flex($indices, p, s, 2, 0.75, 0., &mut ws),
        };
        (format!("{r:?}"), ws.usage().work, ws.usage().bytes)
    }};
}

macro_rules! lod {
    ($m:ident, $p:expr, $indices:expr, $variant:expr, $fuel:expr) => {{
        use $m as m;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let mut p = $p.to_vec();
        let width = [0, 1, 3, 32][$variant % 4];
        let attributes: Vec<f32> = (0..p.len() * width)
            .map(|j| (j % 11) as f32 / 10.)
            .collect();
        let weights = vec![0.5; width];
        let locks: Vec<_> = (0..p.len())
            .map(|j| m::VertexFlags::from_bits(if j % 5 == 0 { 1 } else { 0 }).unwrap())
            .collect();
        let mut config = if $variant & 4 == 0 {
            m::clusterlod::default_config(4)
        } else {
            m::clusterlod::default_config_rt(4)
        }
        .unwrap();
        config.simplify_dilate_borders = $variant & 8 != 0;
        let mut records = Vec::<u32>::new();
        let r = m::clusterlod::build_with_output(
            config,
            m::clusterlod::Mesh {
                indices: $indices,
                positions: &mut p,
                attributes: if width == 0 {
                    None
                } else {
                    Some(
                        m::Attributes::from_interleaved(&attributes, $p.len(), width, width, 0)
                            .unwrap(),
                    )
                },
                vertex_lock: if $variant & 16 == 0 {
                    None
                } else {
                    Some(&locks)
                },
                attribute_weights: &weights,
                attribute_protect_mask: if width == 0 { 0 } else { 1 },
            },
            |g, clusters| {
                records.extend([
                    g.depth as u32,
                    clusters.len() as u32,
                    g.simplified.error.to_bits(),
                ]);
                for c in clusters {
                    records.extend(c.bounds.center.map(f32::to_bits));
                    records.extend(&c.indices);
                }
                Ok(records.len() as i32)
            },
            &mut ws,
        );
        (format!("{r:?}"), records, ws.usage().work)
    }};
}

fn mesh(rng: &mut Rng) -> (Vec<[f32; 3]>, Vec<u32>) {
    // The old small-grid generator could not reach the >=256-face list.
    let large = rng.below(4) == 0;
    let w = if large {
        16 + rng.below(6) as u32
    } else {
        2 + rng.below(9) as u32
    };
    let h = if large {
        12 + rng.below(4) as u32
    } else {
        1 + rng.below(6) as u32
    };
    let scale = [1e-19f32, 1e-10, 1e-3, 1., 37., 1e6, 1e8][rng.below(7)];
    let mut p = Vec::new();
    for y in 0..=h {
        for x in 0..=w {
            let jitter = (rng.next() % 100) as f32 / 400.;
            p.push([x as f32 * scale, y as f32 * scale, jitter * scale]);
        }
    }
    let mut indices = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let a = y * (w + 1) + x;
            let (b, c) = (a + 1, a + w + 1);
            if rng.below(8) != 0 {
                indices.extend([a, b, c]);
            }
            if rng.below(8) != 0 {
                indices.extend([c, b, c + 1]);
            }
        }
    }
    // Occasional duplicates, degenerate faces, far or non-finite points.
    match rng.below(6) {
        0 if !indices.is_empty() => {
            let t = rng.below(indices.len() / 3) * 3;
            let face = [indices[t], indices[t + 1], indices[t + 2]];
            indices.extend(face);
        }
        1 => indices.extend([0, 0, 1]),
        2 => p[0][0] = 3.0e9,
        3 => p[0][1] = f32::NAN,
        _ => {}
    }
    (p, indices)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut rng = Rng(args[1].parse::<u64>().unwrap() | 1);
    let cases: usize = args[2].parse().unwrap();
    for case in 0..cases {
        let (p, indices) = mesh(&mut rng);
        match case % 4 {
            0 => {
                let v = p.len() + rng.below(3);
                let variant = rng.below(3);
                let fuel = limit(&mut rng);
                let a = cache!(baseline, &indices, v, variant, fuel);
                let b = cache!(candidate, &indices, v, variant, fuel);
                assert_eq!(a, b, "cache case {case} variant {variant} fuel {fuel}");
            }
            1 => {
                let stride = [4usize, 8, 12, 12, 12, 16, 20, 32, 64][rng.below(9)];
                let count = rng.below(600);
                let data: Vec<u8> = (0..count * stride)
                    .map(|i| ((i / stride) as u32 * (1 + i as u32 % 5) + rng.next() % 3) as u8)
                    .collect();
                let mut ws = candidate::Workspace::default();
                let version = rng.below(2) as u8;
                let level = if version == 0 { 2 } else { rng.below(4) as u8 };
                let mut source = candidate::codec::encode_vertex_buffer(
                    &data,
                    count,
                    stride,
                    candidate::codec::VertexEncoding::new(version, level).unwrap(),
                    &mut ws,
                )
                .unwrap();
                for _ in 0..rng.below(3) {
                    let at = rng.below(source.len());
                    source[at] ^= 1 << rng.below(8);
                }
                if rng.below(4) == 0 {
                    source.truncate(rng.below(source.len() + 1));
                }
                let count = if rng.below(5) == 0 {
                    rng.below(600)
                } else {
                    count
                };
                let fuel = limit(&mut rng);
                let a = decode!(baseline, count, stride, &source, fuel);
                let b = decode!(candidate, count, stride, &source, fuel);
                assert_eq!(a, b, "decode case {case} {count}x{stride} fuel {fuel}");
            }
            2 => {
                let variant = rng.below(4);
                let fuel = limit(&mut rng);
                let a = meshlets!(baseline, &p, &indices, variant, fuel);
                let b = meshlets!(candidate, &p, &indices, variant, fuel);
                assert_eq!(a, b, "meshlets case {case} variant {variant} fuel {fuel}");
            }
            _ => {
                let fuel = limit(&mut rng);
                let variant = rng.below(32);
                let a = lod!(baseline, &p, &indices, variant, fuel);
                let b = lod!(candidate, &p, &indices, variant, fuel);
                assert_eq!(a, b, "cluster LOD case {case} fuel {fuel}");
            }
        }
    }
}
