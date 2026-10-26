use meshoptimizer_rs::{
    build_meshlets, compute_meshlet_bounds_with_positions, BoundsPositions, Error, MeshletSettings,
    Positions, Workspace,
};

fn main() -> Result<(), Error> {
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let indices = [0, 1, 2, 2, 1, 3];
    let view = Positions::from_packed(&positions);
    let mut workspace = Workspace::default();
    let built = build_meshlets(
        &indices,
        view,
        MeshletSettings::default(),
        0.25,
        &mut workspace,
    )?;
    let positions = BoundsPositions::new(view);
    for m in &built.meshlets {
        let v = m.vertex_offset as usize;
        let t = m.triangle_offset as usize;
        let bounds = compute_meshlet_bounds_with_positions(
            &built.vertices[v..v + m.vertex_count as usize],
            &built.triangles[t..t + m.triangle_count as usize * 3],
            positions,
            &mut workspace,
        )?;
        println!("center {:?}, radius {}", bounds.center, bounds.radius);
    }
    Ok(())
}
