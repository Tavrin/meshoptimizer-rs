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

/// The direct simplifier call a builder level must equal.
fn direct(
    input: &[u32],
    pos: Positions<'_>,
    options: &LodChainOptions<'_>,
    attributes: Attributes<'_>,
    settings: SimplifySettings,
    ws: &mut Workspace,
) -> Result<SimplifiedMesh, Error> {
    match (options.attributes, options.vertex_flags) {
        (Some(a), flags) => {
            simplify_with_attributes(input, pos, a.attributes, a.weights, flags, settings, ws)
        }
        // A zero-weight component contributes nothing, so this is the flags-only call.
        (None, Some(flags)) => {
            simplify_with_attributes(input, pos, attributes, &[0.], Some(flags), settings, ws)
        }
        (None, None) => simplify(input, pos, settings, ws),
    }
}

#[test]
fn direct_identity_across_modes() {
    let caps = [1., 0.5, 0.2];
    for n in [2, 5, 16] {
        let (p, indices) = grid(n);
        let values: Vec<f32> = p.iter().map(|v| v[2]).collect();
        let attributes = Attributes::from_interleaved(&values, p.len(), 1, 1, 0).unwrap();
        let locks: Vec<VertexFlags> = (0..p.len())
            .map(|v| {
                if v % 5 == 0 {
                    VertexFlags::LOCK
                } else {
                    VertexFlags::EMPTY
                }
            })
            .collect();
        let counts = [indices.len() / 2, indices.len() / 4, 0];
        let ratios = [0.5f32, 0.25, 0.0];
        let tris = indices.len() / 3;
        let ratio_counts = ratios.map(|r| (r as f64 * tris as f64) as usize * 3);
        let pos = Positions::from_packed(&p);
        for ratio_mode in [false, true] {
            let (targets, expected_targets) = if ratio_mode {
                (LodTargets::Ratios(&ratios), ratio_counts)
            } else {
                (LodTargets::Counts(&counts), counts)
            };
            for simplify_options in [
                SimplifyOptions::EMPTY,
                SimplifyOptions::ERROR_ABSOLUTE,
                SimplifyOptions::SPARSE,
                SimplifyOptions::SPARSE | SimplifyOptions::ERROR_ABSOLUTE,
            ] {
                for independent in [false, true] {
                    for with_attributes in [false, true] {
                        for with_locks in [false, true] {
                            let mut options =
                                LodChainOptions::new(targets, LodErrors::PerLevel(&caps))
                                    .with_simplify_options(simplify_options)
                                    .with_independent(independent);
                            if with_attributes {
                                options = options
                                    .with_attributes(LodChainAttributes::new(attributes, &[1.]));
                            }
                            if with_locks {
                                options = options.with_vertex_flags(&locks);
                            }
                            let chain =
                                build_lod_chain(&indices, pos, &options, &mut Workspace::default())
                                    .unwrap();
                            let mut ws = Workspace::default();
                            let mut previous = indices.as_slice();
                            for (i, level) in chain.levels.iter().enumerate() {
                                let input = if independent {
                                    indices.as_slice()
                                } else {
                                    previous
                                };
                                let settings = SimplifySettings {
                                    target_index_count: expected_targets[i].min(input.len()),
                                    target_error: caps[i],
                                    options: simplify_options,
                                };
                                let expected =
                                    direct(input, pos, &options, attributes, settings, &mut ws)
                                        .unwrap();
                                let what = format!(
                                    "n={n} ratio={ratio_mode} {simplify_options:?} indep={independent} attr={with_attributes} lock={with_locks} level {i}"
                                );
                                assert_eq!(level.indices, expected.indices, "{what}");
                                assert_eq!(
                                    level.error.to_bits(),
                                    expected.error.to_bits(),
                                    "{what}"
                                );
                                assert_eq!(level.triangle_count, level.indices.len() / 3);
                                assert_eq!(
                                    level.target_reached,
                                    level.indices.len() <= expected_targets[i]
                                );
                                previous = &level.indices;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn locks_without_attributes_hold_vertices() {
    let (p, indices) = grid(4);
    let pos = Positions::from_packed(&p);
    let all = vec![VertexFlags::LOCK; p.len()];
    let options = LodChainOptions::new(LodTargets::Counts(&[0]), LodErrors::Uniform(10.))
        .with_vertex_flags(&all);
    let chain = build_lod_chain(&indices, pos, &options, &mut Workspace::default()).unwrap();
    assert_eq!(chain.levels[0].indices.len(), indices.len());
    assert!(!chain.levels[0].target_reached);
    let free = LodChainOptions::new(LodTargets::Counts(&[0]), LodErrors::Uniform(10.));
    let chain = build_lod_chain(&indices, pos, &free, &mut Workspace::default()).unwrap();
    assert!(chain.levels[0].indices.len() < indices.len());
    // A wrong-length flag slice is the simplifier's InvalidLayout, reported for level 0.
    let short = [VertexFlags::LOCK];
    let bad = LodChainOptions::new(LodTargets::Counts(&[0]), LodErrors::Uniform(10.))
        .with_vertex_flags(&short);
    let error = build_lod_chain(&indices, pos, &bad, &mut Workspace::default()).unwrap_err();
    assert_eq!(error.level, Some(0));
    assert!(matches!(
        error.error,
        Error::InvalidLayout | Error::InvalidParameter
    ));
}

#[test]
fn limits_invalid_and_error_cap() {
    let (p, indices) = grid(8);
    let pos = Positions::from_packed(&p);
    let mut options =
        LodChainOptions::new(LodTargets::Ratios(&[0.5, 0.25]), LodErrors::Uniform(0.));
    let chain = build_lod_chain(&indices, pos, &options, &mut Workspace::default()).unwrap();
    assert!(!chain.levels[0].target_reached);
    assert!(chain.levels[0].stopped_at_error_limit);
    assert!(chain.levels[1].indices.len() <= chain.levels[0].indices.len());
    let invalid = |options: &LodChainOptions<'_>| {
        build_lod_chain(&indices, pos, options, &mut Workspace::default()).unwrap_err()
    };
    options.targets = LodTargets::Counts(&[3, 3]);
    assert_eq!(invalid(&options).error, Error::InvalidParameter);
    options.targets = LodTargets::Ratios(&[0.5, f32::NAN]);
    assert_eq!(invalid(&options).error, Error::InvalidParameter);
    // Ratios still have to decrease strictly; only the rounded counts may coincide.
    options.targets = LodTargets::Ratios(&[0.5, 0.5]);
    assert_eq!(invalid(&options).level, Some(1));
    options.targets = LodTargets::Counts(&[3]);
    options.errors = LodErrors::PerLevel(&[]);
    assert_eq!(invalid(&options).level, None);
    options.errors = LodErrors::Uniform(1.);
    let mut ws = Workspace::new(Limits {
        max_bytes: 1,
        max_work: 100000,
    });
    assert_eq!(
        build_lod_chain(&indices, pos, &options, &mut ws)
            .unwrap_err()
            .error,
        Error::LimitExceeded
    );
}

#[test]
fn rounded_ratio_targets_can_match() {
    let p = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [1., 1., 0.]];
    let options = LodChainOptions::new(LodTargets::Ratios(&[0.75, 0.5]), LodErrors::Uniform(1.));
    let chain = build_lod_chain(
        &[0, 1, 2, 2, 1, 3],
        Positions::from_packed(&p),
        &options,
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(chain.levels.len(), 2);
    // floor(0.75 * 2) and floor(0.5 * 2) are both one triangle.
    assert!(chain.levels.iter().all(|level| level.indices.len() <= 3));
}

#[test]
fn sparse_ignores_unused_extreme_extent() {
    let p = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [f32::MAX, 0., 0.],
        [-f32::MAX, 0., 0.],
    ];
    let pos = Positions::from_packed(&p);
    let options = LodChainOptions::new(LodTargets::Counts(&[0]), LodErrors::Uniform(1.))
        .with_simplify_options(SimplifyOptions::SPARSE);
    let chain = build_lod_chain(&[0, 1, 2], pos, &options, &mut Workspace::default()).unwrap();
    let direct = simplify(
        &[0, 1, 2],
        pos,
        SimplifySettings {
            target_index_count: 0,
            target_error: 1.,
            options: SimplifyOptions::SPARSE,
        },
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(chain.levels[0].indices, direct.indices);
    assert_eq!(chain.levels[0].error.to_bits(), direct.error.to_bits());
    assert_eq!(chain.levels[0].scale, 1.);
    // The dense call rejects the same data; the builder reports the simplifier's error.
    let dense = LodChainOptions::new(LodTargets::Counts(&[0]), LodErrors::Uniform(1.));
    let error = build_lod_chain(&[0, 1, 2], pos, &dense, &mut Workspace::default()).unwrap_err();
    assert_eq!(error.level, Some(0));
    assert_eq!(error.error, Error::NumericalFailure);
}

#[test]
fn usage_covers_the_whole_call() {
    let (p, indices) = grid(8);
    let pos = Positions::from_packed(&p);
    let counts = [indices.len() / 2, indices.len() / 4];
    for sparse in [false, true] {
        let simplify_options = if sparse {
            SimplifyOptions::SPARSE
        } else {
            SimplifyOptions::EMPTY
        };
        let options = LodChainOptions::new(LodTargets::Counts(&counts), LodErrors::Uniform(1.))
            .with_simplify_options(simplify_options);
        let mut ws = Workspace::default();
        let chain = build_lod_chain(&indices, pos, &options, &mut ws).unwrap();
        let usage = ws.usage();
        // Direct calls on the same inputs, plus the builder's own scans: one
        // visit per level for validation, then one scale scan per vertex (once)
        // or per referenced index of each level's input (sparse).
        let mut expected = counts.len() as u64;
        let mut input = indices.as_slice();
        for (i, level) in chain.levels.iter().enumerate() {
            let mut fresh = Workspace::default();
            simplify(
                input,
                pos,
                SimplifySettings {
                    target_index_count: counts[i],
                    target_error: 1.,
                    options: simplify_options,
                },
                &mut fresh,
            )
            .unwrap();
            expected += fresh.usage().work;
            if sparse {
                expected += input.len() as u64;
            }
            input = &level.indices;
        }
        if !sparse {
            expected += p.len() as u64;
        }
        assert_eq!(usage.work, expected, "sparse={sparse}");
        // Metadata and both retained outputs are part of the reported peak.
        let retained: usize = chain.levels.iter().map(|l| l.indices.capacity() * 4).sum();
        assert!(
            usage.bytes >= counts.len() * core::mem::size_of::<LodLevel>() + retained,
            "bytes {}",
            usage.bytes
        );
        // The limit is exact for work: the measured total passes, one less fails.
        let limited = |max_work| {
            let mut ws = Workspace::new(Limits {
                max_bytes: 1 << 30,
                max_work,
            });
            let result = build_lod_chain(&indices, pos, &options, &mut ws);
            (result, ws.usage())
        };
        assert!(limited(expected).0.is_ok());
        let (result, _) = limited(expected - 1);
        assert_eq!(result.unwrap_err().error, Error::LimitExceeded);
        // Even a zero budget fails (the validation visits are counted) and says so.
        let (result, zero) = limited(0);
        let error = result.unwrap_err();
        assert_eq!((error.error, error.level), (Error::LimitExceeded, None));
        assert_eq!(zero.work, counts.len() as u64);
    }
}

#[test]
fn byte_limit_failure_reports_the_failing_level() {
    let (p, indices) = grid(8);
    let pos = Positions::from_packed(&p);
    // A barely reduced first level keeps the second level's scratch as large as
    // the first's, so the peak is reached while the first output is retained.
    let counts = [indices.len() - 6, indices.len() / 2];
    let options = LodChainOptions::new(LodTargets::Counts(&counts), LodErrors::Uniform(1.));
    let mut ws = Workspace::default();
    build_lod_chain(&indices, pos, &options, &mut ws).unwrap();
    let peak = ws.usage().bytes;
    let run = |max_bytes: usize| {
        let mut ws = Workspace::new(Limits {
            max_bytes,
            max_work: 1 << 30,
        });
        let result = build_lod_chain(&indices, pos, &options, &mut ws);
        (result, ws.usage())
    };
    // The workspace reclaims scratch under pressure, so the smallest limit the
    // chain passes can sit below the unpressured peak; find it.
    let (mut lo, mut hi) = (0, peak);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if run(mid).0.is_ok() {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let (result, usage) = run(lo - 1);
    let error = result.unwrap_err();
    assert_eq!(error.error, Error::LimitExceeded);
    assert_eq!(error.level, Some(1));
    assert!(usage.bytes <= peak);
    assert!(run(lo).0.is_ok());
}

#[test]
fn failures_name_their_level() {
    let (p, indices) = grid(4);
    let pos = Positions::from_packed(&p);
    let counts = [indices.len() / 2, 0];
    let options = LodChainOptions::new(
        LodTargets::Counts(&counts),
        LodErrors::PerLevel(&[1., f32::NAN]),
    );
    let error = build_lod_chain(&indices, pos, &options, &mut Workspace::default()).unwrap_err();
    assert_eq!(
        (error.level, error.error),
        (Some(1), Error::InvalidParameter)
    );
    let options = LodChainOptions::new(LodTargets::Counts(&counts), LodErrors::Uniform(1.));
    let mut bad = indices.clone();
    bad[1] = u32::MAX;
    let error = build_lod_chain(&bad, pos, &options, &mut Workspace::default()).unwrap_err();
    assert_eq!(
        (error.level, error.error),
        (Some(0), Error::IndexOutOfBounds)
    );
    let message = format!("{error}");
    assert!(message.contains("level 0"), "{message}");
}

#[test]
fn error_and_scale_fields_are_per_level_raw_values() {
    let (p, indices) = grid(8);
    let pos = Positions::from_packed(&p);
    let counts = [indices.len() / 2, indices.len() / 4];
    let options = LodChainOptions::new(LodTargets::Counts(&counts), LodErrors::Uniform(1.));
    let chain = build_lod_chain(&indices, pos, &options, &mut Workspace::default()).unwrap();
    let scale = simplify_scale(pos).unwrap();
    for level in &chain.levels {
        assert_eq!(level.scale, scale);
    }
    // Successive mode: level 1's error is that of simplifying level 0's output.
    let second = simplify(
        &chain.levels[0].indices,
        pos,
        SimplifySettings {
            target_index_count: counts[1],
            target_error: 1.,
            options: SimplifyOptions::EMPTY,
        },
        &mut Workspace::default(),
    )
    .unwrap();
    assert_eq!(chain.levels[1].error.to_bits(), second.error.to_bits());
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
            let options =
                LodChainOptions::new(LodTargets::Counts(&targets), LodErrors::Uniform(1.0))
                    .with_independent(independent);
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
                assert_eq!(level.error.to_bits(), direct.error.to_bits());
                previous = &level.indices;
            }
        }
        eprintln!("PASS {}", path.display());
    }
}
