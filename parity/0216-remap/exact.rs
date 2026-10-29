//! Exact train-head-vs-candidate comparison for the 0.2.16 remap lane (remap
//! family). Two separately compiled crate versions (`baseline`, `candidate`)
//! run every remap entry point at every work limit from 0 to the success
//! charge + 1, and at byte limits around the success footprint, on small
//! generated meshes. Each call runs twice on one workspace (cold, warm).
//! Results, errors, `Usage`, and complete caller buffers (including partial
//! writes and untouched tails) must be equal.
use std::fmt::Write;

#[derive(Debug, PartialEq)]
struct Observation {
    result: Result<usize, String>,
    output: Vec<u8>,
    usage: (u64, usize),
}

#[derive(Clone)]
struct Stream {
    data: Vec<u8>,
    size: usize,
    stride: usize,
    offset: usize,
}
struct Case {
    name: String,
    count: usize,
    streams: Vec<Stream>,
    positions: Vec<f32>,
    position_stride: usize,
    indices: Vec<u32>,
    remap: Vec<u32>,
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) as u32
    }
    fn below(&mut self, n: u32) -> u32 {
        self.next() % n
    }
}

fn u8s(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}



macro_rules! call {
    ($m:ident, $c:expr, $api:expr, $work:expr, $bytes:expr) => {{
        use $m as m;
        let c: &Case = $c;
        let mut ws = m::Workspace::new(m::Limits {
            max_work: $work,
            max_bytes: $bytes,
        });
        let streams: Vec<m::VertexStream<'_>> = c
            .streams
            .iter()
            .map(|s| m::VertexStream::new(&s.data, c.count, s.size, s.stride, s.offset).unwrap())
            .collect();
        let positions =
            m::Positions::from_interleaved(&c.positions, c.count, c.position_stride, 0).unwrap();
        let key = &c.streams[0];
        let custom = |a: u32, b: u32| {
            key.data[key.offset + a as usize * key.stride] & 1
                == key.data[key.offset + b as usize * key.stride] & 1
        };
        let run = |ws: &mut m::Workspace| {
            let fill = |n: usize| vec![0xa5a5_a5a5u32; n];
            let mut out32: Vec<u32> = Vec::new();
            let mut out8: Vec<u8> = Vec::new();
            let ix = Some(&c.indices[..]);
            let n = c.count;
            let ni = c.indices.len();
            let result: Result<usize, m::Error> = match $api {
                0 => m::generate_vertex_remap(ix, streams[0], ws).map(|r| {
                    out32 = r.remap;
                    r.vertex_count
                }),
                1 => {
                    out32 = fill(n + 2);
                    m::generate_vertex_remap_into(&mut out32, ix, streams[0], ws)
                }
                2 => {
                    out32 = fill(n.saturating_sub(1));
                    m::generate_vertex_remap_into(&mut out32, None, streams[0], ws)
                }
                3 => m::generate_vertex_remap_multi(ix, &streams, ws).map(|r| {
                    out32 = r.remap;
                    r.vertex_count
                }),
                4 => {
                    out32 = fill(n + 1);
                    m::generate_vertex_remap_multi_into(&mut out32, None, &streams, ws)
                }
                5 => m::generate_vertex_remap_custom(ix, positions, custom, ws).map(|r| {
                    out32 = r.remap;
                    r.vertex_count
                }),
                6 => {
                    out32 = fill(n + 1);
                    m::generate_vertex_remap_custom_into(&mut out32, None, positions, custom, ws)
                }
                7 => m::generate_position_remap(positions, ws).map(|r| {
                    out32 = r;
                    0
                }),
                8 => {
                    out32 = fill(n + 1);
                    m::generate_position_remap_into(&mut out32, positions, ws).map(|()| 0)
                }
                9 => m::remap_vertex_buffer(streams[0], &c.remap, ws).map(|v| {
                    out8 = v;
                    0
                }),
                10 | 11 | 12 => {
                    // Out-of-range targets fail validation; size the buffer from valid ones.
                    let used = c.remap.iter().filter(|&&r| (r as usize) < n).map(|&r| r as usize + 1).max().unwrap_or(0);
                    let len = match $api {
                        10 => used * key.size,
                        11 => used * key.size + 7,
                        _ => (used * key.size).saturating_sub(1),
                    };
                    out8 = vec![0x5a; len];
                    m::remap_vertex_buffer_into(&mut out8, streams[0], &c.remap, ws).map(|()| 0)
                }
                13 => m::remap_index_buffer(ix, &c.remap, ws).map(|v| {
                    out32 = v;
                    0
                }),
                14 => m::remap_index_buffer(None, &c.remap, ws).map(|v| {
                    out32 = v;
                    0
                }),
                15 => {
                    out32 = fill(ni + 1);
                    m::remap_index_buffer_into(&mut out32, ix, &c.remap, ws).map(|()| 0)
                }
                16 => {
                    out32 = fill(n);
                    m::remap_index_buffer_into(&mut out32, None, &c.remap, ws).map(|()| 0)
                }
                17 => m::generate_shadow_index_buffer(&c.indices, streams[0], ws).map(|v| {
                    out32 = v;
                    0
                }),
                18 => {
                    out32 = fill(ni + 2);
                    m::generate_shadow_index_buffer_into(&mut out32, &c.indices, streams[0], ws)
                }
                19 => m::generate_shadow_index_buffer_multi(&c.indices, &streams, ws).map(|v| {
                    out32 = v;
                    0
                }),
                20 => {
                    out32 = fill(ni + 1);
                    m::generate_shadow_index_buffer_multi_into(&mut out32, &c.indices, &streams, ws)
                }
                21 => m::filter_index_buffer(&c.indices, streams[0], ws).map(|v| {
                    out32 = v;
                    0
                }),
                22 => m::filter_index_buffer_multi(&c.indices, &streams, ws).map(|v| {
                    out32 = v;
                    0
                }),
                _ => m::generate_tessellation_index_buffer(&c.indices, positions, ws).map(|v| {
                    out32 = v;
                    0
                }),
            };
            let mut output = u8s(&out32);
            output.extend(out8);
            Observation {
                result: result.map_err(|e| format!("{e:?}")),
                output,
                usage: (ws.usage().work, ws.usage().bytes),
            }
        };
        let cold = run(&mut ws);
        let warm = run(&mut ws);
        // Retain scratch from another family, then fail immediately before retry.
        let _ = m::generate_vertex_remap(None, streams[0], &mut ws);
        let retained = run(&mut ws);
        let _ = m::remap_index_buffer(Some(&[u32::MAX]), &[], &mut ws);
        let after_failure = run(&mut ws);
        (cold, warm, retained, after_failure)
    }};
}

