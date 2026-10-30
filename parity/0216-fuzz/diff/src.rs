//! Coverage-guided targets for the 0.2.16 pre-release fuzz (copied from the
//! 0.2.15 qualification harness; the `bounds` target is new).
//!
//! Changed families are differential: every call runs on the train
//! (`candidate`) and on v0.2.15 (`baseline`), cold and then warm on the same
//! workspace, and the Debug form of each Result, every caller buffer and the
//! workspace Usage must be identical. New APIs (sanitize, analyze_mesh,
//! cache score tables, vertex layouts, the LOD chain) check their documented
//! invariants, determinism across cold/warm workspaces, and that a run under
//! a work limit either fails or equals the unlimited run.
//!
//! Input: 16 parameter bytes, then u16 vertex count (<= 384), u16 index count
//! (<= 1152), 12 bytes per position, 2 bytes per index (taken modulo
//! vertex count + 2, so some are out of range), then free bytes for
//! attributes, weights, flags and tables. Missing bytes read as zero.
//! Position mode (param 0 % 4): raw bits; small grid; small grid scaled by
//! 2^(param 1 % 128), which reaches finite coordinates near f32::MAX; raw bits
//! with non-finite values replaced by +-f32::MAX.

pub struct Input<'a> {
    pub q: [u8; 16],
    pub p: Vec<[f32; 3]>,
    pub ix: Vec<u32>,
    pub extra: &'a [u8],
}

fn byte(data: &[u8], i: usize) -> u8 {
    data.get(i).copied().unwrap_or(0)
}

pub fn decode(data: &[u8]) -> Input<'_> {
    let mut q = [0u8; 16];
    for (i, v) in q.iter_mut().enumerate() {
        *v = byte(data, i);
    }
    let rest = data.get(16..).unwrap_or(&[]);
    let n = (usize::from(u16::from_le_bytes([byte(rest, 0), byte(rest, 1)]))) % 385;
    let ni = (usize::from(u16::from_le_bytes([byte(rest, 2), byte(rest, 3)]))) % 1153;
    let pos = 4;
    let word = |i: usize| {
        u32::from_le_bytes([
            byte(rest, pos + i * 4),
            byte(rest, pos + i * 4 + 1),
            byte(rest, pos + i * 4 + 2),
            byte(rest, pos + i * 4 + 3),
        ])
    };
    let scale = f32::from_bits(((u32::from(q[1]) % 128) + 127) << 23);
    let p = (0..n)
        .map(|v| {
            core::array::from_fn(|k| {
                let w = word(v * 3 + k);
                match q[0] % 4 {
                    0 => f32::from_bits(w),
                    1 => (w as u8 as i8) as f32 / 32.0,
                    2 => (w as u8 as i8) as f32 / 32.0 * scale,
                    _ => {
                        let x = f32::from_bits(w);
                        if x.is_finite() {
                            x
                        } else {
                            f32::MAX.copysign(x)
                        }
                    }
                }
            })
        })
        .collect();
    let ipos = pos + n * 12;
    let ix = (0..ni)
        .map(|i| {
            u32::from(u16::from_le_bytes([byte(rest, ipos + i * 2), byte(rest, ipos + i * 2 + 1)]))
                % (n as u32 + 2)
        })
        .collect();
    let extra = rest.get(ipos + ni * 2..).unwrap_or(&[]);
    Input { q, p, ix, extra }
}

impl Input<'_> {
    pub fn n(&self) -> usize {
        self.p.len()
    }
    /// Work limit: unlimited, or a small limit that lands inside the call.
    pub fn fuel(&self) -> u64 {
        if self.q[2] & 1 == 0 {
            u64::MAX
        } else {
            u64::from(u16::from_le_bytes([self.q[3], self.q[4]]))
        }
    }
    pub fn bytes(&self) -> usize {
        if self.q[2] & 2 == 0 {
            1 << 26
        } else {
            usize::from(self.q[5]) * 256
        }
    }
    pub fn lean(&self) -> bool {
        self.q[2] & 4 != 0
    }
    pub fn e(&self, i: usize) -> u8 {
        byte(self.extra, i)
    }
    /// A float from the free bytes: small values, zero, or extremes.
    pub fn float(&self, i: usize) -> f32 {
        let b = self.e(i);
        match b {
            255 => f32::MAX / 8.0,
            254 => f32::INFINITY,
            253 => f32::NAN,
            252 => -1.0,
            251 => f32::MAX,
            250 => f32::from_bits(1),
            _ => b as f32 / 32.0,
        }
    }
    pub fn error(&self, b: u8) -> f32 {
        [0.0, 1e-4, 0.01, 0.1, 0.5, 1.0, 4.0, f32::MAX, f32::INFINITY, f32::NAN, -1.0][usize::from(b) % 11]
    }
    pub fn packed_floats(&self) -> Vec<f32> {
        self.p.iter().flat_map(|v| [v[0], v[1], v[2], -7.0]).collect()
    }
    pub fn le_bytes(&self, stride: usize) -> Vec<u8> {
        let mut b = vec![0x5au8; self.n() * stride];
        for (v, row) in self.p.iter().zip(b.chunks_exact_mut(stride.max(1))) {
            for (k, x) in v.iter().enumerate() {
                row[k * 4..k * 4 + 4].copy_from_slice(&x.to_le_bytes());
            }
        }
        b
    }
}

