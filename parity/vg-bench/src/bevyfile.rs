//! Reader for the reference builder's saved `.meshlet_mesh` file (version 3)
//! and a port of its level-of-detail rule. No reference-crate code is needed
//! here: the layout is parsed from the bytes.
//!
//! File: magic u64, version u64, tight AABB (6 f32), BVH depth u32, then an
//! LZ4 frame holding length-prefixed slices in the order positions (u32),
//! normals (u32), uvs (2 x f32), indices (u8), BVH nodes (400 B), meshlets
//! (32 B), cull data (48 B).
use crate::{
    camera::View,
    cut::{CutMesh, Dag, Selection},
    report::{zstd19, Json},
};
use std::io::Read;

const MAGIC: u64 = 1717551717668;
const VERSION: u64 = 3;
const CENTIMETERS_PER_METER: f32 = 100.0;

#[derive(Clone, Copy, Default)]
struct Aabb {
    center: [f32; 3],
    error: f32,
    half: [f32; 3],
    child_offset: u32,
}
#[derive(Clone, Copy, Default)]
struct Sphere {
    center: [f32; 3],
    radius: f32,
}
struct Bvh {
    aabbs: [Aabb; 8],
    lod: [Sphere; 8],
    child_counts: [u8; 8],
}
struct Meshlet {
    start_bit: u32,
    start_index: u32,
    vertex_count: u32,
    triangle_count: u32,
    bits: [u32; 3],
    quant: u32,
    min: [f32; 3],
}
struct Cull {
    aabb: Aabb,
    lod: Sphere,
}

/// A parsed reference meshlet mesh.
pub struct Bevy {
    pub file_bytes: usize,
    pub payload_bytes: usize,
    pub decompressed: Vec<u8>,
    pub bvh_depth: u32,
    positions: Vec<u32>,
    indices: Vec<u8>,
    bvh: Vec<Bvh>,
    meshlets: Vec<Meshlet>,
    cull: Vec<Cull>,
    vertex_count: u64,
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap_or_default())
}
fn f32_at(b: &[u8], at: usize) -> f32 {
    f32::from_bits(u32_at(b, at))
}
fn f3(b: &[u8], at: usize) -> [f32; 3] {
    [f32_at(b, at), f32_at(b, at + 4), f32_at(b, at + 8)]
}
fn aabb(b: &[u8], at: usize) -> Aabb {
    Aabb { center: f3(b, at), error: f32_at(b, at + 12), half: f3(b, at + 16), child_offset: u32_at(b, at + 28) }
}
fn sphere(b: &[u8], at: usize) -> Sphere {
    Sphere { center: f3(b, at), radius: f32_at(b, at + 12) }
}

fn slice(data: &[u8], at: &mut usize, size: usize) -> Result<std::ops::Range<usize>, String> {
    let len = u64::from_le_bytes(data.get(*at..*at + 8).ok_or("truncated")?.try_into().unwrap_or_default()) as usize;
    *at += 8;
    let end = *at + len.checked_mul(size).ok_or("overflow")?;
    if end > data.len() {
        return Err("truncated slice".into());
    }
    let r = *at..end;
    *at = end;
    Ok(r)
}

