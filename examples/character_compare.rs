//! Reproduce character comparisons using a packed little-endian input file.
//! Header: vertex/index/bone/blend/pose counts (five u32s); positions (3f),
//! indices (u32), joints (4u32), weights (4f), blend deltas (3f), pose matrices (3x4f).
use meshoptimizer_rs::*;
use std::{error::Error as StdError, time::Instant};
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn u(&mut self) -> u32 {
        let (v, rest) = self.0.split_at(4);
        self.0 = rest;
        u32::from_le_bytes(v.try_into().expect("four bytes"))
    }
    fn f(&mut self) -> f32 {
        f32::from_bits(self.u())
    }
}
fn main() -> Result<(), Box<dyn StdError>> {
    let path = std::env::args().nth(1).ok_or("input file required")?;
    let bytes = std::fs::read(&path)?;
    let mut r = Reader(&bytes);
    let [n, ni, nb, ns, np] = core::array::from_fn::<_, 5, _>(|_| r.u() as usize);
    let p: Vec<[f32; 3]> = (0..n).map(|_| core::array::from_fn(|_| r.f())).collect();
    let indices: Vec<u32> = (0..ni).map(|_| r.u()).collect();
    let joints: Vec<u32> = (0..n * 4).map(|_| r.u()).collect();
    let weights: Vec<f32> = (0..n * 4).map(|_| r.f()).collect();
    let shapes: Vec<Vec<[f32; 3]>> = (0..ns)
        .map(|_| (0..n).map(|_| core::array::from_fn(|_| r.f())).collect())
        .collect();
    let poses: Vec<Vec<[[f32; 4]; 3]>> = (0..np)
        .map(|_| {
            (0..nb)
                .map(|_| core::array::from_fn(|_| core::array::from_fn(|_| r.f())))
                .collect()
        })
        .collect();
    assert!(r.0.is_empty());
    let importance = vec![1.; nb];
    let mut data = CharacterInputs::new();
    if nb > 0 {
        data.skin = Some(CharacterSkin::new(&joints, &weights, 4, &importance));
    }
    let shapes_ref: Vec<_> = shapes.iter().map(Vec::as_slice).collect();
    let poses_ref: Vec<_> = poses.iter().map(Vec::as_slice).collect();
    data.blend_shapes = &shapes_ref;
    data.joint_poses = &poses_ref;
    let mut samples = vec![p.clone()];
    for s in &shapes {
        samples.push(
            p.iter()
                .zip(s)
                .map(|(p, s)| core::array::from_fn(|k| p[k] + s[k]))
                .collect(),
        );
    }
    for pose in &poses {
        samples.push(
            (0..n)
                .map(|i| {
                    core::array::from_fn(|k| {
                        (0..4)
                            .map(|j| {
                                let m = pose[joints[i * 4 + j] as usize][k];
                                weights[i * 4 + j]
                                    * (m[0] * p[i][0] + m[1] * p[i][1] + m[2] * p[i][2] + m[3])
                            })
                            .sum()
                    })
                })
                .collect(),
        );
    }
    let mut ws = Workspace::default();
    println!("ratio,method,triangles,bind,max_pose,milliseconds");
    for ratio in [0.5, 0.25, 0.1] {
        let settings = SimplifySettings {
            target_index_count: ((ni / 3) as f64 * ratio) as usize * 3,
            target_error: 100.,
            options: SimplifyOptions::EMPTY,
        };
        for (method, blend_weight, skin_weight) in [
            ("plain", 0., 0.),
            ("character", 1., 1.),
            ("skin_only", 0., 1.),
        ] {
            let mut options = CharacterOptions::default();
            options.blend_shape_weight = blend_weight;
            options.skin_weight = skin_weight;
            let mut timings = Vec::new();
            let mut output = None;
            for repeat in 0..8 {
                let start = Instant::now();
                let mesh = if method == "plain" {
                    simplify(&indices, Positions::from_packed(&p), settings, &mut ws)
                        .map_err(|e| e.to_string())?
                } else {
                    simplify_character(
                        &indices,
                        Positions::from_packed(&p),
                        &data,
                        &options,
                        settings,
                        &mut ws,
                    )
                    .map_err(|e| e.to_string())?
                };
                if repeat > 0 {
                    timings.push(start.elapsed().as_secs_f64() * 1000.);
                }
                if let Some(previous) = &output {
                    let previous: &SimplifiedMesh = previous;
                    assert_eq!(mesh.indices, previous.indices);
                    assert_eq!(mesh.error.to_bits(), previous.error.to_bits());
                }
                output = Some(mesh);
            }
            timings.sort_by(f64::total_cmp);
            let ms = timings[timings.len() / 2];
            let mesh = output.ok_or("missing output")?;
            let mut maxima = Vec::new();
            let mut qo = LodQualityOptions::default();
            qo.samples_per_unit_area = 0.;
            for sample in &samples {
                let p = Positions::from_packed(sample);
                maxima.push(
                    measure_lod_quality(
                        LodQualityMesh::new(&indices, p),
                        LodQualityMesh::new(&mesh.indices, p),
                        &qo,
                        &mut ws,
                    )
                    .map_err(|e| e.to_string())?
                    .symmetric
                    .max,
                );
            }
            println!(
                "{ratio},{method},{},{:.9},{:.9},{ms:.3}",
                mesh.indices.len() / 3,
                maxima[0],
                maxima.iter().copied().fold(0., f64::max)
            );
        }
    }
    // Exercise chain pose evaluation on the same public model as well.
    let co = LodChainOptions::new(
        LodTargets::Ratios(&[0.5, 0.25, 0.1]),
        LodErrors::Uniform(100.),
    )
    .with_independent(true);
    let mut qo = LodQualityOptions::default();
    qo.samples_per_unit_area = 0.;
    let chain = build_character_lod_chain(
        &indices,
        Positions::from_packed(&p),
        &data,
        &CharacterOptions::default(),
        &co,
        &qo,
        &mut ws,
    )
    .map_err(|e| e.to_string())?;
    assert_eq!(chain.quality.len(), samples.len());
    Ok(())
}