/// Positions for layout `l`: packed, strided floats, byte records.
#[macro_export]
macro_rules! positions {
    ($m:ident, $x:expr, $l:expr, $floats:expr, $bytes:expr) => {
        match $l % 3 {
            0 => Ok($m::Positions::from_packed(&$x.p)),
            1 => $m::Positions::from_interleaved($floats, $x.n(), 4, 0),
            _ => $m::Positions::from_bytes($bytes, $x.n(), 16, 0, $m::ByteOrder::LittleEndian),
        }
    };
}

macro_rules! workspace {
    ($m:ident, $x:expr, $fuel:expr) => {{
        let mut ws = $m::Workspace::new($m::Limits {
            max_work: $fuel,
            max_bytes: $x.bytes(),
        });
        if $x.lean() {
            ws.set_retention($m::Retention::Lean);
        }
        ws
    }};
}

/// Run `$body` (which may use `m`, `x`, `ws`) cold then warm on one
/// workspace for both crates and require identical observations.
macro_rules! differential {
    ($x:expr, |$m:ident, $ws:ident| $body:expr) => {{
        let x = $x;
        let fuel = x.fuel();
        let a = {
            use baseline as $m;
            let mut $ws = workspace!($m, x, fuel);
            let cold: String = (|| -> String { $body })();
            let cold = (cold, $ws.usage().work, $ws.usage().bytes);
            let warm: String = (|| -> String { $body })();
            (cold, (warm, $ws.usage().work, $ws.usage().bytes))
        };
        let b = {
            use candidate as $m;
            let mut $ws = workspace!($m, x, fuel);
            let cold: String = (|| -> String { $body })();
            let cold = (cold, $ws.usage().work, $ws.usage().bytes);
            let warm: String = (|| -> String { $body })();
            (cold, (warm, $ws.usage().work, $ws.usage().bytes))
        };
        assert_eq!(a, b, "baseline/candidate differ");
    }};
}

fn flags_of<F>(x: &Input, from_bits: impl Fn(u8) -> F) -> Vec<F> {
    (0..x.n()).map(|i| from_bits(x.e(200 + i) % 8)).collect()
}

pub fn simplify(data: &[u8]) {
    let x = &decode(data);
    let family = x.q[7] % 4;
    let ac = usize::from(x.q[8] % 10);
    let options = u32::from(u16::from_le_bytes([x.q[9], x.q[10]])) & 511;
    let target = usize::from(u16::from_le_bytes([x.q[11], x.q[12]])) % (x.ix.len() + 1);
    let error = x.error(x.q[13]);
    let use_flags = x.q[14] & 1 != 0;
    let attrs: Vec<f32> = (0..x.n() * ac).map(|i| x.float(i % 190) + i as f32 * 0.03125).collect();
    let weights: Vec<f32> = (0..ac).map(|k| if x.q[15] & 1 == 0 { 1.0 } else { x.float(190 + k) }).collect();
    let floats = x.packed_floats();
    let bytes = x.le_bytes(16);
    differential!(x, |m, ws| {
        let flags: Vec<m::VertexFlags> = flags_of(x, |b| m::VertexFlags::from_bits(b).unwrap());
        let flags = if use_flags { Some(flags.as_slice()) } else { None };
        let Ok(opts) = m::SimplifyOptions::from_bits(options) else { return String::from("options") };
        let settings = m::SimplifySettings { target_index_count: target, target_error: error, options: opts };
        let pos = positions!(m, x, x.q[6], &floats, &bytes);
        let at = m::Attributes::from_interleaved(&attrs, x.n(), ac, ac, 0);
        match (pos, at) {
            (Ok(pos), Ok(at)) => match family {
                0 => format!("{:?}", m::simplify(&x.ix, pos, settings, &mut ws)),
                1 => format!("{:?}", m::simplify_with_attributes(&x.ix, pos, at, &weights, flags, settings, &mut ws)),
                2 => {
                    let mut out = vec![0xdeadbeef; x.ix.len() / 2 + 1];
                    let r = m::simplify_with_attributes_into(&mut out, &x.ix, pos, at, &weights, flags, settings, &mut ws);
                    format!("{r:?} {out:?}")
                }
                _ => {
                    let mut out = x.ix.clone();
                    let mut p = x.p.clone();
                    let mut a = attrs.clone();
                    let mut av = m::AttributesMut::from_interleaved(&mut a, x.n(), ac, ac, 0).unwrap();
                    let r = m::simplify_with_update(
                        &mut out,
                        &mut m::PositionsMut::from_packed(&mut p),
                        if ac == 0 { None } else { Some(&mut av) },
                        &weights,
                        flags,
                        settings,
                        &mut ws,
                    );
                    format!("{r:?} {out:?} {:?} {a:?}", p.iter().flatten().map(|v| v.to_bits()).collect::<Vec<_>>())
                }
            },
            (p, a) => format!("{:?} {:?}", p.err(), a.err()),
        }
    });
}

