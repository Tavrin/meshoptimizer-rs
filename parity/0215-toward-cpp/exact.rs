//! Exact v0.2.14-vs-candidate comparison for the 0.2.15 vertex-cache and
//! vertex-decode paths. Two separately compiled crate versions (`baseline`,
//! `candidate`) run every entry point at every work limit from 0 to the
//! success charge + 1 on small inputs, and on every truncated or single-byte
//! corrupted stream of small decode inputs. Results, `Usage`, and complete
//! caller buffers (including partial writes) must be equal.
use std::fmt::Write;

#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<(), String>,
    output: Vec<u8>,
    usage: (u64, usize),
}

fn bytes(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

macro_rules! cache {
    ($m:ident, $indices:expr, $vertices:expr, $api:expr, $fuel:expr) => {{
        use $m as m;
        let indices: &[u32] = $indices;
        let vertices: usize = $vertices;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let mut run = || {
            let mut output = Vec::new();
            let fill = vec![0xdeadbeef; indices.len() + 5];
            let short = vec![0xdeadbeef; indices.len().saturating_sub(1)];
            let result = match $api {
                0 => m::optimize_vertex_cache(indices, vertices, &mut ws).map(|v| output = v),
                1 => {
                    output = fill;
                    m::optimize_vertex_cache_into(&mut output, indices, vertices, &mut ws)
                }
                2 => {
                    output = indices.to_vec();
                    m::optimize_vertex_cache_in_place(&mut output, vertices, &mut ws)
                }
                3 => m::optimize_vertex_cache_strip(indices, vertices, &mut ws).map(|v| output = v),
                4 => {
                    output = fill;
                    m::optimize_vertex_cache_strip_into(&mut output, indices, vertices, &mut ws)
                }
                5 => {
                    output = indices.to_vec();
                    m::optimize_vertex_cache_strip_in_place(&mut output, vertices, &mut ws)
                }
                6 => {
                    output = short;
                    m::optimize_vertex_cache_into(&mut output, indices, vertices, &mut ws)
                }
                7 => {
                    output = short;
                    m::optimize_vertex_cache_strip_into(&mut output, indices, vertices, &mut ws)
                }
                api => {
                    let size = [3, 4, 7, 16, 32][(api - 8) % 5];
                    match (api - 8) / 5 {
                        0 => m::optimize_vertex_cache_fifo(indices, vertices, size, &mut ws)
                            .map(|v| output = v),
                        1 => {
                            output = fill;
                            m::optimize_vertex_cache_fifo_into(
                                &mut output,
                                indices,
                                vertices,
                                size,
                                &mut ws,
                            )
                        }
                        _ => {
                            output = indices.to_vec();
                            m::optimize_vertex_cache_fifo_in_place(
                                &mut output,
                                vertices,
                                size,
                                &mut ws,
                            )
                        }
                    }
                }
            }
            .map_err(|e| format!("{e:?}"));
            Observation {
                result,
                output: bytes(&output),
                usage: (ws.usage().work, ws.usage().bytes),
            }
        };
        let cold = run();
        let warm = run();
        assert_eq!(cold, warm);
        cold
    }};
}
const CACHE_APIS: usize = 8 + 15;

macro_rules! decode {
    ($m:ident, $count:expr, $stride:expr, $source:expr, $api:expr, $fuel:expr) => {{
        use $m as m;
        let (count, stride, source): (usize, usize, &[u8]) = ($count, $stride, $source);
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $fuel,
            max_bytes: usize::MAX,
        });
        let mut output = Vec::new();
        let result = match $api {
            0 => m::codec::decode_vertex_buffer(count, stride, source, &mut ws).map(|v| output = v),
            1 => {
                output = vec![0xa5; count * stride + 3];
                m::codec::decode_vertex_buffer_into(&mut output, count, stride, source, &mut ws)
            }
            2 => {
                output = vec![0x5a; (count * stride).saturating_sub(1)];
                m::codec::decode_vertex_buffer_into(&mut output, count, stride, source, &mut ws)
            }
            3 => m::codec::decode_buffer_view(
                m::codec::Mode::Attributes,
                m::codec::Filter::None,
                count,
                stride,
                source,
                &mut ws,
            )
            .map(|v| output = v),
            _ => {
                output = vec![0x3c; count * stride];
                m::codec::decode_buffer_view_into(
                    &mut output,
                    m::codec::Mode::Attributes,
                    m::codec::Filter::None,
                    count,
                    stride,
                    source,
                    &mut ws,
                )
            }
        }
        .map_err(|e| format!("{e:?}"));
        Observation {
            result,
            output,
            usage: (ws.usage().work, ws.usage().bytes),
        }
    }};
}
const DECODE_APIS: usize = 5;

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

fn grid(w: u32, h: u32) -> Vec<u32> {
    let mut indices = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let a = y * (w + 1) + x;
            let b = a + 1;
            let c = a + w + 1;
            let d = c + 1;
            indices.extend([a, b, c, c, b, d]);
        }
    }
    indices
}

