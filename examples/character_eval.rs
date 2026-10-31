//! Round-two character evaluation; packed input is written by eval_models.py.
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
    fn points(&mut self, n: usize) -> Vec<[f32; 3]> {
        (0..n).map(|_| core::array::from_fn(|_| self.f())).collect()
    }
}
fn compact<T: Copy>(values: &[T], stride: usize, representatives: &[usize]) -> Vec<T> {
    representatives
        .iter()
        .flat_map(|&i| values[i * stride..(i + 1) * stride].iter().copied())
        .collect()
}
fn maximum(values: &[f64]) -> f64 {
    values.iter().copied().fold(0., f64::max)
}
fn p95(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted[(sorted.len() * 95).div_ceil(100) - 1]
}
// Batches prevent tiny calls from being optimized away or timed below resolution.
fn time_mesh(
    mut action: impl FnMut() -> Result<SimplifiedMesh, Error>,
    max_batch: usize,
) -> Result<(SimplifiedMesh, f64), Error> {
    fn batch(
        action: &mut impl FnMut() -> Result<SimplifiedMesh, Error>,
        count: usize,
        reference: &SimplifiedMesh,
    ) -> Result<f64, Error> {
        let mut meshes = Vec::with_capacity(count);
        let start = Instant::now();
        for _ in 0..count {
            meshes.push(std::hint::black_box(action())?);
        }
        let elapsed = start.elapsed().as_secs_f64();
        for mesh in meshes {
            assert_eq!(mesh.indices, reference.indices);
            assert_eq!(mesh.error.to_bits(), reference.error.to_bits());
        }
        Ok(elapsed)
    }
    let reference = std::hint::black_box(action())?;
    let mut count = 1;
    loop {
        let elapsed = batch(&mut action, count, &reference)?;
        if elapsed >= 0.01 || count >= max_batch {
            break;
        }
        count = count.saturating_mul(2).min(max_batch);
    }
    let mut timings = [0.; 3];
    for ms in &mut timings {
        *ms = batch(&mut action, count, &reference)? * 1000. / count as f64;
    }
    timings.sort_by(f64::total_cmp);
    Ok((reference, timings[1]))
}