pub fn simplify_extra(data: &[u8]) {
    let x = &decode(data);
    let target = usize::from(u16::from_le_bytes([x.q[11], x.q[12]])) % (x.ix.len() + 1);
    let error = x.error(x.q[13]);
    let floats = x.packed_floats();
    let bytes = x.le_bytes(16);
    differential!(x, |m, ws| {
        let flags: Vec<m::VertexFlags> = flags_of(x, |b| m::VertexFlags::from_bits(b).unwrap());
        let flags = if x.q[14] & 1 != 0 { Some(flags.as_slice()) } else { None };
        let pos = match positions!(m, x, x.q[6], &floats, &bytes) {
            Ok(p) => p,
            Err(e) => return format!("{e:?}"),
        };
        match x.q[7] % 6 {
            0 => format!("{:?}", m::simplify_sloppy(&x.ix, pos, flags, target, error, &mut ws)),
            1 => {
                let mut out = vec![0xdeadbeef; target + 2];
                let r = m::simplify_sloppy_into(&mut out, &x.ix, pos, flags, target, error, &mut ws);
                format!("{r:?} {out:?}")
            }
            2 => format!("{:?}", m::simplify_prune(&x.ix, pos, error, &mut ws)),
            3 => {
                let mut out = vec![0xdeadbeef; x.ix.len() / 2];
                let r = m::simplify_prune_into(&mut out, &x.ix, pos, error, &mut ws);
                format!("{r:?} {out:?}")
            }
            4 => format!(
                "{:?}",
                m::simplify_points(pos, if x.q[15] & 1 != 0 { Some(pos) } else { None }, x.float(0), target.min(x.n()), &mut ws)
            ),
            _ => {
                let mut out = vec![0xdeadbeef; target.min(x.n()) + 1];
                let r = m::simplify_points_into(&mut out, pos, None, x.float(0), target.min(x.n()), &mut ws);
                format!("{r:?} {out:?}")
            }
        }
    });
}

pub fn overdraw(data: &[u8]) {
    let x = &decode(data);
    let threshold = [1.05, 1.0, -1.0, 0.5, 2.0, f32::MAX, f32::NAN, f32::INFINITY][usize::from(x.q[13] % 8)];
    let floats = x.packed_floats();
    let bytes = x.le_bytes(16);
    differential!(x, |m, ws| {
        let pos = match positions!(m, x, x.q[6], &floats, &bytes) {
            Ok(p) => p,
            Err(e) => return format!("{e:?}"),
        };
        match x.q[7] % 4 {
            0 => format!("{:?}", m::optimize_overdraw(&x.ix, pos, threshold, &mut ws)),
            1 => {
                let mut out = vec![0xdeadbeef; x.ix.len() + 3];
                let r = m::optimize_overdraw_into(&mut out, &x.ix, pos, threshold, &mut ws);
                format!("{r:?} {out:?}")
            }
            2 => {
                let mut out = x.ix.clone();
                let r = m::optimize_overdraw_in_place(&mut out, pos, threshold, &mut ws);
                format!("{r:?} {out:?}")
            }
            _ => {
                let mut out = vec![0xdeadbeef; x.ix.len().saturating_sub(1)];
                let r = m::optimize_overdraw_into(&mut out, &x.ix, pos, threshold, &mut ws);
                format!("{r:?} {out:?}")
            }
        }
    });
}

