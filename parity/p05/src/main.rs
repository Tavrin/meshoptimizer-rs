use meshoptimizer_rs::*;
use std::io::{self, BufRead, Write};

struct Rng(u32);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}
struct Case {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
    texture: Vec<u8>,
}
fn case_with(seed: u32, shape: Option<(u32, u32, u32)>) -> Case {
    let mut rng = Rng(seed + 1);
    let natural_count = 3 + rng.next() % 6;
    let natural_triangles = 1 + rng.next() % 5;
    let (count, triangles, style) = shape.unwrap_or((natural_count, natural_triangles, 0));
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for _ in 0..count {
        positions.push([0; 3].map(|_| ((rng.next() % 17) as i32 - 8) as f32 / 8.0));
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0; 2].map(|_| ((rng.next() % 17) as i32 - 4) as f32 / 8.0));
    }
    let mut indices: Vec<u32> = (0..triangles * 3).map(|_| rng.next() % count).collect();
    let texture = (0..64).map(|_| rng.next() as u8).collect();
    match style {
        0 => {}
        1 => {
            let mut side = 2u32;
            while (side + 1) * (side + 1) <= count {
                side += 1;
            }
            for i in 0..count as usize {
                let x = i as u32 % side;
                let y = i as u32 / side;
                positions[i] = [x as f32 / 16.0, y as f32 / 16.0, 0.0];
                uvs[i] = [x as f32 / 256.0, y as f32 / 256.0];
            }
            let cells = (side - 1) * (side - 1);
            for t in 0..triangles as usize {
                let cell = (t as u32 / 2) % cells;
                let a = (cell / (side - 1)) * side + cell % (side - 1);
                let triangle = if t & 1 == 0 {
                    [a, a + 1, a + side]
                } else {
                    [a + 1, a + side + 1, a + side]
                };
                indices[t * 3..t * 3 + 3].copy_from_slice(&triangle);
            }
        }
        2 => {
            let base = (count / 4).max(3) as usize;
            for i in base..count as usize {
                positions[i] = positions[i % base];
                uvs[i] = uvs[i % base];
            }
        }
        3 => {
            let used = (count / 2).max(3);
            for index in &mut indices {
                *index = (*index % used) * 2;
            }
        }
        4 => {
            assert!(count >= triangles * 3);
            for (i, index) in indices.iter_mut().enumerate() {
                *index = i as u32;
            }
        }
        _ => panic!("unknown benchmark style"),
    }
    Case {
        positions,
        normals,
        uvs,
        indices,
        texture,
    }
}
fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}
fn push_f32(out: &mut Vec<u8>, value: f32) {
    push_u32(out, value.to_bits());
}
fn push_u64(out: &mut Vec<u8>, value: usize) {
    out.extend_from_slice(&(value as u64).to_le_bytes());
}
fn input_hash(case: &Case) -> u64 {
    let mut bytes = Vec::new();
    push_u32(&mut bytes, case.positions.len() as u32);
    push_u32(&mut bytes, case.indices.len() as u32);
    for p in &case.positions {
        for &v in p {
            push_f32(&mut bytes, v);
        }
    }
    for n in &case.normals {
        for &v in n {
            push_f32(&mut bytes, v);
        }
    }
    for uv in &case.uvs {
        for &v in uv {
            push_f32(&mut bytes, v);
        }
    }
    for &v in &case.indices {
        push_u32(&mut bytes, v);
    }
    bytes.extend_from_slice(&case.texture);
    bytes.into_iter().fold(14695981039346656037u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(1099511628211)
    })
}
fn run(family: &str, seed: u32, case: &Case) -> Result<Vec<u8>, Error> {
    let mut workspace = Workspace::default();
    let mut out = Vec::new();
    let positions = Positions::from_packed(&case.positions);
    let restart = if seed & 1 == 0 { 0 } else { 65535 };
    match family {
        "stripify" => {
            for value in stripify(&case.indices, case.positions.len(), restart, &mut workspace)? {
                push_u32(&mut out, value);
            }
        }
        "stripify_bound" => push_u64(&mut out, stripify_bound(case.indices.len())?),
        "unstripify" | "unstripify_bound" => {
            let strip = stripify(&case.indices, case.positions.len(), restart, &mut workspace)?;
            if family == "unstripify" {
                for value in unstripify(&strip, restart, &mut workspace)? {
                    push_u32(&mut out, value);
                }
            } else {
                push_u64(&mut out, unstripify_bound(strip.len())?);
            }
        }
        "vertex_cache" => {
            let s = analyze_vertex_cache(
                &case.indices,
                case.positions.len(),
                3 + seed % 24,
                if seed & 1 == 0 { 0 } else { 16 },
                seed % 4,
                &mut workspace,
            )?;
            push_u32(&mut out, s.vertices_transformed);
            push_u32(&mut out, s.warps_executed);
            push_f32(&mut out, s.acmr);
            push_f32(&mut out, s.atvr);
        }
        "vertex_fetch" => {
            let s = analyze_vertex_fetch(
                &case.indices,
                case.positions.len(),
                (4 + seed % 64) as usize,
                &mut workspace,
            )?;
            push_u32(&mut out, s.bytes_fetched);
            push_f32(&mut out, s.overfetch);
        }
        "overdraw" => {
            let s = analyze_overdraw(&case.indices, positions, &mut workspace)?;
            push_u32(&mut out, s.pixels_covered);
            push_u32(&mut out, s.pixels_shaded);
            push_f32(&mut out, s.overdraw);
        }
        "coverage" => {
            let s = analyze_coverage(&case.indices, positions, &mut workspace)?;
            for value in s.coverage {
                push_f32(&mut out, value);
            }
            push_f32(&mut out, s.extent);
        }
        "omm_measure" => {
            let uvs: Vec<f32> = case.uvs.iter().flatten().copied().collect();
            let result = opacity_map_measure(
                &case.indices,
                &uvs,
                case.positions.len(),
                2,
                8,
                8,
                (seed % 5) as u8,
                if seed & 1 == 0 { 0.0 } else { 2.0 },
                &mut workspace,
            )?;
            push_u32(&mut out, result.levels.len() as u32);
            out.extend_from_slice(&result.levels);
            for value in result.sources {
                push_u32(&mut out, value);
            }
            for value in result.omm_indices {
                out.extend_from_slice(&value.to_le_bytes());
            }
        }
        "omm_rasterize" => {
            let level = (seed % 4) as u8;
            let states = if seed & 1 == 0 { 2 } else { 4 };
            let coords = [
                case.uvs[case.indices[0] as usize],
                case.uvs[case.indices[1] as usize],
                case.uvs[case.indices[2] as usize],
            ];
            let data = opacity_map_rasterize(
                level,
                states,
                coords,
                &case.texture,
                1,
                8,
                8,
                8,
                &mut workspace,
            )?;
            let mut caller = vec![0x5a; data.len() + 1];
            let written = opacity_map_rasterize_into(
                &mut caller,
                level,
                states,
                coords,
                &case.texture,
                1,
                8,
                8,
                8,
                &mut workspace,
            )?;
            assert_eq!(written, data.len());
            assert_eq!(&caller[..written], data.as_slice());
            assert_eq!(caller[written], 0x5a);
            assert_eq!(workspace.usage().bytes, 0);
            out.extend_from_slice(&data);
        }
        "omm_entry_size" => push_u64(
            &mut out,
            opacity_map_entry_size((seed % 13) as u8, if seed & 1 == 0 { 2 } else { 4 })?,
        ),
        "omm_compact" => {
            let states = if seed & 1 == 0 { 2 } else { 4 };
            let mut levels = [0, 1, 2, 3];
            let mut offsets = [0u32; 4];
            let mut data = Vec::new();
            for i in 0..4 {
                offsets[i] = data.len() as u32;
                let size = opacity_map_entry_size(levels[i], states)?;
                for j in 0..size {
                    data.push(case.texture[(i * 13 + j) % 64]);
                }
            }
            let mut indices = [0i32, 1, 2, 3, 0, 2];
            let (count, size) = opacity_map_compact(
                &mut data,
                &mut levels,
                &mut offsets,
                &mut indices,
                states,
                &mut workspace,
            )?;
            push_u64(&mut out, count);
            push_u64(&mut out, size);
            out.extend_from_slice(&data);
            out.extend_from_slice(&levels);
            for value in offsets {
                push_u32(&mut out, value);
            }
            for value in indices {
                out.extend_from_slice(&value.to_le_bytes());
            }
        }
        "tangents" => {
            let generated = generate_tangents(
                Some(&case.indices),
                case.indices.len(),
                positions,
                &case.normals,
                &case.uvs,
                seed % 4,
                &mut workspace,
            )?;
            let allocating_bytes = workspace.usage().bytes;
            let mut caller = vec![[42.0; 4]; generated.len() + 2];
            generate_tangents_into(
                &mut caller,
                Some(&case.indices),
                case.indices.len(),
                positions,
                &case.normals,
                &case.uvs,
                seed % 4,
                &mut workspace,
            )?;
            assert!(caller[..generated.len()]
                .iter()
                .flatten()
                .zip(generated.iter().flatten())
                .all(|(actual, expected)| actual.to_bits() == expected.to_bits()));
            assert_eq!(&caller[generated.len()..], &[[42.0; 4]; 2]);
            assert!(workspace.usage().bytes <= allocating_bytes);
            assert!(workspace.usage().bytes + generated.len() * 16 >= allocating_bytes);
            for t in generated {
                for v in t {
                    push_f32(&mut out, v);
                }
            }
        }
        "normals" => {
            let generated = generate_normals(
                Some(&case.indices),
                case.indices.len(),
                positions,
                [0.5, 1.0, 2.0, 3.0][(seed % 4) as usize],
                (seed % 4) as f32 * 0.5,
                &mut workspace,
            )?;
            let allocating_bytes = workspace.usage().bytes;
            let mut caller = vec![[42.0; 3]; generated.len() + 2];
            generate_normals_into(
                &mut caller,
                Some(&case.indices),
                case.indices.len(),
                positions,
                [0.5, 1.0, 2.0, 3.0][(seed % 4) as usize],
                (seed % 4) as f32 * 0.5,
                &mut workspace,
            )?;
            assert!(caller[..generated.len()]
                .iter()
                .flatten()
                .zip(generated.iter().flatten())
                .all(|(actual, expected)| actual.to_bits() == expected.to_bits()));
            assert_eq!(&caller[generated.len()..], &[[42.0; 3]; 2]);
            assert!(workspace.usage().bytes <= allocating_bytes);
            assert!(workspace.usage().bytes + generated.len() * 12 >= allocating_bytes);
            for n in generated {
                for v in n {
                    push_f32(&mut out, v);
                }
            }
        }
        "remesh" => {
            let resolution = (4 + seed % 5) as usize;
            let bound = remesh_bound(
                &case.indices,
                positions,
                resolution,
                seed % 4,
                &mut workspace,
            )?;
            let mesh = remesh(
                &case.indices,
                positions,
                resolution,
                seed % 4,
                &mut workspace,
            )?;
            push_u64(&mut out, bound);
            push_u64(&mut out, mesh.len() / 3);
            for p in mesh {
                for value in p {
                    push_f32(&mut out, value);
                }
            }
        }
        _ => return Err(Error::InvalidParameter),
    }
    Ok(out)
}
struct ResidentCase {
    key: (u32, u32, u32, u32),
    case: Case,
    hash: u64,
}
fn resident_fixture(
    resident: &mut Option<ResidentCase>,
    key: (u32, u32, u32, u32),
) -> (&Case, u64) {
    if resident.as_ref().is_none_or(|previous| previous.key != key) {
        drop(resident.take());
        let (seed, count, triangles, style) = key;
        let case = case_with(seed, Some((count, triangles, style)));
        let hash = input_hash(&case);
        *resident = Some(ResidentCase { key, case, hash });
    }
    let fixture = resident.as_ref().expect("resident fixture");
    (&fixture.case, fixture.hash)
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    // Match C++'s immutable resident fixture throughout a case's pairs. Keep
    // only one case; neither regeneration nor hash scratch perturbs the
    // allocator between API samples.
    let mut resident: Option<ResidentCase> = None;
    for line in stdin.lock().lines() {
        let line = line.expect("stdin");
        let mut parts = line.split_ascii_whitespace();
        let Some(family) = parts.next() else { continue };
        if family == "BENCH" || family == "PROFILE" {
            let profile = family == "PROFILE";
            let operation = parts.next().expect("benchmark operation");
            let seed: u32 = parts.next().expect("seed").parse().expect("numeric seed");
            let count: u32 = parts
                .next()
                .expect("vertices")
                .parse()
                .expect("numeric count");
            let triangles: u32 = parts
                .next()
                .expect("triangles")
                .parse()
                .expect("numeric triangles");
            let style: u32 = parts.next().expect("style").parse().expect("numeric style");
            let iterations: usize = parts
                .next()
                .expect("iterations")
                .parse()
                .expect("numeric iterations");
            let caller = parts.next().expect("mode") == "caller";
            let (case, hash) = resident_fixture(&mut resident, (seed, count, triangles, style));
            let (ns, bytes) = benchmark(operation, seed, case, iterations, caller);
            if profile {
                writeln!(stdout, "{ns:.6} {bytes}").expect("stdout");
            } else {
                writeln!(stdout, "{hash:016x} {ns:.6} {bytes}").expect("stdout");
            }
            stdout.flush().expect("stdout");
            continue;
        }
        let seed: u32 = parts.next().expect("seed").parse().expect("numeric seed");
        let shape = match (parts.next(), parts.next()) {
            (Some(a), Some(b)) => Some((
                a.parse().expect("vertex count"),
                b.parse().expect("triangle count"),
                parts.next().map_or(0, |s| s.parse().expect("style")),
            )),
            _ => None,
        };
        let case = case_with(seed, shape);
        let result = run(family, seed, &case).expect("valid seeded case");
        write!(stdout, "{:016x} ", input_hash(&case)).expect("stdout");
        for byte in result {
            write!(stdout, "{byte:02x}").expect("stdout");
        }
        writeln!(stdout).expect("stdout");
        stdout.flush().expect("stdout");
    }
}