fn cache_cases() -> Vec<(String, Vec<u32>, usize)> {
    let mut rng = Rng(7);
    let mut cases = vec![
        ("empty".into(), vec![], 0),
        ("one".into(), vec![0, 1, 2], 3),
        (
            "degenerate".into(),
            vec![0, 0, 0, 0, 0, 1, 1, 2, 1, 2, 2, 2],
            3,
        ),
        ("grid".into(), grid(4, 3), 20),
        ("unused-vertices".into(), vec![5, 9, 7, 7, 9, 2], 12),
        ("incomplete".into(), vec![0, 1, 2, 3], 4),
        ("out-of-bounds".into(), vec![0, 1, 2, 2, 1, 4], 4),
        (
            "late-out-of-bounds".into(),
            [grid(2, 2), vec![0, 1, 99]].concat(),
            9,
        ),
    ];
    // Fans exceed the live-score clamp (8) and the 16-entry cache.
    let mut fan = Vec::new();
    for i in 0..40u32 {
        fan.extend([0, 1 + i, 2 + i]);
    }
    cases.push(("fan".into(), fan, 42));
    // Repeated identical triangles: every adjacency search hits late.
    cases.push(("repeated".into(), [0u32, 1, 2].repeat(12), 3));
    for n in [5usize, 17, 31] {
        let v = n / 2 + 3;
        let indices = (0..n * 3).map(|_| rng.next() % v as u32).collect();
        cases.push((format!("random-{n}"), indices, v));
    }
    cases
}

fn encode_cases() -> Vec<(String, usize, usize, Vec<u8>)> {
    let mut rng = Rng(11);
    let mut cases = Vec::new();
    for (count, stride) in [
        (1usize, 4usize),
        (3, 12),
        (16, 12),
        (17, 8),
        (33, 12),
        (40, 16),
        (20, 20),
        (9, 32),
        (5, 64),
        (2, 256),
        (300, 12),
        (257, 4),
        (70, 32),
        (130, 24),
    ] {
        for (version, level) in [(0u8, 2u8), (1, 0), (1, 2), (1, 3)] {
            let data: Vec<u8> = (0..count * stride)
                .map(|i| {
                    let record = i / stride;
                    let lane = i % stride;
                    match lane % 4 {
                        0 => (record * 3 + lane) as u8,
                        1 => (rng.next() % 5) as u8,
                        2 => rng.next() as u8,
                        _ => (record / 7) as u8,
                    }
                })
                .collect();
            let mut ws = candidate::Workspace::default();
            let encoded = candidate::codec::encode_vertex_buffer(
                &data,
                count,
                stride,
                candidate::codec::VertexEncoding::new(version, level).unwrap(),
                &mut ws,
            )
            .unwrap();
            cases.push((
                format!("v{version}l{level}-{count}x{stride}"),
                count,
                stride,
                encoded,
            ));
        }
    }
    cases
}

fn main() {
    let mut log = String::new();
    let mut compared = 0usize;
    let mut differences = 0usize;
    let mut check = |name: &str, a: Observation, b: Observation, log: &mut String| {
        compared += 1;
        if a != b {
            differences += 1;
            writeln!(log, "DIFF {name}: base {a:?} candidate {b:?}").unwrap();
        }
    };
    for (name, indices, vertices) in cache_cases() {
        for api in 0..CACHE_APIS {
            let success = cache!(candidate, &indices, vertices, api, u64::MAX).usage.0;
            for fuel in 0..=success + 1 {
                let a = cache!(baseline, &indices, vertices, api, fuel);
                let b = cache!(candidate, &indices, vertices, api, fuel);
                check(
                    &format!("cache {name} api {api} fuel {fuel}"),
                    a,
                    b,
                    &mut log,
                );
            }
        }
    }
    for (name, count, stride, encoded) in encode_cases() {
        for api in 0..DECODE_APIS {
            let success = decode!(candidate, count, stride, &encoded, api, u64::MAX)
                .usage
                .0;
            let fuels: Vec<u64> = if encoded.len() < 2048 {
                (0..=success + 1).collect()
            } else {
                vec![0, 1, success - 1, success, success + 1, u64::MAX]
            };
            for fuel in fuels {
                let a = decode!(baseline, count, stride, &encoded, api, fuel);
                let b = decode!(candidate, count, stride, &encoded, api, fuel);
                check(
                    &format!("decode {name} api {api} fuel {fuel}"),
                    a,
                    b,
                    &mut log,
                );
            }
            // Every truncation and every single-byte corruption of the stream,
            // with an exact count and one count above and below.
            for cut in 0..encoded.len() {
                let a = decode!(baseline, count, stride, &encoded[..cut], api, u64::MAX);
                let b = decode!(candidate, count, stride, &encoded[..cut], api, u64::MAX);
                check(
                    &format!("decode {name} api {api} cut {cut}"),
                    a,
                    b,
                    &mut log,
                );
            }
            if encoded.len() < 4096 {
                for at in 0..encoded.len() {
                    for flip in [0x01u8, 0x80, 0xff] {
                        let mut bad = encoded.clone();
                        bad[at] ^= flip;
                        let a = decode!(baseline, count, stride, &bad, api, u64::MAX);
                        let b = decode!(candidate, count, stride, &bad, api, u64::MAX);
                        check(
                            &format!("decode {name} api {api} flip {at}/{flip}"),
                            a,
                            b,
                            &mut log,
                        );
                    }
                }
            }
            for other in [count.saturating_sub(1), count + 1, count + 16] {
                let a = decode!(baseline, other, stride, &encoded, api, u64::MAX);
                let b = decode!(candidate, other, stride, &encoded, api, u64::MAX);
                check(
                    &format!("decode {name} api {api} count {other}"),
                    a,
                    b,
                    &mut log,
                );
            }
        }
    }
    print!("{log}");
    println!("compared {compared} differences {differences}");
    if differences != 0 {
        std::process::exit(1);
    }
}