pub fn meshlets(data: &[u8]) {
    let x = &decode(data);
    let floats = x.packed_floats();
    let bytes = x.le_bytes(16);
    let max_vertices = 3 + usize::from(x.q[8] % 254);
    let max_triangles = 1 + usize::from(x.q[9] % 255);
    let min_triangles = 1 + usize::from(x.q[10]) % max_triangles;
    let cone = x.float(0);
    let split = x.float(1);
    differential!(x, |m, ws| {
        let pos = match positions!(m, x, x.q[6], &floats, &bytes) {
            Ok(p) => p,
            Err(e) => return format!("{e:?}"),
        };
        let s = m::MeshletSettings { max_vertices, max_triangles };
        match x.q[7] % 3 {
            0 => format!("{:?}", m::build_meshlets(&x.ix, pos, s, cone, &mut ws)),
            1 => format!("{:?}", m::build_meshlets_flex(&x.ix, pos, s, min_triangles, cone, split, &mut ws)),
            _ => format!("{:?}", m::build_meshlets_spatial(&x.ix, pos, s, min_triangles, split, &mut ws)),
        }
    });
}

pub fn clod(data: &[u8]) {
    let x = &decode(data);
    let variant = usize::from(x.q[7]);
    let width = [0, 1, 3, 32][variant % 4];
    let attributes: Vec<f32> = (0..x.n() * width).map(|j| x.float(j % 128)).collect();
    let weights: Vec<f32> = (0..width).map(|k| x.float(128 + k).abs()).collect();
    let max_triangles = 4 + usize::from(x.q[8] % 64);
    differential!(x, |m, ws| {
        let locks: Vec<m::VertexFlags> = flags_of(x, |b| m::VertexFlags::from_bits(b & 1).unwrap());
        let config = if variant & 4 == 0 {
            m::clusterlod::default_config(max_triangles)
        } else {
            m::clusterlod::default_config_rt(max_triangles)
        };
        let Ok(mut config) = config else { return String::from("config") };
        config.simplify_dilate_borders = variant & 8 != 0;
        let mut p = x.p.clone();
        let mut records = Vec::<u32>::new();
        let attrs = if width == 0 {
            None
        } else {
            match m::Attributes::from_interleaved(&attributes, x.n(), width, width, 0) {
                Ok(a) => Some(a),
                Err(e) => return format!("{e:?}"),
            }
        };
        let r = m::clusterlod::build_with_output(
            config,
            m::clusterlod::Mesh {
                indices: &x.ix,
                positions: &mut p,
                attributes: attrs,
                vertex_lock: if variant & 16 == 0 { None } else { Some(&locks) },
                attribute_weights: &weights,
                attribute_protect_mask: if width == 0 { 0 } else { u32::from(x.q[9]) },
            },
            |g, clusters| {
                records.extend([g.depth as u32, clusters.len() as u32, g.simplified.error.to_bits()]);
                for c in clusters {
                    records.extend(c.bounds.center.map(f32::to_bits));
                    records.extend(&c.indices);
                }
                Ok(records.len() as i32)
            },
            &mut ws,
        );
        format!("{r:?} {records:?}")
    });
}

