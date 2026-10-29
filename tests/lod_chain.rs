use meshoptimizer_rs::*;

fn grid(n: usize) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut p = Vec::new();
    let mut indices = Vec::new();
    for y in 0..=n {
        for x in 0..=n {
            p.push([x as f32, y as f32, ((x * 7 + y * 11) % 31) as f32 / 32.0]);
        }
    }
    for y in 0..n {
        for x in 0..n {
            let a = (y * (n + 1) + x) as u32;
            let b = a + n as u32 + 1;
            indices.extend_from_slice(&[a, a + 1, b, a + 1, b + 1, b]);
        }
    }
    (p, indices)
}

#[test]
fn direct_identity_and_attributes() {
    for n in [2, 5, 16] {
        for independent in [false, true] {
            for attr in [false, true] {
                let (p, indices) = grid(n);
                let values: Vec<f32> = p.iter().map(|v| v[2]).collect();
                let attributes = Attributes::from_interleaved(&values, p.len(), 1, 1, 0).unwrap();
                let targets = [indices.len() / 2, indices.len() / 4, 0];
                let options = LodChainOptions {
                    targets: LodTargets::Counts(&targets),
                    errors: LodErrors::PerLevel(&[1., 0.5, 0.2]),
                    simplify_options: SimplifyOptions::EMPTY,
                    attributes: attr.then_some(LodChainAttributes {
                        attributes,
                        weights: &[1.],
                        vertex_flags: None,
                    }),
                    independent,
                };
                let pos = Positions::from_packed(&p);
                let chain =
                    build_lod_chain(&indices, pos, &options, &mut Workspace::default()).unwrap();
                let mut ws = Workspace::default();
                let mut previous = indices.as_slice();
                for (i, level) in chain.levels.iter().enumerate() {
                    let input = if independent {
                        indices.as_slice()
                    } else {
                        previous
                    };
                    let settings = SimplifySettings {
                        target_index_count: targets[i].min(input.len()),
                        target_error: [1., 0.5, 0.2][i],
                        options: SimplifyOptions::EMPTY,
                    };
                    let expected = if attr {
                        simplify_with_attributes(
                            input,
                            pos,
                            attributes,
                            &[1.],
                            None,
                            settings,
                            &mut ws,
                        )
                    } else {
                        simplify(input, pos, settings, &mut ws)
                    }
                    .unwrap();
                    assert_eq!(level.indices, expected.indices);
                    assert_eq!(level.relative_error.to_bits(), expected.error.to_bits());
                    assert_eq!(level.triangle_count, level.indices.len() / 3);
                    assert_eq!(level.target_reached, level.indices.len() <= targets[i]);
                    previous = &level.indices;
                }
            }
        }
    }
}

#[test]
fn limits_invalid_and_error_cap() {
    let (p, indices) = grid(8);
    let pos = Positions::from_packed(&p);
    let mut options = LodChainOptions {
        targets: LodTargets::Ratios(&[0.5, 0.25]),
        errors: LodErrors::Uniform(0.),
        simplify_options: SimplifyOptions::EMPTY,
        attributes: None,
        independent: false,
    };
    let chain = build_lod_chain(&indices, pos, &options, &mut Workspace::default()).unwrap();
    assert!(!chain.levels[0].target_reached);
    assert!(chain.levels[0].stopped_by_error);
    assert!(chain.levels[1].indices.len() <= chain.levels[0].indices.len());
    options.targets = LodTargets::Counts(&[3, 3]);
    assert!(matches!(
        build_lod_chain(&indices, pos, &options, &mut Workspace::default()),
        Err(Error::InvalidParameter)
    ));
    options.targets = LodTargets::Ratios(&[0.5, f32::NAN]);
    assert!(matches!(
        build_lod_chain(&indices, pos, &options, &mut Workspace::default()),
        Err(Error::InvalidParameter)
    ));
    options.targets = LodTargets::Counts(&[3]);
    options.errors = LodErrors::PerLevel(&[]);
    assert!(matches!(
        build_lod_chain(&indices, pos, &options, &mut Workspace::default()),
        Err(Error::InvalidParameter)
    ));
    options.errors = LodErrors::Uniform(1.);
    let mut ws = Workspace::new(Limits {
        max_bytes: 1,
        max_work: 100000,
    });
    assert!(matches!(
        build_lod_chain(&indices, pos, &options, &mut ws),
        Err(Error::LimitExceeded)
    ));
    let mut ws = Workspace::new(Limits {
        max_bytes: 10000000,
        max_work: 1,
    });
    assert!(matches!(
        build_lod_chain(&indices, pos, &options, &mut ws),
        Err(Error::LimitExceeded)
    ));
}

#[test]
#[ignore = "requires corpus in MESHOPT_LOD_CORPUS"]
fn corpus_identity() {
    let dir = std::env::var_os("MESHOPT_LOD_CORPUS").unwrap();
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|v| v.unwrap().path())
        .collect();
    paths.sort();
    assert!(!paths.is_empty());
    for path in paths {
        let raw = std::fs::read(&path).unwrap();
        let nv = u32::from_le_bytes(raw[0..4].try_into().unwrap()) as usize;
        let ni = u32::from_le_bytes(raw[4..8].try_into().unwrap()) as usize;
        let words: Vec<u32> = raw[8..]
            .as_chunks::<4>()
            .0
            .iter()
            .map(|b| u32::from_le_bytes(*b))
            .collect();
        let p: Vec<[f32; 3]> = words[..nv * 3]
            .as_chunks::<3>()
            .0
            .iter()
            .map(|v| {
                [
                    f32::from_bits(v[0]),
                    f32::from_bits(v[1]),
                    f32::from_bits(v[2]),
                ]
            })
            .collect();
        let indices = &words[nv * 3..nv * 3 + ni];
        let targets = [ni / 2, ni / 4, 0];
        if targets[0] == targets[1] || targets[1] == 0 {
            continue;
        }
        for independent in [false, true] {
            let options = LodChainOptions {
                targets: LodTargets::Counts(&targets),
                errors: LodErrors::Uniform(1.0),
                simplify_options: SimplifyOptions::EMPTY,
                attributes: None,
                independent,
            };
            let pos = Positions::from_packed(&p);
            let chain = build_lod_chain(indices, pos, &options, &mut Workspace::default()).unwrap();
            let mut previous = indices;
            let mut ws = Workspace::default();
            for (i, level) in chain.levels.iter().enumerate() {
                let input = if independent { indices } else { previous };
                let direct = simplify(
                    input,
                    pos,
                    SimplifySettings {
                        target_index_count: targets[i].min(input.len()),
                        target_error: 1.0,
                        options: SimplifyOptions::EMPTY,
                    },
                    &mut ws,
                )
                .unwrap();
                assert_eq!(
                    level.indices,
                    direct.indices,
                    "{} level {i}",
                    path.display()
                );
                assert_eq!(level.relative_error.to_bits(), direct.error.to_bits());
                previous = &level.indices;
            }
        }
        eprintln!("PASS {}", path.display());
    }
}