impl Bevy {
    pub fn parse(file: &[u8]) -> Result<Self, String> {
        if file.len() < 44 {
            return Err("too short".into());
        }
        if u64::from_le_bytes(file[0..8].try_into().unwrap_or_default()) != MAGIC {
            return Err("not a meshlet mesh".into());
        }
        if u64::from_le_bytes(file[8..16].try_into().unwrap_or_default()) != VERSION {
            return Err("unsupported version".into());
        }
        let bvh_depth = u32_at(file, 40);
        let mut data = Vec::new();
        lz4_flex::frame::FrameDecoder::new(&file[44..]).read_to_end(&mut data).map_err(|e| e.to_string())?;
        let mut at = 0;
        let pos = slice(&data, &mut at, 4)?;
        let _normals = slice(&data, &mut at, 4)?;
        let _uvs = slice(&data, &mut at, 8)?;
        let idx = slice(&data, &mut at, 1)?;
        let bvh = slice(&data, &mut at, 400)?;
        let mlt = slice(&data, &mut at, 32)?;
        let cull = slice(&data, &mut at, 48)?;
        if at != data.len() {
            return Err("trailing bytes".into());
        }
        let positions = data[pos.clone()].as_chunks::<4>().0.iter().map(|c| u32_at(c, 0)).collect();
        let indices = data[idx].to_vec();
        let bvh = data[bvh]
            .as_chunks::<400>()
            .0
            .iter()
            .map(|n| {
                let mut b = Bvh { aabbs: [Aabb::default(); 8], lod: [Sphere::default(); 8], child_counts: [0; 8] };
                for s in 0..8 {
                    b.aabbs[s] = aabb(n, s * 32);
                    b.lod[s] = sphere(n, 256 + s * 16);
                }
                b.child_counts.copy_from_slice(&n[384..392]);
                b
            })
            .collect();
        let meshlets: Vec<Meshlet> = data[mlt]
            .as_chunks::<32>()
            .0
            .iter()
            .map(|m| Meshlet {
                start_bit: u32_at(m, 0),
                start_index: u32_at(m, 8),
                vertex_count: u32::from(m[12]) + 1,
                triangle_count: u32::from(m[13]),
                bits: [u32::from(m[16]), u32::from(m[17]), u32::from(m[18])],
                quant: u32::from(m[19]),
                min: f3(m, 20),
            })
            .collect();
        let cull =
            data[cull].as_chunks::<48>().0.iter().map(|c| Cull { aabb: aabb(c, 0), lod: sphere(c, 32) }).collect();
        let vertex_count = meshlets.iter().map(|m| u64::from(m.vertex_count)).sum();
        Ok(Self {
            file_bytes: file.len(),
            payload_bytes: data.len(),
            decompressed: data,
            bvh_depth,
            positions,
            indices,
            bvh,
            meshlets,
            cull,
            vertex_count,
        })
    }

    fn px(&self, v: &View, s: Sphere, e: f32) -> f64 {
        v.error_px(s.center, s.radius, e)
    }

    /// Decoded vertex position as the render shader reconstructs it.
    fn vertex(&self, m: &Meshlet, i: u32) -> [f32; 3] {
        let per_vertex = m.bits[0] + m.bits[1] + m.bits[2];
        let mut bit = m.start_bit + i * per_vertex;
        let mut out = [0f32; 3];
        for (k, o) in out.iter_mut().enumerate() {
            let width = m.bits[k];
            let (word, off) = ((bit / 32) as usize, bit & 31);
            let mut v = u64::from(self.positions[word]) >> off;
            if off + width > 32 {
                v |= u64::from(self.positions[word + 1]) << (32 - off);
            }
            let value = (v & ((1u64 << width) - 1)) as f32;
            *o = (value + m.min[k]) / ((1u32 << m.quant) as f32 * CENTIMETERS_PER_METER);
            bit += width;
        }
        out
    }

    /// Static metrics: sizes, DAG shape, decode speed.
    pub fn shape(&self, file: &[u8], source_triangles: usize, label: &Json) -> String {
        let total: u64 = self.meshlets.iter().map(|m| u64::from(m.triangle_count)).sum();
        let leaf: u64 = self
            .meshlets
            .iter()
            .zip(&self.cull)
            .filter(|(_, c)| c.aabb.error == 0.0)
            .map(|(m, _)| u64::from(m.triangle_count))
            .sum();
        let mut tris: Vec<f64> = self.meshlets.iter().map(|m| f64::from(m.triangle_count)).collect();
        let mut verts: Vec<f64> = self.meshlets.iter().map(|m| f64::from(m.vertex_count)).collect();
        tris.sort_by(f64::total_cmp);
        verts.sort_by(f64::total_cmp);
        let p5 = |v: &[f64]| v.get(((v.len().max(1) - 1) as f64 * 0.05) as usize).copied().unwrap_or(0.0);
        let avg = |v: &[f64]| v.iter().sum::<f64>() / v.len().max(1) as f64;
        label
            .clone()
            .text("kind", "shape")
            .with("source_triangles", source_triangles)
            .with("clusters", self.meshlets.len())
            .with("levels", f64::NAN)
            .with("hierarchy_depth", self.bvh_depth)
            .with("total_triangles", total)
            .with("leaf_triangles", leaf)
            .with("total_over_leaf", total as f64 / leaf.max(1) as f64)
            .with("tris_per_cluster_mean", avg(&tris))
            .with("tris_per_cluster_p5", p5(&tris))
            .with("verts_per_cluster_mean", avg(&verts))
            .with("verts_per_cluster_p5", p5(&verts))
            .with("file_bytes", self.file_bytes)
            .with("file_zstd19_bytes", zstd19(file))
            .with("decompressed_zstd19_bytes", zstd19(&self.decompressed))
            .with("decode_mb_per_s", Self::decode_rate(file))
            .with("resident_bytes", self.payload_bytes)
            .with("bvh_nodes", self.bvh.len())
            .with("meshlet_vertices", self.vertex_count)
            .to_string()
    }