/// Vertex cache: differential for the changed entry points, plus the new
/// caller-defined score tables (determinism, triangle multiset, and the
/// default table equal to optimize_vertex_cache).
pub fn cache(data: &[u8]) {
    let x = &decode(data);
    let n = x.n();
    let fifo = 3 + u32::from(x.q[8] % 62);
    differential!(x, |m, ws| match x.q[7] % 5 {
        0 => format!("{:?}", m::optimize_vertex_cache(&x.ix, n, &mut ws)),
        1 => {
            let mut out = vec![7u32; x.ix.len() + 1];
            let r = m::optimize_vertex_cache_into(&mut out, &x.ix, n, &mut ws);
            format!("{r:?} {out:?}")
        }
        2 => {
            let mut out = x.ix.clone();
            let r = m::optimize_vertex_cache_in_place(&mut out, n, &mut ws);
            format!("{r:?} {out:?}")
        }
        3 => format!("{:?}", m::optimize_vertex_cache_strip(&x.ix, n, &mut ws)),
        _ => format!("{:?}", m::optimize_vertex_cache_fifo(&x.ix, n, fifo, &mut ws)),
    });
    use candidate as m;
    let cache_scores: Vec<f32> = (0..17).map(|i| x.float(i)).collect();
    let live_scores: Vec<f32> = (0..9).map(|i| x.float(17 + i)).collect();
    let fuel = x.fuel();
    let run = |table: &m::VertexScoreTable, fuel: u64| {
        let mut ws = workspace!(m, x, fuel);
        let a = m::optimize_vertex_cache_with_table(&x.ix, n, table, &mut ws);
        let b = m::optimize_vertex_cache_with_table(&x.ix, n, table, &mut ws);
        assert_eq!(a, b, "warm workspace changed the table result");
        let mut out = vec![9u32; x.ix.len()];
        let c = m::optimize_vertex_cache_with_table_into(&mut out, &x.ix, n, table, &mut ws);
        if let Ok(v) = &a {
            assert_eq!(c, Ok(()));
            assert_eq!(&out, v);
        }
        a
    };
    let default = run(&m::VertexScoreTable::default(), fuel);
    let mut ws = workspace!(m, x, fuel);
    assert_eq!(default, m::optimize_vertex_cache(&x.ix, n, &mut ws), "default table differs");
    if let Ok(table) = m::VertexScoreTable::new(&cache_scores, &live_scores) {
        let limited = run(&table, fuel);
        let full = run(&table, u64::MAX);
        if let Ok(v) = &limited {
            assert_eq!(Ok(v), full.as_ref(), "limited success differs");
        }
        if let Ok(v) = &full {
            let mut a: Vec<[u32; 3]> = x.ix.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
            let mut b: Vec<[u32; 3]> = v.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
            a.iter_mut().chain(b.iter_mut()).for_each(|t| {
                let k = (0..3).min_by_key(|&k| (t[k], k)).unwrap_or(0);
                t.rotate_left(k);
            });
            a.sort_unstable();
            b.sort_unstable();
            assert_eq!(a, b, "table result is not a triangle permutation");
        }
    }
}

pub fn remap(data: &[u8]) {
    let x = &decode(data);
    let n = x.n();
    let remap: Vec<u32> = (0..n)
        .map(|i| match x.e(i) {
            255 => u32::MAX,
            254 => n as u32 + 3,
            b => u32::from(b) % (n as u32).max(1),
        })
        .collect();
    // Record size: the packed kernels are the tightly packed 4, 8, 12 and 16
    // byte records (stride == size), everything else takes the copying path.
    let size = [12, 4, 8, 16, 20, 24, 7, 1][usize::from(x.q[8] / 3 % 8)];
    let stride = [size, size + 4, size.max(12)][usize::from(x.q[8] % 3)];
    let rows = x.le_bytes(12);
    let vb: Vec<u8> = (0..n * stride)
        .map(|i| rows[i / stride * 12 + i % stride % 12] ^ (i % stride / 12) as u8 * 0x55)
        .collect();
    differential!(x, |m, ws| {
        let stream = match m::VertexStream::new(&vb, n, size, stride, 0) {
            Ok(s) => s,
            Err(e) => return format!("{e:?}"),
        };
        match x.q[7] % 5 {
            0 => format!("{:?}", m::remap_vertex_buffer(stream, &remap, &mut ws)),
            1 => {
                let mut out = vec![0xa5u8; n * size + 5];
                let r = m::remap_vertex_buffer_into(&mut out, stream, &remap, &mut ws);
                format!("{r:?} {out:?}")
            }
            2 => format!("{:?}", m::remap_index_buffer(if x.q[9] & 1 == 0 { Some(&x.ix) } else { None }, &remap, &mut ws)),
            3 => {
                let mut out = vec![0xdeadbeef; x.ix.len() + 2];
                let r = m::remap_index_buffer_into(&mut out, Some(&x.ix), &remap, &mut ws);
                format!("{r:?} {out:?}")
            }
            _ => format!("{:?}", m::generate_vertex_remap(Some(&x.ix), stream, &mut ws)),
        }
    });
}

