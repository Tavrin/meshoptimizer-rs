fn random(s: &mut u64) -> u32 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s as u32
}
fn main() {
    let mut seed: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    for iteration in 0..100 {
        let n = 3 + random(&mut seed) as usize % 94;
        let p: Vec<[f32; 3]> = (0..n).map(|_| std::array::from_fn(|_| {
            if iteration % 7 == 0 { f32::from_bits(random(&mut seed)) }
            else { (random(&mut seed) as i32 % 100) as f32 * 0.125 }
        })).collect();
        let triangles = 1 + random(&mut seed) as usize % 160;
        let indices: Vec<u32> = (0..triangles * 3).map(|_| random(&mut seed) % n as u32).collect();
        let setting = [(3,1), (8,4), (64,124), (255,512)][random(&mut seed) as usize % 4];
        let variant = 4 + random(&mut seed) as usize % 4;
        let kind = 1 + random(&mut seed) as usize % 3;
        let stride = if kind == 1 { 4 } else { 64 };
        let offset = random(&mut seed) as usize % 2;
        let short = iteration % 2 == 0;
        let a = observe!(baseline, &p, &indices, variant, setting, kind, stride, offset, short, u64::MAX);
        let b = observe!(candidate, &p, &indices, variant, setting, kind, stride, offset, short, u64::MAX);
        assert_eq!(a, b, "unlimited iteration={iteration}");
        for limit in [0, a.1 / 2, a.1, a.1 + 1] {
            let a = observe!(baseline, &p, &indices, variant, setting, kind, stride, offset, short, limit);
            let b = observe!(candidate, &p, &indices, variant, setting, kind, stride, offset, short, limit);
            assert_eq!(a, b, "iteration={iteration} limit={limit}");
        }
    }
}