    /// Time to decode the LZ4 frame of the whole asset; returns MB/s of decompressed bytes (median of 5).
    pub fn decode_rate(file: &[u8]) -> f64 {
        let mut times = Vec::new();
        let mut size = 0usize;
        for _ in 0..5 {
            let t = std::time::Instant::now();
            let mut data = Vec::new();
            if lz4_flex::frame::FrameDecoder::new(&file[44..]).read_to_end(&mut data).is_err() {
                return f64::NAN;
            }
            size = data.len();
            times.push(t.elapsed().as_secs_f64());
        }
        times.sort_by(f64::total_cmp);
        size as f64 / times[2] / 1e6
    }

    fn walk(&self, v: &View, thr: f64, frustum: bool) -> Selection {
        let mut s = Selection { min_parent_error: f32::MAX, ..Selection::default() };
        let mut stack = vec![0usize];
        while let Some(n) = stack.pop() {
            s.nodes += 1;
            let node = &self.bvh[n];
            for slot in 0..8 {
                let a = node.aabbs[slot];
                if self.px(v, node.lod[slot], a.error) < thr {
                    continue;
                }
                if frustum && !v.aabb_visible(a.center, a.half) {
                    continue;
                }
                let count = node.child_counts[slot];
                if count == u8::MAX {
                    stack.push(a.child_offset as usize);
                    continue;
                }
                for k in 0..u32::from(count) {
                    let id = a.child_offset + k;
                    let c = &self.cull[id as usize];
                    s.tested += 1;
                    let px = self.px(v, c.lod, c.aabb.error);
                    if px < thr && (!frustum || v.aabb_visible(c.aabb.center, c.aabb.half)) {
                        s.drawn.push(id);
                        s.claimed_px = s.claimed_px.max(px);
                        s.claimed_world = s.claimed_world.max(f64::from(c.aabb.error));
                        s.min_parent_error = s.min_parent_error.min(a.error);
                    }
                }
            }
        }
        s
    }
}

impl Dag for Bevy {
    fn cut(&self, v: &View, thr: f64) -> Selection {
        self.walk(v, thr, false)
    }
    fn traverse(&self, v: &View, thr: f64, frustum: bool) -> Selection {
        self.walk(v, thr, frustum)
    }
    fn triangles(&self, drawn: &[u32]) -> u64 {
        drawn.iter().map(|&m| u64::from(self.meshlets[m as usize].triangle_count)).sum()
    }
    fn cut_mesh(&self, drawn: &[u32]) -> CutMesh {
        let (mut positions, mut indices) = (Vec::new(), Vec::new());
        for &id in drawn {
            let m = &self.meshlets[id as usize];
            let base = positions.len() as u32;
            for i in 0..m.vertex_count {
                positions.push(self.vertex(m, i));
            }
            let a = m.start_index as usize;
            indices.extend(self.indices[a..a + 3 * m.triangle_count as usize].iter().map(|&i| base + u32::from(i)));
        }
        CutMesh { positions: Some(positions), indices }
    }
    fn is_leaf(&self, c: u32) -> bool {
        self.cull[c as usize].aabb.error == 0.0
    }
}
