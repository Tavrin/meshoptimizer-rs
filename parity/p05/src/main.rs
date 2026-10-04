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
fn case_with(seed: u32, shape: Option<(u32, u32)>) -> Case {
    let mut rng = Rng(seed + 1);
    let natural_count = 3 + rng.next() % 6;
    let natural_triangles = 1 + rng.next() % 5;
    let (count, triangles) = shape.unwrap_or((natural_count, natural_triangles));
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for _ in 0..count {
        positions.push([0; 3].map(|_| ((rng.next() % 17) as i32 - 8) as f32 / 8.0));
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0; 2].map(|_| ((rng.next() % 17) as i32 - 4) as f32 / 8.0));
    }
    let indices = (0..triangles * 3).map(|_| rng.next() % count).collect();
    let texture = (0..64).map(|_| rng.next() as u8).collect();
    Case { positions, normals, uvs, indices, texture }
}
fn push_u32(out: &mut Vec<u8>, value: u32) { out.extend_from_slice(&value.to_le_bytes()); }
fn push_f32(out: &mut Vec<u8>, value: f32) { push_u32(out, value.to_bits()); }
fn push_u64(out: &mut Vec<u8>, value: usize) { out.extend_from_slice(&(value as u64).to_le_bytes()); }
fn input_hash(case: &Case) -> u64 {
    let mut bytes = Vec::new();
    push_u32(&mut bytes, case.positions.len() as u32);
    push_u32(&mut bytes, case.indices.len() as u32);
    for p in &case.positions { for &v in p { push_f32(&mut bytes, v); } }
    for n in &case.normals { for &v in n { push_f32(&mut bytes, v); } }
    for uv in &case.uvs { for &v in uv { push_f32(&mut bytes, v); } }
    for &v in &case.indices { push_u32(&mut bytes, v); }
    bytes.extend_from_slice(&case.texture);
    bytes.into_iter().fold(14695981039346656037u64, |h, b| (h ^ u64::from(b)).wrapping_mul(1099511628211))
}
fn run(family: &str, seed: u32, case: &Case) -> Result<Vec<u8>, Error> {
    let mut workspace = Workspace::default();
    let mut out = Vec::new();
    let positions = Positions::from_packed(&case.positions);
    let restart = if seed & 1 == 0 { 0 } else { 65535 };
    match family {
        "stripify" => for value in stripify(&case.indices, case.positions.len(), restart, &mut workspace)? { push_u32(&mut out, value); },
        "stripify_bound" => push_u64(&mut out, stripify_bound(case.indices.len())?),
        "unstripify" | "unstripify_bound" => {
            let strip = stripify(&case.indices, case.positions.len(), restart, &mut workspace)?;
            if family == "unstripify" { for value in unstripify(&strip, restart, &mut workspace)? { push_u32(&mut out, value); } }
            else { push_u64(&mut out, unstripify_bound(strip.len())?); }
        }
        "vertex_cache" => {
            let s = analyze_vertex_cache(&case.indices, case.positions.len(), 3 + seed % 24, if seed & 1 == 0 { 0 } else { 16 }, seed % 4, &mut workspace)?;
            push_u32(&mut out, s.vertices_transformed); push_u32(&mut out, s.warps_executed); push_f32(&mut out, s.acmr); push_f32(&mut out, s.atvr);
        }
        "vertex_fetch" => { let s = analyze_vertex_fetch(&case.indices, case.positions.len(), (4 + seed % 64) as usize, &mut workspace)?; push_u32(&mut out, s.bytes_fetched); push_f32(&mut out, s.overfetch); }
        "overdraw" => { let s = analyze_overdraw(&case.indices, positions, &mut workspace)?; push_u32(&mut out, s.pixels_covered); push_u32(&mut out, s.pixels_shaded); push_f32(&mut out, s.overdraw); }
        "coverage" => { let s = analyze_coverage(&case.indices, positions, &mut workspace)?; for value in s.coverage { push_f32(&mut out, value); } push_f32(&mut out, s.extent); }
        "omm_measure" => {
            let uvs: Vec<f32> = case.uvs.iter().flatten().copied().collect();
            let result = opacity_map_measure(&case.indices, &uvs, case.positions.len(), 2, 8, 8, (seed % 5) as u8, if seed & 1 == 0 { 0.0 } else { 2.0 }, &mut workspace)?;
            push_u32(&mut out, result.levels.len() as u32);
            out.extend_from_slice(&result.levels);
            for value in result.sources { push_u32(&mut out, value); }
            for value in result.omm_indices { out.extend_from_slice(&value.to_le_bytes()); }
        }
        "omm_rasterize" => {
            let data = opacity_map_rasterize((seed % 4) as u8, if seed & 1 == 0 { 2 } else { 4 }, [case.uvs[case.indices[0] as usize], case.uvs[case.indices[1] as usize], case.uvs[case.indices[2] as usize]], &case.texture, 1, 8, 8, 8, &mut workspace)?;
            out.extend_from_slice(&data);
        }
        "omm_entry_size" => push_u64(&mut out, opacity_map_entry_size((seed % 13) as u8, if seed & 1 == 0 { 2 } else { 4 })?),
        "omm_compact" => {
            let states = if seed & 1 == 0 { 2 } else { 4 };
            let mut levels = [0, 1, 2, 3]; let mut offsets = [0u32; 4]; let mut data = Vec::new();
            for i in 0..4 { offsets[i] = data.len() as u32; let size = opacity_map_entry_size(levels[i], states)?; for j in 0..size { data.push(case.texture[(i * 13 + j) % 64]); } }
            let mut indices = [0i32, 1, 2, 3, 0, 2];
            let (count, size) = opacity_map_compact(&mut data, &mut levels, &mut offsets, &mut indices, states, &mut workspace)?;
            push_u64(&mut out, count); push_u64(&mut out, size); out.extend_from_slice(&data); out.extend_from_slice(&levels);
            for value in offsets { push_u32(&mut out, value); }
            for value in indices { out.extend_from_slice(&value.to_le_bytes()); }
        }
        "tangents" => { for t in generate_tangents(Some(&case.indices), case.indices.len(), positions, &case.normals, &case.uvs, seed % 4, &mut workspace)? { for v in t { push_f32(&mut out, v); } } }
        "normals" => { for n in generate_normals(Some(&case.indices), case.indices.len(), positions, [0.5, 1.0, 2.0, 3.0][(seed % 4) as usize], (seed % 4) as f32 * 0.5, &mut workspace)? { for v in n { push_f32(&mut out, v); } } }
        "remesh" => {
            let resolution = (4 + seed % 5) as usize;
            let bound = remesh_bound(&case.indices, positions, resolution, seed % 4, &mut workspace)?;
            let mesh = remesh(&case.indices, positions, resolution, seed % 4, &mut workspace)?;
            push_u64(&mut out, bound); push_u64(&mut out, mesh.len() / 3);
            for p in mesh { for value in p { push_f32(&mut out, value); } }
        }
        _ => return Err(Error::InvalidParameter),
    }
    Ok(out)
}
fn main() {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line.expect("stdin");
        let mut parts = line.split_ascii_whitespace();
        let Some(family) = parts.next() else { continue };
        if family == "BENCH" {
            let operation = parts.next().expect("benchmark operation");
            let seed: u32 = parts.next().expect("seed").parse().expect("numeric seed");
            let count: u32 = parts.next().expect("vertices").parse().expect("numeric count");
            let triangles: u32 = parts.next().expect("triangles").parse().expect("numeric triangles");
            let iterations: usize = parts.next().expect("iterations").parse().expect("numeric iterations");
            let caller = parts.next().expect("mode") == "caller";
            let case = case_with(seed, Some((count, triangles)));
            let ns = benchmark(operation, seed, &case, iterations, caller);
            writeln!(stdout, "{:016x} {:.6}", input_hash(&case), ns).expect("stdout");
            stdout.flush().expect("stdout");
            continue;
        }
        let seed: u32 = parts.next().expect("seed").parse().expect("numeric seed");
        let shape = match (parts.next(), parts.next()) {
            (Some(a), Some(b)) => Some((a.parse().expect("vertex count"), b.parse().expect("triangle count"))),
            _ => None,
        };
        let case = case_with(seed, shape);
        let result = run(family, seed, &case).expect("valid seeded case");
        write!(stdout, "{:016x} ", input_hash(&case)).expect("stdout");
        for byte in result { write!(stdout, "{byte:02x}").expect("stdout"); }
        writeln!(stdout).expect("stdout");
        stdout.flush().expect("stdout");
    }
}