/// Cluster, meshlet and sphere bounds (the 0.2.16 SIMD root kernel), with and
/// without the cached positions handle. Clusters take up to 1,152 indices and
/// meshlets up to 255 local triangles, so both the small and large paths run.
pub fn bounds(data: &[u8]) {
    let x = &decode(data);
    let floats = x.packed_floats();
    let bytes = x.le_bytes(16);
    let n = x.n();
    let cluster: Vec<u32> = x.ix.iter().copied().take(usize::from(x.q[8]) * 5).collect();
    let vertex_count = usize::from(x.q[9]);
    let vertices: Vec<u32> = (0..vertex_count).map(|i| x.ix.get(i).copied().unwrap_or(i as u32)).collect();
    let triangles: Vec<u8> = (0..usize::from(x.q[10]) * 3)
        .map(|i| {
            let b = x.e(i);
            if vertex_count != 0 && x.q[11] & 1 == 0 {
                (usize::from(b) % vertex_count) as u8
            } else {
                b
            }
        })
        .collect();
    let radii: Vec<f32> = (0..n).map(|i| x.float(i + 300)).collect();
    differential!(x, |m, ws| {
        let pos = match positions!(m, x, x.q[6], &floats, &bytes) {
            Ok(p) => p,
            Err(e) => return format!("{e:?}"),
        };
        let cached = m::BoundsPositions::new(pos);
        let mut out = format!("{:?}", m::compute_cluster_bounds(&cluster, pos, &mut ws));
        out += &format!("{:?}", m::compute_cluster_bounds_with_positions(&cluster, cached, &mut ws));
        out += &format!("{:?}", m::compute_meshlet_bounds(&vertices, &triangles, pos, &mut ws));
        out += &format!("{:?}", m::compute_meshlet_bounds_with_positions(&vertices, &triangles, cached, &mut ws));
        let attributes = m::Attributes::from_interleaved(&radii, n, 1, 1, 0);
        let radii = if x.q[7] & 1 == 0 { attributes.ok() } else { None };
        out += &format!("{:?}", m::compute_sphere_bounds(pos, radii, &mut ws));
        out
    });
}

fn triangles(ix: &[u32]) -> Vec<[u32; 3]> {
    ix.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect()
}

pub fn sanitize(data: &[u8]) {
    use candidate as m;
    let x = &decode(data);
    let floats = x.packed_floats();
    let bytes = x.le_bytes(16);
    let Ok(pos) = positions!(m, x, x.q[6], &floats, &bytes) else { return };
    let Ok(options) = m::SanitizeOptions::from_bits(u32::from(x.q[7] % 16)) else { return };
    let fuel = x.fuel();
    let mut ws = workspace!(m, x, fuel);
    let a = m::sanitize(&x.ix, pos, options, &mut ws);
    let b = m::sanitize(&x.ix, pos, options, &mut ws);
    assert_eq!(a, b, "warm workspace changed sanitize");
    let mut out = vec![0xdeadbeef; m::sanitize_bound(x.ix.len()) + 2];
    let c = m::sanitize_into(&mut out, &x.ix, pos, options, &mut ws);
    let full = m::sanitize(&x.ix, pos, options, &mut workspace!(m, x, u64::MAX));
    match &a {
        Ok(s) => {
            assert_eq!(Ok(s), full.as_ref(), "limited success differs");
            let r = &s.report;
            assert_eq!(r.input_triangles, x.ix.len() / 3);
            assert_eq!(r.output_triangles + r.removed(), r.input_triangles);
            assert_eq!(s.indices.len(), r.index_count());
            assert!(s.indices.iter().all(|&v| (v as usize) < x.n()));
            // Output triangles are a subsequence of the input triangles.
            let input = triangles(&x.ix);
            let mut it = input.iter();
            for t in triangles(&s.indices) {
                assert!(it.any(|u| *u == t), "output triangle not in input order");
            }
            assert_eq!(c.as_ref().ok(), Some(r));
            assert_eq!(&out[..s.indices.len()], &s.indices[..]);
            assert!(out[s.indices.len()..].iter().all(|&v| v == 0xdeadbeef));
            // Sanitizing the output again removes nothing.
            let again = m::sanitize(&s.indices, pos, options, &mut workspace!(m, x, u64::MAX));
            if let Ok(again) = again {
                assert_eq!(again.report.removed(), 0, "sanitize is not idempotent");
                assert_eq!(again.indices, s.indices);
            }
        }
        Err(_) => {
            if full.is_ok() {
                assert!(fuel != u64::MAX || x.bytes() != 1 << 26, "error without a limit");
            }
        }
    }
    let too_small = x.ix.len().saturating_sub(3);
    let mut short = vec![0u32; too_small];
    let _ = m::sanitize_into(&mut short, &x.ix, pos, options, &mut ws);
}