const LAYOUTS: [(usize, usize, usize); 10] =
    [(1, 1, 0), (3, 3, 0), (4, 4, 0), (4, 7, 2), (8, 8, 0), (12, 12, 0), (12, 16, 4), (16, 16, 0), (16, 19, 1), (20, 20, 0)];

fn cases() -> Vec<Case> {
    let mut rng = Rng(0x0215_ba51c5);
    let mut out = Vec::new();
    for count in [0usize, 1, 2, 3, 4, 5, 7, 8, 9, 16, 17, 40] {
        for layout in LAYOUTS {
            for nstreams in [1usize] {
                for alphabet in [2u32, 7] {
                    push_cases(&mut out, &mut rng, count, layout, nstreams, alphabet, 12);
                }
            }
        }
    }
    out
}

fn push_cases(
    out: &mut Vec<Case>,
    rng: &mut Rng,
    count: usize,
    (size, stride, offset): (usize, usize, usize),
    nstreams: usize,
    alphabet: u32,
    max_triangles: u32,
) {
                    let mut streams = Vec::new();
                    for s in 0..nstreams {
                        let (size, stride, offset) = if s == 0 { (size, stride, offset) } else { ([4, 12, 5][s], [4, 12, 5][s] + s - 1, s) };
                        let len = if count == 0 { offset } else { offset + (count - 1) * stride + size };
                        let data = (0..len + 3).map(|_| rng.below(alphabet) as u8).collect();
                        streams.push(Stream { data, size, stride, offset });
                    }
                    let values = [0.0f32, -0.0, 1.0, 0.5, f32::NAN, -2.0];
                    let position_stride = if alphabet == 2 { 3 } else { 4 };
                    let positions: Vec<f32> = (0..count * position_stride)
                        .map(|_| values[rng.below(if alphabet == 2 { 3 } else { 6 }) as usize])
                        .collect();
                    let triangles = if count == 0 { 0 } else { 1 + rng.below(max_triangles) as usize };
                    let indices: Vec<u32> = (0..triangles * 3).map(|_| rng.below(count as u32)).collect();
                    // Index variants: valid, one out-of-range reference, an incomplete triangle.
                    for variant in 0..3 {
                        let mut ix = indices.clone();
                        match variant {
                            1 if !ix.is_empty() => {
                                let at = rng.below(ix.len() as u32) as usize;
                                ix[at] = count as u32 + rng.below(3);
                            }
                            2 => ix.push(0),
                            1 => continue,
                            _ => {}
                        }
                        // Remap variants: first reference of each referenced vertex, an
                        // unmapped referenced entry, an out-of-range target, identity.
                        let mut first = vec![u32::MAX; count];
                        let mut next = 0;
                        for &i in &indices {
                            if first[i as usize] == u32::MAX {
                                first[i as usize] = next;
                                next += 1;
                            }
                        }
                        let mut remaps = vec![first.clone(), (0..count as u32).collect(), vec![u32::MAX; count], vec![0; count]];
                        if count > 2 {
                            let mut holes = vec![u32::MAX; count];
                            holes[0] = 2;
                            holes[count - 1] = 2;
                            remaps.push(holes);
                        }
                        if count > 0 {
                            let mut unmapped = first.clone();
                            if let Some(&i) = indices.last() {
                                unmapped[i as usize] = u32::MAX;
                            }
                            remaps.push(unmapped);
                            let mut far = first.clone();
                            far[rng.below(count as u32) as usize] = [count as u32, u32::MAX - 1][rng.below(2) as usize];
                            remaps.push(far);
                        }
                        for (r, remap) in remaps.into_iter().enumerate() {
                            out.push(Case {
                                name: format!("n{count} s{size}/{stride}/{offset} k{nstreams} a{alphabet} i{variant} r{r}"),
                                count,
                                streams: streams.clone(),
                                positions: positions.clone(),
                                position_stride,
                                indices: ix.clone(),
                                remap,
                            });
                        }
                    }
}

