//! CPU foliage receipt from a caller-prepared binary mesh and alpha mask.
//! Input: u32 vertex/index counts and mask width/height, packed f32 positions,
//! packed f32 UVs, u32 indices, then tightly packed alpha bytes, little endian.
#![allow(clippy::field_reassign_with_default)]
use meshoptimizer_rs::{
    clusterlod::{
        self as cl,
        cut::{self, CutView, Projection, Threshold},
        page::QuantGrid,
        v2::{self, AlphaCoverage, BuildMesh},
    },
    *,
};
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read(std::env::args().nth(1).ok_or("input required")?)?;
    let mut at = 0;
    let mut word = || {
        let x = u32::from_le_bytes(data[at..at + 4].try_into().expect("input word"));
        at += 4;
        x
    };
    let n = word() as usize;
    let ni = word() as usize;
    let w = word();
    let h = word();
    let p: Vec<[f32; 3]> = (0..n)
        .map(|_| {
            [
                f32::from_bits(word()),
                f32::from_bits(word()),
                f32::from_bits(word()),
            ]
        })
        .collect();
    let uv: Vec<f32> = (0..n * 2).map(|_| f32::from_bits(word())).collect();
    let indices: Vec<u32> = (0..ni).map(|_| word()).collect();
    let mask = AlphaMask::new(&data[at..], w, h)?;
    let masks = [Some(mask)];
    let attrs = Attributes::from_interleaved(&uv, n, 2, 2, 0)?;
    let mut config = cl::default_config(32)?;
    config.partition_size = 2;
    config.optimize_clusters = false;
    let grid = QuantGrid::new(-16)?;
    let run = |alpha| {
        let mut m = BuildMesh::new(&indices, &p);
        m.attributes = Some(attrs);
        m.attribute_weights = &[0., 0.];
        if alpha {
            m.alpha_coverage = Some(AlphaCoverage::new(&masks, 0));
        }
        v2::build(config, m, grid, &mut Workspace::default())
    };
    let mut off = run(false)?;
    let mut on = run(true)?;
    // Use the actual container's repaired spheres and quantisation error floor.
    for b in [&mut off, &mut on] {
        let source =
            cl::dag_format::v2::Source::new(&b.groups, &b.clusters, &b.positions, &b.indices);
        let (file, _) = cl::dag_format::v2::write(
            &source,
            &cl::dag_format::v2::WriteOptions::new(grid),
            &mut Workspace::default(),
        )?;
        let reader = cl::dag_format::v2::Reader::read_all(&file, &mut Workspace::default())?;
        let count = b.groups.len();
        for (i, g) in b.groups.iter_mut().enumerate() {
            let record = reader.group(count - 1 - i)?;
            g.center = record.center;
            g.radius = record.radius;
            g.error = record.error;
        }
    }
    let radius = p
        .iter()
        .map(|p| p.iter().map(|v| v * v).sum::<f32>().sqrt())
        .fold(0f32, f32::max);
    let bounds = ImpostorBounds::new([0.; 3], radius * 1.001)?;
    let grid_views = ImpostorGrid::new(ImpostorLayout::Sphere, 3)?;
    let render = |b: &v2::Built, idx: &[u32], resolution| {
        let uvs: Vec<[f32; 2]> = b
            .attributes
            .chunks_exact(b.attribute_stride)
            .map(|a| [a[0], a[1]])
            .collect();
        let sampler = |_: usize, uv: [f32; 2]| {
            let x = ((uv[0] as f64).rem_euclid(1.0) * w as f64) as u32;
            let y = ((uv[1] as f64).rem_euclid(1.0) * h as f64) as u32;
            [
                1.,
                1.,
                1.,
                f32::from(mask.alpha(x.min(w - 1), y.min(h - 1)).unwrap_or(0)) / 255.,
            ]
        };
        let mut options = ImpostorOptions::default();
        options.grid = grid_views;
        options.bounds = Some(bounds);
        options.view_resolution = resolution;
        options.dilation = 0;
        options.coverage_threshold = 1.;
        bake_impostor(
            idx,
            Positions::from_packed(&b.positions),
            None,
            ImpostorAlbedo::Texture {
                uvs: &uvs,
                sampler: &sampler,
            },
            options,
            &mut Workspace::default(),
        )
    };
    let leaf: Vec<u32> = off
        .clusters
        .iter()
        .filter(|c| c.refined < 0)
        .flat_map(|c| {
            off.indices[c.first_index as usize..(c.first_index + c.index_count) as usize]
                .iter()
                .copied()
        })
        .collect();
    println!("source_triangles={} off_stuck={} on_stuck={} alpha_stuck={} raised={} off_dag_triangles={} on_dag_triangles={}",ni/3,off.stuck_groups,on.stuck_groups,on.alpha_coverage_stuck_groups,on.alpha_coverage_raised_groups,off.indices.len()/3,on.indices.len()/3);
    for distance in [4f32, 8., 16., 32., 64.] {
        let view = CutView {
            position: [0., 0., distance],
            projection: Projection::Perspective {
                proj_y: 1.0 / (0.5f32).tan(),
                znear: 0.01,
            },
            viewport_height: 1080.,
            scale: 1.0,
        };
        let resolution = ((2. * radius / distance * (1080. / (2. * (0.5f32).tan()))).round()
            as u32)
            .clamp(8, 512);
        let reference = render(&off, &leaf, resolution)?;
        for (name, b) in [("off", &off), ("on", &on)] {
            let mut cut = vec![0u32; b.clusters.len()];
            let count = cut::select_cut(
                &b.groups,
                &b.clusters,
                &view,
                Threshold::Pixels(1.0),
                None,
                &mut cut,
            )?;
            let chosen: Vec<u32> = cut[..count]
                .iter()
                .flat_map(|&c| {
                    let c = &b.clusters[c as usize];
                    b.indices[c.first_index as usize..(c.first_index + c.index_count) as usize]
                        .iter()
                        .copied()
                })
                .collect();
            let atlas = render(b, &chosen, resolution)?;
            let r = resolution as usize;
            let size = r * 3;
            let mut ratios = Vec::new();
            let mut max_blur = 0f64;
            for v in 0..9 {
                let (ox, oy) = ((v % 3) * r, (v / 3) * r);
                let mut sums = [0u64; 2];
                let mut blur = 0.;
                for y in 0..r {
                    for x in 0..r {
                        let i = (oy + y) * size + ox + x;
                        sums[0] += u64::from(reference.albedo_alpha[i][3]);
                        sums[1] += u64::from(atlas.albedo_alpha[i][3]);
                        let mut d = 0i32;
                        for dy in -1isize..=1 {
                            for dx in -1isize..=1 {
                                let j = (oy + y.saturating_add_signed(dy).min(r - 1)) * size
                                    + ox
                                    + x.saturating_add_signed(dx).min(r - 1);
                                d += i32::from(reference.albedo_alpha[j][3])
                                    - i32::from(atlas.albedo_alpha[j][3]);
                            }
                        }
                        blur += f64::from(d.abs()) / (255. * 9.);
                    }
                }
                ratios.push(if sums[0] == 0 {
                    if sums[1] == 0 {
                        1.
                    } else {
                        f64::INFINITY
                    }
                } else {
                    sums[1] as f64 / sums[0] as f64
                });
                max_blur = max_blur.max(blur / (r * r) as f64);
            }
            println!("distance={distance} resolution={resolution} policy={name} triangles={} ratio_min={:.6} ratio_max={:.6} blurred_max={max_blur:.6}",chosen.len()/3,ratios.iter().copied().fold(f64::INFINITY,f64::min),ratios.iter().copied().fold(0.,f64::max));
        }
    }
    for (name, b) in [("off", &off), ("on", &on)] {
        let report =
            v2::measure_fallback(b, &LodQualityOptions::default(), &mut Workspace::default())?;
        println!(
            "fallback={name} triangles={} sampled_max={} sampled_rms={}",
            report.indices.len() / 3,
            report.quality.symmetric.max,
            report.quality.symmetric.rms
        );
    }
    Ok(())
}
