use meshoptimizer_rs::*;

pub fn run(family: &str, data: &[u8]) {
    let byte = |i: usize| -> u8 { data.get(i).copied().unwrap_or(0) };
    let count = (byte(0) % 12) as usize;
    let index_count = (byte(1) % 36) as usize;
    let positions: Vec<[f32; 3]> = (0..count)
        .map(|i| [0, 1, 2].map(|j| (i32::from(byte(4 + i * 8 + j)) - 128) as f32 / 32.0))
        .collect();
    let normals = vec![[0.0, 0.0, 1.0]; count];
    let uvs: Vec<[f32; 2]> = (0..count)
        .map(|i| [0, 1].map(|j| (i32::from(byte(4 + i * 8 + 3 + j)) - 128) as f32 / 32.0))
        .collect();
    let indices: Vec<u32> = (0..index_count)
        .map(|i| u32::from(byte(4 + count * 8 + i)) % (count as u32 + 2))
        .collect();
    let texture: Vec<u8> = (0..64).map(|i| byte(4 + count * 8 + index_count + i)).collect();
    let positions = Positions::from_packed(&positions);
    let mut workspace = Workspace::default();
    let restart = if byte(2) & 1 == 0 { 0 } else { 65535 };
    match family {
        "stripify" => { let _ = stripify(&indices, count, restart, &mut workspace); }
        "stripify_bound" => { let _ = stripify_bound(indices.len()); }
        "unstripify" => { let _ = unstripify(&indices, restart, &mut workspace); }
        "unstripify_bound" => { let _ = unstripify_bound(indices.len()); }
        "vertex_cache" => { let _ = analyze_vertex_cache(&indices, count, u32::from(byte(2)), u32::from(byte(3)), u32::from(byte(4)), &mut workspace); }
        "vertex_fetch" => { let _ = analyze_vertex_fetch(&indices, count, usize::from(byte(2)), &mut workspace); }
        "overdraw" => { let _ = analyze_overdraw(&indices, positions, &mut workspace); }
        "coverage" => { let _ = analyze_coverage(&indices, positions, &mut workspace); }
        "omm_measure" => {
            let floats: Vec<f32> = uvs.iter().flatten().copied().collect();
            let _ = opacity_map_measure(&indices, &floats, count, 2, u32::from(byte(2)), u32::from(byte(3)), byte(4), f32::from(byte(5)) / 16.0, &mut workspace);
        }
        "omm_rasterize" => {
            let uv = [0, 1, 2].map(|i| uvs.get(i).copied().unwrap_or([0.0, 0.0]));
            // Keep valid smoke entries below the 16-million-microtriangle
            // worst case; the release fuzz campaign must cover large levels.
            let level = if byte(2) > 12 { byte(2) } else { byte(2) % 6 };
            let _ = opacity_map_rasterize(level, byte(3), uv, &texture, usize::from(byte(4)), usize::from(byte(5)), u32::from(byte(6)), u32::from(byte(7)), &mut workspace);
        }
        "omm_entry_size" => { let _ = opacity_map_entry_size(byte(2), byte(3)); }
        "omm_compact" => {
            let mut blob = texture;
            let mut levels = [byte(2), byte(3), byte(4), byte(5)];
            let mut offsets = [u32::from(byte(6)), u32::from(byte(7)), u32::from(byte(8)), u32::from(byte(9))];
            let mut omm = [i32::from(byte(10)) - 4, i32::from(byte(11)) - 4];
            let _ = opacity_map_compact(&mut blob, &mut levels, &mut offsets, &mut omm, byte(12), &mut workspace);
        }
        "tangents" => { let _ = generate_tangents(Some(&indices), indices.len(), positions, &normals, &uvs, u32::from(byte(2)), &mut workspace); }
        "normals" => { let _ = generate_normals(Some(&indices), indices.len(), positions, f32::from(byte(2)) / 64.0, f32::from(byte(3)) / 32.0, &mut workspace); }
        "remesh" => { let _ = remesh(&indices, positions, usize::from(byte(2)), u32::from(byte(3)), &mut workspace); }
        _ => panic!("unknown fuzz family"),
    }
}