// C++ vector setup eagerly commits caller storage. Rust's zeroed Vec can
// obtain lazy calloc pages; commit them outside the API timer in both drivers.
fn commit_caller_storage<T: Copy>(values: &mut [T], zero: T) {
    let size = core::mem::size_of::<T>();
    if size == 0 {
        return;
    }
    let step = (4096 / size).max(1);
    for page in values.chunks_mut(step) {
        page[0] = std::hint::black_box(zero);
    }
    if let Some(last) = values.last_mut() {
        *last = std::hint::black_box(zero);
    }
}

// Match the C++ batch's one pre-sized allocation. This helper remains inside
// the timed closure; input bytes and entry order match the original builder.
#[inline]
fn compact_batch_data(texture: &[u8], states: u8) -> (Vec<u8>, [u32; 4]) {
    let total: usize = (0..4)
        .map(|level| opacity_map_entry_size(level, states).unwrap())
        .sum();
    let mut data = vec![0u8; total];
    let mut offsets = [0u32; 4];
    let mut offset = 0;
    for (level, entry_offset) in offsets.iter_mut().enumerate() {
        *entry_offset = offset as u32;
        let size = opacity_map_entry_size(level as u8, states).unwrap();
        for j in 0..size {
            data[offset + j] = texture[(level * 13 + j) % 64];
        }
        offset += size;
    }
    (data, offsets)
}

