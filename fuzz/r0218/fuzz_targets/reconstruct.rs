#![no_main]
//! `ImpostorAtlas::reconstruct_texel` / `reconstruct_view`: no panic for arbitrary atlases
//! (including field-corrupted ones), frames, blends, points and step counts; determinism;
//! finite output in [0, 1] for valid inputs; `reconstruct_view` equals `reconstruct_texel`
//! at texel centers (exact for power-of-two resolutions).
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;

fn finite(t: &[f32; 4]) -> bool {
    t.iter().all(|x| x.is_finite())
}

fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let flags = c.u8();
    let nv = 1 + usize::from(c.u8()) % 12;
    let nt = usize::from(c.u8()) % 13;
    let layout = if c.u8() & 1 == 0 { ImpostorLayout::Hemisphere } else { ImpostorLayout::Sphere };
    let mut o = ImpostorOptions::default();
    o.grid = ImpostorGrid::new(layout, 2 + u32::from(c.u8()) % 3).unwrap();
    o.view_resolution = 1 + u32::from(c.u8()) % 16;
    o.samples_per_axis = 1;
    o.dilation = u32::from(c.u8()) % 5;
    o.double_sided = flags & 1 != 0;
    let pos: Vec<[f32; 3]> = (0..nv).map(|_| [0, 1, 2].map(|_| coord(c.u8(), false, 16., 128.))).collect();
    let idx: Vec<u32> = (0..nt * 3).map(|_| (usize::from(c.u8()) % nv) as u32).collect();
    let Ok(mut atlas) = bake_impostor(&idx, Positions::from_packed(&pos), None, ImpostorAlbedo::White, o, &mut Workspace::default())
    else {
        return;
    };

    // Reconstruction parameters.
    let dir_raw = flags & 0x80 != 0;
    let direction = if dir_raw {
        [c.raw_f32(), c.raw_f32(), c.raw_f32()]
    } else {
        [0, 1, 2].map(|_| (f32::from(c.u8()) - 128.) / 64.)
    };
    let steps = if flags & 0x40 != 0 { u32::from(c.u8()) % 12 } else { u32::from(c.u8()) % 9 };
    let mut ropts = ImpostorReconstruct::default();
    ropts.parallax_steps = steps;
    let point_raw = flags & 0x20 != 0;
    let point = if point_raw { [c.raw_f32(), c.raw_f32()] } else { [c.unit(), c.unit()] };
    let pow = 1u32 << (c.u8() % 5);
    let odd = 1 + u32::from(c.u8()) % 13;
    let corrupt = c.u8();
    let corrupt_arg = c.u8();

    let frame = ImpostorFrame::from_direction(direction);
    let blend = atlas.grid.blend(direction);
    // from_direction and blend accept exactly the same directions.
    assert_eq!(frame.is_ok(), blend.is_ok(), "frame/blend validity differs for {direction:?}");
    if let (Ok(frame), Ok(blend)) = (frame, blend) {
        let t = atlas.reconstruct_texel(frame, blend, point, ropts);
        assert_eq!(t, atlas.reconstruct_texel(frame, blend, point, ropts), "non-deterministic");
        let in_range = point.iter().all(|x| (0.0..=1.0).contains(x));
        match &t {
            Ok(t) => {
                assert!(in_range && steps <= 8, "accepted invalid input");
                assert!(finite(t) && t.iter().all(|x| (-1e-5..=1.0 + 1e-5).contains(x)), "texel {t:?}");
                if t[3] == 0.0 {
                    assert_eq!(&t[..3], &[0., 0., 0.]);
                }
            }
            Err(e) => {
                assert_eq!(*e, Error::InvalidParameter);
                assert!(!in_range || steps > 8, "valid input rejected");
            }
        }

        // reconstruct_view at a power-of-two resolution equals texel-wise reconstruction exactly.
        let n = (pow * pow) as usize;
        let mut out = vec![[f32::NAN; 4]; n];
        let v = atlas.reconstruct_view(direction, pow, ropts, &mut out);
        match v {
            Ok(()) => {
                assert!(steps <= 8);
                let step = 1.0 / pow as f32;
                for (i, texel) in out.iter().enumerate() {
                    let (x, y) = ((i as u32 % pow) as f32, (i as u32 / pow) as f32);
                    let want = atlas
                        .reconstruct_texel(frame, blend, [(x + 0.5) * step, (y + 0.5) * step], ropts)
                        .unwrap();
                    assert_eq!(*texel, want, "view texel {i} differs from reconstruct_texel");
                }
            }
            Err(e) => assert_eq!((e, steps > 8), (Error::InvalidParameter, true)),
        }
        // Arbitrary resolution: no panic, finite output, exact-length contract.
        let n = (odd * odd) as usize;
        let mut out = vec![[0f32; 4]; n];
        let r = atlas.reconstruct_view(direction, odd, ropts, &mut out);
        if r.is_ok() {
            assert!(out.iter().all(finite));
        }
        let mut short = vec![[0f32; 4]; n - 1];
        match atlas.reconstruct_view(direction, odd, ropts, &mut short) {
            Err(Error::BufferTooSmall) => {}
            Err(Error::InvalidParameter) => assert!(steps > 8),
            other => panic!("short buffer: {other:?}"),
        }
        let mut long = vec![[0f32; 4]; n + 1];
        match atlas.reconstruct_view(direction, odd, ropts, &mut long) {
            Err(Error::InvalidParameter) => {}
            other => panic!("long buffer: {other:?}"),
        }
        // Hand-mutated blend and frame: no panic, finite when accepted.
        let mut bad = blend;
        match corrupt % 6 {
            0 => bad.views[usize::from(corrupt_arg) % 3] = u32::from(corrupt_arg) * 3,
            1 => bad.weights[usize::from(corrupt_arg) % 3] = f32::NAN,
            2 => bad.weights = [f32::from(corrupt_arg) - 128., 3., -2.],
            3 => bad.views = [u32::MAX; 3],
            _ => {}
        }
        if let Ok(t) = atlas.reconstruct_texel(frame, bad, [0.5, 0.5], ropts) {
            assert!(finite(&t), "mutated blend gave {t:?}");
        }
        let mut badf = frame;
        match corrupt / 6 % 4 {
            0 => badf.direction = [f32::NAN, 0., 1.],
            1 => badf.right = [0.; 3],
            2 => badf.up = [f32::INFINITY, 0., 0.],
            _ => {}
        }
        if let Ok(t) = atlas.reconstruct_texel(badf, blend, [0.25, 0.75], ropts) {
            assert!(finite(&t), "mutated frame gave {t:?}");
        }
    }

    // Corrupted atlas fields must be rejected or handled, never panic.
    let k = usize::from(corrupt_arg);
    match corrupt >> 4 {
        1 => atlas.albedo_alpha.truncate(atlas.albedo_alpha.len().saturating_sub(1 + k)),
        2 => atlas.normal_depth.truncate(atlas.normal_depth.len().saturating_sub(1 + k)),
        3 => atlas.size = atlas.size.wrapping_add(u32::from(corrupt_arg)).wrapping_sub(128),
        4 => atlas.bounds.radius = [0., -1., f32::NAN, f32::INFINITY, 1e-30][k % 5],
        5 => atlas.view_resolution = u32::from(corrupt_arg) % 40,
        6 => atlas.bounds.center = [f32::NAN, 0., 0.],
        7 => atlas.size = u32::MAX,
        8 => atlas.view_resolution = u32::MAX,
        _ => return,
    }
    let d = [0.3, 0.8, 0.5];
    if let (Ok(frame), Ok(blend)) = (ImpostorFrame::from_direction(d), atlas.grid.blend(d)) {
        if let Ok(t) = atlas.reconstruct_texel(frame, blend, [0.5, 0.5], ropts) {
            assert!(finite(&t), "corrupted atlas gave {t:?}");
        }
        let mut out = vec![[0f32; 4]; 9];
        let _ = atlas.reconstruct_view(d, 3, ropts, &mut out);
    }
});
