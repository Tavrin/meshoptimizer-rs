#![no_main]
#[path = "../vg_support.rs"]
mod support;
use libfuzzer_sys::fuzz_target;
use meshoptimizer_rs::clusterlod::{
    attributes::Block,
    page::{self, PageView, QuantGrid},
};
fuzz_target!(|data: &[u8]| {
    if data.len() > 1 << 18 {
        return;
    }
    let words = support::words(data);
    let grid = QuantGrid::new(-10).unwrap();
    if let Ok(b) = Block::new(&words, words.get(3).copied().unwrap_or(0) as usize) {
        for k in 0..b.channel_count() {
            for v in 0..words[3] as usize {
                assert!(b.decode(k, v).unwrap().iter().all(|x| x.is_finite()));
            }
        }
    }
    if let Ok(v) = PageView::new(&words, grid) {
        support::snapshot(v);
    }
    if let Ok(n) = page::disk_page_memory_words(data) {
        if n <= page::MAX_PAGE_WORDS {
            let mut out = vec![0; n];
            if page::transcode_page(data, grid, &mut out, &mut support::ws()).is_ok() {
                support::snapshot(PageView::new(&out, grid).unwrap());
            }
        }
    }
    // Grid construction and snapping are bounded scalar decoder inputs too.
    let _ = QuantGrid::new(i32::from(support::byte(data, 0) as i8));
    let x = f32::from_bits(words.first().copied().unwrap_or(0));
    let _ = QuantGrid::from_max_error(x);
    let _ = QuantGrid::from_bounds([x; 3], [1.; 3], u32::from(support::byte(data, 1)));
    let _ = grid.snap_coordinate(x);
    for g in [
        QuantGrid::from_bounds([-1.; 3], [256.; 3], 20).unwrap(),
        QuantGrid::from_max_error(0.01).unwrap(),
    ] {
        let original = [[0.123, 1.456, -0.789], [255., 32., 0.]];
        let mut snapped = original;
        let displacement = g.snap_positions(&mut snapped).unwrap();
        assert!(displacement <= g.max_displacement());
        for (a, b) in original.iter().zip(snapped) {
            assert!(
                a.iter()
                    .zip(b)
                    .map(|(a, b)| (*a as f64 - b as f64).powi(2))
                    .sum::<f64>()
                    .sqrt()
                    <= g.max_displacement() as f64
            );
        }
    }
    support::attribute_fixture(data);
});
