#![allow(clippy::field_reassign_with_default)]
#![cfg(feature = "clusterlod")]
use meshoptimizer_rs::{
    clusterlod::{
        self as cl,
        page::QuantGrid,
        v2::{self, BuildMesh},
    },
    Limits, Workspace,
};

fn grid(n: usize) -> (Vec<[f32; 3]>, Vec<u32>) {
    let p = (0..=n)
        .flat_map(|i| {
            (0..=n).map(move |j| [i as f32, j as f32, ((i * 17 + j * 13) % 7) as f32 * 0.1])
        })
        .collect();
    let mut idx = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let a = (i * (n + 1) + j) as u32;
            let b = a + (n + 1) as u32;
            idx.extend([a, b, a + 1, a + 1, b, b + 1]);
        }
    }
    (p, idx)
}
#[test]
fn default_floor_prices_every_step_and_preserves_legacy() {
    let (p, i) = grid(12);
    let normals = vec![[0., 0., 1.]; p.len()];
    let mut c = cl::default_config(32).unwrap();
    c.optimize_clusters = false;
    c.partition_size = 2;
    let mut mesh = BuildMesh::new(&i, &p);
    mesh.normals = Some(&normals);
    let built = v2::build(
        c,
        mesh,
        QuantGrid::new(-10).unwrap(),
        &mut Workspace::default(),
    )
    .unwrap();
    let mut legacy_positions = p.clone();
    let before = cl::build(
        c,
        cl::Mesh {
            indices: &i,
            positions: &mut legacy_positions,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        &mut Workspace::default(),
    )
    .unwrap();
    let mut fresh = p.clone();
    let after = cl::build(
        c,
        cl::Mesh {
            indices: &i,
            positions: &mut fresh,
            attributes: None,
            vertex_lock: None,
            attribute_weights: &[],
            attribute_protect_mask: 0,
        },
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(before, after);
    assert_eq!(built.attribute_stride, 3);
    #[cfg(feature = "parallel")]
    {
        let parallel = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap()
            .install(|| {
                let mut input = BuildMesh::new(&i, &p);
                input.normals = Some(&normals);
                v2::build(
                    c,
                    input,
                    QuantGrid::new(-10).unwrap(),
                    &mut Workspace::new(Limits {
                        max_bytes: usize::MAX,
                        max_work: u64::MAX,
                    }),
                )
                .unwrap()
            });
        assert_eq!(parallel.indices, built.indices);
        assert_eq!(parallel.positions, built.positions);
        assert_eq!(
            parallel
                .groups
                .iter()
                .map(|g| g.error.to_bits())
                .collect::<Vec<_>>(),
            built
                .groups
                .iter()
                .map(|g| g.error.to_bits())
                .collect::<Vec<_>>()
        );
    }

    assert!(
        built.groups.iter().filter(|g| g.depth == 0).count() > 1,
        "fixture must prepare multiple groups in parallel"
    );
    let mut measured_groups = 0;
    for (g, group) in built.groups.iter().enumerate() {
        let mut input = Vec::new();
        let mut output = Vec::new();
        for c in &built.clusters {
            let t =
                &built.indices[c.first_index as usize..(c.first_index + c.index_count) as usize];
            if c.group as usize == g {
                input.extend_from_slice(t);
            }
            if c.refined == g as i32 {
                output.extend_from_slice(t);
            }
        }
        if group.error == f32::MAX {
            continue;
        }
        measured_groups += 1;
        let mut area = 0f64;
        for t in input.as_chunks::<3>().0 {
            let [a, b, c] = t.map(|v| built.positions[v as usize].map(f64::from));
            let u = [0, 1, 2].map(|k| b[k] - a[k]);
            let v = [0, 1, 2].map(|k| c[k] - a[k]);
            let n = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            area += n.iter().map(|x| x * x).sum::<f64>().sqrt() * 0.5;
        }
        let mut opts = meshoptimizer_rs::LodQualityOptions::default();
        opts.samples_per_unit_area = 4000.0 / area.max(1e-12);
        let q = meshoptimizer_rs::measure_lod_quality(
            meshoptimizer_rs::LodQualityMesh::new(
                &input,
                meshoptimizer_rs::Positions::from_packed(&built.positions),
            ),
            meshoptimizer_rs::LodQualityMesh::new(
                &output,
                meshoptimizer_rs::Positions::from_packed(&built.positions),
            ),
            &opts,
            &mut Workspace::default(),
        )
        .unwrap();
        assert!(f64::from(group.error) >= q.symmetric.max);
        for c in &built.clusters {
            if c.group as usize == g && c.refined >= 0 {
                assert!(group.error >= built.groups[c.refined as usize].error);
            }
        }
    }
    assert!(
        measured_groups > 0,
        "fixture must exercise an accepted measured step"
    );
}
#[test]
fn invalid_inputs_and_limits_fail_closed() {
    let (p, i) = grid(2);
    let c = cl::default_config(8).unwrap();
    let grid = QuantGrid::new(-10).unwrap();
    let mut m = BuildMesh::new(&i, &p);
    m.normals = Some(&[]);
    assert!(v2::build(c, m, grid, &mut Workspace::default()).is_err());
    let mut m = BuildMesh::new(&i, &p);
    m.materials = Some(&[1]);
    assert!(v2::build(c, m, grid, &mut Workspace::default()).is_err());
    let mut ws = Workspace::new(Limits {
        max_bytes: 64,
        max_work: 64,
    });
    assert!(v2::build(c, BuildMesh::new(&i, &p), grid, &mut ws).is_err());
    let mut c = c;
    c.simplify_dilate_borders = true;
    assert!(v2::build(c, BuildMesh::new(&i, &p), grid, &mut Workspace::default()).is_err());
}