fn main() -> Result<(), Box<dyn StdError>> {
    let path = std::env::args().nth(1).ok_or("input required")?;
    let timing_only = std::env::args().any(|arg| arg == "--timing-only");
    let bytes = std::fs::read(&path)?;
    let mut r = Reader(&bytes);
    let [n, ni, nb, ns, np, na, influences, aw, kw] =
        core::array::from_fn::<_, 9, _>(|_| r.u() as usize);
    let p = r.points(n);
    let indices: Vec<_> = (0..ni).map(|_| r.u()).collect();
    let joints: Vec<_> = (0..n * influences).map(|_| r.u()).collect();
    let weights: Vec<_> = (0..n * influences).map(|_| r.f()).collect();
    let shapes: Vec<_> = (0..ns).map(|_| r.points(n)).collect();
    let attr: Vec<_> = (0..n * aw).map(|_| r.f()).collect();
    let keys = &r.0[..n * kw];
    r.0 = &r.0[n * kw..];
    let importance: Vec<_> = (0..nb).map(|_| r.f()).collect();
    let samples: Vec<_> = (0..np).map(|_| r.points(n)).collect();
    assert!(r.0.is_empty());
    let mut ws = Workspace::default();
    // Split wide exact keys into supported byte streams; no hashing or rounding.
    let streams: Vec<_> = (0..kw)
        .step_by(256)
        .map(|offset| VertexStream::new(keys, n, (kw - offset).min(256), kw, offset))
        .collect::<Result<_, _>>()?;
    let remap = generate_vertex_remap_multi(Some(&indices), &streams, &mut ws)?;
    let mut representatives = vec![usize::MAX; remap.vertex_count];
    for (i, &j) in remap.remap.iter().enumerate() {
        if j != u32::MAX {
            representatives[j as usize] = i;
        }
    }
    let indices = remap_index_buffer(Some(&indices), &remap.remap, &mut ws)?;
    let p = compact(&p, 1, &representatives);
    let joints = compact(&joints, influences, &representatives);
    let weights = compact(&weights, influences, &representatives);
    let attr = compact(&attr, aw, &representatives);
    let shapes: Vec<_> = shapes
        .iter()
        .map(|s| compact(s, 1, &representatives))
        .collect();
    let samples: Vec<_> = samples
        .iter()
        .map(|s| compact(s, 1, &representatives))
        .collect();
    eprintln!(
        "weld vertices {n}->{} triangles {}->{} channels {}",
        p.len(),
        ni / 3,
        indices.len() / 3,
        nb + ns * 3
    );
    let shape_refs: Vec<_> = shapes.iter().map(Vec::as_slice).collect();
    let unit = vec![1.; nb];
    let mut data = CharacterInputs::new();
    data.blend_shapes = &shape_refs;
    let ratios = if ni / 3 >= 10000 {
        vec![0.5, 0.25, 0.1, 0.05]
    } else {
        vec![0.5, 0.25, 0.1]
    };
    if std::env::args().any(|arg| arg == "--best-of") {
        // Full character chains meter every sampled pose together, unlike the
        // original harness's separate quality calls. Keep the default byte cap.
        ws = Workspace::new(Limits {
            max_work: 1 << 42,
            ..Limits::default()
        });
        data.skin = if nb == 0 {
            None
        } else {
            Some(CharacterSkin::new(&joints, &weights, influences, &unit))
        };
        let sample_refs: Vec<_> = samples.iter().map(Vec::as_slice).collect();
        data.sample_poses = &sample_refs;
        println!(
            "target,chosen,triangles,plain_max,character_max,selected_max,character_ms,bestof_ms"
        );
        for ratio in ratios {
            let counts = [((ni / 3) as f64 * ratio) as usize * 3];
            let mut co =
                LodChainOptions::new(LodTargets::Counts(&counts), LodErrors::Uniform(100.));
            let attr_weights = vec![1.; aw];
            if aw != 0 {
                co.attributes = Some(LodChainAttributes::new(
                    Attributes::from_interleaved(&attr, p.len(), aw, aw, 0)?,
                    &attr_weights,
                ));
            }
            let extent = samples
                .iter()
                .chain(std::iter::once(&p))
                .map(|s| simplify_scale(Positions::from_packed(s)))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .fold(0.0f32, f32::max);
            let mut qo = LodQualityOptions::default();
            qo.samples_per_unit_area = if extent > 0. {
                1e-6 / f64::from(extent).powi(2)
            } else {
                0.
            };
            let mut character_co = co;
            character_co.attributes = None;
            eprintln!("target {ratio}: character chain");
            let start = Instant::now();
            let character = build_character_lod_chain(
                &indices,
                Positions::from_packed(&p),
                &data,
                &CharacterOptions::default(),
                &character_co,
                &qo,
                &mut ws,
            )?;
            let character_ms = start.elapsed().as_secs_f64() * 1000.;
            eprintln!(
                "target {ratio}: character {character_ms:.3} ms, usage {:?}",
                ws.usage()
            );
            let mut options = CharacterOptions::default();
            options.best_of = true;
            eprintln!("target {ratio}: best-of chain");
            let start = Instant::now();
            let selected = build_character_lod_chain(
                &indices,
                Positions::from_packed(&p),
                &data,
                &options,
                &co,
                &qo,
                &mut ws,
            )?;
            let bestof_ms = start.elapsed().as_secs_f64() * 1000.;
            eprintln!(
                "target {ratio}: best-of {bestof_ms:.3} ms, usage {:?}",
                ws.usage()
            );
            let report = selected.selections[0];
            let max = selected
                .quality
                .iter()
                .fold(0.0f64, |v, q| v.max(q[0].symmetric.max));
            let char_max = character
                .quality
                .iter()
                .fold(0.0f64, |v, q| v.max(q[0].symmetric.max));
            assert_eq!(char_max, report.character_max);
            assert!(max <= report.plain_max && max <= report.character_max);
            let chosen = match report.method {
                CharacterLodMethod::Plain => "plain",
                CharacterLodMethod::Character => "character",
                _ => unreachable!(),
            };
            println!(
                "{ratio},{chosen},{},{:.17},{:.17},{max:.17},{character_ms:.6},{bestof_ms:.6}",
                selected.chain.levels[0].triangle_count, report.plain_max, report.character_max
            );
            let out: Vec<_> = selected.chain.levels[0]
                .indices
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            std::fs::write(format!("{path}.{ratio}.bestof.indices"), out)?;
        }
        return Ok(());
    }
    println!("ratio,method,vertices,triangles,bind,animation_max,animation_p95,morph_max,morph_p95,ms,worst_pose,status");
    for ratio in ratios {
        let settings = SimplifySettings {
            target_index_count: ((ni / 3) as f64 * ratio) as usize * 3,
            target_error: 100.,
            options: SimplifyOptions::EMPTY,
        };
        for method in ["plain", "character", "important"] {
            let imp = if method == "important" {
                &importance
            } else {
                &unit
            };
            data.skin = if nb == 0 {
                None
            } else {
                Some(CharacterSkin::new(&joints, &weights, influences, imp))
            };
            let result = time_mesh(
                || {
                    let positions = std::hint::black_box(Positions::from_packed(&p));
                    let settings = std::hint::black_box(settings);
                    let indices = std::hint::black_box(indices.as_slice());
                    if method == "plain" {
                        if aw == 0 {
                            simplify(indices, positions, settings, std::hint::black_box(&mut ws))
                        } else {
                            simplify_with_attributes(
                                indices,
                                positions,
                                Attributes::from_interleaved(&attr, p.len(), aw, aw, 0)?,
                                &vec![1.; aw],
                                None,
                                settings,
                                std::hint::black_box(&mut ws),
                            )
                        }
                    } else {
                        simplify_character(
                            indices,
                            positions,
                            std::hint::black_box(&data),
                            &CharacterOptions::default(),
                            settings,
                            std::hint::black_box(&mut ws),
                        )
                    }
                },
                (1_000_000 / indices.len().max(1)).clamp(1, 4096),
            );
            let (mesh, ms) = match result {
                Ok(result) => result,
                Err(e) => {
                    println!("{ratio},{method},{},,,,,,,,,{e:?}", p.len());
                    continue;
                }
            };
            if timing_only {
                println!(
                    "{ratio},{method},{},{},,,,,,{ms:.6},,timing_only",
                    p.len(),
                    mesh.indices.len() / 3
                );
                let mut out = Vec::new();
                for index in &mesh.indices {
                    out.extend_from_slice(&index.to_le_bytes());
                }
                std::fs::write(format!("{path}.{ratio}.{method}.indices"), out)?;
                continue;
            }
            let measure = |sample: &[[f32; 3]], ws: &mut Workspace| -> Result<f64, Error> {
                let pos = Positions::from_packed(sample);
                let extent = f64::from(simplify_scale(pos)?);
                let mut qo = LodQualityOptions::default();
                // Every nondegenerate triangle has area below 2 * extent^2,
                // so ceil(area * density) contributes one interior sample.
                qo.samples_per_unit_area = if extent > 0. {
                    1e-6 / (extent * extent)
                } else {
                    0.
                };
                Ok(measure_lod_quality(
                    LodQualityMesh::new(&indices, pos),
                    LodQualityMesh::new(&mesh.indices, pos),
                    &qo,
                    ws,
                )?
                .symmetric
                .max)
            };
            let bind = measure(&p, &mut ws)?;
            let deviations: Vec<_> = samples
                .iter()
                .map(|s| measure(s, &mut ws))
                .collect::<Result<_, _>>()?;
            let worst = deviations
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.total_cmp(b.1))
                .map(|v| v.0)
                .filter(|&index| deviations[index] > bind);
            let worst_label = worst.map_or_else(|| "bind".to_owned(), |index| index.to_string());
            println!(
                "{ratio},{method},{},{},{bind:.9},{:.9},{:.9},{:.9},{:.9},{:.6},{worst_label},ok",
                p.len(),
                mesh.indices.len() / 3,
                maximum(&deviations[..na]),
                p95(&deviations[..na]),
                maximum(&deviations[na..]),
                p95(&deviations[na..]),
                ms
            );
            // Retain indices and exact worst-pose coordinates for independent rendering.
            let prefix = format!("{path}.{ratio}.{method}");
            let mut out = Vec::new();
            for v in &mesh.indices {
                out.extend_from_slice(&v.to_le_bytes());
            }
            std::fs::write(format!("{prefix}.indices"), out)?;
            let mut out = Vec::new();
            for point in worst.and_then(|index| samples.get(index)).unwrap_or(&p) {
                for v in point {
                    out.extend_from_slice(&v.to_le_bytes());
                }
            }
            std::fs::write(format!("{prefix}.pose"), out)?;
        }
    }
    let mut out = Vec::new();
    for v in &indices {
        out.extend_from_slice(&v.to_le_bytes());
    }
    std::fs::write(format!("{path}.source.indices"), out)?;
    Ok(())
}
