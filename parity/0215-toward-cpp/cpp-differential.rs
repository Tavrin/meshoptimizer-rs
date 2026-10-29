//! C++ 1.3 differential (vendored by meshopt 0.7.0) for the vertex-cache
//! optimizers and vertex decoding. Usage: cpp-differential INPUT_DIR, where
//! INPUT_DIR holds the frozen peer inputs (`<case>.bin`). Exits 1 on any
//! difference and prints one summary line.
use meshoptimizer_rs as m;
use std::path::Path;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) as u32
    }
}

fn read(path: &Path) -> (usize, Vec<u32>) {
    let data = std::fs::read(path).unwrap();
    let n = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
    let ni = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
    let start = 8 + n * 12;
    let indices = data[start..start + ni * 4]
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    (n, indices)
}

fn cache(indices: &[u32], vertices: usize, failures: &mut Vec<String>, name: &str) {
    let mut ws = m::Workspace::new(m::Limits {
        max_work: u64::MAX,
        max_bytes: usize::MAX,
    });
    for variant in 0..4 {
        let expected = match variant {
            0 => meshopt::optimize_vertex_cache(indices, vertices),
            1 => meshopt::optimize_vertex_cache_strip(indices, vertices),
            2 => meshopt::optimize_vertex_cache_fifo(indices, vertices, 16),
            _ => meshopt::optimize_vertex_cache_fifo(indices, vertices, 7),
        };
        let actual = match variant {
            0 => m::optimize_vertex_cache(indices, vertices, &mut ws),
            1 => m::optimize_vertex_cache_strip(indices, vertices, &mut ws),
            2 => m::optimize_vertex_cache_fifo(indices, vertices, 16, &mut ws),
            _ => m::optimize_vertex_cache_fifo(indices, vertices, 7, &mut ws),
        }
        .unwrap();
        if actual != expected {
            failures.push(format!("cache {name} variant {variant}"));
        }
    }
}

// One vertex record of N bytes for the safe meshopt wrappers.
#[derive(Clone, Copy)]
struct Record<const N: usize>([u8; N]);
impl<const N: usize> Default for Record<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

fn decode<const N: usize>(data: &[u8], failures: &mut Vec<String>, name: &str) {
    let records: Vec<Record<N>> = data
        .chunks_exact(N)
        .map(|c| Record(c.try_into().unwrap()))
        .collect();
    let count = records.len();
    for version in [0, 1] {
        for level in if version == 0 { 2..=2 } else { 0..=3 } {
            let encoded =
                meshopt::encode_vertex_buffer_level(&records, level, Some(version)).unwrap();
            let mut streams = vec![encoded.clone()];
            if encoded.len() < 3000 {
                let mut rng = Rng(encoded.len() as u64);
                for _ in 0..24 {
                    let mut bad = encoded.clone();
                    let at = rng.next() as usize % bad.len();
                    bad[at] ^= 1 << (rng.next() % 8);
                    streams.push(bad);
                }
                streams.push(encoded[..encoded.len() - 1].to_vec());
            }
            for (k, stream) in streams.iter().enumerate() {
                let expected = meshopt::decode_vertex_buffer::<Record<N>>(stream, count)
                    .map(|v| v.iter().flat_map(|r| r.0).collect::<Vec<u8>>());
                let mut ws = m::Workspace::default();
                let actual = m::codec::decode_vertex_buffer(count, N, stream, &mut ws);
                let same = match (&actual, &expected) {
                    (Ok(a), Ok(e)) => a == e,
                    (Err(_), Err(_)) => true,
                    _ => false,
                };
                if !same {
                    failures.push(format!(
                        "decode {name} v{version} l{level} stream {k}: C++ ok {} ours {:?}",
                        expected.is_ok(),
                        actual.as_ref().err()
                    ));
                }
            }
        }
    }
}

fn main() {
    let inputs = std::env::args().nth(1).expect("input directory");
    let mut failures = Vec::new();
    let mut cases = 0usize;
    let mut rng = Rng(20261028);
    for name in [
        "tiny-smooth",
        "medium-smooth",
        "medium-seams",
        "medium-sparse",
        "million-smooth",
        "real-214e",
        "real-a082",
        "real-b6fd",
    ] {
        let (n, indices) = read(&Path::new(&inputs).join(format!("{name}.bin")));
        cache(&indices, n, &mut failures, name);
        cases += 4;
    }
    for i in 0..400 {
        let faces = 1 + rng.next() as usize % 300;
        let vertices = 3 + rng.next() as usize % (faces * 2 + 1);
        let indices: Vec<u32> = (0..faces * 3)
            .map(|_| rng.next() % vertices as u32)
            .collect();
        cache(&indices, vertices, &mut failures, &format!("random-{i}"));
        cases += 4;
    }
    for i in 0..300 {
        let stride = [4usize, 8, 12, 16, 20, 24, 32, 36, 48, 64, 128, 256][i % 12];
        let count = rng.next() as usize % 700;
        let smooth = rng.next() % 3;
        let data: Vec<u8> = (0..count * stride)
            .map(|b| match smooth {
                0 => rng.next() as u8,
                1 => ((b / stride) * (b % stride + 1) / 3) as u8,
                _ => ((b / stride) as u8 / 5) ^ (rng.next() % 4) as u8,
            })
            .collect();
        let name = format!("random-{i}-{count}x{stride}");
        let f = &mut failures;
        match stride {
            4 => decode::<4>(&data, f, &name),
            8 => decode::<8>(&data, f, &name),
            12 => decode::<12>(&data, f, &name),
            16 => decode::<16>(&data, f, &name),
            20 => decode::<20>(&data, f, &name),
            24 => decode::<24>(&data, f, &name),
            32 => decode::<32>(&data, f, &name),
            36 => decode::<36>(&data, f, &name),
            48 => decode::<48>(&data, f, &name),
            64 => decode::<64>(&data, f, &name),
            128 => decode::<128>(&data, f, &name),
            _ => decode::<256>(&data, f, &name),
        }
        cases += 1;
    }
    for f in &failures {
        println!("DIFF {f}");
    }
    println!("cases {cases} differences {}", failures.len());
    if !failures.is_empty() {
        std::process::exit(1);
    }
}
