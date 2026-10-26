use meshoptimizer_rs::{
    optimize_overdraw, optimize_vertex_cache, simplify, simplify_scale, Error, Positions,
    SimplifyOptions, SimplifySettings, Workspace,
};

fn main() -> Result<(), Error> {
    let positions = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let indices = [0, 1, 2, 2, 1, 3];
    let view = Positions::from_packed(&positions);
    let mut workspace = Workspace::default();
    let scale = simplify_scale(view)?;
    let mut current = indices.to_vec();
    for target_index_count in [6, 3, 0] {
        let settings = SimplifySettings {
            target_index_count,
            target_error: 0.01,
            options: SimplifyOptions::EMPTY,
        };
        let lod = simplify(&current, view, settings, &mut workspace)?;
        let cached = optimize_vertex_cache(&lod.indices, positions.len(), &mut workspace)?;
        current = optimize_overdraw(&cached, view, 1.05, &mut workspace)?;
        println!(
            "{} indices, absolute error {}",
            current.len(),
            lod.error * scale
        );
    }
    Ok(())
}
