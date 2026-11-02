#![no_main]
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;

fn limits() -> Limits {
    Limits {
        max_bytes: 64 << 20,
        max_work: 4_000_000,
    }
}
fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let m = mesh(&mut c, 96);
    let mode = c.u8();
    let points: Vec<[f32; 3]> = (0..1 + usize::from(c.u8() % 3))
        .map(|_| [0, 1, 2].map(|_| (c.unit() - 0.5) * 24.))
        .collect();
    let lo = [0, 1, 2].map(|_| (c.unit() - 0.5) * 16.);
    let ext = [0, 1, 2].map(|_| c.unit() * 8.);
    let mut o = HiddenOptions::default();
    o.views = match mode % 3 {
        0 => HiddenViews::Outside,
        1 => HiddenViews::Points(&points),
        _ => HiddenViews::Bounds {
            min: lo,
            max: [lo[0] + ext[0], lo[1] + ext[1], lo[2] + ext[2]],
        },
    };
    o.subdivisions = 1 + u32::from(c.u8() % 6);
    o.tolerance = if mode & 8 != 0 { f64::from(c.unit()) } else { 0. };
    o.pixel_size = if mode & 16 != 0 { f64::from(c.unit()) } else { 0. };
    let p = || Positions::from_packed(&m.pos);

    let mut ws = Workspace::new(limits());
    let a = remove_hidden_triangles(&m.idx, p(), o, &mut ws);
    let usage = ws.usage();
    // Caller-buffer form: same result, untouched tail, untouched on error.
    let mut out = vec![u32::MAX; m.idx.len() + 3];
    let b = remove_hidden_triangles_into(&mut out, &m.idx, p(), o, &mut Workspace::new(limits()));
    match (&a, &b) {
        (Ok(a), Ok(r)) => {
            assert_eq!(a.report, *r);
            assert_eq!(&out[..r.index_count()], &a.indices[..]);
            assert!(out[r.index_count()..].iter().all(|&x| x == u32::MAX));
        }
        (Err(x), Err(y)) => {
            assert_eq!(x, y);
            assert!(out.iter().all(|&x| x == u32::MAX));
        }
        _ => panic!("alloc/into disagree: {a:?} vs {b:?}"),
    }
    // u16 widths agree with u32 (every index fits).
    let i16 = narrow(&m.idx);
    let w = remove_hidden_triangles_generic::<u16>(&i16, p(), o, &mut Workspace::new(limits()));
    let key = |r: &Result<_, Error>| match r {
        Ok((i, rep)) => format!("{i:?} {rep:?}"),
        Err(e) => format!("{e:?}"),
    };
    assert_eq!(
        key(&a.as_ref().map(|v| (v.indices.iter().map(|&i| i).collect::<Vec<u32>>(), v.report)).map_err(|e| *e)),
        key(&w.as_ref().map(|v| (v.indices.iter().map(|&i| u32::from(i)).collect::<Vec<u32>>(), v.report)).map_err(|e| *e)),
        "u16 differs from u32"
    );
    if let Ok(v) = &a {
        let r = v.report;
        assert_eq!(r.input_triangles, m.idx.len() / 3);
        assert_eq!(r.output_triangles * 3, v.indices.len());
        assert_eq!(r.input_triangles, r.output_triangles + r.removed());
        // Retained triangles are a sub-multiset of the input triangles.
        let mut left: std::collections::HashMap<[u32; 3], usize> = Default::default();
        for t in m.idx.chunks_exact(3) {
            *left.entry([t[0], t[1], t[2]]).or_default() += 1;
        }
        for t in v.indices.chunks_exact(3) {
            let n = left.get_mut(&[t[0], t[1], t[2]]).expect("retained triangle not in input");
            assert!(*n > 0);
            *n -= 1;
        }
        // Reused workspace and metering: the exact work succeeds, one less fails.
        let again = remove_hidden_triangles(&m.idx, p(), o, &mut ws).unwrap();
        assert_eq!(again.indices, v.indices);
        assert_eq!(ws.usage(), usage);
        if usage.work > 0 {
            let tight = Limits { max_bytes: 64 << 20, max_work: usage.work };
            let t = remove_hidden_triangles(&m.idx, p(), o, &mut Workspace::new(tight)).unwrap();
            assert_eq!(t.indices, v.indices);
            let less = Limits { max_work: usage.work - 1, ..tight };
            let e = remove_hidden_triangles(&m.idx, p(), o, &mut Workspace::new(less));
            assert_eq!(e.err(), Some(Error::LimitExceeded));
        }
    }
});
