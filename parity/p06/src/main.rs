//! Resident P06 timing transport; meaningful output is serialized after timing.
#![forbid(unsafe_code)]
use meshoptimizer_rs::codec::*;
use meshoptimizer_rs::parallel::*;
use meshoptimizer_rs::*;
use std::hint::black_box;
use std::io::{self, BufRead, Write};
use std::time::Instant;

struct Mesh {
    positions: Vec<[f32; 3]>,
    indices: Vec<u32>,
    vertices: Vec<u8>,
    levels: Vec<SimplifySettings>,
    encoded: Vec<u8>,
    groups: Vec<clusterlod::Group>,
}
fn meshes(path: &str, count: usize) -> Vec<Mesh> {
    let text = std::fs::read_to_string(path).unwrap();
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    for line in text.lines() {
        let mut fields = line.split_whitespace();
        match fields.next() {
            Some("v") => {
                let xyz: Vec<f32> = fields.take(3).map(|s| s.parse().unwrap()).collect();
                positions.push([xyz[0], xyz[1], xyz[2]]);
            }
            Some("f") => {
                let face: Vec<u32> = fields
                    .map(|s| s.split('/').next().unwrap().parse::<u32>().unwrap() - 1)
                    .collect();
                for i in 1..face.len() - 1 {
                    indices.extend([face[0], face[i], face[i + 1]]);
                }
            }
            _ => {}
        }
    }
    (0..count)
        .map(|i| {
            let positions: Vec<_> = positions
                .iter()
                .map(|p| [p[0] + i as f32 * 3., p[1], p[2] * (1. + i as f32 / 32.)])
                .collect();
            let vertices: Vec<u8> = positions
                .iter()
                .flatten()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            let encoded = encode_vertex_buffer(
                &vertices,
                positions.len(),
                12,
                VertexEncoding::new(0, 2).unwrap(),
                &mut Workspace::default(),
            )
            .unwrap();
            let levels = [0.5, 0.25, 0.125]
                .map(|r| SimplifySettings {
                    target_index_count: (indices.len() as f32 * r) as usize,
                    target_error: 1.,
                    options: SimplifyOptions::EMPTY,
                })
                .to_vec();
            Mesh {
                positions,
                indices: indices.clone(),
                vertices,
                levels,
                encoded,
                groups: Vec::new(),
            }
        })
        .collect()
}
fn serial<I, T>(
    inputs: &[I],
    run: impl Fn(&I, &mut Workspace) -> Result<T, Error>,
) -> Vec<Result<T, Error>> {
    inputs
        .iter()
        .map(|i| run(i, &mut Workspace::default()))
        .collect()
}
trait Payload {
    fn write(&self, out: &mut Vec<u8>);
}
fn word(n: u32, out: &mut Vec<u8>) {
    out.extend(n.to_le_bytes());
}
fn len(n: usize, out: &mut Vec<u8>) {
    out.extend((n as u64).to_le_bytes());
}
fn bounds(b: clusterlod::LodBounds, out: &mut Vec<u8>) {
    for f in b.center.into_iter().chain([b.radius, b.error]) {
        word(f.to_bits(), out);
    }
}
impl Payload for Vec<u8> {
    fn write(&self, out: &mut Vec<u8>) {
        len(self.len(), out);
        out.extend(self);
    }
}
impl Payload for Vec<SimplifiedMesh> {
    fn write(&self, out: &mut Vec<u8>) {
        len(self.len(), out);
        for m in self {
            word(m.error.to_bits(), out);
            len(m.indices.len(), out);
            for &i in &m.indices {
                word(i, out);
            }
        }
    }
}
impl Payload for Meshlets {
    fn write(&self, out: &mut Vec<u8>) {
        len(self.meshlets.len(), out);
        for m in &self.meshlets {
            for w in [
                m.vertex_offset,
                m.triangle_offset,
                m.vertex_count,
                m.triangle_count,
            ] {
                word(w, out);
            }
        }
        len(self.vertices.len(), out);
        for &i in &self.vertices {
            word(i, out);
        }
        self.triangles.write(out);
    }
}
impl Payload for Vec<clusterlod::GroupOutput> {
    fn write(&self, out: &mut Vec<u8>) {
        len(self.len(), out);
        for g in self {
            word(g.group.depth as u32, out);
            bounds(g.group.simplified, out);
            len(g.clusters.len(), out);
            for c in &g.clusters {
                word(c.refined as u32, out);
                len(c.vertex_count, out);
                bounds(c.bounds, out);
                len(c.indices.len(), out);
                for &i in &c.indices {
                    word(i, out);
                }
            }
        }
    }
}
impl Payload for Vec<clusterlod::Node> {
    fn write(&self, out: &mut Vec<u8>) {
        len(self.len(), out);
        for n in self {
            bounds(n.bounds, out);
            for w in [n.group as u32, n.child_offset, n.child_count] {
                word(w, out);
            }
        }
    }
}
fn payload<T: Payload>(results: &[Result<T, Error>]) -> Vec<u8> {
    let mut out = Vec::new();
    len(results.len(), &mut out);
    for r in results {
        r.as_ref()
            .expect("benchmark input must succeed")
            .write(&mut out);
    }
    out
}
fn sample(
    family: &str,
    meshes: &[Mesh],
    parallel: bool,
    pool: &rayon::ThreadPool,
) -> (f64, Vec<u8>) {
    let limits = Limits::default();
    match family {
        "lod" => {
            let inputs: Vec<_> = meshes
                .iter()
                .map(|m| LodChainInput {
                    indices: &m.indices,
                    positions: Positions::from_packed(&m.positions),
                    levels: &m.levels,
                    attributes: None,
                })
                .collect();
            let start = Instant::now();
            let result = black_box(if parallel {
                pool.install(|| simplify_lod_chains_batch(black_box(&inputs), limits))
                    .unwrap()
            } else {
                serial(black_box(&inputs), simplify_lod_chain)
            });
            let time = start.elapsed().as_secs_f64();
            (time, payload(&result))
        }
        "encode" => {
            let inputs: Vec<_> = meshes
                .iter()
                .flat_map(|m| {
                    [
                        EncodeInput::Vertices {
                            vertices: &m.vertices,
                            count: m.positions.len(),
                            stride: 12,
                            encoding: VertexEncoding::DEFAULT,
                        },
                        EncodeInput::Triangles {
                            indices: &m.indices,
                            encoding: IndexEncoding::DEFAULT,
                        },
                        EncodeInput::Sequence {
                            indices: &m.indices,
                            encoding: IndexEncoding::DEFAULT,
                        },
                    ]
                })
                .collect();
            let start = Instant::now();
            let result = black_box(if parallel {
                pool.install(|| encode_buffers_batch(black_box(&inputs), limits))
                    .unwrap()
            } else {
                serial(black_box(&inputs), |i, ws| i.encode(ws))
            });
            let time = start.elapsed().as_secs_f64();
            (time, payload(&result))
        }
        "decode" => {
            let inputs: Vec<_> = meshes
                .iter()
                .map(|m| DecodeInput {
                    source: &m.encoded,
                    view: BufferView::new(Mode::Attributes, Filter::None, m.positions.len(), 12)
                        .unwrap(),
                })
                .collect();
            let start = Instant::now();
            let result = black_box(if parallel {
                pool.install(|| decode_buffer_views_batch(black_box(&inputs), limits))
                    .unwrap()
            } else {
                serial(black_box(&inputs), |i, ws| i.view.decode(i.source, ws))
            });
            let time = start.elapsed().as_secs_f64();
            (time, payload(&result))
        }
        "meshlets" => {
            let inputs: Vec<_> = meshes
                .iter()
                .map(|m| MeshletInput {
                    indices: &m.indices,
                    positions: Positions::from_packed(&m.positions),
                    settings: MeshletSettings::default(),
                    builder: MeshletBuilder::Standard { cone_weight: 0.25 },
                })
                .collect();
            let start = Instant::now();
            let result = black_box(if parallel {
                pool.install(|| build_meshlets_batch(black_box(&inputs), limits))
                    .unwrap()
            } else {
                serial(black_box(&inputs), |i, ws| i.build(ws))
            });
            let time = start.elapsed().as_secs_f64();
            (time, payload(&result))
        }
        "cluster_lod" => {
            let start = Instant::now();
            // Required fresh copies are included in both timed APIs.
            let mut positions: Vec<_> = meshes.iter().map(|m| m.positions.clone()).collect();
            let mut inputs: Vec<_> = meshes
                .iter()
                .zip(&mut positions)
                .map(|(m, p)| ClusterLodInput {
                    config: clusterlod::default_config(64).unwrap(),
                    mesh: clusterlod::Mesh {
                        indices: &m.indices,
                        positions: p,
                        attributes: None,
                        vertex_lock: None,
                        attribute_weights: &[],
                        attribute_protect_mask: 0,
                    },
                })
                .collect();
            let result = black_box(if parallel {
                pool.install(|| build_cluster_lod_batch(black_box(&mut inputs), limits))
                    .unwrap()
            } else {
                inputs
                    .iter_mut()
                    .map(|i| {
                        clusterlod::build(
                            i.config,
                            clusterlod::Mesh {
                                indices: i.mesh.indices,
                                positions: &mut *i.mesh.positions,
                                attributes: i.mesh.attributes,
                                vertex_lock: i.mesh.vertex_lock,
                                attribute_weights: i.mesh.attribute_weights,
                                attribute_protect_mask: i.mesh.attribute_protect_mask,
                            },
                            &mut Workspace::default(),
                        )
                    })
                    .collect()
            });
            let time = start.elapsed().as_secs_f64();
            let mut bytes = payload(&result);
            for p in positions {
                for f in p.into_iter().flatten() {
                    word(f.to_bits(), &mut bytes);
                }
            }
            (time, bytes)
        }
        "hierarchy" => {
            let inputs: Vec<_> = meshes
                .iter()
                .map(|m| ClusterHierarchyInput {
                    groups: &m.groups,
                    node_width: 8,
                    level_count: m.groups.iter().map(|g| g.depth as usize + 1).max().unwrap(),
                })
                .collect();
            let start = Instant::now();
            let result = black_box(if parallel {
                pool.install(|| build_cluster_hierarchies_batch(black_box(&inputs), limits))
                    .unwrap()
            } else {
                serial(black_box(&inputs), |i, ws| {
                    clusterlod::build_hierarchy(i.groups, i.node_width, i.level_count, ws)
                })
            });
            let time = start.elapsed().as_secs_f64();
            (time, payload(&result))
        }
        _ => panic!("unknown family"),
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let family = &args[1];
    let pool = rayon::ThreadPoolBuilder::new()
        .thread_name(|i| format!("p06-worker-{i}"))
        .num_threads(args[2].parse().unwrap())
        .build()
        .unwrap();
    let mut meshes = meshes(&args[3], args[4].parse().unwrap());
    if family == "hierarchy" {
        for m in &mut meshes {
            let mut positions = m.positions.clone();
            m.groups = clusterlod::build(
                clusterlod::default_config(64).unwrap(),
                clusterlod::Mesh {
                    indices: &m.indices,
                    positions: &mut positions,
                    attributes: None,
                    vertex_lock: None,
                    attribute_weights: &[],
                    attribute_protect_mask: 0,
                },
                &mut Workspace::default(),
            )
            .unwrap()
            .iter()
            .map(|g| g.group)
            .collect();
        }
    }
    let mut out = io::stdout().lock();
    out.write_all(b"R").unwrap();
    out.flush().unwrap();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "Q" {
            break;
        }
        assert!(line == "S" || line == "P");
        let (time, bytes) = sample(family, &meshes, line == "P", &pool);
        out.write_all(b"T").unwrap();
        out.write_all(&time.to_le_bytes()).unwrap();
        out.write_all(&(bytes.len() as u64).to_le_bytes()).unwrap();
        out.write_all(&bytes).unwrap();
        out.flush().unwrap();
    }
}