fn benchmark(
    family: &str,
    seed: u32,
    case: &Case,
    iterations: usize,
    caller: bool,
) -> (f64, usize) {
    use std::hint::black_box;
    use std::time::Instant;
    let indices = &case.indices;
    let positions = Positions::from_packed(&case.positions);
    let restart = if seed & 1 == 0 { 0 } else { 65535 };
    let strip = if family == "unstripify" {
        Some(
            stripify(
                indices,
                case.positions.len(),
                restart,
                &mut Workspace::default(),
            )
            .unwrap(),
        )
    } else {
        None
    };
    let uvs: Vec<f32> = if family == "omm_measure" {
        case.uvs.iter().flatten().copied().collect()
    } else {
        Vec::new()
    };
    // Match C++: only the selected caller API's exact bound is resident.
    // Owned outputs and scratch are still allocated inside the API timer.
    let mut out_u32 = if caller && family == "stripify" {
        vec![0u32; stripify_bound(indices.len()).unwrap()]
    } else if caller && family == "unstripify" {
        vec![0u32; unstripify_bound(strip.as_ref().unwrap().len()).unwrap()]
    } else {
        Vec::new()
    };
    let mut out_f32 = if caller && family == "tangents" {
        vec![[0f32; 4]; indices.len()]
    } else {
        Vec::new()
    };
    let mut out_norm = if caller && family == "normals" {
        vec![[0f32; 3]; indices.len()]
    } else {
        Vec::new()
    };
    let mut out_bytes = if caller && family == "omm_rasterize" {
        vec![
            0u8;
            opacity_map_entry_size((seed % 4) as u8, if seed & 1 == 0 { 2 } else { 4 }).unwrap()
        ]
    } else {
        Vec::new()
    };
    let mut out_remesh = if family == "remesh" && caller {
        let bound = remesh_bound(
            indices,
            positions,
            (4 + seed % 5) as usize,
            seed % 4,
            &mut Workspace::default(),
        )
        .unwrap();
        vec![[0f32; 3]; bound * 3]
    } else {
        Vec::new()
    };
    if caller {
        match family {
            "stripify" | "unstripify" => commit_caller_storage(&mut out_u32, 0),
            "omm_rasterize" => commit_caller_storage(&mut out_bytes, 0),
            "tangents" => commit_caller_storage(&mut out_f32, [0.0; 4]),
            "normals" => commit_caller_storage(&mut out_norm, [0.0; 3]),
            "remesh" => commit_caller_storage(&mut out_remesh, [0.0; 3]),
            _ => {}
        }
    }
    if matches!(
        family,
        "stripify_bound" | "unstripify_bound" | "omm_entry_size"
    ) {
        let start = Instant::now();
        match family {
            "stripify_bound" => {
                for iteration in 0..iterations {
                    black_box(stripify_bound(indices.len() + (iteration & 3) * 3).unwrap());
                }
            }
            "unstripify_bound" => {
                for iteration in 0..iterations {
                    black_box(unstripify_bound(indices.len() + (iteration & 3)).unwrap());
                }
            }
            _ => {
                for iteration in 0..iterations {
                    let variant = seed.wrapping_add(iteration as u32);
                    black_box(
                        opacity_map_entry_size(
                            (variant % 13) as u8,
                            if variant & 1 == 0 { 2 } else { 4 },
                        )
                        .unwrap(),
                    );
                }
            }
        }
        return (start.elapsed().as_nanos() as f64 / iterations as f64, 0);
    }
    match family {
        "stripify" => {
            if caller {
                timed_iterations(iterations, |workspace| {
                    black_box(
                        stripify_into(
                            &mut out_u32,
                            indices,
                            case.positions.len(),
                            restart,
                            workspace,
                        )
                        .unwrap(),
                    );
                    black_box(out_u32.as_ptr());
                })
            } else {
                timed_iterations(iterations, |workspace| {
                    keep_output(
                        stripify(indices, case.positions.len(), restart, workspace).unwrap(),
                    );
                })
            }
        }
        "unstripify" => {
            let strip = strip.as_ref().unwrap().as_slice();
            if caller {
                timed_iterations(iterations, |workspace| {
                    black_box(unstripify_into(&mut out_u32, strip, restart, workspace).unwrap());
                    black_box(out_u32.as_ptr());
                })
            } else {
                timed_iterations(iterations, |workspace| {
                    keep_output(unstripify(strip, restart, workspace).unwrap());
                })
            }
        }
        "vertex_cache" => timed_iterations(iterations, |workspace| {
            let result = analyze_vertex_cache(
                indices,
                case.positions.len(),
                3 + seed % 24,
                if seed & 1 == 0 { 0 } else { 16 },
                seed % 4,
                workspace,
            )
            .unwrap();
            black_box(&result);
        }),
        "vertex_fetch" => timed_iterations(iterations, |workspace| {
            let result = analyze_vertex_fetch(
                indices,
                case.positions.len(),
                (4 + seed % 64) as usize,
                workspace,
            )
            .unwrap();
            black_box(&result);
        }),
        "overdraw" => timed_iterations(iterations, |workspace| {
            let result = analyze_overdraw(indices, positions, workspace).unwrap();
            black_box(&result);
        }),
        "coverage" => timed_iterations(iterations, |workspace| {
            let result = analyze_coverage(indices, positions, workspace).unwrap();
            black_box(&result);
        }),
        "omm_measure" => timed_iterations(iterations, |workspace| {
            let output = opacity_map_measure(
                indices,
                &uvs,
                case.positions.len(),
                2,
                8,
                8,
                (seed % 5) as u8,
                if seed & 1 == 0 { 0.0 } else { 2.0 },
                workspace,
            )
            .unwrap();
            keep_output(output.levels);
            keep_output(output.sources);
            keep_output(output.omm_indices);
        }),
        "omm_rasterize" => {
            if caller {
                timed_iterations(iterations, |workspace| {
                    let coords = [
                        case.uvs[indices[0] as usize],
                        case.uvs[indices[1] as usize],
                        case.uvs[indices[2] as usize],
                    ];

                    black_box(
                        opacity_map_rasterize_into(
                            &mut out_bytes,
                            (seed % 4) as u8,
                            if seed & 1 == 0 { 2 } else { 4 },
                            coords,
                            &case.texture,
                            1,
                            8,
                            8,
                            8,
                            workspace,
                        )
                        .unwrap(),
                    );
                    black_box(out_bytes.as_ptr());
                })
            } else {
                timed_iterations(iterations, |workspace| {
                    let coords = [
                        case.uvs[indices[0] as usize],
                        case.uvs[indices[1] as usize],
                        case.uvs[indices[2] as usize],
                    ];

                    keep_output(
                        opacity_map_rasterize(
                            (seed % 4) as u8,
                            if seed & 1 == 0 { 2 } else { 4 },
                            coords,
                            &case.texture,
                            1,
                            8,
                            8,
                            8,
                            workspace,
                        )
                        .unwrap(),
                    );
                })
            }
        }
        "omm_compact" => timed_iterations(iterations, |workspace| {
            let states = if seed & 1 == 0 { 2 } else { 4 };
            let mut levels = [0, 1, 2, 3];
            let (mut data, mut offsets) = compact_batch_data(&case.texture, states);
            let mut omm = [0i32, 1, 2, 3, 0, 2];
            black_box(
                opacity_map_compact(
                    &mut data,
                    &mut levels,
                    &mut offsets,
                    &mut omm,
                    states,
                    workspace,
                )
                .unwrap(),
            );
            black_box(data.as_ptr());
            black_box(levels.as_ptr());
            black_box(offsets.as_ptr());
            black_box(omm.as_ptr());
        }),
        "tangents" => {
            if caller {
                timed_iterations(iterations, |workspace| {
                    generate_tangents_into(
                        &mut out_f32,
                        Some(indices),
                        indices.len(),
                        positions,
                        &case.normals,
                        &case.uvs,
                        seed % 4,
                        workspace,
                    )
                    .unwrap();
                    black_box(out_f32.as_ptr());
                })
            } else {
                timed_iterations(iterations, |workspace| {
                    keep_output(
                        generate_tangents(
                            Some(indices),
                            indices.len(),
                            positions,
                            &case.normals,
                            &case.uvs,
                            seed % 4,
                            workspace,
                        )
                        .unwrap(),
                    );
                })
            }
        }
        "normals" => {
            if caller {
                timed_iterations(iterations, |workspace| {
                    generate_normals_into(
                        &mut out_norm,
                        Some(indices),
                        indices.len(),
                        positions,
                        [0.5, 1.0, 2.0, 3.0][(seed % 4) as usize],
                        (seed % 4) as f32 * 0.5,
                        workspace,
                    )
                    .unwrap();
                    black_box(out_norm.as_ptr());
                })
            } else {
                timed_iterations(iterations, |workspace| {
                    keep_output(
                        generate_normals(
                            Some(indices),
                            indices.len(),
                            positions,
                            [0.5, 1.0, 2.0, 3.0][(seed % 4) as usize],
                            (seed % 4) as f32 * 0.5,
                            workspace,
                        )
                        .unwrap(),
                    );
                })
            }
        }
        "remesh" => {
            if caller {
                timed_iterations(iterations, |workspace| {
                    let resolution = (4 + seed % 5) as usize;

                    black_box(
                        remesh_into(
                            &mut out_remesh,
                            indices,
                            positions,
                            resolution,
                            seed % 4,
                            workspace,
                        )
                        .unwrap(),
                    );
                    black_box(out_remesh.as_ptr());
                })
            } else {
                timed_iterations(iterations, |workspace| {
                    let resolution = (4 + seed % 5) as usize;

                    keep_output(
                        remesh(indices, positions, resolution, seed % 4, workspace).unwrap(),
                    );
                })
            }
        }
        _ => panic!("unknown benchmark family"),
    }
}

// Dispatch once before timing, as in the C++ driver. Each closure is a
// statically selected API; allocation and workspace creation stay timed.
#[inline(never)]
fn timed_iterations(iterations: usize, mut call: impl FnMut(&mut Workspace)) -> (f64, usize) {
    let start = std::time::Instant::now();
    let mut peak_bytes = 0;
    for _ in 0..iterations {
        let mut workspace = Workspace::default();
        call(&mut workspace);
        peak_bytes = peak_bytes.max(workspace.usage().bytes);
    }
    (
        start.elapsed().as_nanos() as f64 / iterations as f64,
        peak_bytes,
    )
}

// Retain the actual output bytes and count without making allocation metadata
// opaque. C++ uses the same pointer/scalar policy before its vector destructor.
#[inline(always)]
fn keep_output<T>(output: Vec<T>) {
    std::hint::black_box(output.as_ptr());
    std::hint::black_box(output.len());
}