fn benchmark(family: &str, seed: u32, case: &Case, iterations: usize, caller: bool) -> f64 {
    use std::hint::black_box;
    use std::time::Instant;
    let indices = &case.indices;
    let positions = Positions::from_packed(&case.positions);
    let restart = if seed & 1 == 0 { 0 } else { 65535 };
    let strip = if family == "unstripify" { Some(stripify(indices, case.positions.len(), restart, &mut Workspace::default()).unwrap()) } else { None };
    let uvs: Vec<f32> = case.uvs.iter().flatten().copied().collect();
    let mut out_u32 = vec![0u32; stripify_bound(indices.len()).unwrap().max(unstripify_bound(indices.len() * 5 / 3).unwrap_or(0)) + 16];
    let mut out_f32 = vec![[0f32; 4]; indices.len()];
    let mut out_norm = vec![[0f32; 3]; indices.len()];
    let mut out_bytes = vec![0u8; 1 << 16];
    let mut out_remesh = if family == "remesh" && caller {
        let bound = remesh_bound(indices, positions, (4 + seed % 5) as usize, seed % 4, &mut Workspace::default()).unwrap();
        vec![[0f32; 3]; bound * 3]
    } else { Vec::new() };
    let start = Instant::now();
    for _ in 0..iterations {
        let mut workspace = Workspace::default();
        match family {
            "stripify" => if caller { black_box(stripify_into(&mut out_u32, indices, case.positions.len(), restart, &mut workspace).unwrap()); } else { black_box(stripify(indices, case.positions.len(), restart, &mut workspace).unwrap()); },
            "stripify_bound" => { black_box(stripify_bound(indices.len()).unwrap()); },
            "unstripify" => { let strip = strip.as_ref().unwrap(); if caller { black_box(unstripify_into(&mut out_u32, strip, restart, &mut workspace).unwrap()); } else { black_box(unstripify(strip, restart, &mut workspace).unwrap()); } },
            "unstripify_bound" => { black_box(unstripify_bound(indices.len()).unwrap()); },
            "vertex_cache" => { black_box(analyze_vertex_cache(indices, case.positions.len(), 3 + seed % 24, if seed & 1 == 0 { 0 } else { 16 }, seed % 4, &mut workspace).unwrap()); },
            "vertex_fetch" => { black_box(analyze_vertex_fetch(indices, case.positions.len(), (4 + seed % 64) as usize, &mut workspace).unwrap()); },
            "overdraw" => { black_box(analyze_overdraw(indices, positions, &mut workspace).unwrap()); },
            "coverage" => { black_box(analyze_coverage(indices, positions, &mut workspace).unwrap()); },
            "omm_measure" => { black_box(opacity_map_measure(indices, &uvs, case.positions.len(), 2, 8, 8, (seed % 5) as u8, if seed & 1 == 0 { 0.0 } else { 2.0 }, &mut workspace).unwrap()); },
            "omm_rasterize" => { let coords = [case.uvs[indices[0] as usize], case.uvs[indices[1] as usize], case.uvs[indices[2] as usize]]; if caller { black_box(opacity_map_rasterize_into(&mut out_bytes, (seed % 4) as u8, if seed & 1 == 0 { 2 } else { 4 }, coords, &case.texture, 1, 8, 8, 8, &mut workspace).unwrap()); } else { black_box(opacity_map_rasterize((seed % 4) as u8, if seed & 1 == 0 { 2 } else { 4 }, coords, &case.texture, 1, 8, 8, 8, &mut workspace).unwrap()); } },
            "omm_entry_size" => { black_box(opacity_map_entry_size((seed % 13) as u8, if seed & 1 == 0 { 2 } else { 4 }).unwrap()); },
            "omm_compact" => {
                let states = if seed & 1 == 0 { 2 } else { 4 };
                let mut levels = [0, 1, 2, 3]; let mut offsets = [0u32; 4]; let mut data = Vec::new();
                for i in 0..4 { offsets[i] = data.len() as u32; let size = opacity_map_entry_size(levels[i], states).unwrap(); for j in 0..size { data.push(case.texture[(i * 13 + j) % 64]); } }
                let mut omm = [0i32, 1, 2, 3, 0, 2];
                black_box(opacity_map_compact(&mut data, &mut levels, &mut offsets, &mut omm, states, &mut workspace).unwrap());
            }
            "tangents" => if caller { black_box(generate_tangents_into(&mut out_f32, Some(indices), indices.len(), positions, &case.normals, &case.uvs, seed % 4, &mut workspace).unwrap()); } else { black_box(generate_tangents(Some(indices), indices.len(), positions, &case.normals, &case.uvs, seed % 4, &mut workspace).unwrap()); },
            "normals" => if caller { black_box(generate_normals_into(&mut out_norm, Some(indices), indices.len(), positions, [0.5, 1.0, 2.0, 3.0][(seed % 4) as usize], (seed % 4) as f32 * 0.5, &mut workspace).unwrap()); } else { black_box(generate_normals(Some(indices), indices.len(), positions, [0.5, 1.0, 2.0, 3.0][(seed % 4) as usize], (seed % 4) as f32 * 0.5, &mut workspace).unwrap()); },
            "remesh" => { let resolution = (4 + seed % 5) as usize; if caller { black_box(remesh_into(&mut out_remesh, indices, positions, resolution, seed % 4, &mut workspace).unwrap()); } else { black_box(remesh(indices, positions, resolution, seed % 4, &mut workspace).unwrap()); } },
            _ => panic!("unknown benchmark family"),
        }
    }
    start.elapsed().as_nanos() as f64 / iterations as f64
}
