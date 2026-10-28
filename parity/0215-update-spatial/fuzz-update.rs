fn random(s: &mut u64) -> u32 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s as u32
}
fn main() {
    let mut seed: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    for iteration in 0..100 {
        let n = 3 + random(&mut seed) as usize % 22;
        let mut c = mesh(random(&mut seed) as usize % 6, n);
        c.ac = [0, 1, 3, 8, 32][random(&mut seed) as usize % 5];
        c.weight = [0., 0.1, 1., 1.0e20, f32::from_bits(1)][random(&mut seed) as usize % 5];
        c.flags = iteration % 3 == 0;
        c.flag_values = (0..n).map(|_| (random(&mut seed) % 8) as u8).collect();
        c.layout = (random(&mut seed) % 8) as u8;
        c.attr_layout = (random(&mut seed) % 3) as u8;
        c.target = (random(&mut seed) as usize % (c.ix.len() / 3 + 1)) * 3;
        c.error = [0., 0.01, 1., f32::MAX][random(&mut seed) as usize % 4];
        for v in &mut c.p {
            for x in v {
                *x = if iteration % 7 == 0 { f32::from_bits(random(&mut seed)) }
                     else { (random(&mut seed) as i32 % 100) as f32 * 0.125 };
            }
        }
        let mut options = random(&mut seed) % 512;
        if baseline::SimplifyOptions::from_bits(options).is_err() { options = 0; }
        let a = observe!(baseline, &c, 2, 0, options, u64::MAX, 1 << 26, false, false);
        let b = observe!(candidate, &c, 2, 0, options, u64::MAX, 1 << 26, false, false);
        assert_eq!(a, b, "unlimited iteration={iteration}");
        let used = a.0.iter().map(|o| o.usage.0).max().unwrap();
        for limit in [0, used / 2, used, used + 1] {
            for lean in [false, true] {
                let a = observe!(baseline, &c, 2, 0, options, limit, 1 << 26, lean, true);
                let b = observe!(candidate, &c, 2, 0, options, limit, 1 << 26, lean, true);
                assert_eq!(a, b, "iteration={iteration} limit={limit}");
            }
        }
    }
}