// Random larger meshes with random limits (deterministic seed per run).
fn fuzz_cases(rng: &mut Rng, n: usize) -> Vec<Case> {
    let mut out = Vec::new();
    for _ in 0..n {
        let count = rng.below(400) as usize;
        let layout = LAYOUTS[rng.below(LAYOUTS.len() as u32) as usize];
        let nstreams = 1;
        let alphabet = 2 + rng.below(30);
        push_cases(&mut out, rng, count, layout, nstreams, alphabet, 600);
    }
    out
}

fn main() {
    let mut log = String::new();
    let mut compared = 0usize;
    let mut differences = 0usize;
    // `exact` sweeps every work limit; `exact fuzz N SEED` draws N random
    // meshes and checks six random work limits plus the success limits.
    let args: Vec<String> = std::env::args().collect();
    let fuzz = args.get(1).is_some_and(|a| a == "fuzz");
    let mut rng = Rng(args.get(3).map_or(1, |s| s.parse().unwrap()));
    let cases = if fuzz { fuzz_cases(&mut rng, args[2].parse().unwrap()) } else { cases() };
    for c in &cases {
        for api in 9..=16 {
            // Remap-variant cases only feed the remap_* entry points.
            if !c.name.ends_with("r0") && !(9..=16).contains(&api) {
                continue;
            }
            let (ok, _, _, _) = call!(candidate, c, api, u64::MAX, usize::MAX);
            let mut limits: Vec<(u64, usize)> = if fuzz {
                let top = ok.usage.0 + 2;
                (0..6).map(|_| (u64::from(rng.next()) % top, usize::MAX)).collect()
            } else {
                (0..=(c.indices.len().max(c.remap.len()) as u64 * 2 + 1)).map(|w| (w, usize::MAX)).collect()
            };
            if !fuzz {
                let work_limits = limits.clone();
                for b in [0, 1, ok.usage.1.saturating_sub(1), ok.usage.1, ok.usage.1 + 1] {
                    limits.extend(work_limits.iter().map(|&(w, _)| (w, b)));
                }
            }
            limits.push((ok.usage.0, usize::MAX));
            for b in [0, 1, ok.usage.1.saturating_sub(1), ok.usage.1, ok.usage.1 + 1] {
                limits.push((u64::MAX, b));
            }
            for (work, bytes) in limits {
                let a = call!(baseline, c, api, work, bytes);
                let b = call!(candidate, c, api, work, bytes);
                compared += 1;
                if a != b {
                    differences += 1;
                    if differences < 50 {
                        writeln!(log, "DIFF {} api {api} work {work} bytes {bytes}: base {a:?} candidate {b:?}", c.name).unwrap();
                    }
                }
            }
        }
    }
    print!("{log}");
    println!("cases {} comparisons {compared} differences {differences}", cases.len());
    if differences != 0 || compared == 0 {
        std::process::exit(1);
    }
}
