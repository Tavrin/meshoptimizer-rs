//! Freeze actual baseline meshlet topology, outside all measured regions.
use meshoptimizer_rs as m;
use std::io::{Read, Write};
fn word(r: &mut impl Read) -> usize {
    let mut b = [0; 4];
    r.read_exact(&mut b).unwrap();
    u32::from_le_bytes(b) as usize
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut input = std::io::BufReader::new(std::fs::File::open(&args[1]).unwrap());
    let n = word(&mut input);
    let ni = word(&mut input);
    let p: Vec<[f32; 3]> = (0..n)
        .map(|_| core::array::from_fn(|_| f32::from_bits(word(&mut input) as u32)))
        .collect();
    let i: Vec<u32> = (0..ni).map(|_| word(&mut input) as u32).collect();
    let mut w = m::Workspace::new(m::Limits {max_bytes: 3usize << 30, ..Default::default()});
    let mesh = m::build_meshlets(&i, m::Positions::from_packed(&p), m::MeshletSettings {max_vertices: 64, max_triangles: 124}, 0., &mut w).unwrap();
    let mut out = std::io::BufWriter::new(std::fs::File::create(&args[2]).unwrap());
    out.write_all(&(mesh.meshlets.len() as u32).to_le_bytes()).unwrap();
    for m in &mesh.meshlets {
        let vertices = &mesh.vertices[m.vertex_offset as usize..][..m.vertex_count as usize];
        let triangles = &mesh.triangles[m.triangle_offset as usize..][..m.triangle_count as usize * 3];
        out.write_all(&m.vertex_count.to_le_bytes()).unwrap();
        out.write_all(&(triangles.len() as u32).to_le_bytes()).unwrap();
        for v in vertices {out.write_all(&v.to_le_bytes()).unwrap();}
        out.write_all(triangles).unwrap();
    }
    out.flush().unwrap();
    out.get_ref().sync_all().unwrap();
    println!("{} meshlets", mesh.meshlets.len());
}
