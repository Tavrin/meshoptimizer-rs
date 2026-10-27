use meshoptimizer_rs::{clusterlod, Attributes, Limits, Workspace};
use std::io::{self, BufRead};

fn run(limit: usize) -> (&'static str, usize) {
    let mut positions = vec![[0.0, 0.0, 0.0]; 2_048];
    positions[1] = [1.0, 0.0, 0.0];
    positions[2] = [0.0, 1.0, 0.0];
    let attributes = vec![0.25_f32; positions.len() * 32];
    let weights = [0.5_f32; 32];
    let view = Attributes::from_interleaved(&attributes, positions.len(), 32, 32, 0).unwrap();
    let mut workspace = Workspace::new(Limits {
        max_bytes: limit,
        max_work: Limits::default().max_work,
    });
    let result = clusterlod::build_with_output(
        clusterlod::default_config(128).unwrap(),
        clusterlod::Mesh {
            indices: &[0, 1, 2],
            positions: &mut positions,
            attributes: Some(view),
            vertex_lock: None,
            attribute_weights: &weights,
            attribute_protect_mask: 0,
        },
        |_, _| Ok(0),
        &mut workspace,
    );
    let kind = match result {
        Ok(()) => "Ok",
        Err(meshoptimizer_rs::Error::LimitExceeded) => "LimitExceeded",
        Err(error) => panic!("unexpected error: {error:?}"),
    };
    (kind, workspace.usage().bytes)
}

fn main() {
    for line in io::stdin().lock().lines() {
        let limit = line.unwrap().parse().unwrap();
        let (kind, peak) = run(limit);
        println!("{kind} {peak}");
    }
}
