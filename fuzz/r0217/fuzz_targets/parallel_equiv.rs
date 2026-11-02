#![no_main]
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::codec::{decode_vertex_buffer, encode_vertex_buffer, VertexEncoding};
use meshoptimizer_rs::parallel::{
    build_meshlets_spatial_parallel, decode_vertex_buffer_parallel, encode_vertex_buffer_parallel,
};
use meshoptimizer_rs::*;
use std::sync::OnceLock;

/// Pools of 1, 2, 4 and 8 workers; the global pool is the fifth case.
fn pools() -> &'static Vec<rayon::ThreadPool> {
    static P: OnceLock<Vec<rayon::ThreadPool>> = OnceLock::new();
    P.get_or_init(|| {
        [1, 2, 4, 8]
            .map(|n| rayon::ThreadPoolBuilder::new().num_threads(n).build().expect("pool"))
            .into()
    })
}
fn each_pool<R: Send>(f: impl Fn() -> R + Sync) -> Vec<R> {
    let mut out = vec![f()];
    for p in pools() {
        out.push(p.install(&f));
    }
    out
}
fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let mode = c.u8();
    if mode & 1 == 0 {
        vertex_codec(&mut c);
    } else {
        spatial(&mut c);
    }
});

fn vertex_codec(c: &mut Cur<'_>) {
    let stride = [4usize, 8, 12, 16, 24, 32, 48, 64][usize::from(c.u8() % 8)];
    let count = 1 + (usize::from(c.u8()) | usize::from(c.u8() & 15) << 8) % 3000;
    let enc = VertexEncoding::new(c.u8() % 2, c.u8() % 10).unwrap_or(VertexEncoding::DEFAULT);
    let step = c.u8();
    let noise = c.u8();
    let pattern: Vec<u8> = (0..stride).map(|_| c.u8()).collect();
    let flips: Vec<(usize, u8)> = (0..usize::from(c.u8() % 5)).map(|_| (usize::from(c.u8()) << 8 | usize::from(c.u8()), c.u8())).collect();
    let mut seed = u32::from(step) << 8 | u32::from(noise) | 1;
    let mut v = Vec::with_capacity(count * stride);
    for i in 0..count {
        for (k, &b) in pattern.iter().enumerate() {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let jitter = if noise != 0 && (seed >> 24) as u8 % 7 == 0 { (seed >> 16) as u8 % noise.max(1) } else { 0 };
            v.push(b.wrapping_add((i as u8).wrapping_mul(step.wrapping_add(k as u8))).wrapping_add(jitter));
        }
    }
    let mut ws = Workspace::default();
    let serial = encode_vertex_buffer(&v, count, stride, enc, &mut ws);
    let par = each_pool(|| encode_vertex_buffer_parallel(&v, count, stride, enc, &mut Workspace::default()));
    for r in &par {
        assert_eq!(*r, serial, "parallel encode differs from serial");
    }
    let Ok(mut enc_bytes) = serial else { return };
    let back = decode_vertex_buffer(count, stride, &enc_bytes, &mut ws).expect("decode own output");
    assert_eq!(back, v);
    let dec = each_pool(|| decode_vertex_buffer_parallel(count, stride, &enc_bytes, &mut Workspace::default()));
    for r in &dec {
        assert_eq!(r.as_ref().expect("parallel decode"), &v);
    }
    // Corrupted streams: parallel result and error equal the serial decoder's.
    for (at, x) in flips {
        let n = enc_bytes.len();
        enc_bytes[at % n] ^= x;
    }
    let s = decode_vertex_buffer(count, stride, &enc_bytes, &mut Workspace::default());
    for r in each_pool(|| decode_vertex_buffer_parallel(count, stride, &enc_bytes, &mut Workspace::default())) {
        assert_eq!(r, s, "parallel decode of a corrupted stream differs");
    }
}

fn spatial(c: &mut Cur<'_>) {
    let w = 2 + usize::from(c.u8()) % 63;
    let h = 2 + usize::from(c.u8()) % 63;
    let amp = c.unit() * 0.9;
    let zamp = c.unit() * 3.;
    let mut seed = u64::from(c.u8()) << 8 | u64::from(c.u8()) | 1;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f32 / (1u64 << 53) as f32
    };
    let pos: Vec<[f32; 3]> = (0..w * h)
        .map(|i| [(i % w) as f32 + amp * rnd(), (i / w) as f32 + amp * rnd(), zamp * rnd()])
        .collect();
    let mut idx = Vec::new();
    for y in 0..h - 1 {
        for x in 0..w - 1 {
            let (a, b, cc, d) = ((y * w + x) as u32, (y * w + x + 1) as u32, ((y + 1) * w + x) as u32, ((y + 1) * w + x + 1) as u32);
            idx.extend([a, b, cc, b, d, cc]);
        }
    }
    // Triangle order: partly shuffled.
    let swaps = usize::from(c.u8()) * idx.len() / 3 / 255;
    let tris = idx.len() / 3;
    for _ in 0..swaps {
        let (i, j) = ((rnd() * tris as f32) as usize % tris, (rnd() * tris as f32) as usize % tris);
        for k in 0..3 {
            idx.swap(i * 3 + k, j * 3 + k);
        }
    }
    let mv = 3 + usize::from(c.u8()) % 254;
    let mt = 1 + usize::from(c.u8()) * 2 % 512;
    let settings = MeshletSettings { max_vertices: mv, max_triangles: mt };
    let min = usize::from(c.u8()) % (mt + 1);
    let fill = c.unit();
    let unlimited = Limits { max_bytes: usize::MAX, max_work: u64::MAX };
    let p = Positions::from_packed(&pos);
    let mut sws = Workspace::new(unlimited);
    let serial = meshoptimizer_rs::build_meshlets_spatial(&idx, p, settings, min, fill, &mut sws);
    let su = sws.usage();
    for (r, u) in each_pool(|| {
        let mut ws = Workspace::new(unlimited);
        let r = build_meshlets_spatial_parallel(&idx, p, settings, min, fill, &mut ws);
        (format!("{r:?}"), ws.usage().work)
    }) {
        assert_eq!(r, format!("{serial:?}"), "parallel spatial meshlets differ from serial");
        assert_eq!(u, su.work, "counted work differs");
    }
}