pub fn analyze(data: &[u8]) {
    use candidate as m;
    let x = &decode(data);
    let floats = x.packed_floats();
    let bytes = x.le_bytes(16);
    let Ok(pos) = positions!(m, x, x.q[6], &floats, &bytes) else { return };
    let options = m::AnalyzeOptions::EMPTY
        .with_weld_positions(x.q[7] & 1 != 0)
        .with_list_limit(usize::from(x.q[8]));
    let fuel = x.fuel();
    let mut ws = workspace!(m, x, fuel);
    let a = m::analyze_mesh(&x.ix, pos, options, &mut ws);
    let b = m::analyze_mesh(&x.ix, pos, options, &mut ws);
    assert_eq!(format!("{a:?}"), format!("{b:?}"), "warm workspace changed analyze_mesh");
    let full = m::analyze_mesh(&x.ix, pos, options, &mut workspace!(m, x, u64::MAX));
    if let Ok(r) = &a {
        assert_eq!(format!("{a:?}"), format!("{full:?}"), "limited success differs");
        assert_eq!(r.input_triangles, x.ix.len() / 3);
    }
    // The classification counts agree with sanitize's report under the same weld choice.
    if let Ok(r) = &full {
        let so = if x.q[7] & 1 != 0 { m::SanitizeOptions::WELD_POSITIONS } else { m::SanitizeOptions::EMPTY };
        let so = m::SanitizeOptions::from_bits(so.bits() | m::SanitizeOptions::COINCIDENT_CORNERS.bits()).unwrap();
        if let Ok(s) = m::sanitize(&x.ix, pos, so, &mut workspace!(m, x, u64::MAX)) {
            let rep = s.report;
            assert_eq!(
                (r.out_of_range, r.non_finite, r.repeated_index),
                (rep.out_of_range, rep.non_finite, rep.repeated_index),
                "analysis and sanitize classify invalid triangles differently"
            );
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Wide {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    tail: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Narrow {
    uv: [f32; 2],
    position: [f32; 3],
}

pub fn vertex_layout(data: &[u8]) {
    use candidate as m;
    let x = &decode(data);
    let wide: Vec<Wide> = x
        .p
        .iter()
        .enumerate()
        .map(|(i, &p)| Wide {
            position: p,
            normal: [x.float(i), x.float(i + 1), x.float(i + 2)],
            uv: [x.float(i + 3), x.float(i + 4)],
            tail: [x.float(i + 5); 4],
        })
        .collect();
    let narrow: Vec<Narrow> = x.p.iter().map(|&p| Narrow { uv: [p[0], p[1]], position: p }).collect();
    let count = usize::from(x.q[9]) % (wide.len() + 1);
    let offset = usize::from(x.q[8] % 64);
    // A declared offset either matches the selector or is rejected.
    let r = m::VertexFieldSnapshot::<3>::from_vertices(&wide[..count], offset, |v: &Wide| &v.position);
    match &r {
        Ok(s) => {
            // An empty slice has no vertex to check the selector against.
            assert!(offset == 0 || count == 0, "mismatched offset accepted");
            let pos = s.positions().unwrap();
            let back = m::analyze_mesh(&[], pos, m::AnalyzeOptions::EMPTY, &mut m::Workspace::default());
            assert!(back.is_ok());
            assert_eq!(s.as_bytes().len(), count * core::mem::size_of::<Wide>());
            for (v, rec) in wide[..count].iter().zip(s.as_bytes().chunks_exact(core::mem::size_of::<Wide>())) {
                for k in 0..3 {
                    assert_eq!(rec[k * 4..k * 4 + 4], v.position[k].to_le_bytes());
                }
            }
        }
        Err(_) => assert!(offset != 0 || count == usize::MAX),
    }
    let s = m::VertexFieldSnapshot::<2>::from_vertices(&wide[..count], 24, |v: &Wide| &v.uv).unwrap();
    let a = s.attributes().unwrap();
    assert_eq!(a.len(), count);
    assert!(s.positions().is_err(), "two components are not positions");
    let s = m::VertexFieldSnapshot::<3>::from_vertices(&narrow[..count], 8, |v: &Narrow| &v.position).unwrap();
    for (v, rec) in narrow[..count].iter().zip(s.as_bytes().chunks_exact(20)) {
        for k in 0..3 {
            assert_eq!(rec[8 + k * 4..12 + k * 4], v.position[k].to_le_bytes());
        }
    }
    // Snapshot positions feed a real call exactly like packed positions.
    let mut w1 = m::Workspace::default();
    let mut w2 = m::Workspace::default();
    let ix: Vec<u32> = x.ix.iter().map(|&v| v % (count.max(1) as u32)).collect();
    let ix = &ix[..if count == 0 { 0 } else { ix.len() / 3 * 3 }];
    let packed: Vec<[f32; 3]> = narrow[..count].iter().map(|v| v.position).collect();
    let a = m::optimize_overdraw(ix, s.positions().unwrap(), 1.05, &mut w1);
    let b = m::optimize_overdraw(ix, m::Positions::from_packed(&packed), 1.05, &mut w2);
    assert_eq!(a, b, "snapshot positions differ from packed positions");
    // Invalid field sizes and misaligned offsets are errors, never panics.
    let _ = m::VertexFieldSnapshot::<4>::from_vertices(&wide[..count], 36, |v: &Wide| &v.tail);
    let _ = m::VertexFieldSnapshot::<3>::from_vertices(&wide[..count], 13, |v: &Wide| &v.normal);
    let _ = m::VertexFieldSnapshot::<0>::from_vertices(&wide[..count], 0, |_: &Wide| &[]);
}

pub fn lod_chain(data: &[u8]) {
    use candidate as m;
    let x = &decode(data);
    let levels = 1 + usize::from(x.q[7] % 4);
    let counts: Vec<usize> = (0..levels).map(|i| usize::from(x.e(i)) * 3 % (x.ix.len() + 3)).collect();
    // QUALIFY_LOD_BOUNDED keeps ratios in [0, 2] so a second campaign can
    // explore past the known ratio-multiplication overflow (see receipts).
    let bounded = std::env::var_os("QUALIFY_LOD_BOUNDED").is_some();
    let ratios: Vec<f32> = (0..levels)
        .map(|i| x.float(8 + i))
        .map(|r| if bounded && !(0.0..=2.0).contains(&r) { 0.5 } else { r })
        .collect();
    let errors: Vec<f32> = (0..levels).map(|i| x.float(16 + i)).collect();
    let ac = usize::from(x.q[8] % 4);
    let attrs: Vec<f32> = (0..x.n() * ac).map(|i| x.float(24 + i % 64)).collect();
    let weights: Vec<f32> = (0..ac).map(|k| x.float(90 + k)).collect();
    let flags: Vec<m::VertexFlags> = flags_of(x, |b| m::VertexFlags::from_bits(b).unwrap());
    let floats = x.packed_floats();
    let bytes = x.le_bytes(16);
    let Ok(pos) = positions!(m, x, x.q[6], &floats, &bytes) else { return };
    let targets = if x.q[9] & 1 == 0 { m::LodTargets::Counts(&counts) } else { m::LodTargets::Ratios(&ratios) };
    let errs = if x.q[9] & 2 == 0 { m::LodErrors::Uniform(x.error(x.q[13])) } else { m::LodErrors::PerLevel(&errors) };
    let mut options = m::LodChainOptions::new(targets, errs).with_independent(x.q[9] & 4 != 0);
    if let Ok(o) = m::SimplifyOptions::from_bits(u32::from(u16::from_le_bytes([x.q[10], x.q[11]])) & 511) {
        options = options.with_simplify_options(o);
    }
    if ac != 0 && x.q[9] & 8 != 0 {
        if let Ok(a) = m::Attributes::from_interleaved(&attrs, x.n(), ac, ac, 0) {
            options = options.with_attributes(m::LodChainAttributes::new(a, &weights));
        }
    }
    if x.q[9] & 16 != 0 {
        options = options.with_vertex_flags(&flags);
    }
    let fuel = x.fuel();
    let mut ws = workspace!(m, x, fuel);
    let a = m::build_lod_chain(&x.ix, pos, &options, &mut ws);
    let b = m::build_lod_chain(&x.ix, pos, &options, &mut ws);
    assert_eq!(format!("{a:?}"), format!("{b:?}"), "warm workspace changed the chain");
    let full = m::build_lod_chain(&x.ix, pos, &options, &mut workspace!(m, x, u64::MAX));
    if let Ok(chain) = &a {
        assert_eq!(format!("{a:?}"), format!("{full:?}"), "limited success differs");
        assert!(chain.levels.len() <= levels);
        for level in &chain.levels {
            assert_eq!(level.indices.len() % 3, 0);
            assert_eq!(level.triangle_count * 3, level.indices.len());
            assert!(level.indices.iter().all(|&v| (v as usize) < x.n()));
        }
    }
}
