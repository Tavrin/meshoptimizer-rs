//! Independent two-sided orthographic CPU depth renderer, dedicated CC0-1.0.
//! Eight tested corner directions, 512x512 pixels; no library raster predicates.
use std::{fs, io::Read};
type P = [f64; 3];
const SIZE: usize = 512;
fn dot(a: P, b: P) -> f64 {
    (a[0] * b[0] + a[1] * b[1]) + a[2] * b[2]
}
fn cross(a: P, b: P) -> P {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn sub(a: P, b: P) -> P {
    std::array::from_fn(|k| a[k] - b[k])
}
fn orient(a: P, b: P, x: f64, y: f64) -> f64 {
    (b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0])
}
fn render(p: &[P], indices: &[u32], side: f64) -> Vec<u16> {
    let mut z = vec![f64::NEG_INFINITY; SIZE * SIZE];
    for t in indices.chunks_exact(3) {
        let [a, b, c] = [p[t[0] as usize], p[t[1] as usize], p[t[2] as usize]];
        let area = orient(a, b, c[0], c[1]);
        if area == 0. {
            continue;
        }
        let lo =
            std::array::from_fn::<_, 2, _>(|k| a[k].min(b[k]).min(c[k]).floor().max(0.) as usize);
        let hi = std::array::from_fn::<_, 2, _>(|k| {
            (a[k].max(b[k]).max(c[k]).ceil() as usize).min(SIZE - 1)
        });
        for y in lo[1]..=hi[1] {
            for x in lo[0]..=hi[0] {
                let u = orient(b, c, x as f64 + 0.5, y as f64 + 0.5) / area;
                let v = orient(c, a, x as f64 + 0.5, y as f64 + 0.5) / area;
                let w = orient(a, b, x as f64 + 0.5, y as f64 + 0.5) / area;
                if u >= 0. && v >= 0. && w >= 0. {
                    let depth = u * a[2] + v * b[2] + w * c[2];
                    z[y * SIZE + x] = z[y * SIZE + x].max(depth);
                }
            }
        }
    }
    z.iter()
        .map(|&z| {
            if z.is_finite() {
                (1. + ((z / side + 1.) * 0.5).clamp(0., 1.) * 65534.).round() as u16
            } else {
                0
            }
        })
        .collect()
}
fn save(path: &str, p: &[u16]) {
    let mut raw = format!("P5\n{SIZE} {SIZE}\n65535\n").into_bytes();
    for v in p {
        raw.extend(v.to_be_bytes());
    }
    fs::write(path, raw).unwrap();
}
fn words(raw: &[u8]) -> Vec<u32> {
    raw.chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect()
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut raw = Vec::new();
    fs::File::open(&args[1])
        .unwrap()
        .read_to_end(&mut raw)
        .unwrap();
    let nv = u32::from_le_bytes(raw[..4].try_into().unwrap()) as usize;
    let p: Vec<P> = raw[8..8 + nv * 12]
        .chunks_exact(12)
        .map(|v| {
            std::array::from_fn(|k| {
                f32::from_le_bytes(v[k * 4..k * 4 + 4].try_into().unwrap()) as f64
            })
        })
        .collect();
    let before = words(&raw[8 + nv * 12..]);
    let after = words(&fs::read(&args[2]).unwrap());
    let min: P = std::array::from_fn(|k| p.iter().map(|v| v[k]).fold(f64::INFINITY, f64::min));
    let max: P = std::array::from_fn(|k| p.iter().map(|v| v[k]).fold(f64::NEG_INFINITY, f64::max));
    let center: P = std::array::from_fn(|k| (min[k] + max[k]) * 0.5);
    let side = (0..3).map(|k| max[k] - min[k]).fold(0., f64::max);
    let mut total = 0;
    let mut counts = Vec::new();
    for view in 0..8 {
        let d: P = std::array::from_fn(|k| {
            if view & (1 << k) == 0 {
                -1. / 3_f64.sqrt()
            } else {
                1. / 3_f64.sqrt()
            }
        });
        let u = cross(d, [1., 0., 0.]);
        let length = dot(u, u).sqrt();
        let u = u.map(|v| v / length);
        let v = cross(d, u);
        let projected: Vec<P> = p
            .iter()
            .map(|&p| {
                let q = sub(p, center);
                [
                    (dot(q, u) / side + 1.) * SIZE as f64 * 0.5,
                    (dot(q, v) / side + 1.) * SIZE as f64 * 0.5,
                    dot(q, d),
                ]
            })
            .collect();
        let a = render(&projected, &before, side);
        let b = render(&projected, &after, side);
        let changed = a.iter().zip(&b).filter(|(a, b)| a != b).count();
        total += changed;
        counts.push(changed);
        save(&format!("{}-{view}-before.pgm", args[3]), &a);
        save(&format!("{}-{view}-after.pgm", args[3]), &b);
    }
    println!("{{\"changed_pixels\":{total},\"per_view\":{counts:?},\"width\":{SIZE},\"height\":{SIZE},\"depth_bits\":16}}");
    if total != 0 {
        std::process::exit(1);
    }
}
