#![no_main]
#[path = "../common.rs"]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::*;

/// Debug output of a u16 result must equal the u32 result's: same numbers, same errors.
fn same<A: std::fmt::Debug, B: std::fmt::Debug>(what: &str, a: A, b: B) {
    assert_eq!(format!("{a:?}"), format!("{b:?}"), "{what}: u16 differs from u32");
}


fn canon(t: &[u32]) -> [u32; 3] {
    let k = (0..3).min_by_key(|&k| t[k]).unwrap();
    [t[k], t[(k + 1) % 3], t[(k + 2) % 3]]
}
fn multiset(i: &[u32]) -> Vec<[u32; 3]> {
    let mut v: Vec<_> = i.chunks_exact(3).map(canon).collect();
    v.sort();
    v
}

fuzz_target!(|data: &[u8]| {
    let mut c = Cur::new(data);
    let m = mesh(&mut c, 200);
    let (x, y, z) = (c.u8(), c.u8(), c.u8());
    let nv = m.pos.len();
    let (i32s, i16s) = (&m.idx, narrow(&m.idx));
    let p = || Positions::from_packed(&m.pos);
    let w = || Workspace::new(Limits { max_bytes: 64 << 20, max_work: 4_000_000 });
    let settings = SimplifySettings {
        target_index_count: i32s.len() * usize::from(x) / 255,
        target_error: f32::from(y) / 255.,
        options: SimplifyOptions::from_bits(u32::from(z)).unwrap_or(SimplifyOptions::EMPTY),
    };
    same("simplify", simplify_generic::<u32>(i32s, p(), settings, &mut w()), simplify_generic::<u16>(&i16s, p(), settings, &mut w()));
    same("simplify_sloppy",
        simplify_sloppy_generic::<u32>(i32s, p(), None, settings.target_index_count, settings.target_error, &mut w()),
        simplify_sloppy_generic::<u16>(&i16s, p(), None, settings.target_index_count, settings.target_error, &mut w()));
    same("simplify_prune", simplify_prune_generic::<u32>(i32s, p(), settings.target_error, &mut w()), simplify_prune_generic::<u16>(&i16s, p(), settings.target_error, &mut w()));
    same("simplify_points", simplify_points_generic::<u32>(p(), None, 0., usize::from(x) % (nv + 1), &mut w()), simplify_points_generic::<u16>(p(), None, 0., usize::from(x) % (nv + 1), &mut w()));
    same("cache", optimize_vertex_cache_generic::<u32>(i32s, nv, &mut w()), optimize_vertex_cache_generic::<u16>(&i16s, nv, &mut w()));
    same("cache_fifo", optimize_vertex_cache_fifo_generic::<u32>(i32s, nv, 16, &mut w()), optimize_vertex_cache_fifo_generic::<u16>(&i16s, nv, 16, &mut w()));
    same("cache_strip", optimize_vertex_cache_strip_generic::<u32>(i32s, nv, &mut w()), optimize_vertex_cache_strip_generic::<u16>(&i16s, nv, &mut w()));
    same("overdraw", optimize_overdraw_generic::<u32>(i32s, p(), 1. + f32::from(x) / 255., &mut w()), optimize_overdraw_generic::<u16>(&i16s, p(), 1. + f32::from(x) / 255., &mut w()));
    same("sanitize", sanitize_generic::<u32>(i32s, p(), SanitizeOptions::EMPTY, &mut w()), sanitize_generic::<u16>(&i16s, p(), SanitizeOptions::EMPTY, &mut w()));
    same("analyze_cache", analyze_vertex_cache_generic::<u32>(i32s, nv, 16, 0, 0, &mut w()), analyze_vertex_cache_generic::<u16>(&i16s, nv, 16, 0, 0, &mut w()));
    same("orient", orient_faces_generic::<u32>(i32s, nv, &mut w()), orient_faces_generic::<u16>(&i16s, nv, &mut w()));
    same("fill_holes", fill_holes_generic::<u32>(i32s, p(), 1 + usize::from(x % 16), &mut w()), fill_holes_generic::<u16>(&i16s, p(), 1 + usize::from(x % 16), &mut w()));
    let s = MeshletSettings { max_vertices: 3 + usize::from(y) % 120, max_triangles: 1 + usize::from(z) % 124 };
    same("meshlets", build_meshlets_generic::<u32>(i32s, p(), s, 0.25, &mut w()), build_meshlets_generic::<u16>(&i16s, p(), s, 0.25, &mut w()));
    same("meshlets_spatial", build_meshlets_spatial_generic::<u32>(i32s, p(), s, 1, 0.5, &mut w()), build_meshlets_spatial_generic::<u16>(&i16s, p(), s, 1, 0.5, &mut w()));
    let bytes = bytes_of(&m.pos);
    if let Ok(vs) = VertexStream::packed(&bytes, 12) {
        same("split", split_non_manifold_vertices_generic::<u32>(i32s, vs, &mut w()), split_non_manifold_vertices_generic::<u16>(&i16s, vs, &mut w()));
        same("stitch", stitch_cracks_generic::<u32>(i32s, vs, p(), Some(f32::from(x) / 255.), &mut w()), stitch_cracks_generic::<u16>(&i16s, vs, p(), Some(f32::from(x) / 255.), &mut w()));
    }
    // Vertex cache optimizers reorder triangles without changing the set, and the
    // in-place forms equal the allocating forms.
    if i32s.len() % 3 == 0 && i32s.iter().all(|&i| (i as usize) < nv) {
        let want = multiset(i32s);
        for (what, r) in [
            ("cache", optimize_vertex_cache_generic::<u32>(i32s, nv, &mut w())),
            ("cache_fifo", optimize_vertex_cache_fifo_generic::<u32>(i32s, nv, 16, &mut w())),
            ("cache_strip", optimize_vertex_cache_strip_generic::<u32>(i32s, nv, &mut w())),
        ] {
            let r = r.unwrap_or_else(|e| panic!("{what} failed on valid input: {e:?}"));
            assert_eq!(multiset(&r), want, "{what} changed the triangle set");
        }
        let mut inplace = i32s.clone();
        optimize_vertex_cache_in_place(&mut inplace, nv, &mut w()).unwrap();
        assert_eq!(Ok(inplace), optimize_vertex_cache(i32s, nv, &mut w()));
    }
    let targets = [0.5f32, 0.25];
    let o = LodChainOptions::new(LodTargets::Ratios(&targets), LodErrors::Uniform(f32::from(y) / 255.))
        .with_simplify_options(settings.options);
    same("lod_chain", build_lod_chain_generic::<u32>(i32s, p(), &o, &mut w()), build_lod_chain_generic::<u16>(&i16s, p(), &o, &mut w()));
});
