#![no_main]
//! `bake_impostor` / `bake_impostor_generic`: no panic on arbitrary meshes, options, albedo
//! sources and normals; determinism; u16 == u32; budgeted == unbudgeted; atlas and report
//! consistency.
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;

fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let flags = c.u8();
    let nv = 1 + usize::from(c.u8()) % 20;
    let nt = usize::from(c.u8()) % 25;
    let special = flags & 0x80 != 0;
    let raw_params = flags & 0x40 != 0;
    let layout = if c.u8() & 1 == 0 { ImpostorLayout::Hemisphere } else { ImpostorLayout::Sphere };
    let (vb, rb, sb, db) = (c.u8(), c.u8(), c.u8(), c.u8());
    let (cutoff, cov) = (c.unit(), c.unit());
    let mode = c.u8();
    let limits = budget(&mut c);

    let mut o = ImpostorOptions::default();
    // Valid ranges most of the time; raw bytes (0, 65.., 4097..) reach the validation errors.
    let views = if raw_params { u32::from(vb) } else { 2 + u32::from(vb) % 3 };
    o.grid = ImpostorGrid::new(layout, views).unwrap_or_default();
    if raw_params {
        // A hand-built invalid grid is not constructible (non_exhaustive); the validated
        // constructor's rejection is the behavior under test.
        assert_eq!(ImpostorGrid::new(layout, views).is_ok(), (2..=64).contains(&views));
        if !(2..=64).contains(&views) {
            assert_eq!(ImpostorGrid::new(layout, views), Err(Error::InvalidParameter));
        }
    }
    o.view_resolution = if raw_params { u32::from(rb) * 17 } else { 1 + u32::from(rb) % 24 };
    o.samples_per_axis = if raw_params { u32::from(sb) % 10 } else { 1 + u32::from(sb) % 3 };
    o.dilation = if raw_params { u32::from(db) % 70 } else { u32::from(db) % 8 };
    o.alpha_cutoff = if raw_params { cutoff * 1.2 - 0.1 } else { cutoff };
    o.coverage_threshold = if raw_params { cov * 1.2 - 0.1 } else { 0.05 + cov * 0.95 };
    o.double_sided = mode & 1 != 0;
    let bc = [c.u8(), c.u8(), c.u8(), c.u8()];
    if mode & 2 != 0 {
        let center = [0, 1, 2].map(|k| (f32::from(bc[k]) - 128.) / 16.);
        let radius = if raw_params { (f32::from(bc[3]) - 20.) / 16. } else { 0.5 + f32::from(bc[3]) / 32. };
        if let Ok(b) = ImpostorBounds::new(center, radius) {
            o.bounds = Some(b);
        }
    }

    let mut pos = vec![[0f32; 3]; nv];
    for p in &mut pos {
        *p = [0, 1, 2].map(|_| coord(c.u8(), special, 16., 128.));
    }
    let span = nv + if flags & 0x20 != 0 { 2 } else { 0 };
    let mut idx: Vec<u32> = (0..nt * 3).map(|_| (usize::from(c.u8()) % span) as u32).collect();
    if flags & 0x10 != 0 && !idx.is_empty() {
        idx.pop();
    }
    let short = flags & 0x08 != 0; // wrong attribute lengths -> InvalidLayout
    let n_attr = if short { nv.saturating_sub(1) } else { nv };
    let normals: Vec<[f32; 3]> = (0..n_attr)
        .map(|k| [0, 1, 2].map(|j| (mix(k as u64, j as u64 + u64::from(bc[0])) % 200) as f32 / 100. - 1.))
        .collect();
    let colors: Vec<[f32; 4]> = (0..n_attr)
        .map(|k| [0, 1, 2, 3].map(|j| (mix(k as u64 + 77, j as u64) % 256) as f32 / 255.))
        .collect();
    let uvs: Vec<[f32; 2]> = (0..n_attr)
        .map(|k| [0, 1].map(|j| (mix(k as u64 + 191, j as u64) % 256) as f32 / 255.))
        .collect();
    let nan_sampler = flags & 0x04 != 0;
    let seed = u64::from(bc[1]);
    let sampler = move |t: usize, uv: [f32; 2]| -> [f32; 4] {
        let h = mix(t as u64 ^ seed, u64::from(uv[0].to_bits()) << 32 | u64::from(uv[1].to_bits()));
        if nan_sampler && h & 0xff == 0 {
            return [f32::NAN, 0., 0., 1.];
        }
        [0, 1, 2, 3].map(|j| ((h >> (8 * j)) & 0xff) as f32 / 255.)
    };
    let albedo = || match (mode >> 2) % 3 {
        0 => ImpostorAlbedo::White,
        1 => ImpostorAlbedo::VertexColors(&colors),
        _ => ImpostorAlbedo::Texture { uvs: &uvs, sampler: &sampler },
    };
    let normals_opt = if mode & 0x10 != 0 { Some(&normals[..]) } else { None };
    let textured = (mode >> 2) % 3 == 2;

    let bake = |ws: &mut Workspace| bake_impostor(&idx, Positions::from_packed(&pos), normals_opt, albedo(), o, ws);
    // The default workspace allows 2^34 work: bound the inputs the baker can be asked for.
    // Documented cost: storage holds one view of 32-byte samples (resolution^2 *
    // samples^2) and work adds dilation passes over the atlas, so both enter the bound.
    let per_texel = u64::from(o.samples_per_axis).pow(2) + u64::from(o.dilation) + 1;
    let cost = u64::from(o.view_resolution).pow(2) * u64::from(o.grid.views_per_axis).pow(2) * per_texel;
    let cheap = cost <= 1 << 23;
    let a = if cheap { bake(&mut Workspace::default()) } else { bake(&mut workspace(limits)) };
    let Ok(atlas) = &a else {
        if cheap {
            // Budgeted arm may only add resource failures.
            if let Err(y) = bake(&mut workspace(limits)) {
                assert!(resource(&y) || Some(&y) == a.as_ref().err(), "budgeted error {y:?}");
            }
        }
        return;
    };
    if !cheap {
        return;
    }
    // Determinism, budget equality and width equality.
    assert_eq!(a, bake(&mut Workspace::default()), "non-deterministic");
    match bake(&mut workspace(limits)) {
        Ok(b) => assert_eq!(*atlas, b, "budgeted atlas differs"),
        Err(e) => assert!(resource(&e), "budgeted failed with {e:?}"),
    }
    let i16: Vec<u16> = idx.iter().map(|&i| i as u16).collect();
    let w = bake_impostor_generic::<u16>(&i16, Positions::from_packed(&pos), normals_opt, albedo(), o, &mut Workspace::default());
    assert_eq!(a, w, "u16 differs from u32");

    // Atlas shape and report.
    let views = o.grid.views_per_axis;
    assert_eq!(atlas.size, views * o.view_resolution);
    assert_eq!(atlas.view_resolution, o.view_resolution);
    assert_eq!(atlas.grid, o.grid);
    let n = (atlas.size as usize) * (atlas.size as usize);
    assert_eq!((atlas.albedo_alpha.len(), atlas.normal_depth.len()), (n, n));
    assert!(atlas.bounds.radius.is_finite() && atlas.bounds.radius >= 0.);
    let r = atlas.report;
    assert!(r.opaque_texels <= r.covered_texels);
    assert!(r.empty_views <= o.grid.view_count().unwrap());
    if !textured {
        assert_eq!(r.sampler_calls, 0);
    }
    let alpha = |a: &[u8; 4]| a[3];
    assert_eq!(r.covered_texels, atlas.albedo_alpha.iter().filter(|t| alpha(t) != 0).count() as u64);
    assert_eq!(r.opaque_texels, atlas.albedo_alpha.iter().filter(|t| alpha(t) == 255).count() as u64);
    if o.dilation == 0 {
        for (t, nd) in atlas.albedo_alpha.iter().zip(&atlas.normal_depth) {
            if alpha(t) == 0 {
                assert_eq!(*nd, [128, 128, 128, 255], "empty texel without dilation");
            }
        }
    }
    for v in 0..o.grid.view_count().unwrap() {
        let [x, y] = atlas.tile_origin(v).unwrap();
        assert!(x + o.view_resolution <= atlas.size && y + o.view_resolution <= atlas.size);
    }
    assert!(atlas.tile_origin(o.grid.view_count().unwrap()).is_none());
});
