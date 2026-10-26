use meshoptimizer_rs::*;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
static OBSERVATIONS: AtomicU64 = AtomicU64::new(0);

fn emit(out: &mut impl Write, value: impl std::fmt::Debug, ws: &Workspace) {
    writeln!(out, "{value:?} {:?}", ws.usage()).unwrap();
    OBSERVATIONS.fetch_add(1, Ordering::Relaxed);
}
fn calls(
    out: &mut impl Write,
    idx: &[u32],
    counts: &[u32],
    p: Positions<'_>,
    s: MeshletSettings,
    min: usize,
    fill: f32,
    target: usize,
    limit: u64,
    bytes: usize,
    short: u8,
) -> u64 {
    let mut high = 0;
    let mut ws = Workspace::new(Limits {
        max_work: limit,
        max_bytes: bytes,
    });
    let r = build_meshlets_spatial(idx, p, s, min, fill, &mut ws);
    high = high.max(ws.usage().work);
    let sizes = r
        .as_ref()
        .map(|m| (m.meshlets.len(), m.vertices.len(), m.triangles.len()))
        .unwrap_or((2, 2, 2));
    emit(out, r, &ws);
    let n = if short == 1 { 0 } else { idx.len() + 2 };
    let marker = Meshlet {
        vertex_offset: 123,
        triangle_offset: 456,
        vertex_count: 789,
        triangle_count: 987,
    };
    let mut descriptors = vec![
        marker;
        if short == 2 {
            sizes.0.saturating_sub(1)
        } else {
            n
        }
    ];
    let mut vertices = vec![
        u32::MAX;
        if short == 3 {
            sizes.1.saturating_sub(1)
        } else {
            n
        }
    ];
    let mut triangles = vec![
        255;
        if short == 4 {
            sizes.2.saturating_sub(1)
        } else {
            n
        }
    ];
    let r = build_meshlets_spatial_into(
        &mut descriptors,
        &mut vertices,
        &mut triangles,
        idx,
        p,
        s,
        min,
        fill,
        &mut ws,
    );
    high = high.max(ws.usage().work);
    emit(out, (r, descriptors, vertices, triangles), &ws);
    for positions in [Some(p), None] {
        let r = partition_clusters(idx, counts, p.len(), positions, target, &mut ws);
        high = high.max(ws.usage().work);
        emit(out, r, &ws);
        let mut assignments = vec![
            u32::MAX;
            if short == 1 {
                0
            } else if short == 2 {
                counts.len().saturating_sub(1)
            } else {
                counts.len() + 2
            }
        ];
        let r = partition_clusters_into(
            &mut assignments,
            idx,
            counts,
            p.len(),
            positions,
            target,
            &mut ws,
        );
        high = high.max(ws.usage().work);
        emit(out, (r, assignments), &ws);
    }
    high
}
fn layouts(p: &[[f32; 3]], visit: impl FnMut(Positions<'_>)) {
    layout_matrix(p, false, visit);
}
fn layout_matrix(p: &[[f32; 3]], all: bool, mut visit: impl FnMut(Positions<'_>)) {
    visit(Positions::from_packed(p));
    let strides: Vec<usize> = if all {
        (3..=64).collect()
    } else {
        vec![3, 4, 16, 64]
    };
    for stride in strides {
        let offsets: Vec<usize> = if all {
            (0..stride).collect()
        } else {
            vec![0, 1]
        };
        for offset in offsets {
            let mut data = vec![0.; p.len() * stride + offset];
            for (i, xyz) in p.iter().enumerate() {
                data[i * stride + offset..i * stride + offset + 3].copy_from_slice(xyz);
            }
            if !p.is_empty() {
                data.truncate((p.len() - 1) * stride + offset + 3);
            }
            visit(Positions::from_interleaved(&data, p.len(), stride, offset).unwrap());
        }
    }
    let strides: Vec<usize> = if all {
        (12..=256).step_by(4).collect()
    } else {
        vec![12, 16, 64, 256]
    };
    for stride in strides {
        let offsets: Vec<usize> = if all {
            (0..stride).collect()
        } else {
            vec![0, 1, 4]
        };
        for offset in offsets {
            for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
                let mut data = vec![0; p.len() * stride + offset];
                for (i, xyz) in p.iter().enumerate() {
                    for (k, x) in xyz.iter().enumerate() {
                        let b = match order {
                            ByteOrder::LittleEndian => x.to_le_bytes(),
                            ByteOrder::BigEndian => x.to_be_bytes(),
                        };
                        data[i * stride + offset + k * 4..i * stride + offset + k * 4 + 4]
                            .copy_from_slice(&b);
                    }
                }
                if !p.is_empty() {
                    data.truncate((p.len() - 1) * stride + offset + 12);
                }
                visit(Positions::from_bytes(&data, p.len(), stride, offset, order).unwrap());
            }
        }
    }
}
fn grid(n: usize) -> (Vec<[f32; 3]>, Vec<u32>) {
    let p = (0..n * n)
        .map(|i| {
            [
                (i % n) as f32,
                (i / n) as f32,
                ((i * 13 % 17) as f32) * 0.25,
            ]
        })
        .collect();
    let mut idx = Vec::new();
    for y in 0..n.saturating_sub(1) {
        for x in 0..n.saturating_sub(1) {
            let a = (y * n + x) as u32;
            idx.extend([
                a,
                a + 1,
                a + n as u32,
                a + n as u32,
                a + 1,
                a + n as u32 + 1,
            ]);
        }
    }
    (p, idx)
}
fn small(out: &mut impl Write) {
    // Vertex-limited, hard-divisibility SAH with COUNTS=true: disjoint
    // triangles reach this path, unlike a connected 3x3 grid.
    let (p, _) = grid(9);
    let idx: Vec<u32> = (0..81).collect();
    let counts = vec![3; 27];
    let positions = Positions::from_packed(&p);
    let s = MeshletSettings {
        max_vertices: 64,
        max_triangles: 32,
    };
    let high = calls(
        out,
        &idx,
        &counts,
        positions,
        s,
        20,
        0.5,
        4,
        u64::MAX,
        1 << 30,
        0,
    );
    for limit in 0..=high + 1 {
        calls(
            out,
            &idx,
            &counts,
            positions,
            s,
            20,
            0.5,
            4,
            limit,
            1 << 30,
            0,
        );
    }
    for kind in 0..18 {
        let (mut p, mut idx) = grid(if kind < 2 { kind } else { 3 });
        match kind {
            3 => p[0][0] = -0.0,
            4 => p[0][0] = f32::from_bits(1),
            5 => p[0][0] = f32::MAX,
            6 => p[0][0] = f32::NAN,
            7 => p[0][0] = f32::INFINITY,
            8 => {
                idx[0] = u32::MAX;
            }
            9 => {
                idx.pop();
            }
            10 => p.fill([0.; 3]),
            11 => idx.fill(0),
            12 => p.fill([f32::MIN_POSITIVE; 3]),
            _ => {}
        }
        for (s, min) in [
            (
                MeshletSettings {
                    max_vertices: 3,
                    max_triangles: 1,
                },
                1,
            ),
            (
                MeshletSettings {
                    max_vertices: 6,
                    max_triangles: 4,
                },
                3,
            ),
            (
                MeshletSettings {
                    max_vertices: 16,
                    max_triangles: 12,
                },
                4,
            ),
        ] {
            let mut counts: Vec<u32> = idx
                .chunks(if kind % 2 == 0 { 3 } else { 6 })
                .map(|c| c.len() as u32)
                .collect();
            if kind == 13 {
                counts.push(0);
            }
            if kind == 14 {
                counts.push(1);
            }
            layouts(&p, |positions| {
                let target = match kind {
                    15 => 0,
                    16 => usize::MAX,
                    _ => 2,
                };
                let high = calls(
                    out,
                    &idx,
                    &counts,
                    positions,
                    s,
                    min,
                    0.5,
                    target,
                    u64::MAX,
                    1 << 30,
                    0,
                );
                for limit in 0..=high + 1 {
                    calls(
                        out,
                        &idx,
                        &counts,
                        positions,
                        s,
                        min,
                        0.5,
                        target,
                        limit,
                        1 << 30,
                        0,
                    );
                }
                for bytes in [0, 1, 64, 1024] {
                    for limit in [0, 1, u64::MAX] {
                        calls(
                            out, &idx, &counts, positions, s, min, 0.5, target, limit, bytes, 0,
                        );
                    }
                }
                for short in 1..=4 {
                    for limit in [high.saturating_sub(1), high, high + 1] {
                        calls(
                            out,
                            &idx,
                            &counts,
                            positions,
                            s,
                            min,
                            0.5,
                            target,
                            limit,
                            1 << 30,
                            short,
                        );
                    }
                }
                calls(
                    out,
                    &idx,
                    &counts,
                    positions,
                    s,
                    min,
                    f32::NAN,
                    target,
                    0,
                    1 << 30,
                    1,
                );
            });
        }
    }
}
fn frozen(out: &mut impl Write, path: &str) {
    let raw = std::fs::read(path).unwrap();
    let words: Vec<u32> = raw
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    let n = words[0] as usize;
    let ni = words[1] as usize;
    let p: Vec<[f32; 3]> = words[2..2 + 3 * n]
        .chunks_exact(3)
        .map(|c| {
            [
                f32::from_bits(c[0]),
                f32::from_bits(c[1]),
                f32::from_bits(c[2]),
            ]
        })
        .collect();
    let idx = &words[2 + 3 * n..2 + 3 * n + ni];
    let counts: Vec<u32> = idx.chunks(96).map(|c| c.len() as u32).collect();
    // Frozen packed case plus byte/interleaved layouts on the bounded corpus.
    if n < 100000 {
        layouts(&p, |p| {
            calls(
                out,
                idx,
                &counts,
                p,
                MeshletSettings::default(),
                16,
                0.5,
                4,
                u64::MAX,
                1 << 30,
                0,
            );
        });
    } else {
        calls(
            out,
            idx,
            &counts,
            Positions::from_packed(&p),
            MeshletSettings::default(),
            16,
            0.5,
            4,
            u64::MAX,
            1 << 30,
            0,
        );
    }
}
fn random(out: &mut impl Write, mut seed: u64, count: usize) {
    for case in 0..count {
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        // Widen the old u8 triangle/vertex ceiling to reach deep BVHs and
        // partition generation wraps, while keeping malformed input coverage.
        let n = if case % 16 == 0 {
            18 + (next() % 24) as usize
        } else {
            2 + (next() % 8) as usize
        };
        let (mut p, mut idx) = grid(n);
        if case % 7 == 0 {
            idx = (0..(p.len() / 3 * 3) as u32).collect();
        }
        let settings = [
            (64, 32, 20),
            (3, 1, 1),
            (16, 16, 9),
            (64, 64, 16),
            (32, 32, 32),
            (255, 511, 257),
            (16, 12, 4),
        ][next() as usize % 7];
        for xyz in &mut p {
            for x in xyz {
                *x = (*x + (next() % 8) as f32) * [0., 0.001, 1., 1000.][(next() % 4) as usize];
            }
        }
        if case % 19 == 0 {
            p[0][0] = f32::from_bits(next() as u32);
        }
        if case % 23 == 0 {
            idx[0] = u32::MAX;
        }
        if case % 29 == 0 {
            idx.pop();
        }
        let chunk = 1 + (next() % 100) as usize;
        let mut counts: Vec<u32> = idx.chunks(chunk).map(|c| c.len() as u32).collect();
        if case % 31 == 0 {
            counts.push(0);
        }
        let limit = if case % 3 == 0 {
            next() % 10000
        } else {
            u64::MAX
        };
        let target = 1 + (next() % 16) as usize;
        let fill = (next() % 17) as f32 / 8.;
        let s = MeshletSettings {
            max_vertices: settings.0,
            max_triangles: settings.1,
        };
        // Include short final byte records, offset and endianness in fuzzing.
        if case % 4 == 0 {
            layouts(&p, |p| {
                calls(
                    out,
                    &idx,
                    &counts,
                    p,
                    s,
                    settings.2,
                    fill,
                    target,
                    limit,
                    1 << 30,
                    0,
                );
            });
        } else {
            calls(
                out,
                &idx,
                &counts,
                Positions::from_packed(&p),
                s,
                settings.2,
                fill,
                target,
                limit,
                1 << 30,
                0,
            );
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out = io::BufWriter::new(io::stdout().lock());
    match args[1].as_str() {
        "small" => small(&mut out),
        "layouts" => {
            let (p, idx) = grid(2);
            layout_matrix(&p, true, |p| {
                let s = MeshletSettings {
                    max_vertices: 3,
                    max_triangles: 3,
                };
                let high = calls(
                    &mut out,
                    &idx,
                    &[3, 3],
                    p,
                    s,
                    2,
                    0.5,
                    2,
                    u64::MAX,
                    1 << 30,
                    0,
                );
                for limit in [0, 1, high.saturating_sub(1), high, high + 1] {
                    calls(&mut out, &idx, &[3, 3], p, s, 2, 0.5, 2, limit, 1 << 30, 0);
                }
            });
        }
        "frozen" => {
            for p in &args[2..] {
                frozen(&mut out, p);
            }
        }
        "random" => random(&mut out, args[2].parse().unwrap(), args[3].parse().unwrap()),
        _ => panic!("unknown mode"),
    }
    eprintln!("observations={}", OBSERVATIONS.load(Ordering::Relaxed));
}
